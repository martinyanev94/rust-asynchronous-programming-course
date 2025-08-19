import asyncio
import queue
import random

class TaskScheduler:
    def __init__(self):
        self.task_queue = queue.PriorityQueue()

    def add_task(self, priority, task):
        self.task_queue.put((priority, task))

    async def run(self):
        while not self.task_queue.empty():
            priority, task = self.task_queue.get()
            print(f"Running task with priority {priority}")
            await task()

async def example_task(name, duration):
    print(f"Task {name} started")
    await asyncio.sleep(duration)
    print(f"Task {name} completed")

if __name__ == "__main__":
    scheduler = TaskScheduler()
    scheduler.add_task(2, lambda: example_task("A", 1))
    scheduler.add_task(1, lambda: example_task("B", 2))
    scheduler.add_task(3, lambda: example_task("C", 1))

    asyncio.run(scheduler.run())
