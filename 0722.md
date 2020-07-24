# 2020/07/22

今天才发现我对实验的完成要求有误解。所以决定先理解实验指导的源码，集中精力做六个实验题。



## lab1实验题：

**首先先放上对中断的理解：**

首先，中断是硬件功能，每种中断例如外部中断、用户软件中断等实际上是硬件规定好的，有自身的中断号。每次中断传入的时候，中断号存在scause寄存器中（可以通过包risc调用）。rcore采用DIRECT方式，每次中断的时候，都会进入到BASE地址处执行，根据scause和其他参数进行对应的操作，而BASE处所对应的就是文件handler.rs的内容。

#### 问题一：在 `rust_main` 函数中，执行 `ebreak` 命令后至函数结束前，`sp` 寄存器的值是怎样变化的？

ebreak中断后，首先涉及到sp变化的是interrupt/interrupt.asm中的代码，通过观察代码我们看到，sp先减去34*8，存储一个context。然后进入handler_interrupt函数，会有一些与函数调用有关的出入栈操作，但是在该函数调用前后，sp的值不变。最后使用_restore处的代码，sp恢复到中断调用前的状态。

#### 问题二：如果去掉 `rust_main` 后的 `panic` 会发生什么，为什么？

panic函数可以使我们的程序停止运行。实际上，rust_main是通过内核起始代码entry.asm通过跳转指令达到的。如果rust_main执行完后没有panic中止，程序会会到entry.asm中运行，可能会发生无法预期的错误。

### 实验：

#### 1、捕获LoadFault

```rust
match scause.cause() {

​    // 断点中断（ebreak）

​    Trap::Exception(Exception::Breakpoint) => breakpoint(context),

​    // 时钟中断

​    Trap::Interrupt(Interrupt::SupervisorTimer) => supervisor_timer(context),

​    Trap::Exception(Exception::LoadFault) => panic!(),





​    // 其他情况，终止当前线程

​    _ => fault(context, scause, stval),

  };


```

如上述代码所示，插入

```rust
Trap::Exception(Exception::LoadFault) => panic!(),
```

即可。

#### 2、在处理异常的过程中，如果程序想要非法访问的地址是 `0x0`，则打印 `SUCCESS!`。

match内内容改为：

```rust
Trap::Exception(Exception::LoadFault) => response_loadfault(stval),
```

并给出response_loadfault的定义：

```rust
fn response_loadfault(stval: usize){

  if stval == 0 as usize{

​    println!("Success!");

  }

}


```

#### 3、添加或修改少量代码，使得运行时触发这个异常，并且打印出 `SUCCESS!`。

在breakpoint函数中，将context.sepc设为了0.

```rust
context.sepc = 0;
```

此后，由于跳出中断的地址始终为0，所以程序开始无限输出Success！

此问题昨晚试验后再考虑解决！