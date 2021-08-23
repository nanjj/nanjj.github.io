# Go Debug

`<2018-07-27 Fri>`

How to do go debug.

## Background

Go has no its own debugger so it turned to gdb. See [Debugging Go Code
with GDB](https://golang.org/doc/gdb) for details.

But GDB does not understand Go programs well. As a better alternative,
[Delve](https://github.com/derekparker/delve) has its place.

## Installation

```
go get github.com/derekparker/delve/cmd/dlv
```

## Code to Debug

`main.go`:

```go
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
        g.Go(func() error {
            Printf("goroutine %02d\n", i)
            return nil
        })
    }
    g.Wait()
}
```

`main_test.go`:

```go
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
```

## Debugging Practices

We will learn:
1. How to set breakpoints
   1. `break main.main`,
   2. `break TestHello`,
   3. `break db.go:27`,
2. How to switch goroutines
   1. `goroutines` to list goroutines,
   2. `goroutine 18` to switch goroutine
3. How to debug a service

## Debug a Program

```bash
$ dlv debug
Type 'help' for list of commands.
(dlv) break main.main
Breakpoint 1 set at 0x10b1faf for main.main() ./main.go:9
(dlv) continue
> main.main() ./main.go:9 (hits goroutine(1):1 total:1) (PC: 0x10b1faf)
     4:         "fmt"
     5:
     6:         "golang.org/x/sync/errgroup"
     7: )
     8:
=>   9: func main() {
    10:         hello()
    11: }
    12:
    13: var Printf = fmt.Printf
    14:
```

## Debug Test Case

```bash
dlv test
Type 'help' for list of commands.
(dlv) break TestHello
Breakpoint 1 set at 0x1137b4b for _/...TestHello() ./main_test.go:9
(dlv) continue
> _.../hello.TestHello() ./main_test.go:9 (hits goroutine(5):1 total:1) (PC: 0x1137b4b)
     4:         "fmt"
     5:         "reflect"
     6:         "testing"
     7: )
     8:
=>   9: func TestHello(t *testing.T) {
    10:         lines := make([]string, 10)
    11:         Printf = func(format string, a ...interface{}) (n int, err error) {
    12:                 i := a[0].(int)
    13:                 lines[i] = fmt.Sprintf(format, a...)
    14:                 return len(lines[i]), nil
(dlv) break 12
Breakpoint 2 set at 0x1138101 for _.../hello.TestHello.func1() ./main_test.go:12
(dlv) continue
> _...hello.TestHello.func1() ./main_test.go:12 
(hits goroutine(15):1 total:1) (PC: 0x1138101)
     7: )
     8:
     9: func TestHello(t *testing.T) {
    10:         lines := make([]string, 10)
    11:         Printf = func(format string, a ...interface{}) (n int, err error) {
=>  12:                 i := a[0].(int)
    13:                 lines[i] = fmt.Sprintf(format, a...)
    14:                 return len(lines[i]), nil
    15:         }
    16:         hello()
    17:         want := []string{
(dlv) print a
[]interface {} len: 1, cap: 1, [
        10,
]
```

## Debug a Service

```bash
$ icp-cland&
$ ps -ef|grep icp-cland
501 89630  3744   0  2:44PM ttys002    0:00.34 icp-cland
$ dlv attach 89630
Type 'help' for list of commands.
(dlv) break github..../cland/icp-cland/icp/service/admin/db.go:27
Breakpoint 1 set at 0x47c771e for github...
(dlv) continue
> github..../cland/icp-cland/icp/...
Warning: debugging optimized function
    22:         *logging.Tracer
    23: }
    24:
    25: func (dba *dbAdmin) Purge(ctx context.Context,
    26:         req *dbs.PurgeRequest) (rep *dbs.PurgeReply, err error) {
=>  27:         sp, ctx, logger := dba.StartSpanFromContext(ctx, "DBAdminPurge")
    28:         defer sp.Finish()
    29:         rep = &dbs.PurgeReply{}
    30:         filter := req.GetFilter()
    31:         var purged int64
    32:         purged, err = model.Purge(filter)
```
