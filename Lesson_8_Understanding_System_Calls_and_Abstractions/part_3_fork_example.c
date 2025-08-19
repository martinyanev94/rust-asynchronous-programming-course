#include <stdio.h>
#include <unistd.h>

int main() {
    pid_t pid = fork();

    if (pid < 0) {
        perror("Fork failed");
        return 1; // An error occurred
    } 
    else if (pid == 0) {
        // Child process
        printf("Hello from child process with PID: %d\n", getpid());
    } 
    else {
        // Parent process
        printf("Hello from parent process with PID: %d, child PID: %d\n", getpid(), pid);
    }

    return 0;
}
