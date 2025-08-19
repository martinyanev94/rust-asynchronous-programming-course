async def risky_task():
    await asyncio.sleep(1)
    raise ValueError("Oops! An error occurred.")

async def main():
    try:
        await risky_task()
    except ValueError as e:
        print(f"Handled an error: {e}")

if __name__ == "__main__":
    asyncio.run(main())
