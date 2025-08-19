import aiohttp
import asyncio

async def fetch_data(url):
    async with aiohttp.ClientSession() as session:
        async with session.get(url) as response:
            data = await response.text()
            print(data)

async def main():
    url = 'https://jsonplaceholder.typicode.com/posts'
    await fetch_data(url)

if __name__ == "__main__":
    asyncio.run(main())
