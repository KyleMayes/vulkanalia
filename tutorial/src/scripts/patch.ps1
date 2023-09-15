if (-NOT ($args.Length -eq 3)) {
    Write-Host "Expected original file, patched file, and prefix number to start applying patch to."
    exit
}

$original = $args[0]
$patched = $args[1]
$start = [int]$args[2]

& "C:\Program Files\Git\usr\bin\diff.exe" -Naur $original $patched | Out-File patch.patch

Get-ChildItem . -Filter *.rs | Sort-Object | Foreach-Object {
    $prefix = [int]$_.Name.Split("_")[0]
    if ($prefix -ge $start -and $_.Name -ne $patched) {
        Get-Content patch.patch | & "C:\Program Files\Git\usr\bin\patch.exe" -f $_.Name
        Remove-Item -ErrorAction Ignore ($_.Name + ".orig")
        Remove-Item -ErrorAction Ignore ($_.Name + ".rej")
    }
}

Remove-Item -ErrorAction Ignore patch.patch
