#include <sys/epoll.h>
#include <unistd.h>

int main() {
    int epoll_fd = epoll_create1(0);
    struct epoll_event event;
    int nfds;

    // Add a file descriptor to monitor
    event.events = EPOLLIN; // Watch for input
    event.data.fd = some_fd; // Replace with your file descriptor
    epoll_ctl(epoll_fd, EPOLL_CTL_ADD, some_fd, &event);

    while (1) {
        struct epoll_event events[10];
        nfds = epoll_wait(epoll_fd, events, 10, -1); // Wait indefinitely
        
        for (int i = 0; i < nfds; i++) {
            // Process the event here
        }
    }
    close(epoll_fd);
    return 0;
}
