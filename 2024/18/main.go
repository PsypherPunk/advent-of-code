package main

import (
	"bufio"
	"container/list"
	_ "embed"
	"fmt"
	"image"
	"strings"
)

//go:embed input.txt
var input string

type Path struct {
	bytePosition image.Point
	steps        int
}

func reachExit(fallingBytes []image.Point, max int) int {
	topLeft := image.Point{0, 0}
	bottomRight := image.Point{max, max}

	queue := list.New()
	queue.PushBack(Path{topLeft, 0})

	seen := make(map[image.Point]bool)
	seen[topLeft] = true

	fallenBytes := map[image.Point]bool{}
	for _, fallingByte := range fallingBytes {
		fallenBytes[fallingByte] = true
	}

	for queue.Len() > 0 {
		current := queue.Remove(queue.Front()).(Path)

		if current.bytePosition.Eq(bottomRight) {
			return current.steps
		}

		for _, direction := range []image.Point{{0, -1}, {1, 0}, {0, 1}, {-1, 0}} {
			next := current.bytePosition.Add(direction)
			if next.X >= 0 && next.X <= max && next.Y >= 0 && next.Y <= max && !fallenBytes[next] && !seen[next] {
				seen[next] = true
				queue.PushBack(Path{next, current.steps + 1})
			}
		}
	}

	return -1
}

func PartOne(input string, fallen int, max int) int {
	scanner := bufio.NewScanner(strings.NewReader(input))

	bytePositions := []image.Point{}
	for scanner.Scan() {
		line := scanner.Text()

		bytePosition := image.Point{}
		fmt.Sscanf(line, "%d,%d", &bytePosition.X, &bytePosition.Y)
		bytePositions = append(bytePositions, bytePosition)
	}

	return reachExit(bytePositions[:fallen], max)
}

func PartTwo(input string, max int) string {
	scanner := bufio.NewScanner(strings.NewReader(input))

	bytePositions := []image.Point{}
	for scanner.Scan() {
		line := scanner.Text()

		bytePosition := image.Point{}
		fmt.Sscanf(line, "%d,%d", &bytePosition.X, &bytePosition.Y)
		bytePositions = append(bytePositions, bytePosition)
	}

	for i := 0; i < len(bytePositions); i++ {
		if steps := reachExit(bytePositions[:i], max); steps == -1 {
			firstByte := bytePositions[i-1]
			return fmt.Sprintf("%d,%d", firstByte.X, firstByte.Y)
		}
	}

	return ""
}

func main() {
	fmt.Println("…what is the minimum number of steps needed to reach the exit?", PartOne(input, 1024, 70))

	fmt.Println("What are the coordinates of the first byte that will prevent the exit from being reachable from your starting position?", PartTwo(input, 70))
}
