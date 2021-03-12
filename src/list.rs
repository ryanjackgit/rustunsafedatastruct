use std::alloc::{self, Layout};
use std::ptr;



/*
*x = equals to ::write for raw pointers, No!
They aren't equivalent. Dereferencing and assigning with = will attempt to drop the LHS value before overwriting with the new one. This code happens to work* because bools don't have destructors, but if you were assigning to some type with one then it would result in a drop call operating on uninitialized memory. Meanwhile ptr::write specifically avoids dropping the LHS before writing over it.

*Even though it happens to work in this case, it's still not behavior that should be relied upon

if t is *mut T , only when T:Copy :
*t=c; 

otherwise,
std::ptr::write(c,t) is right.  *t=c; is a UB.


*/


struct Node<T> {
   data:T,
   next:*mut Node<T>,
   pre:*mut Node<T>,
}


pub struct List<T> {
    head:Option<*mut Node<T>>,    
}

impl <T> List<T> {
    pub fn new() -> Self {
        List {
            head:None,
        }
    }

    pub fn newinit(t:T) -> Self {
        let layout=Layout::new::<Node<T>>();
        let node= unsafe {
           let c= alloc::alloc(layout) as *mut Node<T>;
             //  (*c).data=t;  此种写法不正确
           let mut data_pointer=&mut ((*c).data) as *mut T;
           ptr::write(data_pointer,t);
         //  println!("get memory is {:p}",data_pointer);
           (*c).next=ptr::null_mut::<Node<T>>();
           (*c).pre=ptr::null_mut::<Node<T>>();
    
           c
        };

  
       List {
         head:Some(node)
       }

    }

    pub fn insert_before(&mut self,t:T) {
        let layout=Layout::new::<Node<T>>();
        if let Some( mut ptr) = self.head {

          let node=  unsafe {
                let c= alloc::alloc(layout) as *mut Node<T>;
                //  (*c).data=t;
                let mut data_pointer=&mut ((*c).data) as *mut T;
                ptr::write(data_pointer,t);
              //  println!("get memory is {:p}",data_pointer);
              
                  (*c).next=ptr;
                  (*c).pre=ptr::null_mut::<Node<T>>();
                  (*ptr).pre=c;
                  c
                  
            };
            self.head=Some(node);
             
        } else {
             //  构建初始节点
          
             let node= unsafe {
                let c= alloc::alloc(layout) as *mut Node<T>;
              //  (*c).data=t;
              let mut data_pointer=&mut ((*c).data) as *mut T;
              ptr::write(data_pointer,t);
            //  println!("get memory is {:p}",data_pointer);
            
                (*c).next=ptr::null_mut::<Node<T>>();
                (*c).pre=ptr::null_mut::<Node<T>>();
         
                c
             };
             self.head=Some(node);
        }
    }

    pub fn insert_after(&mut self,t:T) {
        if let Some( mut ptr) = self.head {
            //循环找到最后一个节点
            unsafe {

              while !(*ptr).next.is_null() {
                  ptr=(*ptr).next;
              }

            //在最后一个节点之后添加新节点
           
                let layout=Layout::new::<Node<T>>();
                let c= alloc::alloc(layout) as *mut Node<T>;
               // (*c).data=t;
              let mut data_pointer=&mut ((*c).data) as *mut T;
              ptr::write(data_pointer,t);
            //  println!("get memory is {:p}",data_pointer);

                (*c).next=ptr::null_mut::<Node<T>>();

                (*c).pre=ptr;

                (*ptr).next=c;
            }
        } else {
            //  构建初始节点
            let layout=Layout::new::<Node<T>>();
            let node= unsafe {
               let c= alloc::alloc(layout) as *mut Node<T>;
             //  (*c).data=t;
             let mut data_pointer=&mut ((*c).data) as *mut T;
             ptr::write(data_pointer,t);
           //  println!("get memory is {:p}",data_pointer);
           
               (*c).next=ptr::null_mut::<Node<T>>();
               (*c).pre=ptr::null_mut::<Node<T>>();
        
               c
            };
            self.head=Some(node);
        }
    }

