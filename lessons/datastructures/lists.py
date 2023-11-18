from typing import List


def next_n_multiples_of(base: int, n) -> List[int]:
    return [base ** i for i in range(2, n + 2)]


def test_next_five_multiples_of():

    tests = [
        {
            'input': [2, 5],
            'want': [4, 8, 16, 32, 64]
        },
        {
            'input': [2, 6],
            'want': [4, 8, 16, 32, 64, 128]
        },
        {
            'input': [2, 4],
            'want': [4, 8, 16, 32]
        }
        ,
        {
            'input': [2, 0],
            'want': []
        }
        ,
        {
            'input': [2, -1],
            'want': []
        },
        {
            'input': [-2, 6],
            'want': [4, -8, 16, -32, 64, -128]
        }
    ]
    for test in tests:
        assert next_n_multiples_of(*test['input']) == test['want']


def remove_negatives(numbers: List[int]) -> List[int]:
    return [number for number in numbers if number % 2 == 0]


def test_remove_negatives():
    tests = [
        {
            'input': [2, 6],
            'want': [2, 6]
        },
        {
            'input': [2, 5],
            'want': [2]
        },
        {
            'input': [1, 3],
            'want': []
        }
        ,
        {
            'input': [1],
            'want': []
        }
        ,
        {
            'input': [],
            'want': []
        },
        {
            'input': [0],
            'want': [0]
        }
    ]
    for test in tests:
        assert remove_negatives(test['input']) == test['want']


def swap(numbers: List[int], left: int, right: int) -> List[int]:
    numbers[left], numbers[right] = numbers[right], numbers[left]
    return numbers


def test_swap():
    tests = [
        {
            'input': [[1, 2], 0, 1],
            'want': [2, 1]
        },
        {
            'input': [[1, 2, 3], 0, 2],
            'want': [3, 2, 1]
        },
        {
            'input': [[1, 2, 3], 0, 2],
            'want': [3, 2, 1]
        },
        {
            'input': [[1, 2, 3, 4], 0, -1],
            'want': [4, 2, 3, 1]
        },
        {
            'input': [[1, 2, 3, 4], -3, -1],
            'want': [1, 4, 3, 2]
        },
    ]
    for test in tests:
        assert swap(*test['input']) == test['want']