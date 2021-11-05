package main

import (
	"fmt"
	"testing"
)

var tcs = []struct {
	in  int
	exp int
}{
	{
		in:  5,
		exp: 124515891,
	},
	{
		in:  18,
		exp: 9251071085,
	},
	{
		in:  2018,
		exp: 5941429882,
	},
}

func TestScoreSuite(t *testing.T) {
	for _, tc := range tcs {
		t.Run(string(tc.in), func(t *testing.T) {
			s, ss := score(tc.in)
			if s != tc.exp {
				fmt.Println(ss)
				t.Errorf("result for %d  is %d != %d", tc.in, s, tc.exp)
			}
			fmt.Println("Test passed for", tc.in)
		})
	}

}

var tcs2 = []struct {
	in  string
	exp int
}{
	{
		in:  "51589",
		exp: 9,
	},
	{
		in:  "01245",
		exp: 5,
	},
	{
		in:  "92510",
		exp: 18,
	},
	{
		in:  "59414",
		exp: 2018,
	},
}

func TestBackwardsSuite(t *testing.T) {
	for _, tc := range tcs2 {
		t.Run(tc.in, func(t *testing.T) {
			s := backwards(tc.in)
			if s != tc.exp {
				t.Errorf("result for %s is %d != %d", tc.in, s, tc.exp)
			}
			fmt.Println("Test passed for", tc.in)
		})
	}
}
