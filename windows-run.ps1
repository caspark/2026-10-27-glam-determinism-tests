$ErrorActionPreference = 'Stop'
$ProgressPreference = 'SilentlyContinue'
Set-Location $PSScriptRoot
rustc -vV
if ($LASTEXITCODE -ne 0) { exit $LASTEXITCODE }
cargo --version
if ($LASTEXITCODE -ne 0) { exit $LASTEXITCODE }
Write-Output "RUSTFLAGS=$env:RUSTFLAGS"
Write-Output "CARGO_ENCODED_RUSTFLAGS=$env:CARGO_ENCODED_RUSTFLAGS"
foreach ($version in @('0.30.10', '0.33.12')) {
    $manifest = "v$version/Cargo.toml"
    Write-Output "=== glam $version / Windows default backend ==="
    cargo run --locked --offline --release --manifest-path $manifest --target-dir build
    if ($LASTEXITCODE -ne 0) { exit $LASTEXITCODE }
    Write-Output "=== glam $version / Windows scalar-math ==="
    cargo run --locked --offline --release --manifest-path $manifest --target-dir build --features glam/scalar-math
    if ($LASTEXITCODE -ne 0) { exit $LASTEXITCODE }
}
