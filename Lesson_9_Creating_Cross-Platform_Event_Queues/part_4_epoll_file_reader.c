#include <sys/epoll.h>
#include <fcntl.h>
#include <stdio.h>
#include <unistd.h>

int main() {
    int epoll_fd = epoll_create1(0);
    struct epoll_event event;

    int fd = open("example.txt", O_RDONLY);
    event.events = EPOLLIN;
    event.data.fd = fd;

    epoll_ctl(epoll_fd, EPOLL_CTL_ADD, fd, &event);

    while (1) {
        struct epoll_event events[10];
        int nfds = epoll_wait(epoll_fd, events, 10, -1);
        
        for (int i = 0; i < nfds; ++i) {
            if (events[i].events & EPOLLIN) {
                printf("File descriptor %d is ready for reading\n", events[i].data.fd);
            }
        }
    }

    close(fd);
    return 0;
}
