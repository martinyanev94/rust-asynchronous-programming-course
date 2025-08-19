#include <windows.h>
#include <stdio.h>

DWORD WINAPI ThreadProc(LPVOID lpParameter) {
    HANDLE completionPort = (HANDLE)lpParameter;
    OVERLAPPED ov;
    DWORD bytesTransferred;

    while (GetQueuedCompletionStatus(completionPort, &bytesTransferred, NULL, NULL, INFINITE)) {
        printf("I/O Operation completed with %lu bytes\n", bytesTransferred);
    }
    return 0;
}

int main() {
    HANDLE completionPort = CreateIoCompletionPort(INVALID_HANDLE_VALUE, NULL, 0, 0);
    // Additional initialization...

    // Assuming there are I/O operations initiated...

    // Create worker threads
    for (int i = 0; i < 4; i++) {
        CreateThread(NULL, 0, ThreadProc, completionPort, 0, NULL);
    }

    // Wait for threads to finish...
    
    CloseHandle(completionPort);
    return 0;
}
