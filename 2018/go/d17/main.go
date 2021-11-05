package main

import (
	"bufio"
	"fmt"
	"math"
	"os"
	"strings"
)

type xy struct {
	x, y int
}

var (
	water   = '~'
	falling = '|'
	sand    = '#'
	nothing rune
)

var start = xy{500, 0}

type soil [][]rune

func (s soil) Drop(pos xy) bool {

	initial := pos
	dirs := []xy{xy{0, 1}, xy{-1, 0}, xy{1, 0}}
	dir := 0
	tired := false
	fall := false

	// last := pos

	for {
		nx := pos.x + dirs[dir].x
		ny := pos.y + dirs[dir].y

		// On the edge
		if ny >= len(s) || pos.x < 0 || len(s[ny]) <= nx {
			fall = true
			break
		}

		down := s[pos.y+1][pos.x]

		if dir != 0 && (down == falling || down == nothing) {
			// fmt.Println("reset", pos, down == nothing, down == sand)
			dir = 0
			continue
		}

		n := s[ny][nx]

		if n == falling {
			fall = true
			// fmt.Println("falling")
			break
		}

		if n == nothing {
			pos.x = nx
			pos.y = ny
			continue
		}

		switch dir {
		case 0:
			right := s[pos.y][pos.x+1]
			left := s[pos.y][pos.x-1]

			fall = right == falling || left == falling
			if left != nothing {
				dir = 2
				continue
			}
			tired = false
			if right != nothing {
				tired = true
			}
			dir = 1
			continue
		case 1:
			if tired {
				break
			}
			dir = 2
			continue
		}
		break
	}

	if pos == initial {
		s[pos.y][pos.x] = falling
		return false
	}
	if fall {
		s[pos.y][pos.x] = falling
	} else {
		s[pos.y][pos.x] = water

	}
	return true

}

func (s soil) String() string {
	b := strings.Builder{}
	for y := 0; y < len(s); y++ {
		for x := 0; x < len(s[y]); x++ {
			c := s[y][x]
			if c == nothing {
				c = '.'
			}
			b.WriteString(fmt.Sprintf("%c", c))
		}
		b.WriteString("\n")
	}
	return b.String()
}

func main() {
	f, err := os.Open("input.txt")
	if err != nil {
		panic("could not read input")
	}
	s := bufio.NewScanner(f)

	walls := make([]xy, 0)
	min := xy{math.MaxInt64, math.MaxInt64}
	max := start
	var x0, x1, y0, y1 int
	for s.Scan() {
		r, err := fmt.Sscanf(s.Text(), "x=%d, y=%d..%d", &x0, &y0, &y1)
		x1 = x0
		if r != 3 || err != nil {
			r, err = fmt.Sscanf(s.Text(), "y=%d, x=%d..%d", &y0, &x0, &x1)
			y1 = y0
			if r != 3 || err != nil {
				panic(fmt.Errorf("could not read line: %s %s", err, s.Text()))
			}
		}
		for y := y0; y <= y1; y++ {

			for x := x0; x <= x1; x++ {
				walls = append(walls, xy{x, y})

			}
		}

		if x0 < min.x {
			min.x = x0
		}
		if y0 < min.y {
			min.y = y0
		}
		if x1 > max.x {
			max.x = x1
		}
		if y1 > max.y {
			max.y = y1
		}
	}
	min.x -= 1 // Any x value is allowed
	max.x += 1 // Any x value is allowed

	initial := xy{start.x - min.x, start.y - min.y}
	if initial.y < 0 {
		initial.y = 0
	}
	fmt.Println(min, max, start, initial)
	h := max.x - min.x + 1
	v := max.y - min.y + 1

	plan := make(soil, v)
	for y := 0; y < v; y++ {
		plan[y] = make([]rune, h)
		for x := 0; x < h; x++ {
			plan[y][x] = nothing
		}
	}

	for _, w := range walls {
		plan[w.y-min.y][w.x-min.x] = sand
	}

	for {
		if !plan.Drop(initial) {
			break
		}
		// fmt.Println(plan)
		// time.Sleep(200 * time.Millisecond)
	}
	fmt.Println(plan)

	c := 0
	w := 0
	for _, r := range plan.String() {
		if r == water {
			w++
		}
		if r == falling {
			c++
		}
	}
	fmt.Println(c+w, w)

}
