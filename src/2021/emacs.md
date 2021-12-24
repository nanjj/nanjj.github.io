<div style="text-align: right"><code>2021年12月25日 星期六 上午 阴</code></div>

最近[Emacs开发]少有热闹，大家在讨论迁移[Emacs]开发到[sourcehut]的事。

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

几乎每一条都对我特有吸引力！而且竟然是[Drew DeVault]一人所为！

我拿我的华为[花瓣邮箱]注册一个账号，却迟迟收不到确认信。我怀疑是[花瓣
邮箱]的问题。

## [花瓣邮箱]

[花瓣邮箱]是华为手机升级鸿蒙后推出的一个应用。当时我刚好没有合适的邮箱，
就接受了。简单使用还是可以的，没有仔细看。

如今仔细看[花瓣邮箱]，发现其目前只支持手机客户端和Web客户端收发信，并
不支持POP3或IMAP以及SMTP协议。这对一般手机用户可能也是可以的。另外[花
瓣邮箱]没有帮助文档，有一个智能助手的东西，是问答式的，经常所答非所问。
勉强从一个问题回答的侧面推断出[花瓣邮箱]暂不支持第三方客户端，也就是不
支持POP3, IMAP以及SMTP协议。

除了[花瓣邮箱]，我还有三个邮箱：

1. [Gmail]，
2. [QQ Mail]，
3. [139邮箱]。

[Gmail]在国内很难直接使用，可以配置在[QQ Mail]间接使用，因此作为主力邮
箱使用是不合适的。

转到[QQ Mail]。在打开IMAP和SMTP协议支持的候，[QQ Mail]提示下载QQ，做一
些安全方面的加固。但我在Linux下到哪里去下载QQ呢？链接指向了一个Windows
QQ下载。难道是要我用[Wine]将其灌醉，然后加载运行吗？这种没脑子设计让人
无语。

最终发现[139邮箱]做的中规中矩。[139邮箱]提的口号是手机号就是邮箱。但手
机号当邮箱使用又有手机号暴露的风险。还好有邮箱别名可用。[139邮箱]的帮
助文档很详实。各种协议支持完备。

在[Emacs]配好IMAP后给[Drew DeVault]发信，问他确认信没收到的事。他很快
回信，告诉我`petalmail.com`可能哪里配置的不对，他发到`petalmail.com`的
信被退回来了。这证实了我的猜测。

这里正告[花瓣邮箱]:

1. 老老实实把基本功能做好
   
   邮箱一定要能收发信，收不到信的邮箱叫什么鬼邮箱？！
2. 老老实实去支持基本的协议

	支持IMAP和SMTP协议，方便第三方客户端配置，而不要贸然以云之名把这些
    历史沉淀下来的东西扬弃。
3. 老老实实去写帮助文档

   用一个所谓`智能助手`的东西去糊弄是不行的。

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

提起来让人心酸。这又算多高的梦想呢？又有多难实现呢？ Óscar Fuentes写到：

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
[Emacs]: http://www.gnu.org/software/emacs/
[QQ Mail]: https://mail.qq.com
[Gmail]: https://mail.gmail.com
[139邮箱]: https://mail.10086.cn/
[Wine]: https://www.winehq.org/
