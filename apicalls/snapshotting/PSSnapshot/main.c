#include <windows.h>
#include <stdio.h>
//include the header file for the PSSCAPTURESNAPSHOT function, located in ./include/ProcessSnapshot.h
#include "ProcessSnapshot.h"


// compile with mingw: \gcc.exe -o PSSnapshot.exe main.c
// you may need the ProcessSnapshot.h file in current directory. Even though it ships with Windows SDK, it is difficult to include.
// so I pulled the header file from MS repo and just included manually in same dir.

int main(int argc, char **argv) { 

    // Declare variables 
    DWORD hSnapshot; 
    DWORD dwPID;

    //check for command-line arguments and print help if there are none
    if (argc < 2) {
        //strip the path from the executable name
        char *p = strrchr(argv[0], '\\');
        if (p == NULL) {
            p = argv[0];
        } else {
            p++;
        }
        //print the usage with just the executable name
        printf("Usage: %s <PID> \n", p);
        printf("    or %s 0 (to capture the snapshot of the current process) \n", p);
        return 1;
    }

    //convert the command-line argument to a DWORD
    dwPID = atoi(argv[1]);

    //if PID is set to 0, get PID of current process
    if (dwPID == 0) {
        dwPID = GetCurrentProcessId();
    }

    //get the PID from the command-line argument
    printf("PID: %d \n", dwPID);

//get handle to process
HANDLE hProcess = OpenProcess(PROCESS_ALL_ACCESS, TRUE, dwPID);
//print handle to process 
printf("Process handle: 0x%08X\n", hProcess);

//define snapshothandle for PssCaptureSnapshot, which should be a *HPSS
HPSS snapshothandle = NULL;

//define flags to pass to PssCaptureSnapshot and include the following flags: PSS_CAPTURE_VA_CLONE | PSS_CAPTURE_HANDLES | PSS_CAPTURE_HANDLE_NAME_INFORMATION | PSS_CAPTURE_HANDLE_BASIC_INFORMATION | PSS_CAPTURE_HANDLE_TYPE_SPECIFIC_INFORMATION | PSS_CAPTURE_HANDLE_TRACE | PSS_CAPTURE_THREADS | PSS_CAPTURE_THREAD_CONTEXT | PSS_CAPTURE_THREAD_CONTEXT_EXTENDED | PSS_CREATE_BREAKAWAY | PSS_CREATE_BREAKAWAY_OPTIONAL | PSS_CREATE_USE_VM_ALLOCATIONS | PSS_CREATE_RELEASE_SECTION

PSS_CAPTURE_FLAGS flags = PSS_CAPTURE_VA_CLONE | PSS_CAPTURE_HANDLES | PSS_CAPTURE_HANDLE_NAME_INFORMATION | PSS_CAPTURE_HANDLE_BASIC_INFORMATION | PSS_CAPTURE_HANDLE_TYPE_SPECIFIC_INFORMATION | PSS_CAPTURE_HANDLE_TRACE | PSS_CAPTURE_THREADS | PSS_CAPTURE_THREAD_CONTEXT | PSS_CAPTURE_THREAD_CONTEXT_EXTENDED | PSS_CREATE_BREAKAWAY | PSS_CREATE_BREAKAWAY_OPTIONAL | PSS_CREATE_USE_VM_ALLOCATIONS | PSS_CREATE_RELEASE_SECTION;

// Capture the snapshot
hSnapshot = PssCaptureSnapshot(hProcess, flags, CONTEXT_ALL, &snapshothandle);

//return the error code from hSnapshot
//printf("Error code: %d \n", hSnapshot);

// Check if the snapshot was captured successfully
if (hSnapshot == 0 && snapshothandle != NULL) {
    printf("Snapshot captured successfully! \n");
} else {
    printf("Snapshot capture failed! \n");
}

//dereference the snapshothandle pointer
//HANDLE shandle = *snapshothandle;
HANDLE shandle = snapshothandle;

// print the handle to the snapshot
//printf("Snapshot handle: %d \n", shandle);

//print the handle to the snapshot in hex
printf("Snapshot handle: 0x%08X\n", shandle);

// Close the snapshot
PssFreeSnapshot(hProcess, snapshothandle);

// Close the process handle
CloseHandle(hProcess);

return 0;
}
