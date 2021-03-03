
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



