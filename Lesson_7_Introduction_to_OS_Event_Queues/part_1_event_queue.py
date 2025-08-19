from collections import deque

class EventQueue:
    def __init__(self):
        self.queue = deque()

    def enqueue(self, event):
        self.queue.append(event)

    def dequeue(self):
        if not self.is_empty():
            return self.queue.popleft()
        return None

    def is_empty(self):
        return len(self.queue) == 0

# Example usage
event_queue = EventQueue()
event_queue.enqueue("Event 1")
event_queue.enqueue("Event 2")

print("Processing events...")
while not event_queue.is_empty():
    event = event_queue.dequeue()
    print(f"Processed: {event}")
