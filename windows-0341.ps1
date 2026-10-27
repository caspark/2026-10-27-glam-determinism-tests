$ErrorActionPreference = 'Stop'
$ProgressPreference = 'SilentlyContinue'
Set-Location $PSScriptRoot
rustc -vV
if ($LASTEXITCODE -ne 0) { exit $LASTEXITCODE }
$env:CARGO_HTTP_TIMEOUT = '30'
$env:CARGO_NET_RETRY = '1'
cargo fetch --locked --manifest-path v0.34.1/Cargo.toml
if ($LASTEXITCODE -ne 0) { exit $LASTEXITCODE }
Write-Output '=== Windows 0.34.1 default ==='
cargo run --locked --offline --release --manifest-path v0.34.1/Cargo.toml --target-dir build | Tee-Object -FilePath windows-0341-default.txt
if ($LASTEXITCODE -ne 0) { exit $LASTEXITCODE }
Write-Output '=== Windows 0.34.1 scalar ==='
cargo run --locked --offline --release --manifest-path v0.34.1/Cargo.toml --target-dir build --features glam/scalar-math | Tee-Object -FilePath windows-0341-scalar.txt
if ($LASTEXITCODE -ne 0) { exit $LASTEXITCODE }
if ((Get-Content windows-0341-default.txt) -contains 'runtime_fma=true') {
    Write-Output '=== Windows 0.34.1 +fma ==='
    $env:RUSTFLAGS = '-C target-feature=+fma'
    cargo run --locked --offline --release --manifest-path v0.34.1/Cargo.toml --target-dir build | Tee-Object -FilePath windows-0341-fma.txt
    if ($LASTEXITCODE -ne 0) { exit $LASTEXITCODE }
    Write-Output '=== Windows 0.34.1 fma-scalar ==='
    cargo run --locked --offline --release --manifest-path v0.34.1/Cargo.toml --target-dir build --features glam/scalar-math | Tee-Object -FilePath windows-0341-fma-scalar.txt
    if ($LASTEXITCODE -ne 0) { exit $LASTEXITCODE }
}
