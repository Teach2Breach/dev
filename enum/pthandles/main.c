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

    typedef struct _PROCESS_BASIC_INFORMATION {
    NTSTATUS ExitStatus;
    PPEB PebBaseAddress;
    ULONG_PTR AffinityMask;
    KPRIORITY BasePriority;
    ULONG_PTR UniqueProcessId;
    ULONG_PTR InheritedFromUniqueProcessId;
} PROCESS_BASIC_INFORMATION;

#define STATUS_INFO_LENGTH_MISMATCH 0xC0000004

typedef NTSTATUS (NTAPI *PNtQueryInformationProcess)(
HANDLE ProcessHandle,
PROCESSINFOCLASS ProcessInformationClass,
PVOID ProcessInformation,
ULONG ProcessInformationLength,
PULONG ReturnLength
);

#define STATUS_SUCCESS ((NTSTATUS)0x00000000L)

typedef NTSTATUS (NTAPI *PNtQueryInformationThread)(
HANDLE ThreadHandle,
THREADINFOCLASS ThreadInformationClass,
PVOID ThreadInformation,
ULONG ThreadInformationLength,
PULONG ReturnLength
);

typedef struct _THREAD_BASIC_INFORMATION {
    NTSTATUS ExitStatus;
    PVOID TebBaseAddress;
    CLIENT_ID ClientId;
    ULONG_PTR AffinityMask;
    KPRIORITY Priority;
    KPRIORITY BasePriority;
} THREAD_BASIC_INFORMATION, *PTHREAD_BASIC_INFORMATION;

    // Variables to store the process and thread handles
    HANDLE process_handle = NULL, thread_handle = NULL;
    DWORD pid;
    char filename[MAX_PATH];

    // Loop through all processes
    while (NtGetNextProcess(process_handle, MAXIMUM_ALLOWED, 0, 0, &process_handle) == 0)
    {

        printf("\tProcess Handle: %#x\n", process_handle);

        //Use NtQueryInformationProcess to get the process name and pid
        //https://docs.microsoft.com/en-us/windows/win32/api/winternl/nf-winternl-ntqueryinformationprocess
        //https://docs.microsoft.com/en-us/windows/win32/api/winternl/ns-winternl-_process_basic_information

        PNtQueryInformationProcess NtQueryInformationProcess =
        (PNtQueryInformationProcess)GetProcAddress(GetModuleHandleA("ntdll.dll"),
        "NtQueryInformationProcess");

        //get the pid and PebBaseAddress using NtQueryInformationProcess
        PROCESS_BASIC_INFORMATION pbi;
        NtQueryInformationProcess(process_handle, 0, &pbi, sizeof(pbi), NULL);
        
        //store the pid
        pid = pbi.UniqueProcessId;
        //store the PebBaseAddress
        PPEB peb = pbi.PebBaseAddress;
        
        //print the pid
        printf("\tProcess ID: %d\n", pid);
        //print the PebBaseAddress
        printf("\tPebBaseAddress: %#x\n", peb);

        //define buffer to pass to GetModuleFileNameExW
        wchar_t buffer[MAX_PATH];
        //get the filename
        GetModuleFileNameExW(process_handle, NULL, buffer, MAX_PATH);
        //convert the filename to char  
        wcstombs(filename, buffer, MAX_PATH);

        // Print the process ID and executable file name
        printf("\tFull Path: %s\n", filename);

        //strip the filename to just the executable name
        char *p = strrchr(filename, '\\');
        if (p != NULL)
        {
            p++;
        }
        //print the executable name
        printf("\tExecutable Name: %s\n", p);

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

    // Get the thread's memory address
    DWORD_PTR memoryAddress = threadContext.Rip;

    // print threadContext.Rip
    printf("\t\tThreadContext.Rip: %#x\n", memoryAddress);

    // Call NtQueryInformationThread to get the TEB address
    PNtQueryInformationThread NtQueryInformationThread =
    (PNtQueryInformationThread)GetProcAddress(GetModuleHandleA("ntdll.dll"),
    "NtQueryInformationThread");

    //define buffer to pass to NtQueryInformationThread
    THREAD_BASIC_INFORMATION tbi;

    //get the TEB address
    NtQueryInformationThread(thread_handle, 0, &tbi, sizeof(tbi), NULL);

    //print the thread priority
    printf("\t\tThread Priority: %d\n", tbi.Priority);
    //print the TEB address
    printf("\t\tTEB Address: %#x\n", tbi.TebBaseAddress);

    // Allocate a buffer to hold the thread entry point
    PVOID threadEntryPoint = NULL;
    ULONG tbufferSize = sizeof(threadEntryPoint);
    PVOID tbuffer = VirtualAlloc(NULL, tbufferSize, MEM_COMMIT, PAGE_READWRITE);

    // call NtQueryInformmationThread with ThreadQuerySetWin32StartAddress to get the thread entry point
    NTSTATUS status = NtQueryInformationThread(thread_handle, 9, tbuffer, tbufferSize, NULL);
    // Check if the call was successful
    if (NT_SUCCESS(status)) {
    // The thread entry point is stored as a pointer to a function
    threadEntryPoint = (PVOID)tbuffer;
    // Print the thread entry point
    printf("Thread Entry Point: 0x%p\n", threadEntryPoint);
    } else {
    printf("NtQueryInformationThread failed with status %x\n", status);
    }

    VirtualFree(buffer, tbufferSize, MEM_RELEASE);

        
        }
    }

    return 0;
}
