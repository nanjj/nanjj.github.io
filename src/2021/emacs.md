最近[Emacs开发]少有的热闹，大家在讨论迁移[Emacs]开发到[sourcehut]的事。

## [Sourcehut]

[Sourcehut]是一个开源工具集搭建软件开发平台，提供如下能力：
1. [Git]和[Mercurial]代码仓托管，
2. 强大持续集成，
3. 基于邮件列表和[Git Send Mail]的代码检视与合并，
5. 基于邮件的任务（和缺陷）管理，
6. 久经考验的账户管理与安全，
7. Markdown和Git驱动的Wiki，
8. [代码分享](https://paste.sr.ht)，[静态页面托管](https://srht.site)，
   以及与[第三方服务集成](https://dispatch.sr.ht)

几乎每一条都对我特有吸引力！而且这所有几乎是[Drew DeVault]一人所为。于
是我就拿我的华为[花瓣邮箱]去注册，却迟迟收不到确认信。我怀疑是[花瓣邮
箱]的问题。

## [花瓣邮箱]

于是去检视[花瓣邮箱]，发现[花瓣邮箱]目前只支持手机客户端和Web客户端收
发信，并不支持POP3或IMAP以及SMTP协议。真不知道一个只支持手机客户端和
Web客户端的邮箱有什么用。

检视一下我现有的邮箱：

1. Gmail，
2. QQ Mail，
3. 139信箱。

Gmail在国内很难用。QQ Mail打开IMAP和SMTP协议支持需要下载QQ做一些安全的
加固。我在Linux下到哪里去找QQ呢？链接指向一个Windows的QQ，要我用Wine加
载吗？于是就放弃了。

最终发现139信箱做的中规中矩。在Emacs配置好IMAP后给[Drew DeVault]发了信，
问他确认信没收到的事。他很快回信，告诉我`petalmail.com`可能哪里配置的
不对，他发到`petalmail.com`的信被退回来了。

这里正告[花瓣邮箱]:
1. 老老实实把基本功能做好：邮箱一定要能收发信，
2. 老老实实去支持基本的协议：一定要支持IMAP和SMTP协议，方便第三方客户端配置，
3. 老老实实去写帮助文档，用一个智能助手去糊弄是不行的。

## Emacs的开发环境

回到迁移[Emacs]开发到[sourcehut]的事。这件事[Richard Stallman]也发话了，
明确不可以用[Sourcehut]提供的服务，但可以用[Sourcehut]提供的软件。

Emacs基本没有一个现代的CI流程来保证代码质量。[Richard Stallman]政治性
强，理想主义，很多时候漫无边际，又不切实际，也没给Emacs弄个像样的CI。
就算源码控制工具的选择都一波三折。直到2014年，才由[Eric S. Raymond]拨
乱反正，花了将近一年的时间，最终把Emacs拉到了Git的正轨上来。

如今[Sourcehut]横空出世，众Emacer可算盼到救星了。 [Lars Ingebrigtsen]
写到：

> 我有个梦想，当我提交一个变更，能有一个CI系统在一分钟内告诉我，在那些
> 系统上构建不过。但我猜这梦想不会实现，除非自由软件基金会有它应有的钱。

提起来让人心酸。这又算多高的梦想呢？又有多难实现呢？ [Óscar Fuentes]写
到：

> Emacs在我3.66GHz 8核构建机器上大约用时两分钟，这一半耗时是单线程所致
> （我对构建过程有一个长的分析）。因此降到一种似乎不难。
>
> 但说服某人买一台CI服务器估计美梦永不成真。

[Lars Ingebrigtsen]说：

> 是的，也只在梦中我有闲钱给自由软件基金会捐一台64核的服务器：）

## [Sourcehut]的CI

[Sourcehut]用虚拟机跑CI。构建失败后保留虚拟机10分钟，可以`ssh`上去看到
底出了啥事，为啥失败。

对比其他工具，如[Travis]等基于Docker容器的，有很大易用性方面的提高。但
基于虚机构建成本很高，而且用时较长（虚机从创建到启动到跑完CI，慢的可以
到20分钟）。


这块最合适的工具是系统容器[LXD]。这个提议等[Drew DeVault]通过了我的申
请，可以跟他提提。

[Emacs开发]: https://lists.gnu.org/archive/html/emacs-devel/2021-12/msg02220.html
[Sourcehut]: https://sourcehut.org/
[Git]: https://git-scm.com/
[Mercurial]: https://www.mercurial-scm.org/
[Git Send Mail]: https://git-send-email.io/
[花瓣邮箱]: https://www.petalmail.com/
[Drew DeVault]: https://drewdevault.com/
[Richard Stallman]: https://stallman.org
[Eric S. Raymond]: http://www.catb.org/~esr/
[Travis]: https://travis-ci.org
[LXD]: https://github.com/lxc/lxd
[Lars Ingebrigtsen]:https://lars.ingebrigtsen.no/

