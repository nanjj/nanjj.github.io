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

于是重回Gnus怀抱。配置信箱，订阅邮件列表，写信，读信。

读Gnus的info文档，拉倒最后，有一首小诗：

```
     *Te Deum*


     Not because of victories
     I sing,
     having none,
     but for the common sunshine,
     the breeze,
     the largess of the spring.


     Not for victory
     but for the day’s work done
     as well as I was able;
     not for a seat upon the dais
     but at the common table.

```

这首诗让我莫名感动。

## 新闻组

新闻组已死。国内只剩新帆（news.newsfan.net）一息尚存，也没啥有用信息。
Emacs Gnus对新闻组里中文的支持，在历次重构过程中给弄没了。有人在开发邮
件列表里提讨论，可能有很好的挽救方案。国外的新闻组感觉也没啥新内容。但
邮件列表的确是很好的沟通方式。

## 邮件列表

Emacs开发一直用邮件列表沟通。除此外我感兴趣的，比如：
1. [Linux Containers],
2. [Rust Lang],
3. [Graphviz]

等，背后都是邮件列表的运作模式：
1. 一个Web前端， 满足习惯Web界面的用户需求，
2. 可订阅的邮件列表，满足习惯邮件订阅的用户需求。

我觉得对程序员而言，对散布在世界各地的程序员而言，邮件仍然是最有效的沟
通方式。国内常用微信群，有效的沟通很难展开，有价值的信息很快淹没在信息
的洪流里。还有就是微信公众号，是一种单向沟通，离协作差得远了。

因此还是简单干净的邮件，把大家联系起来。通过邮件协作，通过邮件驱动学习。
驱动开发，以至于整个开发流程，可能是一个现实的方向。

## Emacs开发

回到迁移[Emacs]开发到[sourcehut]的事。这件事[Richard Stallman]也发话了，
明确不可以用[Sourcehut]提供的服务，但可以用[Sourcehut]提供的软件。

Emacs基本没有一个现代的CI流程来保证代码质量。[Richard Stallman]政治性
强，理想主义，很多时候不切实际，也没给Emacs弄个像样的CI。就算源码控制
工具的选择都一波三折。直到2014年初，才由[Eric S. Raymond]拨乱反正，花
了将近一年的时间，到2014年底，把Emacs拉到了Git的正轨上来。

如今[Sourcehut]横空出世，众Emacer可算盼到救星了。 [Lars Ingebrigtsen]
写到：

> 我有个梦想，当我提交一个变更，能有一个CI系统在一分钟内告诉我，在哪些
> 系统上构建不过。但我猜这梦想不会实现，除非自由软件基金会有它应有的钱。

提起来让人心酸。这又算多高的梦想呢？又有多难实现呢？ Óscar Fuentes写到：

> Emacs在我3.66GHz 8核构建机器上大约用时两分钟，这一半耗时是单线程所致
> （我对构建过程有一个长的分析）。因此降到一分钟似乎不难。
>
> 但说服某人买一台CI服务器估计美梦永不成真。

[Lars Ingebrigtsen]说：

> 是的，也只在梦中我有闲钱，给自由软件基金会捐一台64核的服务器：）

## [builds.sr.ht]

[builds.sr.ht]用虚拟机跑CI。构建失败后保留虚拟机10分钟，可以`ssh`上去
看到底出了啥事，为啥失败。

对比其他工具，如[Travis]等基于Docker容器的，有很大易用性方面的提高。但
基于虚机构建成本很高，而且用时较长（虚机从创建到启动到跑完CI，慢的可以
到20分钟）。

这块最合适是系统容器[LXD]。我把这个提议提给了[Drew DeVault]，但没得到
他的回应。我在等待回应的时间里，大致看了他的实现。他任务跑在虚机里，虚
机跑在Docker容器里，Docker容器跑在物理机上。这样他使用了Docker的镜像能
力，也更安全。只是成本还是太高。

反复考察，我感觉[LXD]系统容器还是更适合跑任务。

## [srht.site]

[srht.site]对标github的pages功能。对外只有一个API：
```
curl --oauth2-bearer secret \
	-Fcontent=site.tar.gz https://pages.sr.ht/publish/nanjj.srht.site
```
｀site.tar.gz`是全量网站的打包。这命令还有一个变种：
```
curl --oauth2-bearer secret \
	-Fcontent=sub.tar.gz https://pages.sr.ht/publish/nanjj.srht.site/sub
```
用于更新网站的一个子目录。

这基本就足够了，实现好了足以对标github。

我把自己的小站搬到了[srht.site]:

https://nanjj.srht.site

发现两个问题。

第一个问题，[srht.site]不支持js跨域。

我的站点有两个需要跨域的功能：
1. [MathJax]，
2. [Utteranc]。

[MathJax]其实好办，拿过来就是。[Utteranc]就不好弄了。还好我看留言并不
多，于是干脆去掉了。

第二个问题是[srht.site]没有用浏览器的cache功能，导致每次加载都要重新下载，以至于和

https://nanjj.github.io

比较就慢多了。 这个问题我感觉[srht.site]可以优化。在第二个问题解决之前，
我的小站继续以github为主。

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
[srht.site]: https://srht.site
[mathjax]: https://mathjax.org
[utteranc]: https://utteranc.es
[builds.sr.ht]: https://builds.sr.ht
[Linux Containers]: https://discuss.linuxcontainers.org/
[Rust Lang]: https://users.rust-lang.org/
[graphviz]: https://forum.graphviz.org/
