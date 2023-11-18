from typing import List


def next_n_multiples_of(base: int, n) -> List[int]:
    pass


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
    pass


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