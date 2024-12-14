from functools import reduce

def for_range_loop():
    original_array = [1, 2, 3]
    new_array = []
    for i in range(len(original_array)):
        new_array.append(original_array[i] * 2)
    print(f"for: {original_array=}, {new_array=}")

def for_in_loop():
    original_array = [1, 2, 3]
    new_array = []
    for value in original_array:
        new_array.append(value * 2)
    print(f"for...in: {original_array=}, {new_array=}")

def while_loop():
    original_array = [1, 2, 3]
    new_array = []
    i = 0
    while i < len(original_array):
        new_array.append(original_array[i] * 2)
        i += 1
    print(f"while: {original_array=}, {new_array=}")

def map_operation():
    original_array = [1, 2, 3, 4]
    new_array = list(map(lambda value: value * 2, original_array))
    print(f"map: {original_array=}, {new_array=}")

def filter_operation():
    original_array = [1, -2, 3, 4, -5]
    new_array = list(filter(lambda value: value > 0, original_array))
    print(f"filter: {original_array=}, {new_array=}")

def find_operation():
    original_array = [-1, 3, 6, 7, 8]
    result = next(
        (value for value in original_array if value > 0 and value % 2 == 0), 
        None
    )
    print(f"next (find): {original_array=}, {result=}")


def all_operation():
    original_array = [2, 4, 6]
    result = all(value % 2 == 0 for value in original_array)
    print(f"all: {original_array=}, {result=}")


def any_operation():
    original_array = [1, 3, 5, 6]
    result = any(value % 2 == 0 for value in original_array)
    print(f"any: {original_array=}, {result=}")


def reduce_sum_array():
    original_array = [1, 2, 3, 4]
    result = reduce(
        lambda acc, current: acc + current, 
        original_array, 
        0
    )
    print(f"reduce...sum_array: {original_array=}, {result=}")


def reduce_max_val():
    original_array = [3, 7, 2, 9, 5]
    result = reduce(
        lambda max_val, current: current if current > max_val else max_val, 
        original_array, 
        -float('inf')
    )
    print(f"reduce...max_val: {original_array=}, {result=}")


def reduce_flatten_array():
    original_array = [[1, 2], [3, 4], [5]]
    result = reduce(
        lambda flat, current: flat + current, 
        original_array, 
        []
    )
    print(f"reduce...flatten_array: {original_array=}, {result=}")


for_range_loop()
for_in_loop()
while_loop()
map_operation()
filter_operation()
find_operation()
all_operation()
any_operation()
reduce_sum_array()
reduce_max_val()
reduce_flatten_array()
