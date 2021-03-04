
use crate::vec::ManVec;


pub fn binary_search<T>(key: &T, m: &[T]) -> Option<usize>
    where T:Ord {

    let mut low = 0;
    let mut hi = m.len() - 0;
    while low <= hi {
        let mid = low + (hi - low) / 2;
        let mid_value=&m[mid];
        if key < mid_value {
            hi = mid - 1;
        } else if key > mid_value {
            low = mid + 1;
        } else {
            return Some(mid);
        }
    }
    None
}

pub fn quick_sort<T>(a: &mut [T], lo: isize, hi: isize)
    where T:Ord  {

    if hi <= lo {
        return;
    }

    let lt = partition(a, lo, hi);

    quick_sort(a, lo, lt - 1 as isize);

    quick_sort(a, lt + 1, hi);
}

fn partition<T>(a: &mut [T], lo: isize, hi: isize) -> isize
    where T:Ord  {
    let mut i=lo;
    let mut j=hi;

    loop {

        while i<=hi && a[i as usize]<=a[lo as usize]  {
            i=i+1;
        }

        while j>lo && a[j as usize]>=a[lo as usize]  {
           j=j-1;
        }

        if i<j {
            //println!("exchange {} :{}",i,j);
            a.swap(i as usize,j as usize);
        }

        if i>=j {
            break;
        }


    }

    a.swap(lo as usize,j as usize);


     return j;
}


use rand::{thread_rng, Rng};
use rand::seq::SliceRandom;

pub fn shuffle<T>(a:&mut [T])
    where T:Ord {

    let mut rng = thread_rng();

    a.shuffle(&mut rng);

}


#[test]
pub fn test_sort() {
    let mut v=ManVec::new();
    v.push(1);
    v.push(5);
    v.push(3);
    v.push(2);
    v.push(0);
    shuffle(&mut v);
    let len=v.len()-1;
    quick_sort(&mut v,0,len as isize);

    let vv:&[i32]=&v;

    assert_eq!(&vv[..],&[0,1,2,3,5]);
}


#[test]
pub fn test_binary_search() {
    let mut v=ManVec::new();
    v.push(1);
    v.push(5);
    v.push(3);
    v.push(2);
    v.push(0);
    let len=v.len()-1;
    quick_sort(&mut v,0,len as isize);

    let vv:&[i32]=&v;

    assert_eq!(&vv[..],&[0,1,2,3,5]);

    assert_eq!(Some(4),binary_search(&5,&vv));
}


