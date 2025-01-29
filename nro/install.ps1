function Get-TimeStamp {
    
    return "[{0:MM/dd/yy} {0:HH:mm:ss}]" -f (Get-Date)
    
}
If ($args[0] -like "*dev*") {
    cargo skyline install --install-path rom:/smashline/development.nro
    Write-Output "$(Get-TimeStamp) Installed dev plugin"
}
else{
    cargo skyline install --install-path rom:/skyline/plugins/libparam_config.nro
    Write-Output "$(Get-TimeStamp) Installed plugin"
}