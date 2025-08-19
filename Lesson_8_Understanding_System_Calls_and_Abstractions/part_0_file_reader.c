#include <stdio.h>
#include <unistd.h>
#include <fcntl.h>

int main() {
    int fd;
    char buffer[100];
    
    fd = open("example.txt", O_RDONLY);
    if (fd == -1) {
        perror("Error opening file");
        return 1;
    }
    
    ssize_t bytesRead = read(fd, buffer, sizeof(buffer) - 1);
    if (bytesRead == -1) {
        perror("Error reading file");
        close(fd);
        return 1;
    }
    
    buffer[bytesRead] = '\0'; // Null-terminate the string
    printf("Read %zd bytes: %s\n", bytesRead, buffer);
    
    close(fd);
    return 0;
}
