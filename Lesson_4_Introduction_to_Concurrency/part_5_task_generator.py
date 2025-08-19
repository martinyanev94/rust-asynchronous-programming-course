def task(name):
    print(f'Task {name}: Started')
    yield
    print(f'Task {name}: Resumed')
    yield
    print(f'Task {name}: Finished')

gen1 = task("A")
gen2 = task("B")

while True:
    try:
        next(gen1)
    except StopIteration:
        break
    try:
        next(gen2)
    except StopIteration:
        break
