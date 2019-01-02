package main

import (
	"bufio"
	"fmt"
	"os"
	"strings"
)

var (
	open   = '.'
	tree   = '|'
	lumber = '#'
)

type xy struct {
	x, y int
}

func (pos xy) Less(other xy) bool {
	return pos.y < other.y || (pos.y == other.y && pos.x < other.x)
}

func (pos *xy) Add(delta xy) xy {
	return xy{pos.x + delta.x, pos.y + delta.y}
}

func (pos xy) Invert() xy {
	return xy{-pos.x, -pos.y}

}

func (pos xy) String() string {
	return fmt.Sprintf("<%d, %d>", pos.x, pos.y)
}

func (pos xy) left() xy {
	return xy{pos.x - 1, pos.y}
}
func (pos xy) right() xy {
	return xy{pos.x + 1, pos.y}
}
func (pos xy) up() xy {
	return xy{pos.x, pos.y - 1}
}
func (pos xy) down() xy {
	return xy{pos.x, pos.y + 1}
}

type area [][]rune

func newArea(x, y int) area {
	a := make(area, y)
	for j := 0; j < y; j++ {
		a[j] = make([]rune, x)
	}
	return a
}

func (a area) String() string {
	s := strings.Builder{}

	for j := 0; j < len(a); j++ {
		row := a[j]
		for i := 0; i < len(row); i++ {
			s.WriteString(string(row[i]))
		}
		s.WriteString("\n")
	}
	return s.String()
}

var missing rune

func (a area) get(pos xy) rune {
	if pos.y < 0 || pos.x < 0 || pos.y >= len(a) || pos.x >= len(a[pos.y]) {
		return missing
	}
	return a[pos.y][pos.x]
}

func (a area) counts() map[rune]int {
	counts := map[rune]int{}
	h, v := a.size()
	for j := 0; j < v; j++ {
		for i := 0; i < h; i++ {
			n := a.get(xy{i, j})
			counts[n]++
		}
	}
	return counts
}

func (a area) evolve(pos xy) rune {
	r := a.get(pos)
	counts := map[rune]int{}
	for j := -1; j < 2; j++ {
		for i := -1; i < 2; i++ {
			if i == 0 && j == 0 {
				continue
			}
			n := a.get(xy{pos.x + i, pos.y + j})
			counts[n]++
		}
	}

	switch r {
	// An open acre will become filled with trees if three or more adjacent acres contained trees. Otherwise, nothing happens.
	case open:
		if counts[tree] >= 3 {
			return tree
		}
		return open
	// An acre filled with trees will become a lumberyard if three or more adjacent acres were lumberyards. Otherwise, nothing happens.
	case tree:
		if counts[lumber] >= 3 {
			return lumber
		}
		return tree
	// An acre containing a lumberyard will remain a lumberyard if it was adjacent to at least one other lumberyard and at least one acre containing trees. Otherwise, it becomes open.
	case lumber:
		if counts[lumber] >= 1 && counts[tree] >= 1 {
			return lumber
		}
		return open
	}
	panic(fmt.Errorf("should not reach here. Got '%c' @ %s", r, pos))
}

func (a area) step() area {
	h, v := a.size()
	na := newArea(h, v)

	for j := 0; j < v; j++ {
		for i := 0; i < h; i++ {
			na[j][i] = a.evolve(xy{i, j})
		}
	}
	return na
}

func (a area) size() (int, int) {
	x := 1
	y := len(a)
	if y > 0 {
		x = len(a[0])
	}
	return x, y
}

func (a *area) set(pos xy, val rune) {
	(*a)[pos.y][pos.x] = val
}

func (a area) equal(o area) bool {

	h1, v1 := a.size()
	h2, v2 := a.size()
	if h1 != h2 || v1 != v2 {
		return false
	}
	for j := 0; j < v1; j++ {
		for i := 0; i < h1; i++ {
			pos := xy{i, j}
			if a.get(pos) != o.get(pos) {
				return false
			}
		}
	}
	return true
}

func main() {
	f, err := os.Open("input.txt")
	if err != nil {
		panic("could not open input")
	}
	s := bufio.NewScanner(f)

	a := make(area, 0)
	for s.Scan() {
		t := s.Text()

		row := make([]rune, len(t))
		for ix, c := range t {
			row[ix] = c
		}
		a = append(a, row)
	}
	fmt.Println(len(a))
	fmt.Println(a)

	initial := a
	for i := 0; i < 10; i++ {
		a = a.step()
		fmt.Println(a)
	}
	c := a.counts()
	fmt.Println(c[lumber] * c[tree])

	gens := make([]area, 0)
	a = initial
	maxi := 0
loop:
	for i := 1; i <= 1000000000; i++ {
		a = a.step()
		for ix, o := range gens {
			if a.equal(o) {
				gens = gens[ix:]
				maxi = i
				break loop
			}
		}
		gens = append(gens, a)
		// fmt.Println(area)
	}

	a = gens[(1000000000-maxi)%len(gens)]
	c = a.counts()
	fmt.Println(c[lumber] * c[tree])
}
