import asyncio

async def fetch_data_from_source1():
    await asyncio.sleep(3)
    return "Data from source 1"

async def fetch_data_from_source2():
    await asyncio.sleep(1)
    return "Data from source 2"

async def main():
    task1 = fetch_data_from_source1()
    task2 = fetch_data_from_source2()
    
    results = await asyncio.gather(task1, task2)
    print(results)

asyncio.run(main())
