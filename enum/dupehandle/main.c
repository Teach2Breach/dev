// Include the necessary header files
#include <stdio.h>
#include <windows.h>

int main()
{

    // Declare a HANDLE variable to hold the handle to the process
    // that we want to check
    HANDLE hProcess;

    // Use the OpenProcess function to get a handle to the process
    // that we want to check. In this case, we are checking the
    // current process.
    hProcess = OpenProcess(PROCESS_ALL_ACCESS, FALSE, GetCurrentProcessId());

    // Check if the handle is valid
    if (hProcess == NULL)
    {
        // If the handle is not valid, display an error message
        printf("Error: Unable to get handle to process!\n");
        return 1;
    }

    //print the handle
    printf("Original handle: %#x \n", hProcess);

    // Request the PROCESS_VM_OPERATION access right.
    DWORD desiredAccess = PROCESS_VM_OPERATION;
    BOOL inheritable = FALSE;

    // Duplicate the handle to the process.
    HANDLE hProcessWithAccess;
    if (!DuplicateHandle(GetCurrentProcess(), hProcess, GetCurrentProcess(),
                        &hProcessWithAccess, desiredAccess, inheritable,
                        DUPLICATE_SAME_ACCESS)) {
        // Handle the error.
    } else {
        // You now have a handle to the process with the PROCESS_VM_OPERATION
        // access right, which you can use to allocate memory in the process.
    }

    //print the new handle
    printf("New handle: %#x \n", hProcessWithAccess);

    // Close the handle to the process
    CloseHandle(hProcess);
    CloseHandle(hProcessWithAccess);

    return 0;
}
