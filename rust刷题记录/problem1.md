1. ```rust
    fn read() ->i32{
      let mut line = String::new();
      std::io::stdin().read_line(&mut line);
      line.trim().parse::<i32>().unwrap()
   }
   fn main()
   {
       let mut t = read();
       for i in 0..t{
           let mut ma=-1;
           let mut mb=-1;
               let mut Line=String::new();
               std::io::stdin().read_line(&mut Line);
               let input:Vec<&str>=Line.trim().split(' ').collect();
               let n = input[0].parse::<i32>().unwrap();
               let k1=input[1].parse::<i32>().unwrap();
               let k2=input[2].parse::<i32>().unwrap();
           
               let mut line=String::new();
               std::io::stdin().read_line(&mut line);
               for item in line.trim().split(' '){
                   let s = item.parse::<i32>().unwrap();
                   if ma<s{
                       ma=s;
                   }
              
           }
          
               let mut line=String::new();
               std::io::stdin().read_line(&mut line);
               for item in line.trim().split(' '){
                   let s = item.parse::<i32>().unwrap();
                   if mb<s{
                       mb=s;
                   }
               }
               
           
           if ma>mb{
               println!("YES");
           }
           else{
               println!("NO");
           }
       
       }
       
   }
   ```

   题目来源：codeforces：Goodbye2019 A题，下面是C++代码：

   1. ```c++
      #include <iostream>
      #include <cstdio>
       
      using namespace std;
       
      int main()
      {
          int t,n,k1,k2,s,ma=-1,mb=-1;
          cin>>t;
          while(t--)
          {
          ma=mb=-1;
           cin>>n>>k1>>k2;
           for(int i=0;i<k1;i++)
           {
              scanf("%d",&s);
              if(ma<s) ma=s;
           }
           for(int i=0;i<k2;i++)
           {
               scanf("%d",&s);
               if(mb<s) mb=s;
           }
           if(ma>mb) cout<<"YES"<<endl;
           else      cout<<"NO"<<endl;
          }
          return 0;
      }
      ```

      
