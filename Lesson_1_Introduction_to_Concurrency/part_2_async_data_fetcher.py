import asyncio
import random

async def fetch_data(name, duration):
    print(f"Fetching data for {name}...")
    await asyncio.sleep(duration)
    print(f"Data for {name} fetched after {duration} seconds")

async def main():
    await asyncio.gather(
        fetch_data("API1", random.randint(1, 3)),
        fetch_data("API2", random.randint(1, 3)),
    )

if __name__ == "__main__":
    asyncio.run(main())
