package main

import (
	"fmt"

	"golang.org/x/sync/errgroup"
)

func main() {
	hello()
}

var Printf = fmt.Printf

func hello() {
	g := &errgroup.Group{}
	for i := 0; i < 10; i++ {
		i := i
		g.Go(func() error {
			Printf("goroutine %02d\n", i)
			return nil
		})
	}
	g.Wait()
}
