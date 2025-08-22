# PowerShell script for building and pushing Docker images locally

param(
    [string]$Tag = "latest",
    [string]$Repository = "9194010019/clutch-node",
    [switch]$Push,
    [switch]$NoBuild
)

Write-Host "🐳 Clutch Node Docker Build Script" -ForegroundColor Cyan
Write-Host "=================================" -ForegroundColor Cyan

$ImageName = "${Repository}:${Tag}"

if (-not $NoBuild) {
    Write-Host "🔨 Building Docker image: $ImageName" -ForegroundColor Yellow
    
    docker build -t $ImageName .
    
    if ($LASTEXITCODE -ne 0) {
        Write-Error "❌ Docker build failed!"
        exit 1
    }
    
    Write-Host "✅ Build completed successfully!" -ForegroundColor Green
}

if ($Push) {
    Write-Host "🚀 Pushing to Docker Hub: $ImageName" -ForegroundColor Yellow
    
    # Check if user is logged in to Docker Hub
    $loginCheck = docker info 2>&1 | Select-String "Username"
    if (-not $loginCheck) {
        Write-Host "⚠️  You may need to login to Docker Hub first:" -ForegroundColor Yellow
        Write-Host "   docker login" -ForegroundColor Gray
    }
    
    docker push $ImageName
    
    if ($LASTEXITCODE -ne 0) {
        Write-Error "❌ Docker push failed!"
        exit 1
    }
    
    Write-Host "✅ Push completed successfully!" -ForegroundColor Green
    Write-Host "🔗 Image available at: https://hub.docker.com/r/$Repository" -ForegroundColor Cyan
}

Write-Host ""
Write-Host "📋 Available commands:" -ForegroundColor White
Write-Host "  Test locally:    docker run --rm -p 8081:8081 $ImageName" -ForegroundColor Gray
Write-Host "  Run with config: docker run --rm -p 8081:8081 -v `${PWD}/config:/usr/src/clutch-node/config $ImageName" -ForegroundColor Gray
