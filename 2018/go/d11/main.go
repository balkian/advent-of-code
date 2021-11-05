package main

import (
	"fmt"
)

const xdim, ydim = 300, 300
const code = 8561
//const code = 18

func initGrid() [ydim][xdim]int {
	grid := [ydim][xdim]int{}
	var rack_id, num, hunds int
	for j:=0; j<ydim; j++ {
		for i:=0; i<xdim; i++ {
			rack_id = (i+1)+10
			num = ((rack_id*(j+1)+code)*rack_id)/100
			hunds = num - (num/10)*10
			grid[j][i] = hunds - 5
		}

	}
	return grid
}

type xy struct {
	x, y int 
}

func integrate(grid [ydim][xdim]int) (sum [ydim][xdim]int){
	sum = [ydim][xdim]int{}

	var temp int

	for j:=0; j<ydim; j++ {
		for i:=0; i< xdim; i++ {
			temp = grid[j][i]
			if i > 0 {
				temp += sum[j][i-1]
			}
			if j > 0 {
				temp += sum[j-1][i]
				if i > 0 {
					temp -= sum[j-1][i-1]
				}
			}
			sum[j][i] = temp
		}
	}
	return sum
}

func getMax(grid [ydim][xdim]int, block int) (pos xy, power int){
	var max int

	var val int
	ahead := block-1

	for j:=0; j< ydim-ahead; j++ {
		for i:=0; i< xdim-ahead; i++ {
			val = grid[j+ahead][i+ahead]
			if i > 0 {
				val -= grid[j+ahead][i-1]
			}
			if j > 0 {
				val -= grid[j-1][i+ahead]
				if i > 0 {
					val += grid[j-1][i-1]
				}
			}
			if val > max {
				max = val
				pos = xy{i,j}
			}
		}
	}
	return pos, max
}

func main(){
	g := initGrid()
	sum := integrate(g)
	fmt.Println(getMax(sum, 3))
	max := 0
	bs := 0
	var pos xy
	for i:=1;i<300;i++ {
		tp, tm := getMax(sum, i)
		if tm > max {
			max = tm
			bs = i
			pos = tp
		}
	}
	fmt.Println(pos.x, pos.y, bs)

}