    pub fn remove_after(&mut self) -> Option<T> {
        if let Some(mut ptr) = self.head {

          let value=  unsafe {

            let layout=Layout::new::<Node<T>>();
              //刚好只有一个节点的删除方法
            if (*ptr).next.is_null() {
                let last=ptr::read(&mut (*ptr).data as *mut T);

             
                alloc::dealloc(ptr as *mut u8,layout);
                self.head=None;
                 Some(last)
            } else {
             
             //超过一个节点的删除方法
                while !(*ptr).next.is_null() {
                    ptr=(*ptr).next;
                }
                 
                 let mut pre_ptr=(*ptr).pre;
                 (*pre_ptr).next=ptr::null_mut::<Node<T>>();

                //先读取值，然后再释放内存
                let last=ptr::read(&mut (*ptr).data as *mut T);

             
                alloc::dealloc(ptr as *mut u8,layout);
                 Some(last)
            }

            };
            return value;
  
        } else {
            return None;
        }
    }

    pub fn find(&self,t:&T) -> bool 
    where T:Eq {
        if let Some(ptrone) =self.head {
            let mut ptr=ptrone;
             unsafe {
                 loop {
                     if &(*ptr).data==t {
                         return true;
                     }
                     if (*ptr).next.is_null() {
                       
                      return false;
                      
                     } else {
                      
                       ptr=(*ptr).next;
                     }
                 }
             }
         } else {
             false
         }
    }

    
  
    pub fn remove_current(&mut self,t:&T) 
         where T:Eq  {
             let exist_flag=self.find(t);
  
           if exist_flag {

            if let Some(mut ptrone) =self.head {
                let mut ptr=ptrone;
                //if it is root
                unsafe {
                    //if data is just root
                if &(*ptr).data==t {
                  //  this list is only node
                    if (*ptr).next.is_null() {
                        self.head=None;
                    } else {
                      //remove the head element ,it has next element
                        let next=(*ptr).next;
                        (*next).pre=ptr::null_mut::<Node<T>>();
                        self.head=Some(next);
                    }
                  
                    //drop this node and dealloc it
                    let layout=Layout::new::<Node<T>>();
                    ptr::drop_in_place(&mut (*ptr).data as *mut T);
                    alloc::dealloc(ptr as *mut u8,layout);
                    return;
                }
                
                
                     loop {
                         if &(*ptr).data==t {
                          
                             let pre=(*ptr).pre;
                             let next=(*ptr).next;
                             if next.is_null() {
                              // remove the last element
                                self.remove_after();
                             } else {
                            // remove the middle element
                                (*pre).next=next;
                                (*next).pre=pre;

                                let layout=Layout::new::<Node<T>>();
                                ptr::drop_in_place(&mut (*ptr).data as *mut T);
                                alloc::dealloc(ptr as *mut u8,layout);
                             }

                             return ;
                         }
                         if (*ptr).next.is_null() {
                           
                          return ;
                          
                         } else {
                          
                           ptr=(*ptr).next;
                         }
                     }
                 }
             } 
           }
         }

    pub fn list(&self) {

        if let Some(ptrone) =self.head {
           let mut ptr=ptrone;
            unsafe {
                loop {
               //     println!("the node data is {:?}",1);
                    if (*ptr).next.is_null() {
                      
                     return;
                     
                    } else {
                     
                      ptr=(*ptr).next;
                    }
                }
            }
        } else {
            println!("this list is empty");
        }
    }

    pub fn iter(&self) -> List_Iterator<T> {
        List_Iterator::new(self)
    }

}

impl <T> Drop for List<T> {
    fn drop(&mut self) {
       
      
        if let Some(mut ptr) = self.head {
             unsafe {
                loop {
              
                    let layout=Layout::new::<Node<T>>();                    
                     let mut p_free=ptr;
                     let mut free_flag=false;
                  //   println!("the node is {:p}",p_free);
                      if !(*ptr).next.is_null() {                    
                        ptr=(*ptr).next;
                    } else {
                        free_flag=true;
                    }
      
                         //需释放数据所占用的空间,特别指出,free the space.
                         ptr::drop_in_place(&mut (*p_free).data as *mut T);
                         alloc::dealloc(p_free as *mut u8,layout);

                    if free_flag {
                        break;
                    }   
                   
                 }
               
               
             }
           //  println!("drop ");
        } else {
            println!("it's none ");
        }
    }
}


use std::iter::IntoIterator;

impl <'a,T:'a> IntoIterator for &'a List<T> {
    type Item= &'a T;
    type IntoIter = List_Iterator<'a,T>;
    fn into_iter(self) -> Self::IntoIter {
        List_Iterator::new(&self)
    }
}

