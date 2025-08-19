#include <unistd.h>
#include <fcntl.h>
#include <string.h>

int main() {
    int fd = open("output.txt", O_WRONLY | O_CREAT, 0644);
    if (fd == -1) {
        return 1; // Handle error
    }
    
    const char *message = "Hello, world!";
    ssize_t bytesWritten = write(fd, message, strlen(message));
    
    if (bytesWritten == -1) {
        return 1; // Handle error
    }
    
    close(fd);
    return 0;
}
