package main

import (
	"fmt"
	"testing"
)

var tcs = []struct {
	in  string
	exp int
}{
	{
		in:  "^WNE$",
		exp: 3,
	},
	{
		in:  "^ENWWW(NEEE|SSE(EE|N))$",
		exp: 10,
	}, {
		in:  "^ENNWSWW(NEWS|)SSSEEN(WNSE|)EE(SWEN|)NNN$",
		exp: 18,
	}, {
		in:  "^WSSEESWWWNW(S|NENNEEEENN(ESSSSW(NWSW|SSEN)|WSWWN(E|WWS(E|SS))))$",
		exp: 31,
	},
}

func TestScoreSuite(t *testing.T) {
	for _, tc := range tcs {
		t.Run(string(tc.in), func(t *testing.T) {
			s := dist(tc.in)
			if s != tc.exp {
				t.Errorf("result for %s is %d != %d", tc.in, s, tc.exp)
			}
			fmt.Println("Test passed for", tc.in)
		})
	}

}
