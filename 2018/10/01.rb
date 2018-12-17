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

def grid(min_x, max_x, min_y, max_y, coords)
  (min_y..max_y).each do |y|
    puts (min_x..max_x).map { |x| coords.include?([x, y]) ? '#' : '.' }.join('')
  end
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
  min_x = coords.map { |x| x[0] } .min
  max_x = coords.map { |x| x[0] } .max
  min_y = coords.map { |y| y[1] } .min
  max_y = coords.map { |y| y[1] } .max
  if most_x * most_y > 1000
    grid(min_x, max_x, min_y, max_y, coords)
    break
  end
end
