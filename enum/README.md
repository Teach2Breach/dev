Programs for performing enumeration, likely the 1st stage of a process injection of an existing process.

pthandles - enumerate all system processes using NtGetNextProcess, then enum all threads with NtGetNextThread. 
dupehandle - duplicate a handle with minimum required access for being able to pass to VirtualAllocEx.
allocatechecker - identify every process on the system which you can allocate memory to.
