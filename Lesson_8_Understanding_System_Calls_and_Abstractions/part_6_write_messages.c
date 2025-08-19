#include <stdio.h>
#include <fcntl.h>
#include <unistd.h>

int main() {
    int fd = open("output.txt", O_WRONLY | O_CREAT, 0644);
    
    if (fd == -1) {
        perror("Error opening file");
        return 1;
    }

    const char *messages[] = {
        "First message.\n",
        "Second message.\n",
        "Third message.\n"
    };

    for (int i = 0; i < 3; i++) {
        write(fd, messages[i], strlen(messages[i]));
    }
    
    close(fd);
    return 0;
}
