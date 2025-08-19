#include <stdio.h>
#include <unistd.h>
#include <fcntl.h>
#include <string.h>

int main() {
    int fd = open("output.txt", O_WRONLY | O_CREAT, 0644);
    if (fd == -1) {
        perror("Error opening file");
        return 1; // Handle error
    }
    
    const char *message = "Hello, with error handling!";
    ssize_t bytesWritten = write(fd, message, strlen(message));
    
    if (bytesWritten == -1) {
        perror("Error writing to file");
        close(fd);
        return 1; // Handle error
    }
    
    close(fd);
    return 0;
}
