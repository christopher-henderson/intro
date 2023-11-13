from typing import List

# find_min is a function that takes in a collection (what Python calls a "list")
# of integers and returns to the caller the smallest number that is present in the list.
def find_min(numbers: List[int]) -> int:
    pass

# average is a function that takes in a list of integers and returns
# the average of the numbers (also called the mean).
def average(numbers: List[int]) -> float:
    pass

def main():
    data = [233, 3336, 47, 13, 32, 445, 58, 244, 23, 5, 8, 989, 54, 34, 12, 235, 45]
    print("Testing find_min...")
    execute(lambda: find_min(data), 5)
    print("Testing average...")
    import statistics
    execute(lambda: average(data), statistics.mean(data))

def execute(func, want):
    got = func()
    if got != want:
        print(f"expected {want}, got {got}")
    else:
        print("🤙")

main()
