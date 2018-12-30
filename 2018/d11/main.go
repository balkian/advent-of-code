package main

import (
	"fmt"
)

const xdim, ydim = 300, 300
const code = 8561
//const code = 18
const block = 3

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

func getMax(grid [ydim][xdim]int) (xy){
	var pos xy
	var max int
	sum := [ydim][xdim]int{}

	for j:=0; j< ydim-block+1; j++ {
		for i:=0; i< xdim-block+1; i++ {
			for tj:=j;tj<j+block;tj++ {
				for ti:=i;ti<i+block;ti++ {
					sum[j][i] += grid[tj][ti]
				}
			}
			if sum[j][i] > max {
				max = sum[j][i]
				pos = xy{i,j}
			}
		}
	}
	return pos
}

func main(){
	g := initGrid()
	fmt.Println(getMax(g))

}
