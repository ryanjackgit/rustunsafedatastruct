use std::alloc::{self, dealloc, Layout};
use std::ptr;
use std::mem;
use std::fmt::Debug;
use crate::vec::ManVec;
use std::cmp::{Ord,Ordering,Eq,PartialEq};
use rand::{thread_rng, Rng};
use rand::rngs::StdRng;
use rand::SeedableRng;
use rand::rngs::ThreadRng;


/*
  **********      **********     **********
  *    *   *      *    *   *
  *    *   *      *    *   *
  * x  * * *      *  5 * * *     
  *    *   *      *    *   *
  *    * * *      *    * * *
  *    *   *      *    *   *
  **********      * ********                                                                                                                         
      *
      *
      *
    sentinel
     root


*/

// skiplist max level
const MAX_LEVEL:usize=32;

#[derive(Debug)]
pub struct NodePtr<K:Ord+Default+Debug,V:Default>(*mut Node<K,V>);

impl<K:Ord+Default+Debug,V:Default> Clone for NodePtr<K, V> {
    fn clone(&self) -> NodePtr<K, V> {
        NodePtr(self.0)
    }
}

impl<K:Ord+Default+Debug,V:Default> Copy for NodePtr<K, V> {}

impl<K:Ord+Default+Debug,V:Default> Ord for NodePtr<K, V> {
    fn cmp(&self, other: &NodePtr<K, V>) -> Ordering {
        unsafe { (*self.0).data.0.cmp(&(*other.0).data.0) }
    }
}

impl<K:Ord+Default+Debug,V:Default> PartialOrd for NodePtr<K, V> {
    fn partial_cmp(&self, other: &NodePtr<K, V>) -> Option<Ordering> {
        unsafe { Some((*self.0).data.0.cmp(&(*other.0).data.0)) }
    }
}

impl<K:Ord+Default+Debug,V:Default> PartialEq for NodePtr<K, V> {

    fn eq(&self, other: &NodePtr<K, V>) -> bool {
        self.0 == other.0
    }
}

impl<K:Ord+Default+Debug,V:Default> Eq for NodePtr<K, V> {}

impl<K:Ord+Default+Debug,V:Default> NodePtr<K, V> {


    fn new() -> Self {
        NodePtr(ptr::null_mut())
    }

    fn is_null(&self) -> bool {
         self.0.is_null()
    }


    pub fn get_level(&self) -> usize {
        unsafe {
            (*self.0).get_level()
        }
    }

    pub fn set_level_pointer(&mut self,index:usize,pointer:NodePtr<K,V>) -> bool {
        unsafe {
            (*self.0).set_level_pointer(index,pointer)
        }
    }

    pub fn get_level_pointer(&self,index:usize) -> NodePtr<K,V>{
        unsafe {
            (*self.0).get_level_pointer(index)
        }
    }

    pub fn put_newnodePtr(&mut self,pointer:NodePtr<K,V>) {
        unsafe {
            (*self.0).put_newnodePtr(pointer)
        }
    }

    pub fn pop(&mut self) {
        unsafe {
            (*self.0).pop()
        }
    }

    pub fn get_key(&self) -> &K {
        unsafe {
            (*(self.0)).get_key()
        }
    }

    pub fn get_value(&self) ->&V {
        unsafe {
            (*self.0).get_value()
        }
    }

    pub fn get_mut_value(&mut self) ->&mut V {
        unsafe {
            (*self.0).get_mut_value()
        }
    }

}

pub struct Node<K:Ord+Default+Debug,V:Default> {
    data:(K,V),
    vecPtr:ManVec<NodePtr<K,V>>,
}

impl <K:Ord+Default+Debug,V:Default> Node<K,V> {

    pub fn new(data:(K,V)) -> NodePtr<K, V> {
     
        let layout=Layout::new::<Node<K,V>>();
        let node= unsafe {
           let c= alloc::alloc(layout) as *mut Node<K,V>;
             
           let mut data_pointer=&mut ((*c).data) as *mut (K,V);
           ptr::write(data_pointer,data);
           let mut ptr=ManVec::new();
           let mut data_pointer=&mut ((*c).vecPtr) as *mut ManVec<NodePtr<K,V>>;
           ptr::write(data_pointer,ptr);   
           c
        };

        NodePtr(node)

    }

    pub fn get_level(&self) -> usize {
        self.vecPtr.len()
    }

    pub fn set_level_pointer(&mut self,index:usize,pointer:NodePtr<K,V>) -> bool {
        if let Some(mut p)=self.vecPtr.get_mut(index) {
            *p=pointer;
            true
        } else {
            false
        }
    }

    pub fn get_level_pointer(&self,index:usize) -> NodePtr<K,V> {
        if let Some(p)=self.vecPtr.get(index) {       
           *p
        } else {
            NodePtr::new()
        }
    }

    pub fn put_newnodePtr(&mut self,pointer:NodePtr<K,V>) {
        self.vecPtr.push(pointer);
    }

    pub fn pop(&mut self) {
        self.vecPtr.pop();
    }

    pub fn get_key(&self) ->&K {
        &self.data.0
    }

