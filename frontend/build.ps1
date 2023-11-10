param (
    [Boolean]$prod
)

$Env:PROD = "false"
$tailwind = ""
$trunk = ""

if ($prod) {
    $Env:PROD = "true"
    $tailwind = "--minify"
    $trunk = "--release"
}

$cmd = [string]::Format("tailwind -o .\css\tailwind.css {0} && trunk build {1}",$tailwind,$trunk)
Write-Output $cmd
cargo watch -q -w . -s $cmd
