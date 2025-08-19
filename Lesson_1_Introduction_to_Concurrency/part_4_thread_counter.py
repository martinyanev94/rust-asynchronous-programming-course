import threading
import time

lock = threading.Lock()

def increment_shared_counter(counter):
    for _ in range(5):
        time.sleep(0.1)
        with lock:
            counter[0] += 1
            print(f"Counter value: {counter[0]}")

if __name__ == "__main__":
    shared_counter = [0]
    thread1 = threading.Thread(target=increment_shared_counter, args=(shared_counter,))
    thread2 = threading.Thread(target=increment_shared_counter, args=(shared_counter,))
    
    thread1.start()
    thread2.start()
    
    thread1.join()
    thread2.join()
    
    print(f"Final counter value: {shared_counter[0]}")
