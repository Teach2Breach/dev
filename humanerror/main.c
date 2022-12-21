#include <windows.h>
#include <stdio.h>

// Struct that maps error codes to error messages
typedef struct {
  DWORD error_code;
  char* error_message;
} error_map_t;

// Array of error codes and messages
error_map_t error_map[] = {
  {0, "The operation completed successfully."},
  {1, "Incorrect function."},
  {2, "The system cannot find the file specified."},
  {3, "The system cannot find the path specified."},
  {4, "The system cannot open the file."},
  {5, "Access is denied."},
  {6, "The handle is invalid."},
  {7, "The storage control blocks were destroyed."},
  {8, "Not enough storage is available to process this command."},
  {9, "The storage control block address is invalid."},
  {10, "The environment is incorrect."},
  {11, "An attempt was made to load a program with an incorrect format."},
  {12, "The access code is invalid."},
  {13, "The data is invalid."},
  {14, "Not enough storage is available to complete this operation."},
  {15, "The system cannot find the drive specified."},
  {16, "The directory cannot be removed."},
  {17, "The system cannot move the file to a different disk drive."},
  {18, "There are no more files."},
  {19, "The media is write-protected."},
  {20, "The system cannot find the device specified."},
  {21, "The device is not ready."},
  {22, "The device does not recognize the command."},
  {23, "Data error (cyclic redundancy check)."},
  {24, "The program issued a command but the command length is incorrect."},
  {25, "The drive cannot locate a specific area or track on the disk."},
  {26, "The specified disk or diskette cannot be accessed."},
  {27, "The drive cannot find the sector requested."},
  {28, "The printer is out of paper."},
  {29, "The system cannot write to the specified device."},
  {30, "The system cannot read from the specified device."},
  {31, "A device attached to the system is not functioning."},
  {32, "The process cannot access the file because it is being used by another process."},
  {33, "The process cannot access the file because another process has locked a portion of the file."},
  {34, "The wrong diskette is in the drive. Insert %2 (Volume Serial Number: %3) into drive %1."},
  {36, "Too many files opened for sharing."},
  {38, "Reached the end of the file."},
  {39, "The disk is full."},
  {50, "The request is not supported."},
  {51, "Windows cannot find the network path. Verify that the network path is correct and the destination computer is not busy or turned off. If Windows still cannot find the network path, contact your network administrator."},
  {52, "You were not connected because a duplicate name exists on the network. If joining a domain, go to System in Control Panel to change the computer name and try again. If joining a workgroup, choose another workgroup name."},
  {53, "The network path was not found."},
  {54, "The network is busy."},
  {55, "The specified network resource or device is no longer available."},
  {56, "The network BIOS command limit has been reached."},
  {57, "A network adapter hardware error occurred."},
  {58, "The specified server cannot perform the requested operation."},
  {59, "An unexpected network error occurred."},
  {60, "The remote adapter is not compatible."},
  {61, "The printer queue is full."},
  {62, "Space to store the file waiting to be printed is not available on the server."},
  {63, "Your file waiting to be printed was deleted."},
  {64, "The specified network name is no longer available."},
  {65, "Network access is denied."},
  {66, "The network resource type is not correct."},
  {67, "The network name cannot be found."},
  {68, "The name limit for the local computer network adapter card was exceeded."},
  {69, "The network BIOS session limit was exceeded."},
  {70, "The remote server has been paused or is in the process of being started."},
  {71, "No more connections can be made to this remote computer at this time because there are already as many connections as the computer can accept."},
  {72, "The specified printer or disk device has been paused."},
  {80, "The file exists."},
  {82, "The directory or file cannot be created."},
  {83, "Fail on INT 24."},
  {84, "Storage to process this request is not available."},
  {85, "The local device name is already in use."},
  {86, "The specified network password is not correct."},
  {87, "The parameter is incorrect."},
  {88, "A write fault occurred on the network."},
  {89, "The system cannot start another process at this time."},
  {100, "Cannot create another system semaphore."},
  {101, "The exclusive semaphore is owned by another process."},
  {102, "The semaphore is set and cannot be closed."},
  {103, "The semaphore timeout period has expired."},
  {104, "The semaphore has been abandoned."},
  {105, "The semaphore was released while waiting."},
  {106, "Cannot request exclusive semaphores at interrupt time."},
  {107, "The previous ownership of this semaphore has ended."},
  {108, "Insert the diskette for drive %1."},
  {109, "The program stopped because an alternate diskette was not inserted."},
  {110, "The disk is in use or locked by another process."},
  {111, "The pipe has been ended."},
  {112, "The system cannot open the device or file specified."},
  {113, "The file name is too long."},
  {114, "Cannot request a METAFILEPICT handle from the display."},
  {117, "The RPC server is too busy to complete this operation."},
  {118, "No more bindings."},
  {119, "The system cannot start another process at this time."},
  {120, "There are no child processes to wait for."},
  {121, "The %1 application cannot be run in Win32 mode."},
  {122, "Attempt to use a file handle to an open disk partition for an operation other than raw disk I/O."},
  {123, "The filename, directory name, or volume label syntax is incorrect."},
  {124, "The system call level is not correct."},
  {125, "The disk has no volume label."},
  {126, "The specified module could not be found."},
  {127, "The specified procedure could not be found."},
  {128, "There are no child processes to wait for."},
  {129, "The %1 application cannot be run in Win32 mode."},
  {130, "Attempt to use a file handle to an open file for an operation other than raw disk I/O."},
  {131, "The caller now needs to enumerate the files to find the changes."},
  {132, "The specified invariant Culture Name ID is not supported on this operating system."},
  {133, "The specified Culture Name ID is not supported on this operating system."},
  {134, "The operating system is not presently configured to run this application."},
  {135, "The operating system is not presently configured to run this application."},
  {136, "The operating system is not presently configured to run this application."},
  {137, "The operating system is not presently configured to run this application."},
  {138, "The operating system is not presently configured to run this application."},
  {139, "The operating system is not presently configured to run this application."},
  {140, "The operating system is not presently configured to run this application."},
  {141, "The operating system is not presently configured to run this application."},
  {142, "The operating system is not presently configured to run this application."},
  {143, "The operating system is not presently configured to run this application."},
  {144, "The operating system is not presently configured to run this application."},
  {145, "The operating system is not presently configured to run this application."},
  {146, "The operating system is not presently configured to run this application."},
  {147, "The operating system is not presently configured to run this application."},
  {148, "The operating system is not presently configured to run this application."},
  {149, "The operating system is not presently configured to run this application."},
  {150, "The operating system is not presently configured to run this application."},
  {151, "The operating system is not presently configured to run this application."},
  {152, "The operating system is not presently configured to run this application."},
  {153, "The operating system is not presently configured to run this application."},
  {154, "The operating system is not presently configured to run this application."},
  {155, "The operating system is not presently configured to run this application."},
  // Add more error codes and messages here as needed
};

// Number of elements in the error map array
#define NUM_ERROR_CODES (sizeof(error_map) / sizeof(error_map_t))

int main() {
    // Open a file that does not exist
    HANDLE file_handle = CreateFile("does_not_exist.txt", GENERIC_READ, 0, NULL, OPEN_EXISTING, FILE_ATTRIBUTE_NORMAL, NULL);
  // Get the error code from GetLastError
  DWORD error_code = GetLastError();

  // Look up the error message in the error map
  char* error_message = NULL;
  for (int i = 0; i < NUM_ERROR_CODES; i++) {
    if (error_map[i].error_code == error_code) {
      error_message = error_map[i].error_message;
      break;
    }
  }

  // If the error message was not found in the map, use a default message
  if (error_message == NULL) {
    error_message = "Unknown error.";
  }

  // Print the error code and error message
  printf("Windows error code %d: %s\n", error_code, error_message);

  return 0;
}

