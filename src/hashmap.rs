
use crate::vec::ManVec;
use crate::list::{self,List};

use std::alloc::{self, dealloc, Layout};
use std::ptr;
use std::mem;
use std::fmt::Debug;
use std::cmp::{Ord,Ordering,Eq,PartialEq};
use std::marker::PhantomData;

use std::collections::hash_map::DefaultHasher;
use std::collections::hash_map::RandomState;
use std::collections::hash_map::OccupiedEntry;
use std::collections::hash_map::VacantEntry;

use std::borrow::Borrow;

use std::hash::{Hash, Hasher,BuildHasher};
use std::iter::Iterator;


pub enum Entry<'a, K: 'a, V: 'a> {
    Occupied(OccupiedEntry<'a, K, V>),
    Vacant(VacantEntry<'a, K, V>),
}

pub type DefaultHashBuilder = RandomState;

// attention hash attack-flood,rust use SipHash 1-3 default.
pub struct HashMap<K,V,H=DefaultHashBuilder> {
    vec:ManVec<HashTableList<K,V>>,
    build_hasher:H,
    cap:usize,
}

impl <K:Hash+Eq+PartialEq,V>  HashMap<K,V,DefaultHashBuilder> {

    pub fn new() -> Self {
       let default_capacity=16;
       
       Self::new_with_capacity(default_capacity)
      
    }

    pub fn new_with_capacity(default_capacity:usize) -> Self {
            let mut v=ManVec::new();
            for i in 0..default_capacity {
                let mut list=HashTableList::new();
                v.push(list);
            }

        HashMap {
            vec:v,
            build_hasher:RandomState::new(),
            cap:default_capacity,
       
           
        }
    }

    //The iterator Item type is (&'a K, &'a mut V)

       pub fn iter_mut(&mut self) -> Option<List<(&K, &mut V)>> {
           //构建一个大的包括所有Hashmap的链表值
           if self.item_len() ==0 {
               return None;
           }

           let mut list=List::new();
       
           for i in 0..self.cap {

            if let Some(value_list) = self.vec.get_mut(i as usize) {
       
              if let Some(mut ptr) = value_list.head {
                unsafe {
                    loop {
                        let data=&mut (*ptr).data;
                        list.insert_after((&data.0,&mut data.1));
                        if (*ptr).next.is_null() {
                          
                         break;
                         
                        } else {
                         
                          ptr=(*ptr).next;
                        }
                    }
                }
               
              }
            }
        }

        Some(list)

       }

    pub fn insert(&mut self,k:K,v:V) {
          let mut my_hasher= self.build_hasher.build_hasher();
           k.hash(&mut my_hasher);
           let hash_code=my_hasher.finish();
           let index=hash_code%self.cap as u64;

           if let Some(mut value_list) = self.vec.get_mut(index as usize) {
            
               let existed=match value_list.find(&k) {
                   Some(_) => true,
                   None => false,
               };
               if existed {
                value_list.remove_current(&k);
                let s=(k,v);
                value_list.insert_after(s);
               } else {
                let s=(k,v);
               value_list.insert_after(s);
               }
           } 
    }

  
    pub fn contains_key(&self,k:&K) -> bool {
        let mut my_hasher= self.build_hasher.build_hasher();
        k.hash(&mut my_hasher);
        let hash_code=my_hasher.finish();
        let index=hash_code%self.cap as u64;

        if let Some( value_list) = self.vec.get(index as usize) {
           if let Some(_) = value_list.find(k) {
               return true;
           } else {
               return false;
           }
        } else {
            return false;
        }
             
    }

// k:&Q where   K: Borrow<Q>,
//    Q: Hash + Eq +PartialEq+ ?Sized 
    pub fn get(&self,k:&K) -> Option<&V>  {
        let mut my_hasher= self.build_hasher.build_hasher();
        k.hash(&mut my_hasher);
        let hash_code=my_hasher.finish();
        let index=hash_code%self.cap as u64;

        if let Some(value_list) = self.vec.get(index as usize) {
           if let Some(v) = value_list.find(k) {
                  Some(&v.1)
           } else {
               None
           }
        } else {
            None
        }   
    }

    pub fn get_mut(&mut self,k:&K) -> Option<&mut V> {
        let mut my_hasher= self.build_hasher.build_hasher();
        k.hash(&mut my_hasher);
        let hash_code=my_hasher.finish();
        let index=hash_code%self.cap as u64;

        if let Some(mut value_list) = self.vec.get_mut(index as usize) {
           if let Some(mut v) =value_list.find_mut(k) {
                  Some(&mut v.1)
           } else {
               None
           }
        } else {
            None
        }   
    }

    pub fn remove(&mut self,k:&K) -> Option<(K,V)> {
        let mut my_hasher= self.build_hasher.build_hasher();
        k.hash(&mut my_hasher);
        let hash_code=my_hasher.finish();
        let index=hash_code%self.cap as u64;

        if let Some( mut value_list) = self.vec.get_mut(index as usize) {
            value_list.remove_current(k)
        } else {
            None
        }   
    }

    pub fn item_len(&self) -> usize {
        let mut count=0;
        for i in 0..self.cap {
            if let Some(value_list) = self.vec.get(i as usize) {
          //      println!("index:{} , the count element is {}",i,value_list.size());
              if let Some(_) = value_list.head {
                count+=1;
              }
            }
        }
        count
    }

    pub fn len(&self) -> usize {
        let mut count=0;
        for i in 0..self.cap {
            if let Some(value_list) = self.vec.get(i as usize) {
                println!("index:{} , the count element is {}",i,value_list.size());
           
                count+=value_list.size();
              
            }
        }
        count
    }

    pub fn is_empty(&self) -> bool {
        if self.item_len() ==0 {
            true
        } else {
            false
        }
    }


}



struct Node<K,V> {
    data:(K,V),
    next:*mut Node<K,V>,
    pre:*mut Node<K,V>,
 }
 
 
 pub struct HashTableList<K,V> {
     head:Option<*mut Node<K,V>>,    
 }
 

 impl <K:Hash+Eq+PartialEq,V> HashTableList<K,V> {

     pub fn new() -> Self {
        HashTableList {
             head:None,
         }
     }
 
     pub fn newinit(t:(K,V)) -> Self {

         let layout=Layout::new::<Node<K,V>>();
         let node= unsafe {
            let c= alloc::alloc(layout) as *mut Node<K,V>;
              //  (*c).data=t;  此种写法不正确
            let mut data_pointer=&mut ((*c).data) as *mut (K,V);
            ptr::write(data_pointer,t);
          //  println!("get memory is {:p}",data_pointer);
            (*c).next=ptr::null_mut::<Node<K,V>>();
            (*c).pre=ptr::null_mut::<Node<K,V>>();
     
            c
         };
         HashTableList {
          head:Some(node)
        }

     }
 

 
     pub fn insert_after(&mut self,t:(K,V)) {

         if let Some( mut ptr) = self.head {
             //循环找到最后一个节点
             unsafe {
 
               while !(*ptr).next.is_null() {
                   ptr=(*ptr).next;
               }
 
             //在最后一个节点之后添加新节点
            
                 let layout=Layout::new::<Node<K,V>>();
                 let c= alloc::alloc(layout) as *mut Node<K,V>;
                // (*c).data=t;
               let mut data_pointer=&mut ((*c).data) as *mut (K,V);
               ptr::write(data_pointer,t);
             //  println!("get memory is {:p}",data_pointer);
 
                 (*c).next=ptr::null_mut::<Node<K,V>>();
 
                 (*c).pre=ptr;
 
                 (*ptr).next=c;
             }
         } else {
             //  构建初始节点
             let layout=Layout::new::<Node<K,V>>();
             let node= unsafe {
                let c= alloc::alloc(layout) as *mut Node<K,V>;
              //  (*c).data=t;
              let mut data_pointer=&mut ((*c).data) as *mut (K,V);
              ptr::write(data_pointer,t);
            //  println!("get memory is {:p}",data_pointer);
            
                (*c).next=ptr::null_mut::<Node<K,V>>();
                (*c).pre=ptr::null_mut::<Node<K,V>>();
         
                c
             };
             self.head=Some(node);
         }
     }
 
     pub fn remove_after(&mut self) -> Option<(K,V)> {
         if let Some(mut ptr) = self.head {
 
           let value=  unsafe {
 
             let layout=Layout::new::<Node<K,V>>();
               //刚好只有一个节点的删除方法
             if (*ptr).next.is_null() {
                 let last=ptr::read(&mut (*ptr).data as *mut (K,V));              
                 alloc::dealloc(ptr as *mut u8,layout);
                 self.head=None;
                  Some(last)
             } else {
              
              //超过一个节点的删除方法
                 while !(*ptr).next.is_null() {
                     ptr=(*ptr).next;
                 }
                  
                  let mut pre_ptr=(*ptr).pre;
                  (*pre_ptr).next=ptr::null_mut::<Node<K,V>>();
 
                 //先读取值，然后再释放内存
                 let last=ptr::read(&mut (*ptr).data as *mut (K,V));
 
              
                 alloc::dealloc(ptr as *mut u8,layout);
                  Some(last)
             }
 
             };
             return value;
   
         } else {
             return None;
         }
     }
 
     pub fn find(&self,t:&K) -> Option<&(K,V)> {
         if let Some(ptrone) =self.head {
             let mut ptr=ptrone;
              unsafe {
                  loop {
                      if &(*ptr).data.0==t {
                          return Some(&(*ptr).data);
                      }
                      if (*ptr).next.is_null() {
                        
                       return None;
                       
                      } else {
                       
                        ptr=(*ptr).next;
                      }
                  }
              }
          } else {
              None
          }
     }


     pub fn find_mut(&mut self,t:&K) -> Option<&mut (K,V)> {
        if let Some(ptrone) =self.head {
            let mut ptr=ptrone;
             unsafe {
                 loop {
                     if &(*ptr).data.0==t {
                         return Some(&mut (*ptr).data);
                     }
                     if (*ptr).next.is_null() {
                       
                      return None;
                      
                     } else {
                      
                       ptr=(*ptr).next;
                     }
                 }
             }
         } else {
             None
         }
    }
 
     
   
     pub fn remove_current(&mut self,t:&K) -> Option<(K,V)> {
              let exist_flag= match self.find(t) {
                  Some(_) => true,
                  None => false,
              };
    
            if exist_flag {
 
             if let Some(mut ptrone) =self.head {
                 let mut ptr=ptrone;
                 //if it is root
                 unsafe {
                     //if data is just root
                 if &(*ptr).data.0==t {
                   //  this list is only node
                     if (*ptr).next.is_null() {
                         self.head=None;
                     } else {
                       //remove the head element ,it has next element
                         let next=(*ptr).next;
                         (*next).pre=ptr::null_mut::<Node<K,V>>();
                         self.head=Some(next);
                     }
                   
                     //drop this node and dealloc it
                     let layout=Layout::new::<Node<K,V>>();
                     let v=ptr::read(&mut (*ptr).data as *mut (K,V));
                     alloc::dealloc(ptr as *mut u8,layout);
                     return Some(v);
                 }
                 
                 
                      loop {
                          if &(*ptr).data.0==t {
                           
                              let pre=(*ptr).pre;
                              let next=(*ptr).next;
                              if next.is_null() {
                               // remove the last element
                               return  self.remove_after();
                              } else {
                             // remove the middle element
                                 (*pre).next=next;
                                 (*next).pre=pre;
 
                                 let layout=Layout::new::<Node<K,V>>();
                                let v= ptr::read(&mut (*ptr).data as *mut (K,V));
                                 alloc::dealloc(ptr as *mut u8,layout);
                                 return Some(v);
                              }
 
                             
                          }
                          if (*ptr).next.is_null() {
                            
                           return None;
                           
                          } else {
                           
                            ptr=(*ptr).next;
                          }
                      }
                  }
              } 
            }
           return None;

          }


          pub fn size(&self) -> usize {
             let mut count=0;
            if let Some(ptrone) =self.head {
               let mut ptr=ptrone;
                unsafe {
                    loop {
                        count=count+1;
                        if (*ptr).next.is_null() {
                          
                         return count;
                         
                        } else {
                         
                          ptr=(*ptr).next;
                        }
                    }
                }
            } else {
              
                return count;
            }
        }

       //The iterator Item type is (&'a K, &'a mut V)

       pub fn iter_mut(&mut self) -> HashTable_Mut_Iterator<'_,K,V> {
        HashTable_Mut_Iterator::new(self)
       }

       pub fn into_iter(self) -> HashTable_Into_Iterator<K,V> {
        HashTable_Into_Iterator::new(self)
       }
 }

 
