package main

import (
	"fmt"
	"testing"
)

var tcs = []struct {
	rtype rtype
	gear  gear
	exp   bool
}{
	{rocky, torch, true},
	{wet, torch, false},
	{narrow, torch, true},
	{rocky, torch | neither, true},
	{rocky, torch, true},
	{rocky, climb, true},
	{rocky, neither, false},
	{rocky, neither | climb, true},
	{wet, climb, true},
	{wet, neither, true},
	{wet, torch, false},
	{narrow, climb, false},
	{narrow, torch, true},
	{narrow, neither, true},
	{narrow, neither | torch, true},
	{narrow, climb, false},
	{target, climb, false},
	{target, neither, false},
	{target, torch, true},
}

var tcs2 = []struct {
	gear1, gear2 gear
	exp          gear
}{
	{torch | neither, climb | torch, torch},
	{climb, torch, 1},
}

func TestScoreSuite(t *testing.T) {
	for ix, tc := range tcs {
		t.Run(fmt.Sprintf("%s - %s", str(tc.rtype), str(tc.gear)), func(t *testing.T) {
			s := allowed(tc.rtype, tc.gear)
			if s != tc.exp {
				t.Fatalf("#%d '%s & %s'. expected: %t got: %t", ix, str(tc.rtype), str(tc.gear), s, tc.exp)
			}
		})
	}
	for ix, tc := range tcs2 {
		t.Run(fmt.Sprintf("%s - %s", str(tc.gear1), str(tc.gear2)), func(t *testing.T) {
			s := intersection(tc.gear1, tc.gear2)
			if s != tc.exp {
				t.Fatalf("#%d '%s & %s'. expected: %s got: %s", ix, str(tc.gear1), str(tc.gear2), str(s), str(tc.exp))
			}
		})
	}

}
