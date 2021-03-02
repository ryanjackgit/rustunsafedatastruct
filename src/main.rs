
use rustunsafedatastruct::vec::ManVec;
use rustunsafedatastruct::list::List;
use rustunsafedatastruct::tree::Tree;
use rustunsafedatastruct::bsttree::BSTTree;
use rustunsafedatastruct::maxheap::MaxHeap;
use rustunsafedatastruct::hashmap::HashMap;
use rustunsafedatastruct::quicksortbinarysearch;
use rustunsafedatastruct::time::TimerWatch;

use std::cmp::{Eq};

use std::hash::{Hash,Hasher};

#[derive(Debug)]
enum Weizhi {
  CC(i32),
  DD,
}

/*
impl Drop for Weizhi {
  fn drop(&mut self) {
    println!("weizhi descrtor {:?}",self);
  }
}
*/

#[derive(Debug)]
struct My{
   c:i32,
  z:Box<String>,
  x:Weizhi,
}

impl PartialEq for My {
  fn eq(&self, other: &Self) -> bool {
      self.c == other.c && self.z.eq(&other.z) 
  }
}
impl Eq for My {}


impl Hash for My {
  fn hash<R:Hasher>(&self,state:&mut R)  {
    self.c.hash(state);
    self.z.hash(state);
  }
}



impl Drop for My {
  fn drop(&mut self) {
  //  println!("drop my---------------------");
  }
}


fn main() {

    let mut v=ManVec::capacity(2);
   for i in 0..5 {
  let vtwo=format!("value is {}",i);
  let m=My {
    c:i,
    z:Box::new(vtwo),
    x:Weizhi::CC(i),
  };
    v.push(m);
   
}
  //v.printlnall();

  let m=My {
    c:3,
    z:Box::new("replaced".to_string()),
    x:Weizhi::CC(3),
  };
   
    match v.get_mut(3) {
      Some(mut c) => *c=m,
      None => {},
    };

   
    //v.printlnall();

    let mut list=List::new();
    for i in 0..10 {
        let vtwo=format!("value is {}",i);
        let m=My {
          c:i,
          z:Box::new(vtwo),
          x:Weizhi::DD,
        };
        list.insert_after(m);
         
      }

      list.list();

      let vtwo=format!("value is {}",9);
      let m=My {
        c:9,
        z:Box::new(vtwo),
        x:Weizhi::DD,
      };
      list.remove_current(&m);

      list.list();


//for i in 0..100000 {
  /*
    for i in 0..1 {
      let  pn=std::ptr::null_mut();
      let mut left_leaf=Tree::create_node(Weizhi::DD,pn,pn);
      let mut right_leaf=Tree::create_node(Weizhi::DD,pn,pn);
     let mut left=Tree::create_node(Weizhi::CC(i),left_leaf,right_leaf);
    
      let mut right=Tree::create_node(Weizhi::CC(i+1),pn,pn);
      let node=Tree::create_node(Weizhi::CC(i+2),left,right);
      let mut root=Tree::new_as_node(node);
      root.last_root();
    }
    */
//}
let mut root=BSTTree::newnew() ;
let  pn=std::ptr::null_mut();
let mut node=BSTTree::create_node(4,pn,pn);
//let mut root=BSTTree::new_as_node(node);
root.insert_node(node);

let mut right_leaf=BSTTree::create_node(6,pn,pn);
root.insert_node(right_leaf);

let mut left=BSTTree::create_node(5,pn,pn);
root.insert_node(left);

let mut left=BSTTree::create_node(3,pn,pn);
root.insert_node(left);

let mut left=BSTTree::create_node(10,pn,pn);
root.insert_node(left);

let mut left=BSTTree::create_node(9,pn,pn);
root.insert_node(left);

root.middle_root();
let n=9;
let bool_flag=root.search_node(&n);
println!("the find {} result  is {}",n,bool_flag);


let mut mh=MaxHeap::new();
mh.push(1);
mh.push(4);
mh.push(7);

//mh.print();
mh.push(9);
mh.push(0);
mh.push(13);

mh.push(15);
mh.print();

println!("the peek is {:?}",mh.peek());
let ss=mh.pop();
println!("the pop is {:?}",ss);
mh.print();
println!("the peek is {:?}",mh.peek());
let ss=mh.pop();
println!("the pop is {:?}",ss);
mh.print();



let begin=TimerWatch::new();
let mut hash_map=HashMap::new();
for i in 0..10 {
let vtwo=format!("value is {}",i);
let m=My {
  c:i,
  z:Box::new(vtwo),
  x:Weizhi::CC(i),
};
   hash_map.insert(m,"rust".to_string());
}

println!("take the time {} millis",begin.passed());




for (x,ref mut y) in &mut hash_map.iter_mut().unwrap() {
 
  println!("key:{:?},value:{}",x,y);
  **y="java".to_string();
}

let vtwo=format!("value is {}",1);
let m=My {
  c:1,
  z:Box::new(vtwo),
  x:Weizhi::CC(1),
};

println!("thie key is {:?} :  value is {:?} ",&m,hash_map.get(&m).unwrap());



for (x,ref mut y) in  &mut hash_map.iter_mut().unwrap()  {
 
  println!("key:{:?},value:{}",x,y);
 
}

   
   hash_map.len();

let  selfiterator=hash_map.iter_mut().unwrap();


for (x,y) in  selfiterator {
  println!("key:{:?},value:{}",x,y);
}




   use rand::random;
   let mut vec=ManVec::new() ;
   let mut vv:u32=0;
   let count=10000;
  for i in 0..count {
       let x: u32 = random();
    //   print!("  {} is {}  max:{}",i,x,u32::MAX);
      vv=x;
       vec.push(x);
  }

   let last = vec.len() - 1 ;
   let begin=TimerWatch::new();
   quicksortbinarysearch::quick_sort(&mut vec, 0, last as isize);
   

   /*
  for i in 0..count {
       println!("the value is {:?}",vec[i]);
   }

   */

   let find = vv;

   let index = quicksortbinarysearch::binary_search(&find, &vec);

   println!(" vv :{} index is {}",vv,index.unwrap());
   println!("the sort take time {} millis",begin.passed());


}