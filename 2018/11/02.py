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

power_levels = []
for size in range(3, 300):
    squares = sum(grid[x:x-size+1 or None, y:y-size+1 or None] for x in range(size) for y in range(size))
    max_ = squares.max()
    coords = numpy.where(squares == squares.max())
    power_levels.append((size, max_, coords[0][0], coords[1][0]))

print(max(power_levels, key=lambda p: p[1]))
