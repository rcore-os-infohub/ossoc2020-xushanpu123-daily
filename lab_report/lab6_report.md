#### 问题一：原理：使用条件变量之后，分别从线程和操作系统的角度而言读取字符的系统调用是阻塞的还是非阻塞的？

答案：对线程是阻塞的，对操作系统是非阻塞的。

#### 问题二：如果要让用户线程能够使用 `Vec` 等，需要做哪些工作？如果要让用户线程能够使用大于其栈大小的动态分配空间，需要做哪些工作？

答案：(1)必须实现用户态下内存申请的系统调用；（2）需要通过系统调用来向操作系统内核申请更大的空间。

#### 实验：实现get_tid()

用户态下，需要加入：

```rust
pub fn sys_gettid() -> isize {

  syscall(SYSCALL_GETTID, 0, 0, 0, 0, 0, 0)

}
```

内核态下，需要加入：

```rust
SYS_GETTID => sys_gettid(),
```

最后，要实现sys_gettid():

```rust
pub(super) fn sys_gettid() -> SyscallResult {

  SyscallResult::Proceed(PROCESSOR.lock().current_thread().id)

}
```

#### 实验：实现进程复制sys_fork()

#### 实验：将一个文件打包进用户镜像，并让一个用户进程读取它并打印其内容。需要实现 `sys_open`，将文件描述符加入线程的 `descriptor` 中，然后通过 `sys_read` 来读取。

先找到文件，把文件描述符放入进程的descriptor，然后再实现sys_open，需要为sys_openat分配系统调用号，在用户态下加入syscall，再在os/kernel/syscall.rs中加入系统调用，在fs.rs中实现具体功能，具体详见代码。