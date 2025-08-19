import asyncio

async def task_1():
    print("Task 1 started")
    await asyncio.sleep(2)  # Simulate an I/O-bound task
    print("Task 1 completed")

async def task_2():
    print("Task 2 started")
    await asyncio.sleep(1)  # Simulate an I/O-bound task
    print("Task 2 completed")

async def main():
    print("Starting tasks...")
    await asyncio.gather(task_1(), task_2())

if __name__ == "__main__":
    asyncio.run(main())
