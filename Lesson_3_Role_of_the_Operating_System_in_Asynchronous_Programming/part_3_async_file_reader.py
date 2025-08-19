import asyncio
import aiofiles

async def read_file(file_path):
    async with aiofiles.open(file_path, 'r') as file:
        contents = await file.read()
        print(f"Read file contents: {contents}")

async def main():
    await read_file('example.txt')

if __name__ == "__main__":
    asyncio.run(main())
