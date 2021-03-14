
use rustunsafedatastruct::vec::{self,ManVec};
use rustunsafedatastruct::list::List;
use rustunsafedatastruct::tree::Tree;
use rustunsafedatastruct::bsttree::BSTTree;
use rustunsafedatastruct::maxheap::MaxHeap;
use rustunsafedatastruct::hashmap::HashMap;
use rustunsafedatastruct::quicksortbinarysearch;
use rustunsafedatastruct::time::TimerWatch;
use rustunsafedatastruct::redblacktree;
use rustunsafedatastruct::skiplist::SkipList;

use std::cmp::Eq;

use std::hash::{Hash,Hasher};

#[derive(Debug)]
enum Weizhi {
  CC(i32),
  DD,
}


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

    let m1=My {
      c:32,
      z:Box::new("replaced".to_string()),
      x:Weizhi::CC(32),
    };

   v.insert(2,m1);

   match v.get_mut(2) {
    Some(mut c) => {println!("this is the data:{:?}",c)},
    None => {},
  };

  v.remove(2);

  match v.get_mut(2) {
    Some(mut c) => {println!("this two is the data:{:?}",c)},
    None => {},
  };

  let  xx=&*v;
 // for x in xx {
    println!("the x is {:?}",&xx[1..3]);
 // }
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

   let mut vec=ManVec::new() ;

   let count=10000;
  for i in 0..count {
      
       vec.push(i);
  }

  quicksortbinarysearch::shuffle(&mut vec);

   let last = vec.len() - 1 ;
   let begin=TimerWatch::new();
   quicksortbinarysearch::quick_sort(&mut vec, 0, last as isize);
   

   let find = &109;

   let index = quicksortbinarysearch::binary_search(find, &vec);

   println!(" vv :{} index is {}",find,index.unwrap());
   println!("the sort take time {} millis",begin.passed());



   let mut m = redblacktree::RedBlackTree::new();
  let begin=TimerWatch::new();
  
   for i in 100..100000 {
   m.insert(i, i+1);
   }

   println!("the RedBlackTree insert  take time {} millis",begin.passed());

   m.replace_or_insert(999999, 4);

  println!("the result is {:?}",m.get(&999999));

  let mut v=SkipList::new();

  let begin=TimerWatch::new();
  
  for i in 100..100000 {
  v.insert((i, i+1));
  }


  println!("the result is {:?}",v.find(&999));

  println!("the SkipList insert  take time {} millis",begin.passed());


  v.insert((34, 56));
  
  println!("the result is {:?}",v.find(&34));

  v.insert((34, 57));
  
  println!("the result is {:?}",v.find(&34));

  v.remove(&34);

  println!("the result is {:?}",v.find(&34));


}