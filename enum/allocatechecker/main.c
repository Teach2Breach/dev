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

    //declare an array for holding the pids of processes that pass the duplicate_handle check
    DWORD pid_array[100];
    int i = 0;

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
        printf("\tProcess Handle: %#x\n", process_handle);

        //get the thread_handle of the process
        //thread_handle = OpenThread(THREAD_ALL_ACCESS, FALSE, tid);

        // Loop through all threads of the current process
        while (NtGetNextThread(process_handle, thread_handle, THREAD_ALL_ACCESS, 0, 0, &thread_handle) == 0)
        {
            // Print the thread handle
            printf("\tThread Handle: %#x\n", thread_handle);

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

    //print the memory address of the thread entry point
    printf("Thread entry point: %#p \n", threadContext.Rip);

        }

        //attempt to duplicate the process handle to get a new handle with the PROCESS_VM_OPERATION permission
        HANDLE new_process_handle;
        BOOL duplicate_handle = DuplicateHandle(GetCurrentProcess(), process_handle, GetCurrentProcess(), &new_process_handle, PROCESS_VM_OPERATION, FALSE, 0);
        if (!duplicate_handle)
        {
            printf("Failed to duplicate handle.\n");
            continue;
        }
        else
        {
            printf("Successfully duplicated handle for process pid: %d\n", pid);
            //add to the array of pids
            pid_array[i] = pid;
            i++;
        }



    }   

    //print the filenames which duplicate_handle succeeded for
    printf("The following processes have the PROCESS_VM_OPERATION permission:\n");
    for (int j = 0; j < i; j++)
    {
        //print just the unique pids
        if (pid_array[j] != pid_array[j + 1])
        {
            printf("pid: %d\n", pid_array[j]);
        }
    }

    return 0;
}
