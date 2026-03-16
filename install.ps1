# Term Note 安装脚本 (Windows)
$ErrorActionPreference = "Stop"

$repo = "你的用户名/term-note"
$releaseUrl = "https://api.github.com/repos/$repo/releases/latest"

Write-Host "⬇️ 正在获取最新版本信息..." -ForegroundColor Cyan

# 获取最新版本
try {
    $release = Invoke-RestMethod -Uri $releaseUrl -Method Get
    $version = $release.tag_name
    Write-Host "✅ 找到最新版本: $version" -ForegroundColor Green
} catch {
    Write-Host "❌ 获取版本信息失败" -ForegroundColor Red
    exit 1
}

# 查找 Windows 资源
$asset = $release.assets | Where-Object { $_.name -like "*windows*.zip" }
if (-not $asset) {
    Write-Host "❌ 未找到 Windows 版本" -ForegroundColor Red
    exit 1
}

$downloadUrl = $asset.browser_download_url
$tempZip = "$env:TEMP\term-note.zip"
$installDir = "$env:LOCALAPPDATA\term-note"

# 下载
Write-Host "⬇️ 正在下载 $version..." -ForegroundColor Cyan
Invoke-WebRequest -Uri $downloadUrl -OutFile $tempZip -UseBasicParsing

# 解压
Write-Host "📦 正在解压..." -ForegroundColor Cyan
if (Test-Path $installDir) {
    Remove-Item -Recurse -Force $installDir
}
Expand-Archive -Path $tempZip -DestinationPath $installDir -Force
Remove-Item $tempZip

# 添加到 PATH
$exePath = "$installDir\term-note"
$currentPath = [Environment]::GetEnvironmentVariable("Path", "User")
if ($currentPath -notlike "*$exePath*") {
    Write-Host "🔧 正在添加到 PATH..." -ForegroundColor Cyan
    [Environment]::SetEnvironmentVariable("Path", "$currentPath;$exePath", "User")
}

Write-Host ""
Write-Host "✅ 安装完成！" -ForegroundColor Green
Write-Host "   版本: $version"
Write-Host "   位置: $exePath\term-note.exe"
Write-Host ""
Write-Host "使用方法:" -ForegroundColor Yellow
Write-Host "   term-note        # 启动应用"
Write-Host "   term-note --help # 显示帮助"
Write-Host ""
Write-Host "请重新打开终端以使用命令" -ForegroundColor Cyan