    pub fn get_value(&self) ->&V {
        &self.data.1
    }

    pub fn get_mut_value(&mut self) ->&mut V {
        &mut self.data.1
    }

}

//  impl <K:Ord+Default+Debug,V:Default> Drop for Node<K,V>

pub struct SkipList<K:Ord+Default+Debug,V:Default> { 
    root:NodePtr<K,V>,
    level:usize,

    //也可以不需要，主要为了抛硬币随机，每个跳表实例种子应该不同
    rng:ThreadRng,
}

impl<K:Ord+Default+Debug,V:Default>  SkipList<K,V> {

    pub fn new() ->  Self {
        let mut m=Node::new((K::default(),V::default()));
        let level=MAX_LEVEL;
        let mut rng = rand::thread_rng();
        SkipList {
            root:m,
            level:level,
           // rngInstance:StdRng::seed_from_u64(seed),
           rng:rng,
        }
    }



pub fn insert(&mut self,mut data:(K,V)) {

    let mut null=NodePtr::new();

    //计算随机高度
    let mut true_count=1;
    for _ in 0..self.level {
    if self.flipCoin() {
       true_count+=1;
    }
    }

  //  println!("insert ,the random size is {}",true_count);
      //作为第一个节点加入
   if self.root.get_level()==0 {
  
    let mut newNodePtr=Node::new(data);
    for i in 0..true_count {
          //哨兵高度随着增长，各层均指向该节点
        self.root.put_newnodePtr(newNodePtr);
          //右边哨兵高度以内指针全部设为null
        newNodePtr.put_newnodePtr(null);
       }
     }   else {
      
         let mut newNodePtr=Node::new(data);

         for i in 0..true_count {         
            //右边高度以内指针全部设为null
          newNodePtr.put_newnodePtr(null);
         }

         let rootLen=self.root.get_level();
         let  path=self.find_path(&mut newNodePtr);
       
        if let None=path {
           //  println!("had the same value and changed!");
             //newNodePtr 需要释放
             unsafe {
                //需释放数据所占用的空间,特别指出,free the space.
                let layout=Layout::new::<Node<K,V>>();    
                ptr::drop_in_place(&mut (*(newNodePtr.0)).data as *mut (K,V));
        
                ptr::drop_in_place(&mut (*newNodePtr.0).vecPtr as *mut ManVec<NodePtr<K,V>>);
                alloc::dealloc(newNodePtr.0 as *mut u8,layout);
               }
              
             return;
         }
           let mut path=path.unwrap();
           let path_len=path.len();
       //    println!("path.len:{}",path_len);
           let len=newNodePtr.get_level();
       //    println!("newNodePtr.len:{}",len);
       //     println!("root.len:{}",rootLen);

           let mut i=0;
          //只需要插入与新节点层度相同
            for j in (0..path_len).rev() {
                if i==len {
                 
                    break;
                }
          //      println!("the pop p is  {:?}",path[j].get_key());
              
          //      println!("i is is {}",i);
                //矫正各级指针指向
                let next=path[j].get_level_pointer(i);
                newNodePtr.set_level_pointer(i,next);
                path[j].set_level_pointer(i,newNodePtr);

                i+=1;
            }

            if rootLen< true_count {
                //增高跟节点的总体高度，并把指针值指向新节点
                let gap=true_count-rootLen;
             //   println!("gap is {}",gap);
                for i in 0..gap {
                   self.root.put_newnodePtr(newNodePtr);
               
                  // newNodePtr.set_level_pointer(path_len+i,null_nodeptr);
                }
            }

         //   println!("--------------insert end");
    
    }
}

// 为查找插入点记录其各层前驱指针
fn find_path(&self,node:&mut NodePtr<K,V>) -> Option<ManVec<NodePtr<K,V>>> {
  
   // println!("the size of {}",std::mem::size_of::<NodePtr<K, V>>());
   // println!("the align of {}",std::mem::align_of::<NodePtr<K, V>>());
    let k=node.get_key();

    let mut path_save=ManVec::new();

    let  null=NodePtr::new();

    let mut currentLevel=self.root.get_level();

    let mut pre=self.root;
    let mut current_p=self.root;

    for i in (0..currentLevel).rev() {
        current_p=pre;
       
        current_p=current_p.get_level_pointer(i);
        if  current_p==null || current_p.get_key() > k {
            path_save.push(pre);
            continue;
        }
      
        while current_p.get_key()< k  {
            if current_p.get_level_pointer(i) ==null  {
                path_save.push(current_p);
                pre=current_p;
                break;
            } else {
                pre=current_p;
                current_p=current_p.get_level_pointer(i);
            }
        }

        if  current_p.get_key() == k    {
            std::mem::swap(current_p.get_mut_value(),node.get_mut_value());
        
            return None;
        }
    }

    Some(path_save)
}

pub fn remove(&mut self,k:&K)  {

    let null=NodePtr::new();
    //标记最终要删除的节点
    let mut last_find_node=NodePtr::new();

    let mut currentLevel=self.root.get_level();
    let mut current_p=self.root;
    let mut head=self.root;
    let mut prev=self.root;

    for i in (0..currentLevel).rev() {
        current_p=prev;
        current_p= current_p.get_level_pointer(i);

        if  current_p==null || current_p.get_key() > k {

            continue;
        }
        
        while current_p.get_key()< k  {
            
            if current_p.get_level_pointer(i) ==null  {
               prev=current_p;
               break;
            } else {
                prev=current_p;
                current_p=current_p.get_level_pointer(i);
            }
              
        }
      
        if  current_p.get_key()==k {

             prev.set_level_pointer(i,current_p.get_level_pointer(i));
             last_find_node=current_p;
           }       
    }

    if last_find_node==null {
        println!("not find this node");
        return;
    }

    unsafe {
        //需释放数据所占用的空间,特别指出,free the space.
        let layout=Layout::new::<Node<K,V>>();    
        ptr::drop_in_place(&mut (*(last_find_node.0)).data as *mut (K,V));

        ptr::drop_in_place(&mut (*last_find_node.0).vecPtr as *mut ManVec<NodePtr<K,V>>);
        alloc::dealloc(last_find_node.0 as *mut u8,layout);
       }

    //头节点为空连接的同时删除
    for i in (0..currentLevel).rev() {
        
        current_p= head.get_level_pointer(i);
        if current_p==null {
            head.pop();
        }
    }
 
}


pub fn find(&self,k:&K) -> Option<&V> {

    let null=NodePtr::new();
    let mut currentLevel=self.root.get_level();
    let mut current_p=self.root;

    let mut prev=self.root;

    for i in (0..currentLevel).rev() {
        current_p=prev;

        current_p= current_p.get_level_pointer(i);

        if  current_p==null || current_p.get_key() > k  {

            continue;
        }
     
        while current_p.get_key()< k  {
            
            if current_p.get_level_pointer(i) ==null  {      
                prev=current_p;
                break;
            } else {
                prev=current_p;
                current_p=current_p.get_level_pointer(i);
            }
              
        }

        if  current_p.get_key()==k {
            let v= unsafe {
                (*current_p.0).get_value()
            };
            return Some(v);
           } 
    }

    None

}


pub fn find_mut(&mut self,k:&K) -> Option<&mut V> {

    let null=NodePtr::new();
    let mut currentLevel=self.root.get_level();
    let mut current_p=self.root;

    let mut prev=self.root;

    for i in (0..currentLevel).rev() {
        current_p=prev;

        current_p= current_p.get_level_pointer(i);

        if  current_p==null || current_p.get_key() > k  {

            continue;
        }
     
        while current_p.get_key()< k  {
            
            if current_p.get_level_pointer(i) ==null  {      
                prev=current_p;
                break;
            } else {
                prev=current_p;
                current_p=current_p.get_level_pointer(i);
            }
              
        }

        if  current_p.get_key()==k {
            let v= unsafe {
                (*current_p.0).get_mut_value()
                
            };
            return Some(v);
           }       
    }

    None
}


// 抛硬币决定节点有几层
pub fn flipCoin(&mut self) -> bool {    
    let rand:u32=self.rng.gen();
	if (rand%2) == 1 {
	    true
    } else {
		false
    }
}

}

