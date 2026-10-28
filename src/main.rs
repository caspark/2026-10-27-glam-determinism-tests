use std::error::Error;
use std::path::{Path, PathBuf};
use std::process::{Command, ExitCode, Stdio};

const EXAMPLES: &[&str] = &[
    "zero_selection",
    "vec4_reductions",
    "vec4_product",
    "vec4_length",
    "normalization",
    "cross_product",
    "monotonic_lerp",
    "mat3a_inverse",
    "mat4_inverse",
    "mat4_transpose",
    "affine_inverse",
    "quaternion_mul",
    "quaternion_slerp",
    "zero_padding",
    "zero_rounding",
];

type Result<T> = std::result::Result<T, Box<dyn Error>>;
type Values = Vec<(String, String)>;

#[derive(Clone, Copy)]
enum Target {
    Native,
    Wasm,
    WasmSimd,
}

impl Target {
    fn name(self) -> &'static str {
        match self {
            Self::Native => "native",
            Self::Wasm => "wasm",
            Self::WasmSimd => "wasm-simd",
        }
    }

    fn target_dir(self, root: &Path) -> PathBuf {
        match self {
            Self::Native => root.join("target"),
            Self::Wasm => root.join("target/wasm-scalar"),
            Self::WasmSimd => root.join("target/wasm-simd"),
        }
    }
}

fn main() -> ExitCode {
    match run() {
        Ok(different) => ExitCode::from(u8::from(different)),
        Err(error) => {
            eprintln!("{error}");
            ExitCode::from(2)
        }
    }
}

fn run() -> Result<bool> {
    let mut args: Vec<_> = std::env::args().skip(1).collect();
    if args.iter().any(|arg| arg == "--help" || arg == "-h") {
        println!("Usage: cargo run --release -- [compare|native|wasm|wasm-simd] [example] [--scalar-math]");
        println!("Default: compare all examples, first with libm, then with libm + scalar-math.");
        println!("--scalar-math runs only the scalar-math pass. Other modes print one target's results.");
        println!("Exit codes: 0 = equal/success, 1 = compared results differ, 2 = build/runtime error.");
        println!("Examples: {}", EXAMPLES.join(", "));
        return Ok(false);
    }
    let scalar_only = cfg!(feature = "scalar-math") || args.iter().any(|arg| arg == "--scalar-math");
    args.retain(|arg| arg != "--scalar-math");
    if args.len() > 2 {
        return Err("Expected a mode and an optional example name. Use --help.".into());
    }
    let mode = args.first().map(String::as_str).unwrap_or("compare");
    let targets: &[Target] = match mode {
        "compare" => &[Target::Native, Target::Wasm, Target::WasmSimd],
        "native" => &[Target::Native],
        "wasm" => &[Target::Wasm],
        "wasm-simd" => &[Target::WasmSimd],
        _ => return Err(format!("Unknown mode: {mode}. Use --help.").into()),
    };
    let names: Vec<&str> = match args.get(1) {
        Some(name) if EXAMPLES.contains(&name.as_str()) => vec![name],
        Some(name) => return Err(format!("Unknown example: {name}. Use --help.").into()),
        None => EXAMPLES.to_vec(),
    };
    let root = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    let passes: &[bool] = if scalar_only { &[true] } else { &[false, true] };
    let mut different = false;
    for &scalar in passes {
        for &target in targets {
            build(&root, &names, target, scalar)?;
        }
        println!("\nglam 0.34.1: libm{}", if scalar { " + scalar-math" } else { "" });
        print!("{:<48}", "example/value");
        for target in targets {
            print!(" {:>16}", target.name());
        }
        println!();
        let mut unequal = 0;
        let mut rows = 0;
        for &name in &names {
            let values = targets.iter().map(|&target| example_result(&root, name, target)).collect::<Result<Vec<_>>>()?;
            for column in &values[1..] {
                if column.len() != values[0].len() || column.iter().zip(&values[0]).any(|(a, b)| a.0 != b.0) {
                    return Err(format!("{name}: result labels differ between targets").into());
                }
            }
            for (i, (label, _)) in values[0].iter().enumerate() {
                print!("{:<48}", format!("{name}/{label}"));
                for column in &values {
                    print!(" {:>16}", column[i].1);
                }
                if values.iter().any(|column| column[i].1 != values[0][i].1) {
                    print!("  DIFF");
                    unequal += 1;
                }
                println!();
                rows += 1;
            }
        }
        if mode == "compare" {
            println!("{unequal}/{rows} result rows differ");
        }
        different |= unequal != 0;
    }
    Ok(different)
}

fn build(root: &Path, names: &[&str], target: Target, scalar: bool) -> Result<()> {
    let mut command = Command::new(std::env::var_os("CARGO").unwrap_or_else(|| "cargo".into()));
    command.current_dir(root)
        .args(["build", "--locked", "--quiet", "--release", "--target-dir"])
        .arg(target.target_dir(root))
        .env_remove("RUSTFLAGS")
        .env_remove("CARGO_ENCODED_RUSTFLAGS")
        .env_remove("CARGO_BUILD_TARGET");
    if !matches!(target, Target::Native) {
        command.args(["--target", "wasm32-wasip1"]);
    }
    if matches!(target, Target::WasmSimd) {
        command.env("RUSTFLAGS", "-C target-feature=+simd128");
    }
    if scalar {
        command.args(["--features", "scalar-math"]);
    }
    for name in names {
        command.args(["--example", name]);
    }
    let status = command.stdout(Stdio::null()).status()?;
    if !status.success() {
        return Err(format!("{} example build failed: {status}", target.name()).into());
    }
    Ok(())
}

fn example_result(root: &Path, name: &str, target: Target) -> Result<Values> {
    let mut command = if matches!(target, Target::Native) {
        Command::new(target.target_dir(root).join("release/examples").join(format!("{name}{}", std::env::consts::EXE_SUFFIX)))
    } else {
        let mut command = Command::new("wasmtime");
        command.arg(target.target_dir(root).join("wasm32-wasip1/release/examples").join(format!("{name}.wasm")));
        command
    };
    let output = command.output().map_err(|error| {
        let hint = if matches!(target, Target::Native) { "" } else { ". Wasm modes require the Wasmtime CLI on PATH" };
        format!("{name}/{}: {error}{hint}", target.name())
    })?;
    if !output.status.success() {
        return Err(format!("{name}/{}: {}\n{}", target.name(), output.status, String::from_utf8_lossy(&output.stderr)).into());
    }
    let mut values: Values = Vec::new();
    for line in std::str::from_utf8(&output.stdout)?.lines() {
        let fields: Vec<_> = line.split_whitespace().collect();
        if fields.len() != 2 || !matches!(fields[1].len(), 8 | 16) || u64::from_str_radix(fields[1], 16).is_err() {
            return Err(format!("{name}: expected a label and 8 or 16 hexadecimal digits, got {line:?}").into());
        }
        if values.iter().any(|(label, _)| label == fields[0]) {
            return Err(format!("{name}: duplicate result label {}", fields[0]).into());
        }
        values.push((fields[0].into(), fields[1].into()));
    }
    if values.is_empty() {
        return Err(format!("{name}: example produced no results").into());
    }
    Ok(values)
}
