package main

import (
	"bufio"
	"fmt"
	"os"
	"sort"
)

type pos struct {
	x, y, z int
}

func abs(a int) int {
	if a > 0 {
		return a
	}
	return -a
}

func (p pos) dist(o pos) int {
	return abs(p.x-o.x) + abs(p.y-o.y) + abs(p.z-o.z)

}

type bot struct {
	pos
	r int
}

func (b bot) walk() []pos {
	neigh := make([]pos, 0)
	for j := -b.r; j <= b.r; j++ {
		for i := -b.r; i <= b.r; i++ {
			for h := -b.r; h <= b.r; h++ {
				p := pos{b.x + h, b.y + i, b.z + j}
				if b.dist(p) <= b.r {
					neigh = append(neigh, p)
				}
			}
		}
	}
	return neigh
}

func main() {
	f, err := os.Open("input.txt")
	if err != nil {
		panic("could not read file")
	}
	s := bufio.NewScanner(f)

	bots := make([]bot, 0)

	for s.Scan() {
		b := bot{}
		_, err := fmt.Sscanf(s.Text(), "pos=<%d,%d,%d>, r=%d", &b.x, &b.y, &b.z, &b.r)
		if err != nil {
			panic(err)
		}
		bots = append(bots, b)
	}
	sort.Slice(bots, func(i, j int) bool { return bots[i].r > bots[j].r })
	fmt.Println(bots)
	// fmt.Println(bots[0], bots[len(bots)-1])
	leader := bots[0]
	c := 0
	for _, b := range bots {
		if leader.dist(b.pos) <= leader.r {
			c++
		}
	}
	fmt.Println("In range: ", c)
	// w := leader.walk()
	// fmt.Println(len(w))
	inrange := map[*bot]map[*bot]bool{}

	for _, b1 := range bots {
		inrange[&b1] = map[*bot]bool{}
		for _, b2 := range bots {
			inrange[&b1][&b2] = b1.dist(b2.pos) <= b1.r+b2.r
		}
	}

	contained := map[*bot][]*bot{}

	for _, b1 := range bots {
		contained[&b1] = make([]*bot, 0)
		for _, b2 := range bots {
			if b1.r < b2.r && b1.dist(b2.pos) <= 2*b1.r+b2.r {
				contained[&b1] = append(contained[&b1], &b2)
			}
		}
	}

	groups := [][]*bot{[]*bot{}}

	for ix, b := range bots {
		if len(contained[&b]) > 0 {
			continue
		}
		fmt.Printf("%3d/%3d - %d\n", ix, len(bots), len(groups))
		ngs := map[string][]*bot{}
		for gx, g := range groups {
			match := true
			ng := []*bot{&b}
			key := ""

			for _, bi := range g {
				if !inrange[&b][bi] {
					match = false
					continue
				}
				ng = append(ng, bi)
				key += fmt.Sprintf("-%d,%d", bi.x, bi.y)
			}
			if match {
				groups[gx] = ng
			} else {
				ngs[key] = ng
			}
		}
		for _, ng := range ngs {
			groups = append(groups, ng)
		}
	}

	sort.Slice(groups, func(i, j int) bool { return len(groups[i]) < len(groups[j]) })

	best := groups[len(groups)-1]
	fmt.Println(best)
	fmt.Println(len(best))
}
