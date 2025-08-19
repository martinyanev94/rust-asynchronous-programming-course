import multiprocessing
import time

def calculate_square(number):
    print(f"Square of {number}: {number * number}")
    time.sleep(1)

if __name__ == "__main__":
    numbers = [1, 2, 3, 4, 5]
    processes = []

    for number in numbers:
        process = multiprocessing.Process(target=calculate_square, args=(number,))
        processes.append(process)
        process.start()

    for process in processes:
        process.join()

    print("All processes completed.")
