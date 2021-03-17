# Determine the path of the latest installed Windows 10 SDK by sorting the names of the directories as if they are version objects
$latestSdkPath = Get-ChildItem "C:\Program Files (x86)\Windows Kits\10\bin" -Filter "10.*" | Sort "{[version] $_}" | Select-Object -Last 1
$signToolExe = $latestSdkPath.FullName + "\x64\signtool.exe"

& $signToolExe sign /n "Open Source Developer, Joergen Tjernoe" /t http://time.certum.pl/ /fd sha1 /v @args