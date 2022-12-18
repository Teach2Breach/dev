Programs for performing enumeration, likely the 1st stage of a process injection of an existing process. Be aware, some of these may be WIP and not work or not be OPSEC safe. I'm not responsible for what you do with this information, including if you kill your beacon or whatever. However, I will label tools that use OPSEC focused techniques and avoid common APIs, etc... in **Bold** . Make of that what you will.

**pthandles** - enumerate all system processes using NtGetNextProcess, then enum all threads with NtGetNextThread. Has also been updated to remove common functions and replace with NtQueryInformationProcess and NtQueryInformationThread. 

dupehandle - duplicate a handle with minimum required access for being able to pass to VirtualAllocEx.

allocatechecker - identify every process on the system which you can allocate memory to.
