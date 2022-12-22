#include <windows.h>
#include <stdio.h>
#include "humanerror.h"

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
