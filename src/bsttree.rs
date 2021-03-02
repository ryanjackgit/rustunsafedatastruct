use crate::list::List;
use crate::vec::ManVec;

use std::alloc::{self, dealloc, Layout};
use std::ptr;
use std::mem;
use std::fmt::Debug;
use std::cmp::{Ord,Ordering};


 pub struct TreeNode<T: std::fmt::Debug+Ord> {
    data: T,
    left: *mut TreeNode<T>,
    right: *mut TreeNode<T>,
}



pub struct BSTTree<T: std::fmt::Debug+Ord>{
    root:Option<*mut TreeNode<T>>,
}

impl <T: std::fmt::Debug+Ord> BSTTree<T> {
    pub fn newnew() -> Self {
        BSTTree {
            root:None,
        }
    }
   pub  fn new(t:T) -> Self {

            let layout=Layout::new::<TreeNode<T>>();
            let node= unsafe {
               let c= alloc::alloc(layout) as *mut TreeNode<T>;
             //  (*c).data=t;
             let mut data_pointer=&mut (*c).data as *mut T;
             ptr::write(data_pointer,t);
           //  println!("get memory is {:p}",data_pointer);
           
               (*c).left=ptr::null_mut::<TreeNode<T>>();
               (*c).right=ptr::null_mut::<TreeNode<T>>();
        
               c
            };
        

            BSTTree {
            root:Some(node),
        }
    }

    pub fn new_as_node(root:*mut TreeNode<T>) ->Self {
        BSTTree {
        root:Some(root),
        }
    }

    pub fn middle_root(&self) {
        unsafe {
        if let Some(root) =self.root {
             
              if !(*root).left.is_null() {
              let left= BSTTree {
                 root:Some((*root).left),
              };
              left.middle_root();
              std::mem::forget(left);
            }

            let node_data=&(*root).data;
            println!("the data is {:?}",node_data);

            if !(*root).right.is_null() {
                let right= BSTTree {
                   root:Some((*root).right),
                };
                right.middle_root();
                std::mem::forget(right);
              }

           

        } else {
            return;
        }
    }
    }

    pub fn create_node(t:T,left: *mut TreeNode<T>,right: *mut TreeNode<T>) -> *mut TreeNode<T> {
        let layout=Layout::new::<TreeNode<T>>();
        unsafe {
           let c= alloc::alloc(layout) as *mut TreeNode<T>;
         //  (*c).data=t;
         let mut data_pointer=&mut (*c).data as *mut T;
         ptr::write(data_pointer,t);
       //  println!("get memory is {:p}",data_pointer);
       
           (*c).left=left;
           (*c).right=right;
    
           c
        }
    }

    pub fn insert_node(&mut self,t: *mut TreeNode<T>)  {
     
        let data= unsafe {
            & (*t).data
        };
              if let Some(mut xroot)=self.root {
                 let xroot_data= unsafe {
                    & (*xroot).data
                };
                if data<=xroot_data {
                
                    let mut left= unsafe {(*xroot).left};
                    if left.is_null() {
                        unsafe {
                            (*xroot).left=t;
                            return;
                        }
                    }
                    let mut left_root=BSTTree {
                        root:Some(left),
                    };
                    left_root.insert_node(t);
                    std::mem::forget(left_root);
                } else {
               
                    let mut right= unsafe {(*xroot).right};
                    if right.is_null(){
                        unsafe {
                            (*xroot).right=t;
                            return;
                        }
                    }
                    let mut roght_root=BSTTree {
                        root:Some(right),
                    };
                    roght_root.insert_node(t);
                    std::mem::forget(roght_root);
                }
               
              } else {
          
                  println!("the BSTTree is null");
                  self.root=Some(t);
                 
              }
    }

    pub fn search_node(&mut self,t:&T)-> bool {
             if let Some(mut xroot) = self.root {
                 let root_data=unsafe {& (*xroot).data };
                 if root_data==t {
                     return true;
                 }
                  if (t>root_data) {
                    let mut right= unsafe {(*xroot).right};
                    
                    if right.is_null(){
                       return false;
                    }
                    
                    let mut roght_root=BSTTree {
                        root:Some(right),
                    };
                    let find_flag=roght_root.search_node(t);
                    std::mem::forget(roght_root);
                    return find_flag;
                } else {
                    let mut left= unsafe {(*xroot).left};
                    
                    if left.is_null(){
                       return false;
                    }
                    
                    let mut left_root=BSTTree {
                        root:Some(left),
                    };
                    let find_flag=left_root.search_node(t);
                    std::mem::forget(left_root);
                    return find_flag;
                }
                  
             } else {
                 return false;
             }
    }
    
}


impl <T: std::fmt::Debug+Ord> Drop for BSTTree<T> {
    fn drop(&mut self) {
       if let Some(root) =self.root {
          drop_middle_root(root);
       }
    }
  }
  
  
  pub fn drop_middle_root<T: std::fmt::Debug+Ord>(root:*mut TreeNode<T>) {
      let layout=Layout::new::<TreeNode<T>>();
      unsafe {
     
      
         let  ss_root=root;
  
  
       if !(*ss_root).left.is_null() {
          
          let p_root=(*ss_root).left;
          drop_middle_root(p_root);
      }
  
          if !(*ss_root).right.is_null() {
           
              let p_root= (*ss_root).right;
            
             drop_middle_root(p_root);
           
         
            }
  
         std::ptr::drop_in_place(&mut (*root).data as *mut T);
         alloc::dealloc(root as *mut u8,layout);
       
  }
  
  }
  
  

