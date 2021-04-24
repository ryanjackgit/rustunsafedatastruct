

use std::{
    cell::UnsafeCell,
    fmt,
    ops::{Deref, DerefMut},
    sync::atomic::{
        AtomicBool,
        Ordering::*,
    },
};


/*
锁的本质原理：锁就是一块内存空间，值为1,表示已被一个线程拥有，为0表示已被释放，对该值的操作都需要是原子操作,需要计算机硬件系统的支持，两种方式：

1,多核状态下锁总线，就是Lock前缀。锁住总线内存地址后，只有该线程能访问该内存达到原子访问目的。
2,CPU自带的一些原子操作，如Automic类型的CAS操作

锁需要解决几个问题：

   1,争抢到一个内存，如果抢到了就算是得到了锁，可以继续干活；
   2, 如果没抢到，可以选择：
      2.1  继续抢（spin）
      2.2  调用某个系统调用把自己挂起来排队
    3,别的线程释放锁后，会通知排队挂起来的一个或几个线程。醒过来的线程再去重复第一步。

早期的锁，所有这些步骤都是内核态的。但后来大家发现，步骤1用CAS在用户态就可以干了。而多线程大部分的时候抢锁都是没有竞争的，一抢就能抢到。一下子没抢到多抢几次大概率也能抢到了。

那么能不能用一直做spin，永远不做2.2和3呢？答案是不行的。如果遇到了竞争，这也就意味着大量空耗CPU。

因此后来的锁的设计一般都优化成了这样：

1. 在用户态写一段代码来抢锁，典型的实现是用CAS把一个指定的变量从0变成1。如果抢到了就结束了。此时是用户态的。

2. 如果抢不到，就看看是不是锁的持有者就是自己。如果是，也算是抢到了（当然要对变量做特殊的标记）。否则就spin几次重新抢。这也是用户态的。

3. 如果重试了N次，实在抢不到，此时调用futex_wait进入内核态，去把自己挂起+排队，等着被释放锁的线程futex_wake。

所以只有3进入内核态了。考虑到大部分情况都不是竞争很激烈的情况下，3根本就不用做。这样的锁的设计避免了由于系统调用导致的上下文切换，无疑很大的提高了效率。

在Java语言中，Java的synchronized用JVM的monitor实现。而monitor实现内部用到了pthread_mutex和pthread_cond。这俩是pthread标准接口，实现在glibc里。
而这俩的内部实现在Linux上目前都用到了futex。所以整体可以理解为futex帮助Java在Linux上实现了synchronized在内核那部分阻塞的功能；同时用户态的抢锁，重入控制等功能由JVM自己实现,如我们常说的偏向锁，轻量级锁等也是由JVM在用户态实现。两块代码共同提供了完整的synchronized功能。

所以当我们随便写一段synchornized会阻塞的Java代码：

	
public class TestFutex {
     private Integer a = new Integer(1);
 
     synchronized void showA() {
         System.out.println(a);
         try {
             Thread.sleep(3000);
         } catch(InterruptedException e) {
 
         }
     }
     class T extends Thread {
         @Override
         public void run() {
             showA();
         }
     }
 
     public T newThread() {
         return new T();
     }
 
     public static void main(String[] args) {
         TestFutex tf = new TestFutex();
         T t1 = tf.newThread();
         T t2 = tf.newThread();
         t1.start();
         t2.start();
     }
 }

并用strace去查看效果，你就会看到：(特别是Futex系统调用，就是挂起和唤醒线程。类似java中LockSupport.park 线程和 unpark线程)

	
root@ba32a8cedf75:/test# strace -e futex java TestFutex
futex(0x7f8dacc130c8, FUTEX_WAKE_PRIVATE, 2147483647) = 0
futex(0x7f8dad64e9d0, FUTEX_WAIT, 97, NULL1
……

顺便说一句，基于AQS实现的JUC的那些ReentrantLock，Semaphore等内部也是类似的。其LockSupport.park内部用的也是这套东西。



*/

/* in Rust standand lib ,Mutex<T> define :

pub struct Mutex<T: ?Sized> {
    inner: Box<sys::Mutex>,// on linux plaform, pthread_mutex_t 
    poison: poison::Flag,  //中毒标记,  如果线程panic导致锁未释放，其它被该锁阻塞的线程将抛出中毒Error.
    data: UnsafeCell<T>,
}

the same RwLock:

pub struct RwLock<T: ?Sized> {
    inner: Box<sys::RWLock>,
    poison: poison::Flag,
    data: UnsafeCell<T>,
}


*/


//SpinLock example:

#[derive(Debug, Clone, Copy)]
pub struct MutexErr;

pub struct MutexGuard<'a,T> 
where T:'a {
    mutex:&'a Mutex<T>,
}

pub struct Mutex<T> {
    locked: AtomicBool,
    inner:UnsafeCell<T>,
}


impl <T> Mutex<T> {
    pub fn new(t:T) -> Self {
        Mutex {
            locked: AtomicBool::new(false),
            inner:UnsafeCell::new(t),
        }
    }

    pub fn try_lock<'a>(&'a self) -> Result<MutexGuard<'a, T>, MutexErr> {
        if self.locked.swap(true, Acquire) {
            Err(MutexErr)
        } else {
            Ok(MutexGuard {
                mutex: self,
            })
        }
    }

    pub fn lock<'a>(&'a self) -> MutexGuard<'a, T> {
        loop {
            if let Ok(m) = self.try_lock() {
                break m;
            }
        }
    }

}

unsafe impl<T> Send for Mutex<T>
where
    T: Send,
{
}

unsafe impl<T> Sync for Mutex<T>
where
    T: Send,
{
}

impl<T> Drop for Mutex<T> {
    fn drop(&mut self) {
        unsafe {
            self.inner.get().drop_in_place()
        }
    }
}

impl<'a, T> Deref for MutexGuard<'a, T> {
    type Target = T;

    fn deref(&self) -> &T {
        unsafe {
            &*self.mutex.inner.get()
        }
    }
}

impl<'a, T> DerefMut for MutexGuard<'a, T> {
    fn deref_mut(&mut self) -> &mut T {
        unsafe {
            &mut *self.mutex.inner.get()
        }
    }
}

impl<'a, T> fmt::Debug for MutexGuard<'a, T>
where
    T: fmt::Debug,
{
    fn fmt(&self, fmtr: &mut fmt::Formatter) -> fmt::Result {
        write!(fmtr, "{:?}", &**self)
    }
}

impl<'a, T> Drop for MutexGuard<'a, T> {
    fn drop(&mut self) {
        let _prev = self.mutex.locked.swap(false, Release);
        debug_assert!(_prev);
    }
}



