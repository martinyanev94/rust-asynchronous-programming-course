import asyncio

async def fetch_data(cancel_event):
    print("Fetching data...")
    try:
        await asyncio.sleep(5)  # Simulating long-running I/O operation
    except asyncio.CancelledError:
        print("Fetch data operation was cancelled.")
        return
    print("Data received!")

async def main():
    cancel_event = asyncio.Event()
    task = asyncio.create_task(fetch_data(cancel_event))
    
    await asyncio.sleep(2)  # Simulating some waiting time
    task.cancel()  # Cancelling the task
    await task  # Awaiting the cancellation

asyncio.run(main())
