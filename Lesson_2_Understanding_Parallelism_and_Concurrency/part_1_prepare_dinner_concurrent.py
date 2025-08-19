from concurrent.futures import ThreadPoolExecutor
import time

def cook_item(item, time_needed):
    print(f"Cooking {item} for {time_needed} seconds...")
    time.sleep(time_needed)
    print(f"{item} is ready!")

def prepare_dinner():
    items = [("pasta", 3), ("sauce", 2), ("salad", 1)]
    with ThreadPoolExecutor(max_workers=3) as executor:
        for item, duration in items:
            executor.submit(cook_item, item, duration)

if __name__ == "__main__":
    prepare_dinner()
