## 0
AI大模型来了，Fabrice Bellard用[TextSynth](https://textsynth.com)给了
他的想法。

> flying dragon breathing fire: 
> ![](./flying-dragon-breathing-fire.jpg)

## 1

他把AI大模型分为三类：
1. 补全（Completion），
2. 翻译（Translation），
3. 图像生成（Image Generation）。

然后定了一套REST JSON API与之交互。

这没啥，那些做云计算，云服务的所谓专家们，可能都比他做得好。这不是大神
该做的工作。

## 2

他写了套框架来支撑AI大模型的运行。使得巨大的AI大模型，在笔记本上也可以
运行，就算没有高端GPU也可以运行，就算只有CPU也可以运行。我在我笔记本上，
运行了5分钟，勉强算出上面的图。

这有些了不起。但也不算太出奇。比如Tabnine的离线版本也可以做到。Tabnine
的AI大模型算补全类的。补全类的AI大模型在TextSynth下运行的也很好。他强
在支持很多的AI大模型。因此他可能找到了通用的方法。但他没说，只以MIT协
议发布了二进制版本的[LibNC](https://bellard.org/libnc/),没有开源。

因此这是一个核心问题：如何使用较低成本资源运行AI大模型。

## 3

他提供将近100种AI大模型。这些模型是怎么训练出来的？他没有说。因此这是
一个秘密。

这个秘密揭示另一个核心问题：如何使用较低的资源成本训练AI大模型。显然
Fabrice Bellard知道问题的答案。




