# 手工测试生命太短
`Wednesday, 28 February 2007`
>
> Life is too Short for Manual Testing

对做手工测试而言, 生命太短.

对上面的论述的另一种说法是:

公理0: 手工测试是不敏捷的. 我们再来检视一下自动化测试公理1及推论:

公理1: 一个好的手工测试不可能自动化

推论1A: 如果你真正自动化了一个手工测试, 那这个手工测试不可能是一个好的
手工测试.

推论1B: 如果你有一个好的自动化测试, 那它和你自动化的手工测试一定不一样.

自动化测试公理1基本封死了另一条自动化之路: 从手工的测试创建自动测试.
尽管这个自动化之路有一些好处, 比如对要测的功能可以相互验证一下.

综合公理0和公理1, 手工测试是无用的; 再根据剃刀原则, 手工测试应该被剃掉.
很残酷吧?

产品在设计之初, 就应该考虑测试. 如果一个产品非常的难测, 那它的质量就非
常的难测. 如果它的测试可以完全自动化, 那开发人员就能够很快得到反馈, 测
试人员会腾出脑筋考虑增加更多的自动化测试; 正面的迭代一旦形成就难以遏制.
而如果它的测试只能手工完成, 测试人员会忙于测试的执行, 很少有时间考虑测
试的质量; 开发人员得不到及时反馈; 一旦发现了问题就很严重, 甚至很多都要
推倒重来; 项目会慢慢演变为大家的噩梦. 这样只能靠中国式的艰苦奋斗才能脱
困境, 而且只是脱困境而已, 完成的活儿离漂亮还差的很远.