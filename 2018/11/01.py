#!/usr/bin/env python3

import numpy


with open("input.txt") as i:
    serial: int = int(i.read().strip())


def get_power_level(x: int, y: int):
    rack_id: int = x + 10
    power_level: int = rack_id * y
    power_level += serial
    power_level *= rack_id
    power_level //= 100
    power_level %= 10
    power_level -= 5
    return power_level


grid = numpy.fromfunction(get_power_level, (300, 300))

squares = sum(grid[x:x-3+1 or None, y:y-3+1 or None] for x in range(3) for y in range(3))
print(numpy.where(squares == squares.max()))
