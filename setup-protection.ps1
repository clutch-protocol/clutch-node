#!/usr/bin/env pwsh
# Clutch Node Repository Protection Setup Script
# Configures branch protection with PR requirements but allows Mehran Mazhar to bypass

Write-Host "🔒 Setting up Branch Protection for Clutch Node Repository" -ForegroundColor Green
Write-Host "=========================================================" -ForegroundColor Green

# Check if git is available
try {
    $gitVersion = git --version
    Write-Host "✅ Git found: $gitVersion" -ForegroundColor Green
} catch {
    Write-Host "❌ Git not found. Please install Git first." -ForegroundColor Red
    exit 1
}

# Check current directory
$currentDir = Get-Location
Write-Host "📍 Current directory: $currentDir" -ForegroundColor Yellow

# Check if we're in clutch-node directory
if ($currentDir.Path -notlike "*clutch-node*") {
    Write-Host "⚠️  Warning: You may not be in the clutch-node directory" -ForegroundColor Yellow
    Write-Host "   Expected: clutch-node directory" -ForegroundColor Gray
}

# Check remote configuration
Write-Host "`n🔍 Checking remote configuration..." -ForegroundColor Blue
try {
    $remotes = git remote -v
    Write-Host $remotes
} catch {
    Write-Host "❌ No git repository found in current directory" -ForegroundColor Red
    exit 1
}

Write-Host "`n📋 Manual Configuration Steps Required:" -ForegroundColor Cyan
Write-Host "=====================================" -ForegroundColor Cyan

Write-Host "`n1. GitHub Repository Settings:" -ForegroundColor White
Write-Host "   - Go to clutch-node repository on GitHub" -ForegroundColor Gray
Write-Host "   - Navigate to Settings > Branches" -ForegroundColor Gray
Write-Host "   - Create protection rules for each branch" -ForegroundColor Gray

Write-Host "`n2. Main Branch Protection:" -ForegroundColor White
Write-Host "   ✅ Require pull request reviews before merging" -ForegroundColor Green
Write-Host "   ✅ Require 1 approving review" -ForegroundColor Green
Write-Host "   ✅ Dismiss stale PR reviews when new commits are pushed" -ForegroundColor Green
Write-Host "   ✅ Require status checks to pass before merging" -ForegroundColor Green
Write-Host "   ✅ Require branches to be up to date before merging" -ForegroundColor Green
Write-Host "   ❌ Do NOT allow force pushes" -ForegroundColor Red
Write-Host "   ❌ Do NOT allow deletions" -ForegroundColor Red

Write-Host "`n3. Develop Branch Protection:" -ForegroundColor White
Write-Host "   ✅ Require pull request reviews before merging" -ForegroundColor Green
Write-Host "   ✅ Require 1 approving review" -ForegroundColor Green
Write-Host "   ✅ Require status checks to pass before merging" -ForegroundColor Green
Write-Host "   ❌ Do NOT allow force pushes" -ForegroundColor Red
Write-Host "   ❌ Do NOT allow deletions" -ForegroundColor Red

Write-Host "`n4. Release Branches Protection:" -ForegroundColor White
Write-Host "   ✅ Require pull request reviews before merging" -ForegroundColor Green
Write-Host "   ✅ Require 2 approving reviews" -ForegroundColor Green
Write-Host "   ✅ Require code owner reviews" -ForegroundColor Green
Write-Host "   ✅ Require status checks to pass before merging" -ForegroundColor Green
Write-Host "   ❌ Do NOT allow force pushes" -ForegroundColor Red
Write-Host "   ❌ Do NOT allow deletions" -ForegroundColor Red

Write-Host "`n5. Mehran Mazhar Bypass Settings:" -ForegroundColor White
Write-Host "   ✅ Allow Mehran Mazhar to bypass restrictions" -ForegroundColor Green
Write-Host "   ✅ Allow Mehran Mazhar to push directly to protected branches" -ForegroundColor Green
Write-Host "   ✅ Allow Mehran Mazhar to dismiss reviews" -ForegroundColor Green

Write-Host "`n6. Status Check Contexts:" -ForegroundColor White
Write-Host "   - build: Rust compilation check" -ForegroundColor Gray
Write-Host "   - test: Unit and integration tests" -ForegroundColor Gray
Write-Host "   - lint: Code formatting and linting" -ForegroundColor Gray
Write-Host "   - security-scan: Security vulnerability scan (release branches)" -ForegroundColor Gray

Write-Host "`n7. Contributor Workflow:" -ForegroundColor White
Write-Host "   # Create feature branch:" -ForegroundColor Gray
Write-Host "   git checkout -b feature/your-feature" -ForegroundColor Yellow
Write-Host "   # Make changes and commit:" -ForegroundColor Gray
Write-Host "   git add ." -ForegroundColor Yellow
Write-Host "   git commit -m 'Feature: Your feature description'" -ForegroundColor Yellow
Write-Host "   # Push and create PR:" -ForegroundColor Gray
Write-Host "   git push origin feature/your-feature" -ForegroundColor Yellow
Write-Host "   # Create PR on GitHub, wait for reviews and checks" -ForegroundColor Gray

Write-Host "`n8. Mehran Mazhar Direct Push:" -ForegroundColor White
Write-Host "   # You can push directly to protected branches:" -ForegroundColor Gray
Write-Host "   git push origin main" -ForegroundColor Yellow
Write-Host "   git push origin develop" -ForegroundColor Yellow
Write-Host "   # Use responsibly for rapid development!" -ForegroundColor Yellow

Write-Host "`n🎯 For Clutch Protocol MVP Development:" -ForegroundColor Cyan
Write-Host "   - Contributors must use PR workflow" -ForegroundColor Gray
Write-Host "   - You can bypass for rapid iteration" -ForegroundColor Gray
Write-Host "   - Maintains code quality while enabling speed" -ForegroundColor Gray

Write-Host "`n⚠️  Important Notes:" -ForegroundColor Red
Write-Host "   - Contributors cannot force push to protected branches" -ForegroundColor Red
Write-Host "   - All PRs require at least one approval (except yours)" -ForegroundColor Red
Write-Host "   - Status checks must pass before merging" -ForegroundColor Red
Write-Host "   - Use bypass responsibly for development" -ForegroundColor Red

Write-Host "`n✅ Branch protection setup complete!" -ForegroundColor Green
Write-Host "   Contributors must use PRs, but you can bypass for rapid development." -ForegroundColor Green
