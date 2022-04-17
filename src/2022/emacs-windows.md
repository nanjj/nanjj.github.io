<div style="text-align: right"><code>2022年04月17日 星期日 下午</code></div>

我的Emacs窗口管理如下：

| 快捷键  | 命令                   |
|---------|------------------------|
| `C-x 0` | `delete-window`        |
| `C-x 1` | `delete-other-windows` |
| `C-x 2` | `split-window-below`   |
| `C-x 3` | `split-window-right`   |
| `C-x 4` | `delete-other-window`  |

其中`delete-other-window`如下：

```
(defun delete-other-window()
  "Delete other window"
  (interactive)
  (delete-window (next-window)))
```

这是唯一的扩展，其他几个都是按Emacs的默认在用。

好像这几个操作就足够了。其他操作类似增大缩小窗口我用的不多，而且这么多
年来好像不用也没啥过不去的。

