# chapter5 报告

## 功能实现

首先，在`syscall/process.rs`中实现`sys_spawn`，通过结合`fork` 和`exec`两者的功能实现。总体的设计思路类似于把两者结合起来，不同点在于，`spawn`通过直接读取`elf`文件的内容来创建地址空间。

然后，对于`stride`调度算法的实现，首先实现了`syscall/process.rs`中的`sys_set_priority`系统调用，然后在`config`中定义一个`BIG_STRIDE`常数，用来实现`stride`的相关功能。在`TaskControlBlockInner`中加入三个字段`pass`、`stride`、`priority`。最后，由于执行任务是在`run_task()`函数中，通过调用`fetch_task()`来选取要执行的进程。在`fetch()`中根据`stride`调度即可。

最后，通过`git cherry-pick`命令将之前的提交合并。由于框架代码有变动，所以在这里也要相应的做调整。

## 问答题

实际情况不是轮到p1执行，原因是p2执行结束之后，加上`pass`后，u8溢出，会发生截断导致p2小于p1。

如果保证进程优先级全部 >= 2的情况下，如果按照算法执行，`pass <= BigSride/2`。不考虑溢出时，满足条件`STRIDE_MAX - STRIDE_MIN <= BigStride / 2`。

假设所有进程的优先级都大于等于2，这意味着步长的最大值和最小值都至少是2的倍数。当步长从`STRIDE_MIN`增加到`STRIDE_MAX`时，步长值的变化范围至多是`BigStride`的一半，即`(STRIDE_MAX - STRIDE_MIN) <= BigStride / 2）`。

根据提示，大于`BigStride/2`的值视为负数，所以有：

补全代码：

```rust
use core::cmp::Ordering;

struct Stride(u64);

impl PartialOrd for Stride {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        // ...
        Some((self.0 as i64).cmp(&other.0 as i64));
    }
}

impl PartialEq for Stride {
    fn eq(&self, other: &Self) -> bool {
        false
    }
}

```

