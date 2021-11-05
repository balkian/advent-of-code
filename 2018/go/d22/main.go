package main

import (
	"container/heap"
	"fmt"
	"math"
	"strings"
)

type rtype int

const (
	rocky rtype = iota
	wet
	narrow
	target
	start
)

var strs = map[interface{}]string{
	rocky:   "rocky",
	wet:     "wet",
	narrow:  "narrow",
	neither: "neither",
	climb:   "climb",
	torch:   "torch",
}

func str(i interface{}) string {
	c, ok := strs[i]
	if !ok {
		c = fmt.Sprintf("%v", i)
	}
	return c
}

type gear int

const (
	neither gear = 1 << iota
	climb
	torch
)

var gearset = []gear{torch, climb, neither}

var allow = map[rtype]gear{
	rocky:  climb | torch,
	wet:    climb | neither,
	narrow: torch | neither,
	target: torch,
	start:  torch,
}

type region struct {
	geo, ero int
	rtype    rtype
}

type Plan [][]region

func allowed(rt rtype, g gear) bool {
	return g&allow[rt] != 0
}

func (p Plan) isallowed(x, y int, g gear) bool {
	return x >= 0 && y >= 0 && y < len(p) && x < len(p[y]) && allowed(p[y][x].rtype, g)
}

func intersection(g1, g2 gear) gear {
	return g1 & g2
}

func (p Plan) intersection(x0, y0, x1, y1 int) gear {
	return intersection(p.allowed(x0, y0), p.allowed(x1, y1))
}

func (p Plan) allowed(x, y int) gear {
	return allow[p[y][x].rtype]
}

func (p Plan) String() string {
	s := strings.Builder{}

	chars := []string{".", "=", "|"}
	for _, row := range p {
		for _, val := range row {
			s.WriteString(chars[val.rtype])
		}
		s.WriteString("\n")
	}
	return s.String()

}

func (p Plan) PrintX(x, y, lx, ly int) {
	fmt.Println()
	if ly <= 0 {
		ly = len(p)
	}
	if lx <= 0 {
		lx = len(p[lx])
	}
	chars := []string{".", "=", "|", "T", "S"}
	for j := 0; j < ly; j++ {
		for i := 0; i <= lx; i++ {
			if i == x && j == y {
				fmt.Print("X")
				continue
			}
			fmt.Print(chars[p[j][i].rtype])
		}
		fmt.Println()
	}
	fmt.Println()

}

// geologic index -> erosion level -> type

// The geologic index can be determined using the first rule that applies from the list below:
//
// The region at 0,0 (the mouth of the cave) has a geologic index of 0.
// The region at the coordinates of the target has a geologic index of 0.
// If the region's Y coordinate is 0, the geologic index is its X coordinate times 16807.
// If the region's X coordinate is 0, the geologic index is its Y coordinate times 48271.
// Otherwise, the region's geologic index is the result of multiplying the erosion levels of the regions at X-1,Y and X,Y-1.
//
// A region's erosion level is its geologic index plus the cave system's depth, all modulo 20183. Then:
//
// If the erosion level modulo 3 is 0, the region's type is rocky.
// If the erosion level modulo 3 is 1, the region's type is wet.
// If the erosion level modulo 3 is 2, the region's type is narrow.
func Populate(depth, tx, ty int) Plan {
	index := make(Plan, depth)
	for ix := range index {
		index[ix] = make([]region, tx+100)
	}

	for j := 0; j < len(index); j++ {
		for i := 0; i < len(index[j]); i++ {
			if j == ty && i == tx {
				index[j][i].geo = 0
			} else if i == 0 {
				index[j][i].geo = j * 48271
			} else if j == 0 {
				index[j][i].geo = i * 16807
			} else {
				index[j][i].geo = index[j-1][i].ero * index[j][i-1].ero
			}
			index[j][i].ero = (index[j][i].geo + depth) % 20183
			index[j][i].rtype = rtype(index[j][i].ero % 3)
		}
	}
	return index
}

type candidate struct {
	x, y, distance int
	gear           gear
	path           []candidate
	heur           int
}
type xyg struct {
	x, y int
	g    gear
}

type xy struct {
	x, y int
}

// Heap of candidates, ordered by distance
type Heap []candidate

func (h Heap) Len() int           { return len(h) }
func (h Heap) Less(i, j int) bool { return h[i].distance < h[j].distance }
func (h Heap) Swap(i, j int)      { h[i], h[j] = h[j], h[i] }

