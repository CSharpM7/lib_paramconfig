$cargoContents = Get-Content -Path ..\cargo.toml
$pluginName = $cargoContents[1].replace('name = ','').replace('"','')
$pluginNRO = "target/aarch64-skyline-switch/release/lib"+$pluginName+".nro"
$newName = "libparam_config.nro"
$emuLocation = Get-Item -Path ..\..\emulocation.txt | Get-Content -Tail 1

function Get-TimeStamp {
    return "[{0:MM/dd/yy} {0:HH:mm:ss}]" -f (Get-Date)
}
function PluginWasBuilt {
    return Test-Path $pluginNRO
}
if (PluginWasBuilt) {Remove-Item $pluginNRO}

If (!($args[0] -like "*emu*")) {
    If ($args[0] -like "*dev*") {
        cargo skyline install --install-path rom:/smashline/development.nro
        Write-Output "$(Get-TimeStamp) Installed dev plugin"
    }
    else {
        cargo skyline install --install-path rom:/skyline/plugins/$newName
        Write-Output "$(Get-TimeStamp) Installed plugin"
    }
}
else {
    cargo skyline build --release

    if (!(PluginWasBuilt)) {return}

    $newLocation = $emuLocation+"sdmc/atmosphere/contents/01006A800016E000/romfs/skyline/plugins/"
    if (Test-Path $newLocation$newName) {Remove-Item $newLocation$newName}
    Move-Item -Path $pluginNRO -Destination $newLocation$newName -Force
    Write-Output "$(Get-TimeStamp) Installed plugin to emulator"

    #Invoke-Item $newLocation
}