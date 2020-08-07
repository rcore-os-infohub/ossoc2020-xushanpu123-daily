题目链接：https://codeforces.com/contest/1384/problem/A

代码：

```rust
use std::str;
macro_rules! read {
    ($($t: ty),+) => ({
        let mut a_str = String::new();
        io::stdin().read_line(&mut a_str).expect("read error");
        let mut a_iter = a_str.split_whitespace();
        (
            $(
            a_iter.next().unwrap().parse::<$t>().expect("parse error"),
            )+
        )
    });
}

macro_rules! read_n {
    ($e: expr, $t: ty) => {
        {
            let mut v = Vec::new();
            let mut a_str = String::new();
            io::stdin().read_line(&mut a_str).expect("read error");
            let mut a_iter = a_str.split_whitespace();
            for i in 0..$e {
                v.push(a_iter.next().unwrap().parse::<$t>().expect("parse error"));
            }
            v
        }
    };
}

fn main() {
   let (t,) = read!(i32);
   for _ in 0..t{
       let (n,) = read!(usize);
       let mut v = read_n!(n,usize);
       let mut s1:Vec<u8> = Vec::new();
       for _ in 0..100{
           s1.push(97);
       }
       let res = str::from_utf8(&s1).unwrap();
       println!("{}",res);
       for i in 0..n{
           let mut s2 = s1.clone();
           let d = v[i];
           if s1[d] == 97{
               s2[d] = 98;
           }
           else{
               s2[d] = 97;
           }
           let res = str::from_utf8(&s2).unwrap();
           println!("{}",res);
           s1 = s2;
       }
          
   }
    
}
```

