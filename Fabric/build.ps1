$ErrorActionPreference = "Stop"

$build_dir = "build"
$target_dir = "target"

if (-not (Test-Path $build_dir)) {
    New-Item -ItemType Directory -Path $build_dir | Out-Null
}

if (-not (Test-Path $target_dir)) {
    New-Item -ItemType Directory -Path $target_dir | Out-Null
}

Write-Host "Building Rust core..."
Set-Location src/core
cargo build --release
Copy-Item "target/release/fabric.dll" "../../$target_dir/" -Force
Set-Location ../..

Write-Host "Building C# wrapper..."
$csc = "C:\Windows\Microsoft.NET\Framework64\v4.0.30319\csc.exe"
& $csc /target:library /out:$target_dir\Fabric.dll /unsafe wrapper/Fabric.cs

Write-Host "Build complete. Output files are in the $target_dir directory."
