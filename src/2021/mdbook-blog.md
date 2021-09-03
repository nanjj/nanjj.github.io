# Mdbook预处理插件

我学了点[Rust]语言，用[Rust]语言写了个[Mdbook]预处理插件。

## 预处理插件

[Mdbook]预处理插件是一个可执行程序，[Mdbook]做预处理时会把markdown内容
以及配置通过标准输入发给它，它做预处理，把结果输出到标准输出。

以下是本博客的`book.toml`配置：
```
{{#include ../../book.toml}}
```

可以看到我写的预处理插件配置为：
```
[preprocessor.blog]
```

这样[Mdbook]会通过标准输入发配置上下文和书的内容给`mdbook-blog`，
`mdbook-blog`做预处理，把结果打到标准输出。

因此我需要写这个`mdbook-blog`,`Cargo.toml`:
```
{{#include ../../Cargo.toml}}
```

实现在`src/main.rs`:
```
{{#include ../main.rs}}
```

## [Mdbook]与博客

[Mdbook]为写书而有，并非为博客所写。我把我的博客架构在[Mdbook]上也就是
图省事。博客需要有一个首页，一般的首页是把最新几篇摘要生成一个列表放在
那。我这个插件是把首页重定向到最新一篇。这实现简单，而且用起来还不错。

## [Rust]的学习

这个[Mdbook]插件是我用[Rust]写的第一个程序。通过这个程序的编写，我对
[Rust]的包结构，模块组织，基本语法以及错误处理有了点认识。另外着重看了
下[Rust]的[自动测试]。

[Rust]把自动测试分为3类:
1. 单元测试，
2. 文档测试，
3. 集成测试。

其中文档测试是其他语言所不具备的。写在文档里的例子，在`cargo test`的时
候也会被执行。

单元测试如何做，敏捷开发界有许多争议，比如私有方法是否要覆盖之类。但单
元测试是否要做，大概是没有争议的。[Rust]选了比较实际的路线，把选择权给
了开发。

集成测试其实就是接口测试，只测对外暴露的接口。`libs`集成测试没问题，
`binary`没有对外接口，咋做呢？[Rust]给了个方案，同时后`src/main.rs`和
`src/libs.rs`，让`binary`也可以成为`libs`，这样问题就解决了。

综合其上，三种测试统称为自动测试。一个遗留问题是代码覆盖率，不知如何做。

[Rust]: https://kaisery.github.io/trpl-zh-cn/
[Mdbook]: https://rust-lang.github.io/mdBook/
[自动测试]: https://doc.rust-lang.org/book/ch11-00-testing.html


