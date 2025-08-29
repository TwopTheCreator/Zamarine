Param(
    [string]$ImageName = "zamarine/build:bookworm",
    [string]$ContainerName = "zamarine-build",
    [string]$Dockerfile = "docker/Dockerfile",
    [switch]$BuildKernelFirst
)

$ErrorActionPreference = "Stop"

# Ensure Docker is available
docker version | Out-Null

$repoRoot = Split-Path -Parent $MyInvocation.MyCommand.Path
Set-Location (Split-Path -Parent $repoRoot)

Write-Host "Building Docker image $ImageName..."
docker build -t $ImageName -f $Dockerfile . | cat

if ($BuildKernelFirst) {
  Write-Host "Building kernel before ISO..."
  .\scripts\build-kernel.ps1
}

Write-Host "Running container to build ISO..."
docker rm -f $ContainerName 2>$null | Out-Null
docker run --name $ContainerName --rm -t \
  -v ${PWD}:/workspace \
  -e LB_DOWNLOADS=/workspace/out/downloads \
  $ImageName | cat

Write-Host "Copying ISO artifacts..."
New-Item -ItemType Directory -Force -Path out | Out-Null
# Artifacts already in /workspace/out via bind mount
Get-ChildItem out

Write-Host "Done. ISO should be in the 'out' directory."


