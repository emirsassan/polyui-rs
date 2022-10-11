# Get ci parameter to check if running with ci
param(
    [Parameter()]
    [Switch]$ci
)

# Get temp folder
$temp = [System.IO.Path]::GetTempPath()

# Get current running dir
$currentLocation = $((Get-Location).path)

# Check to see if a command exists (eg if an app is installed)
Function CheckCommand {

   Param ($command)

   $oldPreference = $ErrorActionPreference

   $ErrorActionPreference = 'stop'

   try { if (Get-Command $command) { RETURN $true } }

   Catch { RETURN $false }

   Finally { $ErrorActionPreference = $oldPreference }

}

Write-Host "PolyUI Development Environment Setup" -ForegroundColor Magenta
Write-Host @"

To set up your machine for PolyUI development, this script will do the following:

1) Check for Rust and Cargo

2) Install pnpm (if not installed)

3) Install the latest version of Node.js using pnpm

4) Install LLVM (compiler for ffmpeg-rust)

4) Download ffmpeg and set as an environment variable

"@ 

Write-Host "Checking for Rust and Cargo..." -ForegroundColor Yellow
Start-Sleep -Milliseconds 150

$cargoCheck = CheckCommand cargo

if ($cargoCheck -eq $false) {
   Write-Host @"
Cargo is not installed.

To use PolyUI on Windows, Cargo needs to be installed.
The Visual Studio C++ Build tools are also required.
Instructions can be found here:

https://tauri.app/v1/guides/getting-started/prerequisites/#setting-up-windows

Once you have installed Cargo, re-run this script.

"@
   Exit
}
else {
   Write-Host "Cargo is installed."
}

Write-Host
Write-Host "Checking for pnpm..." -ForegroundColor Yellow
Start-Sleep -Milliseconds 150

$pnpmCheck = CheckCommand pnpm
if ($pnpmCheck -eq $false) {

   Write-Host "pnpm is not installed. Installing now."
   Write-Host "Running the pnpm installer..."

   #pnpm installer taken from https://pnpm.io
   Invoke-WebRequest https://get.pnpm.io/install.ps1 -useb | Invoke-Expression

   # Reset the PATH env variables to make sure pnpm is accessible 
   $env:PNPM_HOME = [System.Environment]::GetEnvironmentVariable("PNPM_HOME", "User")
   $env:Path = [System.Environment]::ExpandEnvironmentVariables([System.Environment]::GetEnvironmentVariable("Path", "User"))

}
else {
   Write-Host "pnpm is installed."
}

# A GitHub Action takes care of installing node, so this isn't necessary if running in the ci.
if ($ci -eq $True) {
   Write-Host
   Write-Host "Running with Ci, skipping Node install." -ForegroundColor Yellow
}
else {
   Write-Host
   Write-Host "Using pnpm to install the latest version of Node..." -ForegroundColor Yellow
   Write-Host "This will set your global Node version to the latest!"
   Start-Sleep -Milliseconds 150

   # Runs the pnpm command to use the latest version of node, which also installs it
   Start-Process -Wait -FilePath "pnpm" -ArgumentList "env use --global latest" -PassThru -Verb runAs
}



# The ci has LLVM installed already, so we instead just set the env variables.
if ($ci -eq $True) {
   Write-Host
   Write-Host "Running with Ci, skipping LLVM install." -ForegroundColor Yellow

   $VCINSTALLDIR = $(& "${env:ProgramFiles(x86)}\Microsoft Visual Studio\Installer\vswhere.exe" -latest -property installationPath)
   Add-Content $env:GITHUB_ENV "LIBCLANG_PATH=${VCINSTALLDIR}\VC\Tools\LLVM\x64\bin`n"

} else {
   Write-Host
   Write-Host "Downloading the LLVM installer..." -ForegroundColor Yellow
   # Downloads latest installer for LLVM
   $filenamePattern = "*-win64.exe"
   $releasesUri = "https://api.github.com/repos/llvm/llvm-project/releases/latest"
   $downloadUri = ((Invoke-RestMethod -Method GET -Uri $releasesUri).assets | Where-Object name -like $filenamePattern ).browser_download_url

   Start-BitsTransfer -Source $downloadUri -Destination "$temp\llvm.exe"

   Write-Host
   Write-Host "Running the LLVM installer..." -ForegroundColor Yellow
   Write-Host "Please follow the instructions to install LLVM."
   Write-Host "Ensure you add LLVM to your PATH."

   Start-Process "$temp\llvm.exe" -Wait
}

Write-Host
Write-Host "Your machine has been setup for PolyUI development!"