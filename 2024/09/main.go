package main

import (
	_ "embed"
	"fmt"
	"slices"
	"strconv"
	"strings"
)

//go:embed input.txt
var input string

type File struct {
	start int
	size  int
}

func getLeftmostFreeSpaceBlock(blocks []int) int {
	for i, id := range blocks {
		if id == -1 {
			return i
		}
	}

	return -1
}

func getEndOfDiskBlock(blocks []int) int {
	for i := len(blocks) - 1; i >= 0; i-- {
		if blocks[i] != -1 {
			return i
		}
	}

	return -1
}

func checksum(blocks []int) int {
	checksum := 0

	for i, id := range blocks {
		if id != -1 {
			checksum += i * id
		}
	}

	return checksum
}

func PartOne(input string) int {
	blocks := []int{}

	fileId := 0
	for i, digit := range strings.TrimSpace(input) {
		size, err := strconv.Atoi(string(digit))
		if err != nil {
			fmt.Println("bad character: ", digit)
			continue
		}

		if i%2 == 0 {
			for range size {
				blocks = append(blocks, fileId)
			}
			fileId++
		} else {
			for range size {
				blocks = append(blocks, -1)
			}
		}
	}

	for {
		leftmostFreeSpaceBlock := getLeftmostFreeSpaceBlock(blocks)
		if leftmostFreeSpaceBlock == -1 {
			break
		}
		endOfDiskBlock := getEndOfDiskBlock(blocks)
		if endOfDiskBlock == -1 || endOfDiskBlock <= leftmostFreeSpaceBlock {
			break
		}

		blocks[leftmostFreeSpaceBlock] = blocks[endOfDiskBlock]
		blocks[endOfDiskBlock] = -1
	}

	return checksum(blocks)
}

func PartTwo(input string) int {
	blocks := []int{}

	fileId := 0
	files := make(map[int]File)
	for i, digit := range strings.TrimSpace(input) {
		size, err := strconv.Atoi(string(digit))
		if err != nil {
			fmt.Println("bad character: ", digit)
			continue
		}

		if i%2 == 0 {
			files[fileId] = File{len(blocks), size}
			for range size {
				blocks = append(blocks, fileId)
			}
			fileId++
		} else {
			for range size {
				blocks = append(blocks, -1)
			}
		}
	}
	fileId = slices.Max(blocks)

	for id := fileId; id >= 0; id-- {
		for i := 0; i < files[id].start; i++ {
			if blocks[i] == -1 {
				if i+files[id].size >= len(blocks) {
					continue
				}
				free := true
				for f := i; f < i+files[id].size; f++ {
					if blocks[f] != -1 {
						free = false
						break
					}
				}
				if free {
					for move := i; move < i+files[id].size; move++ {
						blocks[move] = id
						blocks[files[id].start+move-i] = -1
					}
					files[id] = File{i, files[id].size}
				}
			}
		}
	}

	return checksum(blocks)
}

func main() {
	fmt.Println("What is the resulting filesystem checksum?", PartOne(input))

	fmt.Println("What is the resulting filesystem checksum?", PartTwo(input))
}