impl <'a,T:'a> IntoIterator for &'a mut List<T> {
    type Item= &'a mut T;
    type IntoIter = List_Mut_Iterator<'a,T>;
    fn into_iter(mut self) -> Self::IntoIter {
        List_Mut_Iterator::new(self)
    }
}


impl <T> IntoIterator for List<T> {
    type Item= T;
    type IntoIter = List_Self_Iterator<T>;
    fn into_iter(self) -> Self::IntoIter {

    let state;
       
    if let Some(headpoiter) = self.head {
        state=Some(headpoiter);
    
    } else {
        state=None;
     
    }

    //aatention!!!!   don't free this,that iterator will free 
    std::mem::forget(self);

    List_Self_Iterator {
       state:state,
   }

    }
}



pub struct List_Mut_Iterator<'a,T> {
    data:&'a mut List<T>,
    state:Option<*mut Node<T>>,    
  //  _marker:PhantomData<&'a T>
}

impl <'a,T> List_Mut_Iterator<'a,T> {
        fn new(list:&'a mut List<T>) -> Self {
            let state;
            if let Some(headpoiter) = list.head {
                state=Some(headpoiter);
            } else {
                state=None;
            }
            List_Mut_Iterator {
               data:list,
               state:state,
           
           }
        }
}


impl <'a,T> Iterator for List_Mut_Iterator<'a,T> {
    type Item=&'a mut T;

    fn next(&mut self) -> Option<Self::Item> {
       match self.state {
           None => None,
           Some(pointer) => {
               //找到指针下一个值
               unsafe {
                   let next=(*pointer).next;
                   if next.is_null() {
                     self.state=None;
                   } else {
                   self.state=Some(next);
                   }
               //    let data=ptr::read(&(*next).data as *const T);
                   let data= &mut (*pointer).data;
               
                   Some(data)
               }
           }
       }
    }
}



pub struct List_Iterator<'a,T> {
    data:&'a List<T>,
    state:Option<*mut Node<T>>,    
  //  _marker:PhantomData<&'a T>
}

impl <'a,T> List_Iterator<'a,T> {
        fn new(list:&'a List<T>) -> Self {
            let state;
            if let Some(headpoiter) = list.head {
                state=Some(headpoiter);
            } else {
                state=None;
            }
            List_Iterator {
               data:list,
               state:state,
           //    _marker:PhantomData,
           }
        }
}


impl <'a,T> Iterator for List_Iterator<'a,T> {
    type Item=&'a T;

    fn next(&mut self) -> Option<Self::Item> {
       match self.state {
           None => None,
           Some(pointer) => {
               //找到指针下一个值
               unsafe {
                   let next=(*pointer).next;
                   if next.is_null() {
                     self.state=None;
                   } else {
                   self.state=Some(next);
                   }
               //    let data=ptr::read(&(*next).data as *const T);
                   let data= &(*pointer).data;
               
                   Some(data)
               }
           }
       }
    }
}


pub struct List_Self_Iterator<T> {
    
   // data:List<T>,
    
    
    state:Option<*mut Node<T>>,    
 
}



impl <T> Iterator for List_Self_Iterator<T> {
    type Item=T;

    fn next(&mut self) -> Option<Self::Item> {
       match self.state {
           None => None,
           Some(pointer) => {
               //找到指针下一个值
               unsafe {
                   let next=(*pointer).next;
                   if next.is_null() {
                     self.state=None;
                   } else {
                   self.state=Some(next);
                   }

                let data=ptr::read(&mut (*pointer).data as *mut T);
                let layout=Layout::new::<Node<T>>();
                alloc::dealloc(pointer as *mut u8,layout);
               
                Some(data)
               }
           }
       }
    }
}
 


impl <T> Drop for List_Self_Iterator<T> {
    fn drop(&mut self) {
        match self.state {
            None => {},
            Some(mut ptr) => {

                unsafe {
                     loop {
              
                    let layout=Layout::new::<Node<T>>();
                     
                     let mut p_free=ptr;
                     let mut free_flag=false;
                     println!("the node is {:p}",p_free);
                      if !(*ptr).next.is_null() {                    
                        ptr=(*ptr).next;
                    } else {
                        free_flag=true;
                    }
      
                         //需释放还未iterator过的数据所占用的空间,特别指出
                         ptr::drop_in_place(&mut (*p_free).data as *mut T);
                         alloc::dealloc(p_free as *mut u8,layout);

                    if free_flag {
                        break;
                    }   
                   
                 }
                }

            }
    }
}
}
