# Write a function that implements division.
#
# You may discard any remainder the comes from division.
#
# assert divide(10, 2) == 5
# assert divide(11, 2) == 5
# assert divide(1, 2) == 0
#
# May be given negative numbers.
#
# assert divide(2, 2) == 1
# assert divide(-2, 2) == -1
# assert divide(2, -2) == -1
# assert divide(-2, -2) == 1
#
# You are NOT allowed to use the builtin division operator (/).
def divide(numerator: int, denominator: int) -> int:
    sign = sign_of(numerator, denominator)
    answer = 0
    numerator = abs(numerator)
    denominator = abs(denominator)
    while numerator >= denominator:
        numerator -= denominator
        answer += 1
    return answer * sign


# Write a function that implements multiplication.
#
# assert multiply(2, 5) == 10
# assert multiply(11, 2) == 22
# assert multiply(1, -12) == -12
#
# May be given negative numbers.
#
# assert multiply(2, 2) == -4
# assert multiply(-2, 2) == -4
# assert multiply(2, -2) == -4
# assert multiply(-2, -2) == 4
#
# You are NOT allowed to use the builtin multiplication operator (*).
def multiply(left: int, right: int):
    sign = sign_of(left, right)
    answer = 0
    left = abs(left)
    right = abs(right)
    for i in range(0, right):
        answer += left
    return answer * sign


def sign_of(a: int, b: int) -> int:
    product = a * b
    return 1 if product >= 0 else -1


def assert_eq(got, want):
    assert got == want, f"got {got}, want {want}"

assert_eq(divide(10, 2), 5)
assert_eq(divide(11, 2), 5)
assert_eq(divide(1, 2), 0)
assert_eq(divide(2, 2), 1)
assert_eq(divide(-2, 2), -1)
assert_eq(divide(2, -2), -1)
assert_eq(divide(-2, -2), 1)

assert_eq(multiply(2, 5), 10)
assert_eq(multiply(11, 2), 22)
assert_eq(multiply(1, -12), -12)
assert_eq(multiply(2, 2), 4)
assert_eq(multiply(-2, 2), -4)
assert_eq(multiply(2, -2), -4)
assert_eq(multiply(-2, -2), 4)
