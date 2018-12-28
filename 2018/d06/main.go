package main

import (
	"bufio"
	"fmt"
	"math"
	"os"
	"sort"
	"strings"
)

type xy struct {
	x, y int
}

func distance(c1, c2 xy) int {
	return int(math.Abs(float64(c1.x-c2.x)) + math.Abs(float64(c1.y-c2.y)))
}

type coordinate struct {
	xy
	id   int
	size int
}

var tie = coordinate{xy: xy{0, 0}, id: -1}

type location struct {
	coordinate  *coordinate
	distance    int
	sumDistance int
}

type board struct {
	coordinates []coordinate
	locs        map[xy]*location
	h, v        int
}

func newBoard() board {
	return board{coordinates: make([]coordinate, 0),
		locs: map[xy]*location{}}
}

func (b *board) addCoordinate(c coordinate) {
	b.coordinates = append(b.coordinates, c)
	if c.y > b.v {
		b.v = c.y
	}
	if c.x > b.h {
		b.h = c.x
	}
}

type candidate struct {
	xy
	*location
}

type candidates []candidate

func (c candidates) Len() int {
	return len(c)
}

func (c candidates) Less(i, j int) bool {
	return c[i].distance < c[j].distance
}

// Swap swaps the elements with indexes i and j.
func (c candidates) Swap(i, j int) {
	c[j], c[i] = c[i], c[j]
}

// dijkstra computes the distance from every location to its closest coordinate
// I used this intead of calculating the distance to every coordinate because
// it seemed more elegant. But in part 2 I had to calculate the distances anyway.
func (b *board) dijkstra() {
	// fmt.Println("Running dijkstra")
	candidates := make(candidates, 0, b.h*b.v/2)
	for c := range b.coordinates {
		candidates = append(candidates, candidate{
			b.coordinates[c].xy,
			&location{coordinate: &b.coordinates[c],
				distance: 0,
			},
		})
	}
	times := 0
	sort.Sort(candidates)
	for len(candidates) > 0 {
		cand := candidates[0]
		candidates = candidates[1:]
		exist, ok := b.locs[cand.xy]
		if ok {
			if exist.distance < cand.distance || (exist.distance <= cand.distance && exist.coordinate == cand.coordinate) {
				continue
			}
			if exist.distance == cand.distance {
				exist.distance = 0
				exist.coordinate.size--
				exist.coordinate = &tie
				b.locs[cand.xy] = exist
			}
		} else {
			cand.coordinate.size++
			b.locs[cand.xy] = cand.location
		}
		for i := -1; i < 2; i++ {
			for j := -1; j < 2; j++ {
				if i == j || i == -j { // Remove Diagonals and i == j == 0
					continue
				}
				x := cand.x + i
				y := cand.y + j
				if b.inside(x, y) {
					times++
					candidates = append(candidates,
						candidate{xy: xy{x: x, y: y},
							location: &location{distance: cand.distance + 1,
								coordinate: cand.coordinate}})

				}
			}
		}

	}
}

func (b *board) inside(x, y int) bool {
	return x >= 0 && x <= b.h && y >= 0 && y <= b.v
}

func (b *board) maxFinite() (xy, int) {
	tr := map[int]bool{}
	for i := 0; i <= b.h; i++ {
		tr[b.locs[xy{i, 0}].coordinate.id] = true
		tr[b.locs[xy{i, b.v}].coordinate.id] = true
	}
	for j := 0; j <= b.v; j++ {
		tr[b.locs[xy{0, j}].coordinate.id] = true
		tr[b.locs[xy{b.h, j}].coordinate.id] = true
	}
	max := 0
	var maxXy xy
	for finite := range b.coordinates {
		coord := b.coordinates[finite]
		_, ok := tr[coord.id]
		if ok { // infinite
			continue
		}
		if coord.size > max {
			max = coord.size
			maxXy = coord.xy
		}
		fmt.Printf("Coord %s: %d\n", string('A'+coord.id), coord.size)
	}
	return maxXy, max
}

func (b *board) getDistances() {
	for i := 0; i <= b.h; i++ {
		for j := 0; j <= b.v; j++ {
			this := xy{x: i, y: j}
			loc := b.locs[this]
			loc.sumDistance = 0
			for c := range b.coordinates {
				dist := distance(b.coordinates[c].xy, this)
				loc.sumDistance += dist
			}
		}
	}
}

func (b *board) areaWithin(l int) (area int) {
	b.getDistances()
	for i := 1; i <= b.h; i++ {
		for j := 1; j <= b.v; j++ {
			this := b.locs[xy{x: i, y: j}]
			if this.sumDistance < l {
				area++
			}
		}
	}
	return area
}

func (b board) String() string {
	s := strings.Builder{}
	for j := 0; j <= b.v; j++ {
		for i := 0; i <= b.h; i++ {
			loc, ok := b.locs[xy{x: i, y: j}]
			if !ok {
				s.WriteString(" ")
				continue
			}

			if *loc.coordinate == tie {
				s.WriteString(".")
				continue
			}

			if loc.distance == 0 {
				s.WriteString(string(loc.coordinate.id + 'A'))
				continue

			}
			s.WriteString(string(loc.coordinate.id + 'a'))

		}
		s.WriteString("\n")
	}
	return s.String()

}

func main() {
	f, err := os.Open("input.txt")
	if err != nil {
		panic("Could not read input")
	}
	s := bufio.NewScanner(f)
	b := newBoard()
	for i := 0; s.Scan(); i++ {
		c := coordinate{}
		c.id = i
		fmt.Sscanf(s.Text(), "%d, %d", &c.x, &c.y)
		b.addCoordinate(c)
	}
	// fmt.Println(b)
	b.dijkstra()
	fmt.Println(b)
	pos, m := b.maxFinite()
	fmt.Println("Max size: ", m, " loc: ", pos)
	fmt.Println("Area within 10000: ", b.areaWithin(10000))
}