//可变值的迭代器  iter_mut 
 pub struct HashTable_Mut_Iterator<'a,K,V> {
     data:&'a mut HashTableList<K,V>,
     state:Option<*mut Node<K,V>>,    
 }

 impl <'a,K:Hash+Eq+PartialEq,V> HashTable_Mut_Iterator<'a,K,V> {
         fn new(list:&'a mut HashTableList<K,V>) -> Self {
             let state;
             if let Some(headpoiter) = list.head {
                 state=Some(headpoiter);
             } else {
                 state=None;
             }
            HashTable_Mut_Iterator {
                data:list,
                state:state,
            }
         }
 }

 impl <'a,K:Hash+Eq+PartialEq,V> Iterator for HashTable_Mut_Iterator<'a,K,V> {
     type Item=(&'a K,&'a mut V);

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
                    Some((&(*next).data.0,&mut (*next).data.1))
                }
            }
        }
     }
 }


 //消耗型迭代器     into_iterator
 pub struct HashTable_Into_Iterator<K,V> {
    data:HashTableList<K,V>,
    state:Option<*mut Node<K,V>>,    
}

impl <K:Hash+Eq+PartialEq,V> HashTable_Into_Iterator<K,V> {
        fn new(list:HashTableList<K,V>) -> Self {
            let state;
            if let Some(headpoiter) = list.head {
                state=Some(headpoiter);
            } else {
                state=None;
            }
           HashTable_Into_Iterator {
               data:list,
               state:state,
           }
        }
}

