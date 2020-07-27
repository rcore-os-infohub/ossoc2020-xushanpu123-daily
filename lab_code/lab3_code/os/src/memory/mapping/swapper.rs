//! 页面置换算法

use super::*;
use crate::memory::{frame::FrameTracker, *};
use alloc::collections::VecDeque;

/// 管理一个线程所映射的页面的置换操作
pub trait Swapper {
    /// 新建带有一个分配数量上限的置换器
    fn new(quota: usize) -> Self;

    /// 是否已达到上限
    fn full(&self) -> bool;

    /// 取出一组映射
    fn pop(&mut self) -> Option<(VirtualPageNumber, FrameTracker)>;

    /// 添加一组映射（不会在以达到分配上限时调用）
    fn push(&mut self, vpn: VirtualPageNumber, frame: FrameTracker, entry: *mut PageTableEntry);

    /// 只保留符合某种条件的条目（用于移除一段虚拟地址）
    fn retain(&mut self, predicate: impl Fn(&VirtualPageNumber) -> bool);
}

pub type SwapperImpl = FIFOSwapper;

/// 页面置换算法基础实现：FIFO
pub struct FIFOSwapper {
    /// 记录映射和添加的顺序
    queue: VecDeque<(VirtualPageNumber, FrameTracker)>,
    /// 映射数量上限
    quota: usize,
}

impl Swapper for FIFOSwapper {
    fn new(quota: usize) -> Self {
        Self {
            queue: VecDeque::new(),
            quota,
        }
    }
    fn full(&self) -> bool {
        self.queue.len() == self.quota
    }
    fn pop(&mut self) -> Option<(VirtualPageNumber, FrameTracker)> {
        self.queue.pop_front()
    }
    fn push(&mut self, vpn: VirtualPageNumber, frame: FrameTracker, _entry: *mut PageTableEntry) {
        self.queue.push_back((vpn, frame));
    }
    fn retain(&mut self, predicate: impl Fn(&VirtualPageNumber) -> bool) {
        self.queue.retain(|(vpn, _)| predicate(vpn));
    }
}
pub struct ClockSwapper {
    vec: Vec<(VirtualPageNumber, FrameTracker, *mut PageTableEntry)>,
    quota: usize,
    cur: usize,    //表示指针目前所指向的帧的位置
}
unsafe impl Send for ClockSwapper {}
impl Swapper for ClockSwapper {
    fn new(quota: usize) -> Self {
        Self {
            vec: Vec::new(),
            quota,
            cur: 0,
        }
    }

    fn full(&self) -> bool {
        self.vec.len() == self.quota
    }

    fn pop(&mut self) -> Option<(VirtualPageNumber, FrameTracker)> {
        if self.vec.len() == 0 {
            return None;
        }
        while true
         {
            //获取进程页表项地址
            let pte = self.vec[self.position].2;
            //查询该页的标志位
            let flags = unsafe { pte.as_ref().unwrap().flags() };
            //判断是否标志位为
            if flags.contains(Flags::ACCESSED) {
                //改变标志位
                let modified_flags = flags & !Flags::ACCESSED ;
                unsafe { pte.as_mut().unwrap().set_flags(modified_flags) };
            } else {
                //移除
                let removed_page = self.vec.remove(self.position);
                return Some((removed_page.0, removed_page.1));
            }
            self.cur = (self.cur + 1) % self.quota;
        }
    }

    fn push(&mut self, vpn: VirtualPageNumber, frame: FrameTracker, entry: *mut PageTableEntry) {
        self.vec.insert(self.position, (vpn, frame, entry));
    }

    fn retain(&mut self, predicate: impl Fn(&VirtualPageNumber) -> bool) {
        self.vec.retain(|(vpn, _, _)| predicate(vpn));
    }
}