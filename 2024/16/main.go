package main

import (
	"bufio"
	_ "embed"
	"fmt"
	"image"
	"sort"
	"strings"
)

type ReindeerMaze struct {
	tiles map[image.Point]bool
	start image.Point
	end   image.Point
}

type Reindeer struct {
	position  image.Point
	direction image.Point
}

type ReindeerChoice struct {
	reindeer Reindeer
	score    int
}

//go:embed input.txt
var input string

func reindeerOlympics(maze ReindeerMaze) int {
	queue := []ReindeerChoice{{Reindeer{maze.start, image.Point{1, 0}}, 0}}
	seen := make(map[Reindeer]bool)

	for len(queue) > 0 {
		sort.Slice(queue, func(a, b int) bool {
			return queue[a].score < queue[b].score
		})

		// TODO: container/list?
		current := queue[0]
		queue = queue[1:]

		if current.reindeer.position.Eq(maze.end) {
			return current.score
		}

		if seen[current.reindeer] {
			continue
		}

		seen[current.reindeer] = true

		next := current.reindeer.position.Add(current.reindeer.direction)
		if maze.tiles[next] {
			queue = append(queue, ReindeerChoice{
				Reindeer{
					next,
					current.reindeer.direction,
				},
				current.score + 1,
			},
			)
		}

		queue = append(queue,
			ReindeerChoice{
				Reindeer{
					current.reindeer.position,
					image.Point{current.reindeer.direction.Y, -current.reindeer.direction.X},
				},
				current.score + 1000,
			},
			ReindeerChoice{
				Reindeer{
					current.reindeer.position,
					image.Point{-current.reindeer.direction.Y, current.reindeer.direction.X},
				},
				current.score + 1000,
			},
		)
	}

	return -1
}

func PartOne(input string) int {
	scanner := bufio.NewScanner(strings.NewReader(input))

	var maze ReindeerMaze
	maze.tiles = make(map[image.Point]bool)
	y := 0
	for scanner.Scan() {
		for x, r := range scanner.Text() {
			switch r {
			case 'S':
				maze.start = image.Point{x, y}
				maze.tiles[image.Point{x, y}] = true
			case 'E':
				maze.end = image.Point{x, y}
				maze.tiles[image.Point{x, y}] = true
			case '.':
				maze.tiles[image.Point{x, y}] = true
			}
		}
		y++
	}

	return reindeerOlympics(maze)
}

func PartTwo(input string) int {
	return 0
}

func main() {
	fmt.Println("What is the lowest score a Reindeer could possibly get?", PartOne(input))

	fmt.Println("", PartTwo(input))
}
