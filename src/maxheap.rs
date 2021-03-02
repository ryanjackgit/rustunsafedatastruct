use crate::list::List;
use crate::vec::ManVec;

use std::alloc::{self, dealloc, Layout};
use std::ptr;
use std::mem;
use std::fmt::Debug;
use std::cmp::{Ord,Ordering};

pub fn swap<T>(t1:&mut T,t2:&mut T) {
    unsafe {
       let temp=ptr::read(t1 as *mut T);
       ptr::copy_nonoverlapping(t2 as *const T,t1 as *mut T,1);
       ptr::write(t2 as *mut T,temp);
    }
}

pub struct MaxHeap<T:Ord+Debug> {
    data:ManVec<T>,
}

impl <T:Ord+Debug> MaxHeap<T> {
    pub fn new() ->  Self {
        MaxHeap {
            data:ManVec::new(),
        }
    }
    pub fn push(&mut self,t:T) {
         if self.data.is_null() {
             self.data.push(t);
             return;
         }
         self.data.push(t);
         self.swim();

    }

    pub fn peek(&self) -> Option<&T> {
        if self.data.is_null() {
            None
        } else {
            self.data.get(0)
        }
    }

    pub fn pop(&mut self) -> Option<T> {
        if self.data.is_null() {
            None
        } else if  self.data.len()==1 {
           self.data.pop()
        } else {
           //把最后一个与第一个互相交换，而然pop最后一个,再做sink操作
           let len=self.data.len();
           let  index=len-1;
           self.data.swap(0,index);
           let t=self.data.pop();
           self.sink();
           t
        }
    }

    fn swim(&mut self) {
        let len=self.data.len();
        let mut index=len-1;
      
        while index>=1 {
        
            let mut inner_data=&self.data;
            let cur=inner_data.get(index).unwrap();
            let i=(index-1)/2;
            let parent=inner_data.get(i).unwrap();
             if cur>parent {
                 let mut data=&mut self.data;
                 data.swap(index,i);
                 index=i;
             } else {
                 break;
             }
        }

    }

    pub fn sink(&mut self) {
        let len=self.data.len();
       
        if len==0 || len==1 {
            return;
        }
        let mut index=0;
        while 2*index+1 <len {
            let cur=self.data.get(index).unwrap();
            let mut cur_index=2*index+1;
           let mut left=self.data.get(2*index+1).unwrap();
          
           if 2*index+2<len  {
            let right=self.data.get(2*index+2).unwrap();
            if right>left {
                left=right;
                cur_index+=1;
            }

           }

           if left>cur {
             self.data.swap(index,cur_index);
             index=cur_index;
           } else {
               break;
           }
        }
    }

    pub fn print(&self) {
        self.data.printlnall();
    }
}




