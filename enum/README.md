Programs for performing enumeration, likely the 1st stage of a process injection of an existing process.

pthandles - enumerate all system processes using NtGetNextProcess, then enum all threads with NtGetNextThread. 
injecthandle - its busted. basically I'm building a process injection based on pthandles initial enum. I'll move it to a new folder when its further along.