impl <K:Hash+Eq+PartialEq,V> Iterator for HashTable_Into_Iterator<K,V> {
    type Item=(K,V);

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
                   let data=ptr::read(&mut (*next).data as *mut (K,V));
                   Some((data.0,data.1))
               }
           }
       }
    }
}
 
 impl <K,V> Drop for HashTableList<K,V> {
     fn drop(&mut self) {
        
       
         if let Some(mut ptr) = self.head {
              unsafe {
                 let layout=Layout::new::<Node<K,V>>();
 
                 let mut pp_head=ptr;
 
                 while !(*ptr).next.is_null() {
                     ptr=(*ptr).next;
                 }
                 
                let mut pp;
            
                 while !(*ptr).pre.is_null()  {
                    
                      pp=(*ptr).pre;
                      //释放数据所占用的空间,特别指出, free space and dealloc it.
                      ptr::drop_in_place(&mut (*ptr).data as *mut (K,V));
                      alloc::dealloc(ptr as *mut u8,layout);
                      ptr=pp;
                 } 
 
               
                 ptr::drop_in_place(&mut (*pp_head).data as *mut (K,V));
                 alloc::dealloc(pp_head as *mut u8,layout);
                
                
              }
            //  println!("drop ");
         } else {
            // println!("it's none ");
         }
     }
 }
 

