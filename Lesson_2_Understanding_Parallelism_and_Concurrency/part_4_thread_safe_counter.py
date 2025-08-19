import threading

shared_counter = 0
lock = threading.Lock()

def increment_counter():
    global shared_counter
    for _ in range(1000000):
        with lock:
            shared_counter += 1

threads = []
for i in range(2):
    thread = threading.Thread(target=increment_counter)
    threads.append(thread)
    thread.start()

for thread in threads:
    thread.join()

print(f"Final counter value: {shared_counter}")
