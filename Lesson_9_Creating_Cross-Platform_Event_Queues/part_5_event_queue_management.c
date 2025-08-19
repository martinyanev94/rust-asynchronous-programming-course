#ifdef _WIN32
#include <windows.h>
#define EventQueue HANDLE
#define CreateEventQueue() CreateIoCompletionPort(INVALID_HANDLE_VALUE, NULL, 0, 0)
#define AddToEventQueue(eqp, fd) CreateIoCompletionPort(fd, eqp, (ULONG_PTR)fd, 0)

#elif __APPLE__
#include <sys/event.h>
#include <sys/types.h>
#define EventQueue int
#define CreateEventQueue() kqueue()
#define AddToEventQueue(eqp, fd) EV_SET(&change, fd, EVFILT_READ, EV_ADD | EV_ENABLE, 0, 0, NULL)

#else
#include <sys/epoll.h>
#define EventQueue int
#define CreateEventQueue() epoll_create1(0)
#define AddToEventQueue(eqp, fd) epoll_ctl(eqp, EPOLL_CTL_ADD, fd, &event)
#endif

void ProcessEvents(EventQueue eq) {
    // Platform-independent processing logic
}
