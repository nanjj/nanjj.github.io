## 0
[Org Remark]是一个Emacs下用[Org Mode]对文本文件做标记记笔记的工具。

![](org-remark.png)

[Org Remark]: https://github.com/nobiot/org-remark
[Org Mode]: https://orgmode.org

## 1

这有什么用呢？

我在精读代码的时候用它来记笔记。就像读书一样，有时候迷惑不解，就先把疑
惑记下来。有时候恍然大悟，就把一点心得记下来。这笔记就像读纸质书，写在
书边上笔记。就如1621年费马读丢番图的《算术》，读到：

$$
a^2 + b^2 = c^2
$$

有无数组整数解存在，由此想到费马猜想：

$$
a^n + b^n = c^n
$$

当 n > 2， 整数解不存在。 继而在纸边上写下：

> 我想到了一个绝妙的证明，但是这里太窄了，写不下。

[Org Remark]给的纸边可不窄，整个[Org Mode]文件。[Org Mode]数学公式的支
持也很好，费马再无借口，费马定理或能提前370多年得证。

## 2

[Org Remark]安装配置简单。

1. 安装
   ```
   M-x package-install org-remark
   ```
2. 配置
   ```
   M-x customize-variable org-remark-global-tracking-mode t
   M-x customize-variable org-remark-notes-file-name ~/.emacs.d/marginalia.org
   ```
3. 快捷键
   ```
   (define-key global-map (kbd "C-c n m") #'org-remark-mark)
   (with-eval-after-load 'org-remark
     (define-key org-remark-mode-map (kbd "C-c n o") #'org-remark-open)
     (define-key org-remark-mode-map (kbd "C-c n [") #'org-remark-prev)
     (define-key org-remark-mode-map (kbd "C-c n ]") #'org-remark-next)
     (define-key org-remark-mode-map (kbd "C-c n r") #'org-remark-remove))
   ```

笔记文件名我选的绝对路径，也就是说所有笔记都记在一个文件
（`~/.emacs.d/marginalia.org`）。如果你选相对路径，比如
`marginalia.org`，那每个目录都会生成一个`marginalia.org`文件。阅读源码
时候一般都有版本控制Git在，稍有不便。

## 3

和读书笔记工具一样，[Org Remark]有两个使用模式：
1. 记笔记，
2. 读笔记。

在阅读的时候随手标注：
```
    M-x org-remark-mark
```
快捷键：`C-c n m`，记下所思所想：
```
    M-x org-remark-open
```
快捷键：`C-c n o`，在再次阅读的时候看到自己的标记，再次打开：
```
    M-x org-remark-open
```
快捷键：`C-c n o`，到上一个标记：
```
    M-x org-remark-prev
```
快捷键：`C-c n [`，到下一个标记：
```
    M-x org-remark-next
```
快捷键：`C-c n ]`，如果看到有些不实（比如费马
看到自己的标注），可以删掉：
```
    M-x org-remark-remove
```
快捷键：`C-c n r`。

## 4

这些笔记是用文本格式的[Org Mode]所写，一些有价值的东西将来方便整理出来，
就如本篇文档一样。







