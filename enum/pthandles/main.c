#include <stdio.h>
#include <windows.h>
#include <psapi.h>
#include <winternl.h>
#include <winnt.h>

int main()
{

    NTSTATUS NTAPI NtGetNextProcess(
    HANDLE ProcessHandle,
    ACCESS_MASK DesiredAccess,
    ULONG HandleAttributes,
    ULONG Flags,
    PHANDLE NewProcessHandle
    );

NTSTATUS NTAPI NtGetNextThread(
    HANDLE ProcessHandle,
    HANDLE ThreadHandle,
    ACCESS_MASK DesiredAccess,
    ULONG HandleAttributes,
    ULONG Flags,
    PHANDLE NewThreadHandle
    );

    // Variables to store the process and thread handles
    HANDLE process_handle = NULL, thread_handle = NULL;
    DWORD pid;
    char filename[MAX_PATH];

    // Loop through all processes
    while (NtGetNextProcess(process_handle, MAXIMUM_ALLOWED, 0, 0, &process_handle) == 0)
    {
        // Get the process ID and executable file name
        pid = GetProcessId(process_handle);
        //define buffer to pass to GetModuleFileNameExW
        wchar_t buffer[MAX_PATH];
        //get the filename
        GetModuleFileNameExW(process_handle, NULL, buffer, MAX_PATH);
        //convert the filename to char  
        wcstombs(filename, buffer, MAX_PATH);

        // Print the process ID and executable file name
        printf("\tFile Name: %s\n", filename);
        printf("\tProcess ID: %d\n", pid);
        printf("\tProcess Handle: %d\n", process_handle);

        //get the thread_handle of the process
        //thread_handle = OpenThread(THREAD_ALL_ACCESS, FALSE, tid);

        // Loop through all threads of the current process
        while (NtGetNextThread(process_handle, thread_handle, THREAD_ALL_ACCESS, 0, 0, &thread_handle) == 0)
        {
            // Print the thread handle
            printf("\tThread Handle: %d\n", thread_handle);

            //check memory permissions
            // Get the thread's context information, including the memory address
    CONTEXT threadContext;
    threadContext.ContextFlags = CONTEXT_FULL;
    BOOL success = GetThreadContext(thread_handle, &threadContext);
    if (!success)
    {
        printf("Failed to get thread context.\n");
        //CloseHandle(thread_handle);
        //return 1;
        continue;
    }
    // Query the memory region using the thread handle and memory address
    MEMORY_BASIC_INFORMATION memInfo;
    SIZE_T bytesRead = VirtualQueryEx(process_handle, (LPCVOID)threadContext.Rip, &memInfo, sizeof(memInfo));
    if (bytesRead == 0)
    {
        printf("Failed to query memory region.\n");
        //CloseHandle(thread_handle);
        //return 1;
        continue;
    }
    printf("Memory region base address: %p\n", memInfo.BaseAddress);
    //printf("Memory region allocation base: %p\n", memInfo.AllocationBase);
    printf("Memory region size: %d\n", memInfo.RegionSize);
    printf("Memory region protect: %x\n", memInfo.Protect);
    printf("Memory region alloc protect: %ld\n", memInfo.AllocationProtect);

    // Check the permissions of the memory region
    if (memInfo.Protect & (80 | 40 | 04))
    {
        printf("BINGO BANGO! Memory region is writeable.\n");
    }
    else
    {
        printf("Memory region is not writeable.\n");
    }

        }
    }

    return 0;
}


