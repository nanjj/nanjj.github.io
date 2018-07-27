package main

import (
	"fmt"
	"reflect"
	"testing"
)

func TestHello(t *testing.T) {
	lines := make([]string, 10)
	Printf = func(format string, a ...interface{}) (n int, err error) {
		i := a[0].(int)
		lines[i] = fmt.Sprintf(format, a...)
		return len(lines[i]), nil
	}
	hello()
	want := []string{
		"goroutine 00\n",
		"goroutine 01\n",
		"goroutine 02\n",
		"goroutine 03\n",
		"goroutine 04\n",
		"goroutine 05\n",
		"goroutine 06\n",
		"goroutine 07\n",
		"goroutine 08\n",
		"goroutine 09\n",
	}
	if !reflect.DeepEqual(want, lines) {
		t.Fatal(want, lines)
	}
}
