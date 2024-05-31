param (
    [int]$port = 5000
)

$portPid = Get-NetTCPConnection -LocalPort $port | Select-Object -ExpandProperty OwningProcess
if ($portPid) {
    Stop-Process -Id $portPid -Force
}

netsh interface portproxy delete v4tov4 listenport=$port listenaddress=0.0.0.0
netsh interface portproxy add v4tov4 listenport=$port listenaddress=0.0.0.0 connectport=$port connectaddress=127.0.0.1
