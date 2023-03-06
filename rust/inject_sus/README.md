Spawns a process in a suspended state (in the POC its notepad), then uses the following API calls to inject shellcode (POC is calc) for execution:
- VirtualAllocEx
- WriteProcessMemory
- VirtualProtectEx
- GetThreadContext
- SetThreadContext
- ResumeThread

This is a proof of concept, as are all the tools in this repo. You absolutely should modify them for additional OPSEC before using on OPs. Better yet, just snip the code that you need, or read it for learning about exploitation, and then use only what you need or build off it as a foundation.

Usage:

Building and testing binary: (this image also shows the suspended process before shellcode injection)
![image](https://user-images.githubusercontent.com/105792760/223000917-fb7d57f7-f1d9-452e-8d1d-35e71b18edc2.png)

Shellcode execution: (note how this method does *not* hollow the original process but actually replaces it in tasklist)
![image](https://user-images.githubusercontent.com/105792760/223001084-5e6f7090-0536-443e-9924-5eec6143bdc2.png)
