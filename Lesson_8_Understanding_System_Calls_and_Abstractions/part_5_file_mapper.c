#include <sys/mman.h>
#include <fcntl.h>
#include <unistd.h>
#include <stdio.h>
#include <stdlib.h>

int main() {
    int fd = open("example.txt", O_RDONLY);
    if (fd == -1) {
        perror("Error opening file");
        return 1;
    }

    size_t length = lseek(fd, 0, SEEK_END); // Get the file size
    lseek(fd, 0, SEEK_SET); // Reset file offset

    char *mapped = mmap(NULL, length, PROT_READ, MAP_PRIVATE, fd, 0);
    if (mapped == MAP_FAILED) {
        perror("Error mapping file");
        close(fd);
        return 1;
    }

    printf("Mapped content: %.*s\n", (int)length, mapped);
    munmap(mapped, length); // Unmap the file
    close(fd);
    return 0;
}
