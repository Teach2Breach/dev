#include <windows.h>
#include <stdio.h>

int main()
{
    // Define variables for the process creation flags
    DWORD dwCreationFlags = CREATE_SUSPENDED;

    // Define the structure for the new process
    STARTUPINFO si;
    ZeroMemory(&si, sizeof(si));
    si.cb = sizeof(si);

    // Define the structure for the new process's primary thread
    PROCESS_INFORMATION pi;
    ZeroMemory(&pi, sizeof(pi));

    // Create the new process in a suspended state
    if (!CreateProcess("C:\\Windows\\System32\\notepad.exe", NULL, NULL, NULL, FALSE, dwCreationFlags, NULL, NULL, &si, &pi))
    {
        printf("CreateProcess failed (%d).\n", GetLastError());
        return 1;
    }

    // The new process has been created in a suspended state, so you can do any additional setup or processing here before resume it

    // Resume the suspended process
    if (!ResumeThread(pi.hThread))
    {
        printf("ResumeThread failed (%d).\n", GetLastError());
        return 1;
    }

    // Close the handles to the new process and its primary thread
    CloseHandle(pi.hProcess);
    CloseHandle(pi.hThread);

    return 0;
}
