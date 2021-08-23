# 繁简转换

`Monday, 15 November 2010`

繁简转换并没有尽善尽美的解决方案，按单字转换有98%的准确率，剩下那2%，
需要分词了。分词本身就没有100%正确的算法了。

基于iconv可以这么做：
```bash
iconv -f BIG5 -t GB2312
```
注意这是按单字转的。如果要从UTF-8过来，还更麻烦些：

```bash
iconv -f UTF-8 -t BIG5 | iconv -f BIG5 -t GB2312 | iconv -f GB2312 -t UTF-8
```
这也不支持GBK的。

更好些的是`cconv`：

`cconv -f BIG5 -t GBK`

处理UTF也方便些：
```
cconv -f UTF8-CN -t UTF8-TW
```
支持更大些字符集。

据说也支持分词但我没试出来。还提供php的库支持。那用`JNI`集成到Java里也
不难，至少在Linux下是这样。
