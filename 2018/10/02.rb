#!/usr/bin/env ruby

def star(line)
  pattern = /^position=<([[ -]][0-9]+), ([[ -]][0-9]+)> velocity=<([[ -]][0-9]+), ([[ -]][0-9]+)>/
  groups = pattern.match(line).captures
  {
    x: groups[0].strip.to_i,
    y: groups[1].strip.to_i,
    dx: groups[2].strip.to_i,
    dy: groups[3].strip.to_i
  }
end

def tick(seconds, points)
  points.map { |p| [p[:x] + p[:dx] * seconds, p[:y] + p[:dy] * seconds] }
end

points = File.new('input.txt').readlines.map { |line| star(line) }

seconds = 0
loop do
  seconds += 1
  coords = tick(seconds, points)
  shared_x = coords.group_by { |point| point[0] }
  shared_y = coords.group_by { |point| point[1] }
  most_x = shared_x.values.map(&:length).max
  most_y = shared_y.values.map(&:length).max
  if most_x * most_y > 1000
    puts seconds
    break
  end
end