#[test]
fn test_hashmap() {
    let mut h=HashMap::new();
    h.insert(1,3);
    h.insert(2,5);
    
    assert_eq!(h.get(&2), Some(&5));
    assert_eq!(h.len(),2);
    assert_eq!(h.remove(&1),Some((1,3)));
    assert_eq!(h.len(),1);
    h.insert(2,6);
    assert_eq!(h.len(),1);
    assert_eq!(h.get(&2), Some(&6));
    assert_eq!(h.remove(&2),Some((2,6)));
    assert_eq!(h.remove(&3),None);
    assert_eq!(h.len(),0);

}

#[test]
fn test_hashmap_other() {
    let mut h=HashMap::new();

    for i in 0..10000 {
        h.insert(i,i+1);

    }


    assert_eq!(h.get(&289), Some(&290));
    assert_eq!(h.remove(&289),Some((289,290)));
    h.insert(289,290);
    assert_eq!(h.get(&289), Some(&290));
    h.insert(289,290);
    assert_eq!(h.get(&289), Some(&290));

    for i in 0..2000 {
        assert_eq!(h.remove(&i),Some((i,i+1)));
    }
    assert_eq!(h.get(&289), None);

}


#[test]
fn test_listhashtable() {

    let mut hl=HashTableList::new();
    hl.insert_after((1,5));
    hl.insert_after((2,5));
    hl.insert_after((3,5));
    assert_eq!(hl.remove_after(),Some((3,5)));
    assert_eq!(hl.remove_after(),Some((2,5)));

    assert_eq!(hl.find(&1),Some(&(1,5)));
    
}