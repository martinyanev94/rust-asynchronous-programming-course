#include <sys/event.h>
#include <sys/types.h>
#include <fcntl.h>
#include <stdio.h>
#include <unistd.h>

int main() {
    int kq = kqueue();
    struct kevent change;
    
    // Assuming we have a file descriptor (fd)
    int fd = open("example.txt", O_RDONLY);

    EV_SET(&change, fd, EVFILT_READ, EV_ADD | EV_ENABLE, 0, 0, NULL);

    while (1) {
        struct kevent event;
        int nev = kevent(kq, &change, 1, &event, 1, NULL);
        
        if (nev > 0) {
            printf("File descriptor %d is ready for reading\n", event.ident);
        }
    }

    close(fd);
    return 0;
}