// Push adds a candidate to the heap
func (h *Heap) Push(x interface{}) {
	*h = append(*h, x.(candidate))
}

// Pop gets the best candidate from the heap
func (h *Heap) Pop() interface{} {
	old := *h
	n := len(old)
	x := old[n-1]
	*h = old[0 : n-1]
	return x
}

func manhattan(c0, c1 candidate) int {
	return int(math.Abs(float64(c0.x-c1.x)) + math.Abs(float64(c0.y-c1.y)))
}

func (p Plan) Dijkstra(tx, ty int) (int, candidate) {
	p[ty][tx].rtype = target
	p[0][0].rtype = start
	dists := map[xyg]int{}
	var (
		cand, c    candidate
		x, y       int
		cd         int
		found      bool
		candidates = &Heap{}
	)
	directions := []xy{{-1, 0}, {1, 0}, {0, 1}, {0, -1}}
	heap.Push(candidates, candidate{0, 0, 0, torch, make([]candidate, 0), 0})

	for candidates.Len() > 0 {
		cand = heap.Pop(candidates).(candidate)

		// fmt.Println(candidates.Len(), cand.distance)
		cd, found = dists[xyg{cand.x, cand.y, cand.gear}]
		if found && cd <= cand.distance {
			continue
		}
		dists[xyg{cand.x, cand.y, cand.gear}] = cand.distance

		if cand.x == tx && cand.y == ty {
			return cand.distance, cand
		}

		path := make([]candidate, 0)
		for _, i := range cand.path {
			path = append(path, i)
		}
		path = append(path, cand)

		for _, opt := range gearset {

			for _, d := range directions {
				y = cand.y + d.y
				x = cand.x + d.x
				if !p.isallowed(x, y, opt) || !p.isallowed(cand.x, cand.y, opt) {
					continue
				}

				c = candidate{x: x, y: y, distance: cand.distance + 1, gear: opt, path: path}
				if opt != cand.gear {
					c.distance += 7
				}

				if cd, found = dists[xyg{x, y, opt}]; found && cd <= c.distance {
					continue // We've already found a solution for this region
				}

				heap.Push(candidates, c)
			}

		}

	}
	panic("could not find a route")
}

func (p Plan) Solve(tx, ty int) int {
	sum := 0
	for j := 0; j <= ty; j++ {
		for i := 0; i <= tx; i++ {
			sum += int(p[j][i].rtype)
		}
	}
	return sum
}

func main() {
	// Profiling
	// f, err := os.Create("profile")
	// if err != nil {
	// 	log.Fatal(err)
	// }

	// pprof.StartCPUProfile(f)
	// defer pprof.StopCPUProfile()

	p := Populate(510, 10, 10)
	fmt.Println(p.Solve(10, 10))
	dist, cand := p.Dijkstra(10, 10)
	for ix, c := range cand.path {
		fmt.Println(ix, c.x, c.y, c.distance, str(c.gear))
		p.PrintX(c.x, c.y, 10, 15)
	}
	fmt.Println(cand.x, cand.y, cand.distance, str(cand.gear))
	p.PrintX(cand.x, cand.y, 10, 15)
	fmt.Println(dist, len(cand.path))
	p = Populate(11820, 7, 782)
	fmt.Println(p.Solve(7, 782))
	dist, cand = p.Dijkstra(7, 782)
	fmt.Println(dist)

	path := append(cand.path, cand)
	last := path[0]
	if last.gear != torch {
		panic("not starting with a torch")
	}
	cdist := 0
	for _, c := range path[1:] {
		// fmt.Printf("(%d,%d)->", last.x, last.y)
		cdist++
		if manhattan(c, last) != 1 {
			panic(fmt.Errorf("bad path, manhattan distance: %d", manhattan(c, last)))
		}
		if c.gear != last.gear {
			cdist += 7
		}
		if cdist != c.distance {
			panic(fmt.Errorf("bad path, distance: %d %d", cdist, c.distance))
		}
		if !p.isallowed(c.x, c.y, c.gear) || !p.isallowed(last.x, last.y, c.gear) {
			panic(fmt.Errorf("gear not allowed: %s @ (%d,%d)", str(c.gear), c.x, c.y))
		}
		last = c
	}

	if cdist != cand.distance {
		panic("wrong distance")
	}
	if cand.gear != torch {
		panic("not ending with a torch")
	}

}
