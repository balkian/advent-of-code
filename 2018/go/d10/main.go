package main

import (
	"bufio"
	"fmt"
	"math"
	"os"
	"strings"
	"time"
)

type xy struct {
	x, y int
}

type speed struct {
	x, y int
}

type point struct {
	pos   xy
	speed speed
}

type board struct {
	points []*point
	max    xy
	min    xy
	res    int
	t      int
}

func NewBoard(points []*point) board {
	b := board{points: points}
	b.res = 50
	b.update(0)
	return b
}

func (b board) String() string {
	s := strings.Builder{}

	dx := b.max.x - b.min.x
	dy := b.max.y - b.min.y

	var h, v int

	f := 1.0

	if dx > dy {
		f = float64(b.res) / float64(dx)
		h = b.res
		v = int(f * float64(dy))
		if v == 0 {
			v = 1
		}
	} else {
		f = float64(b.res) / float64(dy)
		v = b.res
		h = int(f * float64(dx))
		if h == 0 {
			h = 1
		}
	}

	grid := make([][]bool, v)
	for j := 0; j < v; j++ {
		grid[j] = make([]bool, h)
	}
	for _, p := range b.points {
		x := int(float64(p.pos.x-b.min.x) * f)
		y := int(float64(p.pos.y-b.min.y) * f)
		if x == h {
			x = h - 1
		}
		if y == v {
			y = v - 1
		}
		// fmt.Println(x, y, h, v, dx, dy, f)
		grid[y][x] = true
	}

	for j := 0; j < v; j++ {
		for i := 0; i < h; i++ {
			p := grid[j][i]
			if p {
				s.WriteString("#")
				continue
			}
			s.WriteString(".")
		}
		s.WriteString("\n")
	}
	return s.String()
}

func (b *board) update(delta int) {
	max := xy{x: math.MinInt64, y: math.MinInt64}
	min := xy{x: math.MaxInt64, y: math.MaxInt64}
	for _, p := range b.points {
		pos := p.pos

		pos.x = pos.x + p.speed.x*delta
		pos.y = pos.y + p.speed.y*delta
		if pos.x > max.x {
			max.x = pos.x
		}
		if pos.y > max.y {
			max.y = pos.y
		}
		if pos.x < min.x {
			min.x = pos.x
		}
		if pos.y < min.y {
			min.y = pos.y
		}
		p.pos = pos
	}
	b.max = max
	b.min = min
	b.t += delta
}

func main() {
	f, err := os.Open("input.txt")
	if err != nil {
		panic("Could not read input")
	}
	s := bufio.NewScanner(f)
	points := make([]*point, 0)
	for i := 0; s.Scan(); i++ {
		p := point{}
		fmt.Sscanf(s.Text(), "position=<%d, %d> velocity=<%d, %d>", &(p.pos.x), &(p.pos.y), &(p.speed.x), &(p.speed.y))
		points = append(points, &p)
	}
	b := NewBoard(points)
	// 10658
	init := 10650
	b.update(init)
	for i := 0; i < 11000; i++ {
		b.update(1)
		fmt.Println(init+i, b.t)
		for j := 0; j < 50; j++ {
			fmt.Print("#")
		}
		fmt.Println()
		fmt.Println(b.String())
		time.Sleep(200 * time.Millisecond)
	}

}
