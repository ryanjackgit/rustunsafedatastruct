use std::alloc::{self, dealloc, Layout};
use std::ptr;
use std::mem;
use std::fmt::Debug;
use std::ops::{Index,IndexMut,Range};
use std::marker::PhantomData;

pub struct ManVec<T> {
  // in std implemention,ptr is a  Unique<T> ,It's own type T,if T is Send/Sync,Vec<T> is Send/Sync.
    ptr: *mut T,
    cap:usize,
    len:usize,
    _marker: PhantomData<T>,
}


impl <T> ManVec<T> {
  
  pub fn len(&self) -> usize {
    self.len
  }

  pub fn is_null(&self) -> bool {
      if self.len==0 {
        true
      } else {
        false
      }
  }

  pub fn get_mut(&mut self,index:usize) -> Option<&mut T> {
    if index<0 || index>=self.len {
      return None;
    }
    let v=unsafe {
         &mut (*self.ptr.offset(index as isize))
    };
    return Some(v);

  }

  pub fn get(&self,index:usize) -> Option<&T> {
    if index<0 || index>=self.len {
      return None;
    }
    let v=unsafe {
         & (*self.ptr.offset(index as isize))
    };
    return Some(v);

  }

  pub fn swap(&mut self,one:usize,two:usize) {
    if one!=two {
    unsafe {

      let temp_one_pointer=self.ptr.offset(one as isize);
      let temp_two_pointer=self.ptr.offset(two as isize);
      let temp=ptr::read(temp_one_pointer as *mut T);
      ptr::copy_nonoverlapping(temp_two_pointer as *const T,temp_one_pointer as *mut T,1);
      ptr::write(temp_two_pointer as *mut T,temp);

    }
  }
  }

  pub fn capacity(cap:usize) -> Self {
    assert!(mem::size_of::<T>() != 0, "We're not ready to handle ZSTs");

    let ptr= unsafe {
      let layout=Layout::array::<T>(cap).unwrap();
      let ptr = alloc::alloc(layout);
      ptr
    };

    let ptrone=ptr as *mut T;

    
    ManVec {
        ptr:ptrone,
        cap:cap,
        len:0,
        _marker: PhantomData,
    }

  }

  pub fn new() -> Self {

    assert!(mem::size_of::<T>() != 0, "We're not ready to handle ZSTs");

    let ptrone = unsafe {
      //it's a waste!  
      mem::align_of::<T>() as *mut T
    };
    
      ManVec {
          ptr:ptrone,
          cap:0,
          len:0,
          _marker: PhantomData,
      }
  }
  pub fn newinit(t:T,cap:usize) -> Self  {

    assert!(mem::size_of::<T>() != 0, "We're not ready to handle ZSTs");

    let ptr= unsafe {
    let layout=Layout::array::<T>(cap).unwrap();
    let ptr = alloc::alloc(layout);

    let offset=mem::size_of::<T>() as usize;

        let my_data= ptr.offset((0*offset) as isize) as *mut T;
        //  *my_data=t;
        ptr::write(my_data,t);
        ptr

    };

    let ptrone=ptr as *mut T;

    
      ManVec {
          ptr:ptrone,
          cap:cap,
          len:1,
          _marker: PhantomData,
      }
  }


  pub fn push(&mut self,t:T) {
   //   println!("begin to push ---");

      assert!(mem::size_of::<T>() != 0, "We're not ready to handle ZSTs");
   
    if self.len+1>self.cap {
     //   println!("the cap is full");
        if self.cap==0 {
          self.cap=1;
        }
        self.growth();
    }
    let offset=mem::size_of::<T>() as usize;
    unsafe {
    let mut my_data= self.ptr.offset(self.len as isize) as *mut T;
  //  *my_data=t; 此种写法不正确
     ptr::write(my_data,t);
     self.len+=1;
     //println!("successful to push ---");
    }
   
  }

  pub fn printlnall(&self) {
  
    unsafe {
      for i in 0..self.len {
      //  println!("the {} is : {:?}",i,(*(self.ptr.offset(((i)) as isize) as *mut T)));
      }
    }
  }

