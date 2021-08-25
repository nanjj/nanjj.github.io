# Rust再学习

`<2021-08-24 二>，雨夜`

## 缘起

几年前我曾经学习过[Rust]。其时，我发现[Rust]编译速度很慢，语法有些烧脑。
于是浅尝辄止一下[Rust]，很快就转向了[Go]语言，并深深迷恋。不知为何，当
时并没有注意到[Cargo]包管理的存在。

[Go]语言的包管理没有一步到位。[Go]最初包管理是分布式的，有些像Java
Classpath的设计，叫GOPATH，是线性的。而Java也渐渐引入OSGI的设计，进一
步模块化。[GO]包管理也出现许多第三方，最终统一为`Go Mod`。不能说`Go
Mod`多成功，只能算不失败罢了：中间走了弯路，最终也没有达到理想的状况，
但的确是工作的。

[Go]另一块薄弱项是和`C`语言的交互，上下文切换性能差太多了。这也是没有
办法的事，算引入`GC`的代价。但和`C`关系不好就阻碍了[Go]向下发展。有用
[Go]写操作系统的，但从没人提用[Go]开发Linux内核。

最近我发现[Cargo]设计干净，有美感，也实干，工程气息浓郁。
[BottleRocket]项目把[Cargo]用到了极致，但也远没有探到[Cargo]的上限。

## 在学

这次[Rust]再学，没有把语言放在前面学，而是先把周边搞清楚：
1. [Cargo]日常使用,
2. [Workspace], [Package], [Crate], [Module]依次搞清楚，
3. [Unit Testing]如何写，
4. [Rust Doc]以及[mdBook]

最后才慢慢看[Rust]的语法：
1. [Common Programming Concepts],
2. [Ownership], [Smart Pointers],
3. [Fearless Concurrency]

在看[mdBook]时候，顺便把本博客更新了下。

## 过往

读过去自己写的东西，是一个逐渐了解自己的过程。好似在读一封封来自过去的
信，有恍若隔世的感觉。过去自己的想法甚至能解当今的困惑。细细思索，倍感
神奇。

由此打算重拾写作，为将来的自己，提供一点寻找初心的线索。

[Rust]: https://rust-lang.org/
[Go]: https://golang.org/
[Cargo]: https://doc.rust-lang.org/cargo/index.html
[BottleRocket]: https://github.com/bottlerocket-os/bottlerocket/
[Workspace]: https://doc.rust-lang.org/cargo/reference/workspaces.html
[Package]: https://doc.rust-lang.org/cargo/appendix/glossary.html#package
[Crate]: https://doc.rust-lang.org/cargo/appendix/glossary.html#crate
[Module]: https://doc.rust-lang.org/cargo/appendix/glossary.html#module
[Unit Testing]: https://doc.rust-lang.org/book/ch11-01-writing-tests.html
[Rust Doc]: https://doc.rust-lang.org/stable/rust-by-example/meta/doc.html
[mdbook]: https://rust-lang.github.io/mdBook/index.html
[Ownership]: https://doc.rust-lang.org/book/ch04-00-understanding-ownership.html
[Common Programming Concepts]:https://doc.rust-lang.org/book/ch03-00-common-programming-concepts.html
[Fearless Concurrency]: https://doc.rust-lang.org/book/ch16-00-concurrency.html
[Smart Pointers]: https://doc.rust-lang.org/book/ch15-00-smart-pointers.html
