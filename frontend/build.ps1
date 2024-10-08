param (
    [Boolean]$p
)

$Env:PROD = "false"
$tailwind = ""
$trunk = ""

if ($p) {
    $Env:PROD = "true"
    $tailwind = "--minify"
    $trunk = "--release --public-url /"
}

$cmd = [string]::Format("tailwind -o .\css\tailwind.css {0} && trunk build {1}",$tailwind,$trunk)
Write-Output $cmd
cargo watch -q -w . -s $cmd
