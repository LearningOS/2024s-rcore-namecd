# chapter3 练习

## 实现的功能

根据实验指导书的提示内容，首先修改了`os/stc/task/task.rc`中的`TaskControlBlock`块，加入了两个新内容：`task_begin_time` 和 `task_syscall_times` 作为某个具体任务控制块的信息。

在文件`os/src/task/mod.rs`新增两个函数，`get_take_info()` 和 `increase_syscall_times()`分别用来查询`task`的信息、逐渐增加某个具体的`task`中的`task_syscall_times`

在文件`os/src/syscall/mod.rc`中的`syscall()`函数中，`match`之前通过调用之前实现的`increase_syscall_times()`来完成对当前任务系统调用次数的递增

## 问答题

1. 三个bad测例出错信息为：

```Rust
sbi：RustSBI-QEMU Version 0.2.0-alpha.2

[kernel] PageFault in application, bad addr = 0x0, bad instruction = 0x804003ac, kernel killed it.
[kernel] IllegalInstruction in application, kernel killed it.
[kernel] IllegalInstruction in application, kernel killed it.
[kernel] Panicked at src/trap/mod.rs:72 Unsupported trap Exception(LoadFault), stval = 0x18!
```

在访问非法地址`0x0`时，出现访存错误，`trap`的原因在进行特权级切换时，被`trap_handler`捕获。

在U态使用S态特权指令`sret`时出现，错误，批处理系统已经杀死对应任务

在U态访问S态寄存器`sstatus`出现错误，批处理系统杀死对应任务。

2. `trap.S`
   1. L40：刚进入`__restore`时，`a0`代表了内核栈的栈顶指针。`__restore`可以用于在特权级转换时，恢复`Trap`的上下文。以及在任务切换后，通过`__restore`恢复任务的上下文。
   2. L43-L48：特殊处理了`CSR`寄存器`sstatus` 、`sepc` 、`sscratch` ，通过处理`sstatus` 、`sepc` ，让程序优先恢复这些CSR寄存器的目的是防止出现`Trap`嵌套的情况导致值被覆盖掉。`sepc`存放切换回用户态之后应该执行的指令地址。而`sscrath`保存了用户栈的栈顶指针，最后通过交换`sp`和`sscrath`的值，让程序正确恢复到用户态。
   3. L50-L56: `X2`是`sp`寄存器，存放栈顶指针，`X4`用不到
   4. L60:`sp`中的值为用户栈，另一个为内核栈
   5. 发生状态切换的在指令`sret`，执行该指令之后，cpu的pc指向了`sepc`的值。
   6. L13：交换了`sp`和`sscratch`的值。现在`sp`指向内核栈
   7. `ecall`指令

## 荣誉准则

1. 在完成本次实验的过程（含此前学习的过程）中，我曾分别与 **以下各位** 就（与本次实验相关的）以下方面做过交流，还在代码中对应的位置以注释形式记录了具体的交流对象及内容：

   > *无*

2. 此外，我也参考了 **以下资料** ，还在代码中对应的位置以注释形式记录了具体的参考来源及内容：

   > [chapter3练习 - rCore-Tutorial-Guide-2024S 文档 (learningos.cn)](https://learningos.cn/rCore-Tutorial-Guide-2024S/chapter3/5exercise.html) 
   >
   >  [rCore 作业讲解与答疑 - 飞书云文档 (feishu.cn)](https://sjodqtoogh.feishu.cn/docx/ZoqBdmcmAoXi9yxZUkucMmxBnzg) 

3. 我独立完成了本次实验除以上方面之外的所有工作，包括代码与文档。 我清楚地知道，从以上方面获得的信息在一定程度上降低了实验难度，可能会影响起评分。

4. 我从未使用过他人的代码，不管是原封不动地复制，还是经过了某些等价转换。 我未曾也不会向他人（含此后各届同学）复制或公开我的实验代码，我有义务妥善保管好它们。 我提交至本实验的评测系统的代码，均无意于破坏或妨碍任何计算机系统的正常运转。 我清楚地知道，以上情况均为本课程纪律所禁止，若违反，对应的实验成绩将按“-100”分计。

