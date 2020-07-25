#### 问题一：线程切换之中，页表是何时切换的？页表的切换会不会影响程序 / 操作系统的运行？为什么？

​		 prepare_next_thread()中执行next_thread.prepare(),再通过self.process.write().memory_set.activate()激活页表，切换成功。

页表切换不会影响程序运行，因为保存的上下文可以恢复，寄存器及堆栈不会发生变化，可以正常执行

页表切换不会影响操作系统的运行，因为页表切换是发生在操作系统内的，而每个进程的页表都映射了操作系统内的部分空间，如中断处理部分。

#### 问题二：如果不使用 `sscratch` 提供内核栈，而是像原来一样，遇到中断就直接将上下文压栈，请举出（思路即可，无需代码）：

- 一种情况不会出现问题

- 一种情况导致异常无法处理（指无法进入 `handle_interrupt`）

- 一种情况导致产生嵌套异常（指第二个异常能够进行到调用 `handle_interrupt`，不考虑后续执行情况）

- 一种情况导致一个用户进程（先不考虑是怎么来的）可以将自己变为内核进程，或以内核态执行自己的代码

  答案：1、只运行一个进程和一个线程。

  ​            2、线程的堆栈寄存器sp发生了切换，但是却无法维护。

  ​            3、用户访问到另一个线程时发生了页表的切换，但是堆栈sp并没有更新，所以访问原来的栈空间发生了异常。

  ​           4、用户进程的sp落到了内核进程附近，又因为保存寄存器修改了sp的值，使得sp落入了内核进程之内。

  #### 实验：当键盘按下 Ctrl + C 时，操作系统应该能够捕捉到中断。实现操作系统捕获该信号并结束当前运行的线程（你可能需要阅读一点在实验指导中没有提到的代码）

  分析：Ctrl+C是外部中断，中断号为3，因此只需要在handler.rs中加入对对应信号的处理即可。

   

  ```rust
  // 外部中断（键盘输入）
  
  ​    Trap::Interrupt(Interrupt::SupervisorExternal) => supervisor_external(context),
  ```

  上述代码加入对外部中断的处理。

  ```rust
  fn supervisor_external(context: &mut Context) -> *mut Context {
  
    let c = console_getchar();
  
    // println!("{}", c);
  
    match c {
  
  ​    3  => {  // Ctrl C
  
  ​      PROCESSOR.get().kill_current_thread();
  
  ​      PROCESSOR.get().prepare_next_thread();
  
  ​    }
  
  ​    _  => {}
  
    }
  
    context
  
  }
  ```

  通过上述代码，能够捕捉到ctrl+C信号并且做处理。

  

  #### 实验：实现进程的 `fork()`。目前的内核线程不能进行系统调用，所以我们先简化地实现为“按 F 进行 fork”。fork 后应当为目前的进程复制一份几乎一样的拷贝。

  分析：按F属于外部中断，因此我们首先应该对这种情况进行捕获和处理，此处先抽象出fork函数：

  ```rust
  fn supervisor_external(context: &mut Context) -> *mut Context {
  
    let c = console_getchar();
  
    // println!("{}", c);
  
    match c {
  
  ​    .......
  
  ​    102 => {  // f
  
  ​      let thread = PROCESSOR.get().current_thread().fork(context).unwrap();
  
  ​      PROCESSOR.get().add_thread(thread);
  
  ​    }
  
  ​    _  => {}
  
    }
  
    context
  
  }
  ```

然后进一步实现fork（）的功能：

​       分析pub fn fork(&self, context: &Context) -> MemoryResult<Arc<Thread>>的步骤：

​       新建线程与父线程有相同Context，但是必须要开辟自己的堆栈，因此第一步必须开辟向所属进程申请一部分空间作为自己的堆栈区，然后在页表中写好新堆栈与虚拟地址的对应关系。此后，根据新申请的堆栈基地址和父线程的堆栈占空间大小，可以求出新线程的sp，把它赋值给新线程的Context，最后把所有新值构建成新线程的结构体作为函数的返回值。

 

```rust
pub fn fork(&self, context: &Context) -> MemoryResult<Arc<Thread>> {

​    let stack = self.process

​      .write()

​      .alloc_page_range(STACK_SIZE, Flags::READABLE | Flags::WRITABLE)?;

​    for p in 0..STACK_SIZE {

​      *VirtualAddress(stack.start.0 + p).deref::<u8>()

​        = *VirtualAddress(self.stack.start.0 + p).deref::<u8>();

​    }

​    let mut new_context = context.clone();

​    // new_context.set_sp(stack.end.into());  // FIXIT

​    let sp = context.sp() - self.stack.start.0 + stack.start.0;

​    new_context.set_sp(sp);

​    Ok(Arc::new(Thread {

​      id: unsafe {

​        THREAD_COUNTER += 1;

​        THREAD_COUNTER

​      },

​      stack,

​      process: self.process.clone(),

​      inner: Mutex::new(ThreadInner {

​        context: Some(new_context),

​        sleeping: false,

​      }),

​    }))

  }

}
```

