package main

import (
	"bufio"
	"fmt"
	"os"
	"strings"
)

// Point is a 4-dimensional point in space
type Point [4]int

func (p Point) neighborsMap(n int, ns map[Point]bool) map[Point]bool {
	if ns == nil {
		ns = map[Point]bool{}
	}
	if n <= 0 {
		ns[p] = true
		return ns
	}
	ns[p] = true
	// In every direction, move 1
	for ix := range p {
		np := p
		np[ix] = p[ix] + 1
		ns = np.neighborsMap(n-1, ns)

		np = p
		np[ix] = p[ix] - 1
		ns = np.neighborsMap(n-1, ns)
	}
	return ns
}

func (p Point) neighbors(n int) []Point {
	ns := p.neighborsMap(n, nil)
	ps := make([]Point, 0)
	for n, ok := range ns {
		if !ok {
			continue
		}
		ps = append(ps, n)
	}
	return ps
}

// Label for the partitions
type Label int

var nolabel Label

// Partition stores points and the group to which they belong
type Partition struct {
	point Point
	label Label
	other map[int]*Partition
}

// NewPartition returns a new partition, ready to be used
func NewPartition() *Partition {
	return &Partition{
		other: map[int]*Partition{}}
}

// Get the partition label for a given point
func (d *Partition) Get(pos Point) (Label, bool) {
	for _, p := range pos {
		other, ok := d.other[p]
		if !ok {
			return Label(0), false
		}
		d = other
	}
	return d.label, true
}

// Set the partition label of a point
func (d *Partition) Set(pos Point, l Label) {
	t := d
	for _, p := range pos {
		other, ok := t.other[p]
		if !ok {
			other = NewPartition()
			t.other[p] = other
		}
		t = other
	}
	t.point = pos
	t.label = l
}

// Propagate labels (not really l-prop, tho)
func (d *Partition) Propagate() {
	visited := map[Point]bool{}
	for _, part := range d.leaves() {
		parent := part.point
		l := part.label
		pending := []Point{parent}
		for len(pending) > 0 {
			p := pending[0]
			pending = pending[1:]
			if _, v := visited[p]; v {
				// Already visited
				continue
			}
			visited[p] = true
			d.Set(p, l)
			for _, n := range p.neighbors(3) {
				_, ok := d.Get(n)
				if !ok {
					continue
				}
				pending = append(pending, n)
			}
		}
	}
}

func (d Partition) leaves() []Partition {
	ls := make([]Partition, 0)

	if d.label != nolabel {
		ls = append(ls, d)
		return ls
	}
	for _, o := range d.other {
		ls = append(ls, o.leaves()...)
	}
	return ls
}

// Groups gets the map from label to points with that label
func (d Partition) Groups() map[Label][]Point {
	m := map[Label][]Point{}
	for _, p := range d.leaves() {
		t, ok := m[p.label]
		if !ok {
			t = make([]Point, 0)
		}
		t = append(t, p.point)
		m[p.label] = t
	}
	return m
}

func (d Partition) String() string {
	s := strings.Builder{}
	for _, l := range d.leaves() {
		s.WriteString(fmt.Sprintf("%v %d\n", l.point, l.label))
	}
	return s.String()
}

func main() {
	f, err := os.Open("input.txt")
	if err != nil {
		panic("could not read input")
	}
	r := bufio.NewScanner(f)
	d := NewPartition()
	ps := make([]Point, 0)
	var x, y, z, t int
	for i := 0; r.Scan(); i++ {
		fmt.Sscanf(r.Text(), "%d,%d,%d,%d", &x, &y, &z, &t)
		p := Point{x, y, z, t}
		d.Set(p, Label(i+1))
		ps = append(ps, p)
		// fmt.Println(x, y, z, t)
	}
	d.Propagate()
	fmt.Println(len(ps), len(d.Groups()))
}