  fn growth(&mut self) {
     // println!("the enlarge capcity------");
      self.cap=2*self.cap;
      let after_poiner=unsafe {
      let layout=Layout::array::<T>(self.cap).unwrap();
      //alloc::realloc(self.ptr as *mut u8, layout, self.cap) as *mut T;
      //重新分配并移动就数据到新位置
       let ptr = alloc::alloc(layout);
    //  let offset=mem::size_of::<T>() as usize;

      ptr::copy_nonoverlapping(self.ptr, ptr as *mut T, self.len);

      ptr  as *mut T
      };
      self.ptr = after_poiner;
  }


  pub fn pop(&mut self) ->Option<T> {
    if self.len==0 {
        return None;
    }
  let t= unsafe {
    ptr::read((self.ptr.offset((self.len-1) as isize) as *mut T))
  };
  self.len=self.len-1;
  Some(t)
  }

//ManVec other function,for example insert and remove ,insert  need move the other element to the right.this use ptr::copy

  pub fn insert(&mut self, index: usize, elem: T) {
    // Note: `<=` because it's valid to insert after everything
    // which would be equivalent to push.
    assert!(index <= self.len, "index out of bounds");
    if self.cap == self.len { self.growth(); }

    unsafe {
        if index < self.len {
            // ptr::copy(src, dest, len): "copy from source to dest len elems"
            ptr::copy(self.ptr.offset(index as isize),
                      self.ptr.offset(index as isize + 1),
                      self.len - index);
        }
        ptr::write(self.ptr.offset(index as isize), elem);
        self.len += 1;
    }
}

pub fn remove(&mut self, index: usize) -> T {
  // Note: `<` because it's *not* valid to remove after everything
  assert!(index < self.len, "index out of bounds");
  unsafe {
      self.len -= 1;
      let result = ptr::read(self.ptr.offset(index as isize));
      ptr::copy(self.ptr.offset(index as isize + 1),
                self.ptr.offset(index as isize),
                self.len - index);
      result
  }
}

  

}

impl<T> Drop for ManVec<T> {
    fn drop(&mut self) {

   //   let offset=mem::size_of::<T>() as usize;
  //    println!("the size of T is {}",offset);
  //    println!("the align of T {}",std::mem::align_of::<T>());
  if self.cap!=0 {

      unsafe {
       
        for i in 0..self.len {
           // drop(std::ptr::read())
         ptr::drop_in_place(self.ptr.offset(i as isize) as *mut T);
        }
        //系统回收内存

        let layout=Layout::array::<T>(self.cap).unwrap();
   
        alloc::dealloc(self.ptr as *mut u8, layout);
        }
    }

  }
}






impl<T> Index<usize> for ManVec<T> {
  type Output = T;

  fn index(&self, index: usize) -> &Self::Output {
      self.get(index).unwrap()
  }
}

impl<T> IndexMut<usize> for ManVec<T> {
 

  fn index_mut(&mut self, index: usize) -> &mut Self::Output {
      self.get_mut(index).unwrap()
  }
}


// form  ManVec<T>  to slice   解引用来实现
use std::ops::Deref;

impl<T> Deref for ManVec<T> {
    type Target = [T];
    fn deref(&self) -> &[T] {
        unsafe {
            std::slice::from_raw_parts(self.ptr, self.len)
        }
    }
}

use std::ops::DerefMut;

impl<T> DerefMut for ManVec<T> {
    fn deref_mut(&mut self) -> &mut [T] {
        unsafe {
            std::slice::from_raw_parts_mut(self.ptr, self.len)
        }
    }
}


unsafe impl<T:Send+Sync> Send for ManVec<T> {}
//unsafe impl<T:Send+Sync>  Sync for ManVec<T> {}


#[test]
pub fn test_vec_slice() {

  let mut v=ManVec::new();
  v.push(2);
  v.push(3);
  v.push(4);
  let v_slice:&[i32]=&v;
  assert_eq!(&v_slice[0..2],&[2,3]);
  let v_slice:&mut [i32]=&mut v;
  v_slice[1]=4;
  assert_eq!(&v_slice[0..2],&[2,4]);
  v_slice.sort();
  assert_eq!(&v_slice[0..3],&[2,4,4]);
  assert_eq!(v_slice.binary_search(&2),Ok(0));
  
}

#[test]
pub fn test_vec_acrossthread() {

  let mut v=ManVec::new();
  v.push(2);
  v.push(3);
  v.push(4);

 let th=std::thread::spawn(move || {
  v.push(5);
  let mut sum=0;
  for i in 0..4 {
    sum+=v[i];
  }
 assert_eq!(sum,14);
   });

th.join().unwrap();

}