impl <K:Ord+Default+Debug,V:Default> Drop for SkipList<K,V> {
    fn drop(&mut self) {
       
        let null=NodePtr::new();
        if self.root==null {
            return;
        }
        let mut ptr=self.root;
        let layout=Layout::new::<Node<K,V>>();    
        unsafe {            
            loop {

                 let mut free_flag=false;
                 let mut p_free=ptr;
                  if !ptr.get_level_pointer(0).is_null() {                    
                      ptr=ptr.get_level_pointer(0);
                } else {
                    free_flag=true;
                }
  
                     //需释放数据所占用的空间,特别指出,free the space.
                     ptr::drop_in_place(&mut (*(p_free.0)).data as *mut (K,V));
                
                     ptr::drop_in_place(&mut (*p_free.0).vecPtr as *mut ManVec<NodePtr<K,V>>);
                     alloc::dealloc(p_free.0 as *mut u8,layout);

                if free_flag {
                    break;
                }   
               
             }
             

         }
    }
}


#[test]
fn test_skiplist() {

 let mut v=SkipList::new();
    //  let mut current_p_next=self.root;
  v.insert((1,6));
  assert_eq!(v.find(&1),Some(&6));
  v.insert((4,7));
  assert_eq!(v.find(&4),Some(&7));

  v.insert((5,6));
  assert_eq!(v.find(&5),Some(&6));
  v.insert((6,8));
  assert_eq!(v.find(&6),Some(&8));

  v.insert((56,6));
  assert_eq!(v.find(&56),Some(&6));
  v.insert((56,8));
  assert_eq!(v.find(&56),Some(&8));

  v.remove(&56);
  assert_eq!(v.find(&56),None);

  for i in 100..100000 {
      v.insert((i,i+1));
  }

  assert_eq!(v.find(&999),Some(&1000));

  v.remove(&999);

  assert_eq!(v.find(&999),None);


}




