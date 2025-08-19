import asyncio

async def fetch_data():
    print("Fetching data...")
    await asyncio.sleep(3)  # Simulating I/O operation
    print("Data received!")

async def main():
    task1 = asyncio.create_task(fetch_data())
    task2 = asyncio.create_task(fetch_data())

    await task1
    await task2

asyncio.run(main())
