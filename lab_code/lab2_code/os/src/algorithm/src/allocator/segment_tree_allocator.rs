use super::Allocator;
use alloc::{vec, vec::Vec};//提供线段树结构实现的分配器 [`Segment_Tree_Allocator`]

/// 使用线段树实现分配器
///
/// 在 `Vec` 末尾进行加入 / 删除。
/// 每个元素 tuple `(start, end)` 表示 [start, end) 区间为可用。

pub struct  node{
    l:usize,
    r:usize,
    available:bool,
}
impl node {
    pub fn new(l:usize,r:usize)->Self{
        Self{
            l,
            r,
            available:false,
        }
    }
}
pub struct SegmentTreeAllocator {
    tree: Vec<node>,
}
impl SegmentTreeAllocator{
    pub fn build(&mut self,u:usize,l:usize,r:usize){
        self.tree[u]=node{l,r,available:true};
        if l<r{
            let mid:usize = (l+r)/2;
            self.build(u*2, l, mid);
            self.build(u*2+1,mid+1,r);
    }
}
    pub fn pushup(&mut self,u:usize){
        self.tree[u].available=self.tree[u*2].available|self.tree[u*2+1].available;
    }
    pub fn modify(&mut self,u:usize,k:usize,d:bool){
        let mid = (self.tree[u].l+self.tree[u].r)/2;
        if k<=mid{
            self.modify(u*2,k,d);
        }
        else{
            self.modify(u*2+1,k,d);
        }
        self.pushup(u);
    }
    pub fn search(&self,u:usize)->Option<usize>{
        if self.tree[u].available==false{
            None
        }
        else{
            if self.tree[u].l==self.tree[u].r{
                Some(self.tree[u].l)
            }
            else{
            if self.tree[u*2].available==true{
                return self.search(u*2);
            }
            else{
                return self.search(u*2+1);
            }
        }
        }
    }
}

impl Allocator for SegmentTreeAllocator {
    fn new(capacity: usize) -> Self {
        let mut tree = vec![node::new(1,capacity)];
        for i in 1..(4*capacity){
            tree.push(node::new(1,capacity));
        }
        let mut res = Self{tree};
        res.build(1,1,capacity);
        res
        }
    

    fn alloc(&mut self) -> Option<usize> {
       let res = self.search(1);
       if let Some(x)=res{
           self.modify(1,x,false);
       }
       res
    }

    fn dealloc(&mut self, index: usize) {
       self.modify(1,index,true);
    }

}