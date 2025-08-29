Param(
    [string]$ImageName = "zamarine/kernel:6.8.12",
    [string]$ContainerName = "zamarine-kernel",
    [string]$Dockerfile = "docker/Dockerfile.kernel",
    [string]$OutDir = "out/kernel"
)

$ErrorActionPreference = "Stop"

docker version | Out-Null

$repoRoot = Split-Path -Parent $MyInvocation.MyCommand.Path
Set-Location (Split-Path -Parent $repoRoot)

New-Item -ItemType Directory -Force -Path $OutDir | Out-Null

Write-Host "Building kernel image $ImageName..."
docker build -t $ImageName -f $Dockerfile . | cat

Write-Host "Compiling kernel (this may take a long time)..."
docker rm -f $ContainerName 2>$null | Out-Null
docker run --name $ContainerName --rm -t \
  -v ${PWD}/$OutDir:/out \
  $ImageName | cat

Write-Host "Kernel packages in $OutDir:" 
Get-ChildItem $OutDir


