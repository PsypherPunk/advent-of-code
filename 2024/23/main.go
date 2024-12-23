package main

import (
	"bufio"
	_ "embed"
	"fmt"
	"sort"
	"strings"
)

//go:embed input.txt
var input string

// https://en.wikipedia.org/wiki/Bron%E2%80%93Kerbosch_algorithm
func bronKerbosch(r, p, x []string, adjacencyList map[string]map[string]struct{}, cliques *[][]string) {
	if len(p) == 0 && len(x) == 0 {
		*cliques = append(*cliques, append([]string{}, r...))
		return
	}

	for i := 0; i < len(p); i++ {
		node := p[i]
		newR := append(r, node)
		newP := []string{}
		newX := []string{}

		for _, neighbor := range p {
			if _, ok := adjacencyList[node][neighbor]; ok {
				newP = append(newP, neighbor)
			}
		}

		for _, neighbor := range x {
			if _, ok := adjacencyList[node][neighbor]; ok {
				newX = append(newX, neighbor)
			}
		}

		bronKerbosch(newR, newP, newX, adjacencyList, cliques)

		p = append(p[:i], p[i+1:]...)
		x = append(x, node)
		i--
	}
}

func PartOne(input string) int {
	adjacencyList := make(map[string]map[string]struct{})

	scanner := bufio.NewScanner(strings.NewReader(input))
	for scanner.Scan() {
		line := scanner.Text()
		parts := strings.Split(line, "-")
		a, b := parts[0], parts[1]
		if adjacencyList[a] == nil {
			adjacencyList[a] = make(map[string]struct{})
		}
		if adjacencyList[b] == nil {
			adjacencyList[b] = make(map[string]struct{})
		}
		adjacencyList[a][b] = struct{}{}
		adjacencyList[b][a] = struct{}{}
	}

	interConnected := [][]string{}
	for a := range adjacencyList {
		for b := range adjacencyList[a] {
			if b <= a {
				continue
			}
			for c := range adjacencyList[b] {
				if c <= b {
					continue
				}
				if _, ok := adjacencyList[a][c]; !ok {
					continue
				}
				interConnected = append(interConnected, []string{a, b, c})
			}
		}
	}

	count := 0
	for _, setOfThree := range interConnected {
		for _, computer := range setOfThree {
			if strings.HasPrefix(computer, "t") {
				count++
				break
			}
		}
	}

	return count
}

func PartTwo(input string) string {
	adjacencyList := make(map[string]map[string]struct{})

	scanner := bufio.NewScanner(strings.NewReader(input))
	for scanner.Scan() {
		line := scanner.Text()
		parts := strings.Split(line, "-")
		a, b := parts[0], parts[1]
		if adjacencyList[a] == nil {
			adjacencyList[a] = make(map[string]struct{})
		}
		if adjacencyList[b] == nil {
			adjacencyList[b] = make(map[string]struct{})
		}
		adjacencyList[a][b] = struct{}{}
		adjacencyList[b][a] = struct{}{}
	}

	computers := []string{}
	for computer := range adjacencyList {
		computers = append(computers, computer)
	}
	cliques := [][]string{}
	bronKerbosch([]string{}, computers, []string{}, adjacencyList, &cliques)

	largestClique := []string{}
	for _, clique := range cliques {
		if len(clique) > len(largestClique) {
			largestClique = clique
		}
	}

	sort.Strings(largestClique)
	password := strings.Join(largestClique, ",")

	return password
}

func main() {
	fmt.Println("How many contain at least one computer with a name that starts with t?", PartOne(input))

	fmt.Println("What is the password to get into the LAN party?", PartTwo(input))
}
