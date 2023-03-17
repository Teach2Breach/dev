# Process Snapshot Shellcode Injection
This program uses the Windows Process Snapshot API to capture a snapshot of a target process and injects shellcode into the process using a new handle created by duplicating the process handle.

API Functions
This program utilizes the following API functions:

**PssCaptureSnapshot:** Captures a snapshot of a target process and its state, including its threads and handles.<BR>
**PssQuerySnapshot:** Queries a snapshot of a target process and its state, including its threads and handles.<BR>
**PssFreeSnapshot:** Frees a snapshot of a target process and its state, including its threads and handles.<BR>
**VirtualAllocEx:** Allocates memory within the virtual address space of the target process.<BR>
**WriteProcessMemory:** Writes data to the memory of a target process.<BR>
**VirtualProtectEx:** Changes the protection settings of a region of memory in the target process.<BR>
**CreateRemoteThread:** Creates a thread in the target process to execute the shellcode.<BR>
**DuplicateHandle:** Creates a new handle with the desired access rights and a new process handle to the target process.<BR><BR>
Shellcode:
The program includes a shellcode that launches the Windows Calculator (calc.exe) using assembly code.

Usage:<BR>
Compile the program using the following command:<BR>

```gcc.exe main.c -o snapinject.exe -l ntdll```<BR>
The program takes the name of the target process as a command line argument. 
 <BR><BR> 
 For example, to inject shellcode into RuntimeBroker.exe, run the following command:
<BR><BR>
```.\PSSnapshot.exe <process_name>```<BR>
The program will search for a process with a matching filename and inject the shellcode into the process. If successful, the shellcode will execute and launch the Calculator application.<BR><BR>
  
  *As always, this is a Proof of Concept, to highlight techniques you can use for maldev. In this case, I am focusing on using PSSCaptureSnapshot API to create a snapshot of a target process and show how you might then inject into the snapshot process. Note that the snapshot process is a clone process that gets created in a suspended state.<BR><BR>
  It is not completely OPSEC friendly. Although the enum used to obtain the process handle with NtGetNextProcess and DuplicateHandle are stealthy, I am using a really standard injection after that. It also has some obvious gotchas, like the shellcode being in plaintext in the program and not encrypted at compile time or anything. So, as usual, learn from the code and use what you like. It's not meant to be ready to compile and run on target.<BR><BR>
  Fun note though... Even in its shitty state, only the shellcode trips Defender :)*<BR><BR>
  Also, the target <process_name> is case sensitive. I'm too tired to fix it right now. Maybe later. Enjoy.
