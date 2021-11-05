package main

import (
	"bufio"
	"fmt"
	"io/ioutil"
	"math"
	"os"
	"os/exec"
	"runtime"
	"sort"
	"strings"
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

type tile int

const (
	missing tile = '?'
	blank   tile = ' '  // blank
	ud      tile = '|'  // vertical line
	lr      tile = '-'  // horizontal line
	dr      tile = '/'  // curve down to right
	ld      tile = '\\' // curve left to down
	cross   tile = '+'  // intersection
	crash   tile = 'X'  // crash

	cl tile = '<' // cart going up
	cr tile = '>' // cart going right
	cu tile = '^' // cart going up
	cd tile = 'v' // cart going down
)

var output = map[tile]map[xy]xy{
	lr: map[xy]xy{
		left:  right,
		right: left,
	},
	ud: map[xy]xy{
		up:   down,
		down: up,
	},
	dr: map[xy]xy{
		down:  right,
		right: down,
		left:  up,
		up:    left,
	},
	ld: map[xy]xy{
		left:  down,
		down:  left,
		up:    right,
		right: up,
	},
	cross: map[xy]xy{
		left:  any,
		right: any,
		up:    any,
		down:  any,
	},
}

var (
	up    = xy{x: 0, y: -1}
	down  = xy{x: 0, y: 1}
	right = xy{x: 1, y: 0}
	left  = xy{x: -1, y: 0}
	stay  = xy{}
	any   = xy{math.MinInt16, -math.MinInt16}
)

type cart struct {
	pos   xy
	dir   xy
	turns int
}

var cartGlyphs = map[xy]tile{
	up:    cu,
	down:  cd,
	right: cr,
	left:  cl,
}

func (c cart) String() string {
	return string(cartGlyphs[c.dir])
}

func (c *cart) Move() {
	if c.dir == any {
		panic("stopped cart")

	}
	c.pos = c.pos.Add(c.dir)
}

func (c *cart) Follow(t tile) {
	if t != cross {
		if t == blank {
			panic(fmt.Errorf("derrailed %s", c))
		}
		c.dir = output[t][c.dir.Invert()]
		return
	}
	switch c.turns % 3 {
	case 0:
		c.dir = xy{c.dir.y, -c.dir.x}
	case 1:
		c.dir = c.dir
	case 2:
		c.dir = xy{-c.dir.y, c.dir.x}
	}
	c.turns++
}

type roads [][]tile

func (r roads) get(pos xy) tile {
	if pos.y < 0 || pos.x < 0 || pos.y >= len(r) || pos.x >= len(r[pos.y]) {
		return missing
	}
	return r[pos.y][pos.x]
}

func (r roads) size() (int, int) {
	x := 1
	y := len(r)
	if y > 0 {
		x = len(r[0])
	}
	return x, y
}

func (r *roads) set(pos xy, val tile) {
	(*r)[pos.y][pos.x] = val
}

type board struct {
	roads roads
	carts []*cart
}

func (b board) String() string {
	s := strings.Builder{}
	plan := map[xy]*cart{}
	for _, c := range b.carts {
		plan[c.pos] = c
	}
	for y, row := range b.roads {
		for x, cell := range row {
			c, ok := plan[xy{x, y}]
			if ok {
				s.WriteString(c.String())
				continue
			}
			s.WriteString(string(cell))
		}
		s.WriteString("\n")
	}
	// for i, c := range b.carts {
	// 	s.WriteString(fmt.Sprintf("Cart %d @ [%s]%s\n", i, c.pos, c.dir))
	// }
	return s.String()
}

func NewBoard(input string) board {
	roads := make([][]tile, 0)
	carts := make([]*cart, 0)
	s := bufio.NewScanner(strings.NewReader(input))
	for y := 0; s.Scan(); y++ {
		row := make([]tile, 0)
		for x, val := range s.Text() {
			pos := xy{x, y}
			switch t := tile(val); t {
			case blank:
				fallthrough
			case ud:
				fallthrough
			case lr:
				fallthrough
			case dr:
				fallthrough
			case ld:
				fallthrough
			case cross:
				row = append(row, t)
			case cl:
				carts = append(carts, &cart{pos: pos, dir: left})
				row = append(row, missing)
			case cr:
				carts = append(carts, &cart{pos: pos, dir: right})
				row = append(row, missing)
			case cd:
				carts = append(carts, &cart{pos: pos, dir: down})
				row = append(row, missing)
			case cu:
				carts = append(carts, &cart{pos: pos, dir: up})
				row = append(row, missing)
			default:
				panic(fmt.Errorf("unknown character %c", t))
			}
		}
		roads = append(roads, row)
	}
	b := board{roads: roads, carts: carts}
	b.fix()
	return b
}

func (b *board) fix() {
	roads := b.roads
	for y, row := range roads {
		for x, t := range row {
			if t == missing {
				pos := xy{x, y}
				matches := make([]tile, 0)
				l := output[roads.get(pos.left())][right]
				d := output[roads.get(pos.down())][up]
				u := output[roads.get(pos.up())][down]
				r := output[roads.get(pos.right())][left]
				if l != stay { // piece to the left is connected on the right side
					if d != stay {
						matches = append(matches, ld)
					}
					if u != stay {
						matches = append(matches, dr)
					}
					if r != stay {
						matches = append(matches, lr)
					}
				}
				if r != stay { // piece to the right is connected on the left side
					if u != stay {
						matches = append(matches, ld)
					}
					if d != stay {
						matches = append(matches, dr)
					}
				}
				if u != stay && d != stay {
					matches = append(matches, ud)
				}

				if len(matches) > 1 {
					roads.set(pos, cross)
					continue
				}
				roads.set(pos, matches[0])
			}
		}
	}
	b.roads = roads
}

func (b *board) Step() []xy {
	crashed := make([]xy, 0)
	sort.Slice(b.carts, func(i, j int) bool { return b.carts[i].pos.Less(b.carts[j].pos) })
loop:
	for ix, c := range b.carts {
		if c == nil {
			continue
		}
		c.Move()
		t := b.roads.get(c.pos)
		for jx, c2 := range b.carts {
			if ix == jx || c2 == nil {
				continue
			}
			if c.pos == c2.pos {
				crashed = append(crashed, c.pos)
				// b.roads.set(c.pos, crash)
				b.carts[ix] = nil
				b.carts[jx] = nil
				continue loop
			}
		}
		c.Follow(t)
	}
	nc := make([]*cart, 0)
	for _, c := range b.carts {
		if c == nil {
			continue
		}
		nc = append(nc, c)
	}
	b.carts = nc
	return crashed

}

var clear map[string]func() //create a map for storing clear funcs

func init() {
	clear = make(map[string]func()) //Initialize it
	clear["linux"] = func() {
		cmd := exec.Command("clear") //Linux example, its tested
		cmd.Stdout = os.Stdout
		cmd.Run()
	}
	clear["windows"] = func() {
		cmd := exec.Command("cmd", "/c", "cls") //Windows example, its tested
		cmd.Stdout = os.Stdout
		cmd.Run()
	}
}

func CallClear() {
	value, ok := clear[runtime.GOOS] //runtime.GOOS -> linux, windows, darwin etc.
	if ok {                          //if we defined a clear func for that platform:
		value() //we execute it
	} else { //unsupported platform
		panic("Your platform is unsupported! I can't clear terminal screen :(")
	}
}

func main() {
	i, err := ioutil.ReadFile("input.txt")
	if err != nil {
		panic("could not read input")
	}
	b := NewBoard(string(i))
	var crash []xy
	debug := false
	for i := 0; true; i++ {
		if debug {
			CallClear()
			fmt.Println(b)
			reader := bufio.NewReader(os.Stdin)
			text, _ := reader.ReadString('\n')
			if text == "c\n" {
				debug = false

			}
		}
		crash = b.Step()
		if len(crash) > 0 {
			fmt.Println("Crashed at:")
			for ci, c := range crash {
				fmt.Printf("\t %d: %s\n", ci, c)
			}
		}
		if len(b.carts) == 1 {
			fmt.Printf("Last cart standing: %s\n", b.carts[0].pos)
			break
		} else if len(b.carts) < 2 {
			fmt.Println("No carts left")
			break
		}
	}
	fmt.Println("Done")
}
