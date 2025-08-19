import threading
import time

def task(name, duration):
    print(f"Task {name} started")
    time.sleep(duration)
    print(f"Task {name} completed after {duration} seconds")

if __name__ == "__main__":
    thread1 = threading.Thread(target=task, args=("A", 3))
    thread2 = threading.Thread(target=task, args=("B", 2))
    
    thread1.start()
    thread2.start()
    
    thread1.join()
    thread2.join()
    
    print("All tasks completed.")
