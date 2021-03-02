use crate::list::List;
use crate::vec::ManVec;

use std::alloc::{self, dealloc, Layout};
use std::ptr;
use std::mem;
use std::fmt::Debug;


pub struct TreeNode<T: std::fmt::Debug> {
    data: T,
    left: *mut TreeNode<T>,
    right: *mut TreeNode<T>,
}



pub struct Tree<T: std::fmt::Debug>{
    root:Option<*mut TreeNode<T>>,
}

impl <T: std::fmt::Debug> Tree<T> {
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
        

        Tree {
            root:Some(node),
        }
    }

    pub fn new_as_node(root:*mut TreeNode<T>) ->Self {
        Tree {
        root:Some(root),
        }
    }

    pub fn first_root(&self) {
        unsafe {
        if let Some(root) =self.root {
              let node_data=&(*root).data;
              println!("the data is {:?}",node_data);
              if !(*root).left.is_null() {
              let left= Tree {
                 root:Some((*root).left),
              };
              left.first_root();
              std::mem::forget(left);
            }
            if !(*root).right.is_null() {
                let right= Tree {
                   root:Some((*root).right),
                };
                right.first_root();
                std::mem::forget(right);
              }

        } else {
            return;
        }
    }
    }

    pub fn last_root(&self) {
        unsafe {
        if let Some(root) =self.root {
             
              if !(*root).left.is_null() {
              let left= Tree {
                 root:Some((*root).left),
              };
        
              left.last_root();
              std::mem::forget(left);
            }
            if !(*root).right.is_null() {
                let right= Tree {
                   root:Some((*root).right),
                };
             
                right.last_root();
                std::mem::forget(right);
              }

              let node_data=&(*root).data;
              println!("the data is {:?}",node_data);

        } else {
            return;
        }
    }
    }


    pub fn middle_root(&self) {
        unsafe {
        if let Some(root) =self.root {
             
              if !(*root).left.is_null() {
              let left= Tree {
                 root:Some((*root).left),
              };
              left.middle_root();
              std::mem::forget(left);
            }

            let node_data=&(*root).data;
            println!("the data is {:?}",node_data);

            if !(*root).right.is_null() {
                let right= Tree {
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


}


impl <T: std::fmt::Debug> Drop for Tree<T> {
  fn drop(&mut self) {
     if let Some(root) =self.root {
        drop_middle_root(root);
     }
  }
}


pub fn drop_middle_root<T: std::fmt::Debug>(root:*mut TreeNode<T>) {
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





