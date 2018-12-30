package main

import (
	"bufio"
	"fmt"
	"os"
	"strings"
)

type xy struct {
	x, y int
}

type speed struct {
	x, y int
}


type point struct {
	pos xy
	speed speed
}


type board struct {
	grid map[xy]bool
	points []*point
}


func (b board) String() string {
	max := xy{}
	min := xy{}
	for pos := range b.grid {
		if pos.x > max.x {
			max.x = pos.x
		}
		if pos.y > max.y {
			max.y = pos.y
		}
		if min.x == 0 || pos.x < min.x {
			min.x = pos.x
		}
		if min.y == 0 || pos.y < min.y {
			min.y = pos.y
		}
	}
	s := strings.Builder{}
	for j:=min.y; j<=max.y; j++ {
		for i:=min.x; i<=max.x; i++ {
			p := b.grid[xy{x:i,y:j}]
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

func (b *board) update(delta float64, factor int) {
	newgrid := map[xy]bool{}
	for _, p := range b.points {
		p.pos.x = p.pos.x+int(float64(p.speed.x)*delta)
		p.pos.y = p.pos.y+int(float64(p.speed.y)*delta)
		newgrid[xy{p.pos.x/factor, p.pos.y/factor}] = true
	}
	b.grid = newgrid
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
	b := board{}
	b.points = points
	// fmt.Println(b)
	b.update(10658, 1)
	for i:=0; i< 1; i++ {
		b.update(1, 1)
		fmt.Println(i)
		for j:=0; j< 50; j++ {
			fmt.Print("#")
		}
		fmt.Println()
		fmt.Println(b)
	}


}
