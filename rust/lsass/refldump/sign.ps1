#convert the following script to work from any desktop
$dir= Read-Host -Prompt 'Certs output directory: '
$pw= Read-Host -Prompt 'password: '
$rde=Read-Host -Prompt 'binary location: '
$cn=Read-Host -Prompt 'issuer to spoof: '

#add a check to locate the needed binaries instead of hardcoding this
cd 'C:\Program Files (x86)\Windows Kits\10\bin\10.0.22000.0\x64'

rm $dir\$cn*

.\makecert -n "CN=$cn" -r -pe -a sha512 -cy authority -sv $dir\$cn.pvk $dir\$cn.cer

.\pvk2pfx.exe -pvk $dir\$cn.pvk -spc $dir\$cn.cer -pfx $dir\$cn.pfx -po $pw

.\signtool.exe sign /fd sha512 /f $dir\$cn.pfx /p $pw $rde 

.\signtool.exe timestamp /t http://timestamp.digicert.com $rde

cd '~'
