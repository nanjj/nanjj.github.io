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

我拿我的华为[花瓣邮箱]注册一个账号，却迟迟收不到确认信。我有些怀疑[花
瓣邮箱]存在问题。

## [花瓣邮箱]

[花瓣邮箱]是华为手机升级鸿蒙后推出的一个应用。当时我刚好没有合适的主力
邮箱可用，就欣然接受，打算有时间把邮箱用起来。

### 国内邮箱

除了[花瓣邮箱]，我还有三个邮箱：
1. [Gmail]，
2. [QQ Mail]，
3. [139邮箱]。

### Gmail的使用

我之前主力邮箱是[Gmail]。[Gmail]在没有合适代理的情况下在国内很难直接使
用。我把[Gmail]邮箱配置在[QQ Mail]的手机客户端里，[QQ Mail]帮我翻墙出
去，我勉强可以收发信。可谓细若游丝，命悬一线地在用。使用场景也剩下唯一
的一个：发电子书到Kindle邮箱。亚马逊有个服务听在那，把附件转化后推到我
的Kindle上。

因此在国内[Gmail]作为主力邮箱是不合适的。

### 深入花瓣

如今仔细看[花瓣邮箱]，发现其目前只支持手机客户端和Web客户端收发信，不
支持任何一种协议包括POP3，IMAP以及SMTP协议。[花瓣邮箱]也没有帮助文档，
当你需要帮助的时候，“智能助手”跳出来回答你的问题，是问答式的，经常所
答非所问。勉强从一个问题回答的侧面推断出[花瓣邮箱]暂不支持第三方客户端，
也就是不支持POP3, IMAP以及SMTP协议。另外我迟迟收不到确认信。基本打消了
我把[花瓣邮箱]作为我的主力邮箱，配置在Emacs里使用的x想法。

### QQ邮箱

[QQ Mail]倒是支持主流协议，但需要打开一下。在打开IMAP和SMTP协议支持的
候，[QQ Mail]提示下载QQ，做一些安全方面的校验。但我在Linux下到哪里去下
载QQ呢？链接指向了一个Windows QQ下载。难道是要我用[Wine]将其灌醉，然后
加载运行吗？这种无脑设计让人不知道说什么。最终放弃扶正QQ邮箱的打算。

### 139邮箱

[139邮箱]提的口号是手机号就是邮箱。但手机号当邮箱使用又有手机暴露的风
险。这也是我一直没有怎么正经用的原因。如今回头重新看139信箱，发现它提
供了邮箱别名的功能。选名的过程中发现[139邮箱]帮助文档非常详实，各种协
议支持完备。打开协议支持也需要一些安全方面的验证，但是使用的是手机验证。
那这就方便多了。

在[Emacs]配好[139邮箱]后给[Drew DeVault]发信，问他确认信没收到的事。他
很快回信，告诉我花瓣可能哪里配置的不对，他发到我花瓣的信被退回来了。这
证实了我的猜测。

### 正告花瓣

这里正告[花瓣邮箱]:

1. 老老实实把基本功能做好
   
   邮箱一定要能收发信，收不到信的邮箱叫什么鬼邮箱？！
2. 老老实实去支持基本的协议

	支持IMAP和SMTP协议，方便第三方客户端配置，而不要贸然以云之名把这些
    历史沉淀下来的东西扬弃。
3. 老老实实去写帮助文档

   用一个所谓`智能助手`的东西去糊弄是不行的。



## Gnus

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
