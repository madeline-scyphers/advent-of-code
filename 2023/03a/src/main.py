"""Problem 3A"""
from __future__ import annotations


from pathlib import Path
import os


def main():
    assert process_data(read_file(Path(__file__).parent.parent / "test_data.txt")) == 4361
    print(process_data(read_file(Path(__file__).parent.parent / "data.txt")))


def process_data(data: list[list[str]]) -> int:
    """
    The engine schematic (your puzzle input) consists of a visual representation of the engine.
    There are lots of numbers and symbols you don't really understand,
    but apparently any number adjacent to a symbol, even diagonally, is a "part number"
    and should be included in your sum. (Periods (.) do not count as a symbol.)

    Here is an example engine schematic:

    467..114..
    ...*......
    ..35..633.
    ......#...
    617*......
    .....+.58.
    ..592.....
    ......755.
    ...$.*....
    .664.598..

    In this schematic, two numbers are not part numbers because they are not adjacent
    to a symbol: 114 (top right) and 58 (middle right).
    Every other number is adjacent to a symbol and so is a part number; their sum is 4361.
    """
    running_sum = 0
    for row_idx, line in enumerate(data):
        current_num = []
        is_parta_num = False
        for col_idx, char in enumerate(line):
            if char.isdigit():
                current_num.append(char)
                if not is_parta_num:
                    is_parta_num = check_if_is_part_num(data, row_idx, col_idx)
            else:
                if is_parta_num:
                    running_sum += int("".join(current_num))
                is_parta_num = False
                current_num = []
    return running_sum


def check_if_is_part_num(data: list[list[str]], row_idx: int, col_idx: int) -> bool:
    """Checks if the number is part of a bigger number
    and if the number is adjacent to any symbol"""
    line = data[row_idx]
    for i in range(max(row_idx - 1, 0), min(row_idx + 2, len(data))):
        for j in list(range(max(col_idx - 1, 0), min(col_idx + 2, len(line)))):
            if data[i][j] != "." and data[i][j] != os.linesep and not data[i][j].isdigit():
                return True
    return False


def read_file(file_path: Path | str) -> list[list[str]]:
    with open(file_path, "r") as file:
        # we use readlines instead of read().splitlines() because we want to keep the newlines
        # we do this, so we don't have to check if we found a part number at the end of a line and after each numeric
        data = [list(line) for line in file.readlines()]
    return data


if __name__ == '__main__':
    main()
