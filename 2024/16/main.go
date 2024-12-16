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
	path     []image.Point
	reindeer Reindeer
	score    int
}

//go:embed input.txt
var input string

func reindeerOlympics(maze ReindeerMaze) int {
	queue := []ReindeerChoice{
		{
			[]image.Point{},
			Reindeer{maze.start, image.Point{1, 0}},
			0,
		},
	}
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
				[]image.Point{},
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
				[]image.Point{},
				Reindeer{
					current.reindeer.position,
					image.Point{current.reindeer.direction.Y, -current.reindeer.direction.X},
				},
				current.score + 1000,
			},
			ReindeerChoice{
				[]image.Point{},
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

func winningPaths(maze ReindeerMaze, winningScore int) [][]image.Point {
	queue := []ReindeerChoice{
		{
			[]image.Point{maze.start},
			Reindeer{maze.start, image.Point{1, 0}},
			0,
		},
	}

	seen := make(map[Reindeer]int)
	var paths [][]image.Point

	for len(queue) > 0 {
		// TODO: container/list?
		current := queue[0]
		queue = queue[1:]

		if current.score > winningScore {
			continue
		}

		if previousScore, ok := seen[current.reindeer]; ok && previousScore < current.score {
			continue
		}
		seen[current.reindeer] = current.score

		if current.reindeer.position.Eq(maze.end) && current.score == winningScore {
			paths = append(paths, current.path)
			continue
		}

		next := current.reindeer.position.Add(current.reindeer.direction)
		if maze.tiles[next] {
			nextPath := make([]image.Point, len(current.path))
			copy(nextPath, current.path)

			queue = append(queue, ReindeerChoice{
				append(nextPath, next),
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
				current.path,
				Reindeer{
					current.reindeer.position,
					image.Point{current.reindeer.direction.Y, -current.reindeer.direction.X},
				},
				current.score + 1000,
			},
			ReindeerChoice{
				current.path,
				Reindeer{
					current.reindeer.position,
					image.Point{-current.reindeer.direction.Y, current.reindeer.direction.X},
				},
				current.score + 1000,
			},
		)
	}

	return paths
}

func readMap(input string) ReindeerMaze {
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

	return maze
}

func PartOne(input string) int {
	maze := readMap(input)

	return reindeerOlympics(maze)
}

func PartTwo(input string) int {
	maze := readMap(input)
	lowestScore := reindeerOlympics(maze)

	bestPaths := winningPaths(maze, lowestScore)

	tiles := make(map[image.Point]struct{})
	for _, path := range bestPaths {
		for _, tile := range path {
			tiles[tile] = struct{}{}
		}
	}

	return len(tiles)
}

func main() {
	fmt.Println("What is the lowest score a Reindeer could possibly get?", PartOne(input))

	fmt.Println("How many tiles are part of at least one of the best paths through the maze?", PartTwo(input))
}
