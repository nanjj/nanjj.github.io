`2021-08-25 夜，晴`

在Emacs下，[Rust]开发有两个选择：
1. [Rust Mode], 官方自带，
2. [Rustic]，第三方所写。

该如何选择呢？
## Rust Mode选择

我选[Rust Mode]。这里说一下我的理由：
1. [Rust Mode]在README里推荐了[Rustic]，说明[Rust Mode]作者较为宽厚，
2. [Rustic]对自己分叉[Rust Mode]的行为进行了辩解，但没有多少站得住的理由，
3. [Rust Mode]只做最核心的内容，周边留给其他包做，这体现Unix编程哲学，
4. [Rustic]啥都做但有些做的不好，比如[Cargo]命令。

我用Emacs自带的Package管理：

```
package-install rust-mode
package-install cargo
```

我习惯把code format绑到`M-q`上：
```
(define-key rust-mode-map (kbd "M-q") 'rust-format-buffer)
```
`Cargo.el`所提供的命令`cargo-process-add`需要`cargo-edit`支持：
```
cargo install cargo-edit
```

至此，语法加亮，代码规整，常见命令支持都有了。还差一个代码补齐和跳转，
由Emacs的LSP能力补足。

## LSP的选择

Emacs下LSP(Language Server Protocol)有两个选择:
1. [LSP],
2. [Eglot]

这又如何选呢？

[LSP]我在沉迷Go语言的时候用过，怎么说呢？ [LSP]不坏，但远算不上优秀。
为提供酷炫功能，[LSP]用了一些不标准的Lisp API，以至于Emacs升级到28.0之
后，这些API被废弃掉了，[LSP]就常有抽风行为。我不可能为了一个勉强工作的
LSP把Emacs版本固守在27.0。于是我转向了[Eglot]：

```
package-delete lsp-mode
package-install eglot
```
我也认真读了[Eglot]作者与[LSP]作者的[讨论](https://github.com/joaotavora/eglot/issues/180)。

他们显然都不是平心静气，但如果你是一个好的程序员，你能很容易分辨出是非
高下来。于是我毫不犹豫地继续使用[Eglot]。

[Eglot]需要装上`rls`才能工作:
```
rustup update
rustup component add rls rust-analysis rust-src
```

给`rust-mode`加上`eglot`支持：
```
(add-hook 'rust-mode-hook 'eglot-ensure)
```
重启Emacs，搞定。最后写几句给程序员的话。

## 致程序员

我时常见到一些手上有咖喱味道的程序员，他们这也摸摸那也碰碰，把恶心味道
散播到各处。他们的代码之所以写，是为博眼球，而并非发自内心。他们算不上
纯粹的程序员。这没什么，世界仍然需要他们，有他们的存身之地。但他们假装
对代码、对技术的热爱就让纯粹程序员们恶心了。我想这就是[Eglot]作者烦的
不行[LSP]作者的原因。

[Rust Mode]: https://github.com/rust-lang/rust-mode
[Rustic]: https://github.com/brotzeit/rustic
[Rust]: https://rust-lang.org/
[Cargo]: https://cargo.io
[LSP]: https://github.com/emacs-lsp/lsp-mode
[Eglot]: https://github.com/joaotavora/eglot
