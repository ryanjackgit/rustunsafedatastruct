use std::alloc::{self, dealloc, Layout};
use std::ptr;
use std::mem;
use std::fmt::{self,Debug};
use std::cmp::{Ord,Ordering,Eq,PartialEq};


pub struct RedBlackTree<K: Ord, V> {
    root: Node<K, V>,
    len: usize,
}


impl<K: Ord, V> Drop for RedBlackTree<K, V> {

    fn drop(&mut self) {
        self.clear();
    }
}


impl<K: Ord, V> RedBlackTree<K, V> {
   
    pub fn new() -> Self {
        RedBlackTree {
            root: Node::null(),
            len: 0,
        }
    }

  
    pub fn len(&self) -> usize {
        self.len
    }


    pub fn is_empty(&self) -> bool {
        self.root.is_null()
    }

    /*
     * 对红黑树的节点(x)进行左旋转
     *
     * 左旋示意图(对节点x进行左旋)：
     *      px                              px
     *     /                               /
     *    x                               y
     *   /  \      --(左旋)-->           / \                #
     *  lx   y                          x  ry
     *     /   \                       /  \
     *    ly   ry                     lx  ly
     *
     *
     */

    unsafe fn left_rotate(&mut self, mut node: Node<K, V>) {
        let mut temp = node.right();
        node.set_right(temp.left());

        if !temp.left().is_null() {
            temp.left().set_parent(node.clone());
        }

        temp.set_parent(node.parent());
        if node == self.root {
            self.root = temp.clone();
        } else if node == node.parent().left() {
            node.parent().set_left(temp.clone());
        } else {
            node.parent().set_right(temp.clone());
        }

        temp.set_left(node.clone());
        node.set_parent(temp.clone());
    }

    /*
     * 对红黑树的节点(y)进行右旋转
     *
     * 右旋示意图(对节点y进行左旋)：
     *            py                               py
     *           /                                /
     *          y                                x
     *         /  \      --(右旋)-->            /  \                     #
     *        x   ry                           lx   y
     *       / \                                   / \                   #
     *      lx  rx                                rx  ry
     *
     */

    unsafe fn right_rotate(&mut self, mut node: Node<K, V>) {
        let mut temp = node.left();
        node.set_left(temp.right());

        if !temp.right().is_null() {
            temp.right().set_parent(node.clone());
        }

        temp.set_parent(node.parent());
        if node == self.root {
            self.root = temp.clone();
        } else if node == node.parent().right() {
            node.parent().set_right(temp.clone());
        } else {
            node.parent().set_left(temp.clone());
        }

        temp.set_right(node.clone());
        node.set_parent(temp.clone());
    }

  
    
    pub fn replace_or_insert(&mut self, k: K, mut v: V) -> Option<V> {
        let node = self.find_node(&k);
        if node.is_null() {
            self.insert(k, v);
            return None;
        }

        unsafe {
            mem::swap(&mut v, &mut (*node.0).data.1);
        }

        Some(v)
    }


    unsafe fn insert_fixup(&mut self, mut node: Node<K, V>) {
        let mut parent;
        let mut gparent;

        while node.parent().is_red_color() {
            parent = node.parent();
            gparent = parent.parent();
            //若“父节点”是“祖父节点的左孩子”
            if parent == gparent.left() {
                // Case 1条件：叔叔节点是红色
                let mut uncle = gparent.right();
                if !uncle.is_null() && uncle.is_red_color() {
                    uncle.set_black_color();
                    parent.set_black_color();
                    gparent.set_red_color();
                    node = gparent;
                    continue;
                }

                // Case 2条件：叔叔是黑色，且当前节点是右孩子
                if parent.right() == node {
                    self.left_rotate(parent);
                    let temp = parent;
                    parent = node;
                    node = temp;
                }

                // Case 3条件：叔叔是黑色，且当前节点是左孩子。
                parent.set_black_color();
                gparent.set_red_color();
                self.right_rotate(gparent);
            } else {
                // Case 1条件：叔叔节点是红色
                let mut uncle = gparent.left();
                if !uncle.is_null() && uncle.is_red_color() {
                    uncle.set_black_color();
                    parent.set_black_color();
                    gparent.set_red_color();
                    node = gparent;
                    continue;
                }

                // Case 2条件：叔叔是黑色，且当前节点是右孩子
                if parent.left() == node {
                    self.right_rotate(parent);
                    let temp = parent;
                    parent = node;
                    node = temp;
                }

                // Case 3条件：叔叔是黑色，且当前节点是左孩子。
                parent.set_black_color();
                gparent.set_red_color();
                self.left_rotate(gparent);
            }
        }
        self.root.set_black_color();
    }


    pub fn insert(&mut self, k: K, v: V) {
        self.len += 1;

        let mut node = Node::new(k, v);
        let mut y = Node::null();
        let mut x = self.root;

        while !x.is_null() {
            y = x;
            match node.cmp(&&mut x) {
                Ordering::Less => {
                    x = x.left();
                }
                _ => {
                    x = x.right();
                }
            };
        }

        node.set_parent(y);

        if y.is_null() {
            self.root = node;
        } else {
            match node.cmp(&&mut y) {
                Ordering::Less => {
                    y.set_left(node);
                }
                _ => {
                    y.set_right(node);
                }
            };
        }

        node.set_red_color();

        unsafe {
            self.insert_fixup(node);
        }
    }


    fn find_node(&self, k: &K) -> Node<K, V> {
        if self.root.is_null() {
            return Node::null();
        }
        let mut temp = &self.root;
        unsafe {
            loop {
                let next = match k.cmp(&(*temp.0).data.0) {
                    Ordering::Less => &mut (*temp.0).left,
                    Ordering::Greater => &mut (*temp.0).right,
                    Ordering::Equal => return *temp,
                };
                if next.is_null() {
                    break;
                }
                temp = next;
            }
        }
        Node::null()
    }

  
    fn first_child(&self) -> Node<K, V> {
        if self.root.is_null() {
            Node::null()
        } else {
            let mut temp = self.root;
            while !temp.left().is_null() {
                temp = temp.left();
            }
            return temp;
        }
    }

    fn last_child(&self) -> Node<K, V> {
        if self.root.is_null() {
            Node::null()
        } else {
            let mut temp = self.root;
            while !temp.right().is_null() {
                temp = temp.right();
            }
            return temp;
        }
    }

    
    pub fn get_first(&self) -> Option<(&K, &V)> {
        let first = self.first_child();
        if first.is_null() {
            return None;
        }
        unsafe { Some((&(*first.0).data.0, &(*first.0).data.1)) }
    }

    
    pub fn get_last(&self) -> Option<(&K, &V)> {
        let last = self.last_child();
        if last.is_null() {
            return None;
        }
        unsafe { Some((&(*last.0).data.0, &(*last.0).data.1)) }
    }

    
    pub fn pop_first(&mut self) -> Option<(K, V)> {
        let first = self.first_child();
        if first.is_null() {
            return None;
        }
        unsafe { Some(self.delete(first)) }
    }

    pub fn pop_last(&mut self) -> Option<(K, V)> {
        let last = self.last_child();
        if last.is_null() {
            return None;
        }
        unsafe { Some(self.delete(last)) }
    }

  
    pub fn get_first_mut(&mut self) -> Option<(&K, &mut V)> {
        let first = self.first_child();
        if first.is_null() {
            return None;
        }
        unsafe { Some((&(*first.0).data.0, &mut (*first.0).data.1)) }
    }



    pub fn get_last_mut(&mut self) -> Option<(&K, &mut V)> {
        let last = self.last_child();
        if last.is_null() {
            return None;
        }
        unsafe { Some((&(*last.0).data.0, &mut (*last.0).data.1)) }
    }


    pub fn get(&self, k: &K) -> Option<&V> {
        let node = self.find_node(k);
        if node.is_null() {
            return None;
        }

        unsafe { Some(&(*node.0).data.1) }
    }

    pub fn get_mut(&mut self, k: &K) -> Option<&mut V> {
        let node = self.find_node(k);
        if node.is_null() {
            return None;
        }

        unsafe { Some(&mut (*node.0).data.1) }
    }


    pub fn contains_key(&self, k: &K) -> bool {
        let node = self.find_node(k);
        if node.is_null() {
            return false;
        }
        true
    }


    fn clear_recurse(&mut self, current: Node<K, V>) {
        if !current.is_null() {
            unsafe {
                self.clear_recurse(current.left());
                self.clear_recurse(current.right());
      
               let data=ptr::read(current.0 as *const RedBlackTreeNode<K,V>);
               let layout=Layout::new::<RedBlackTreeNode<K,V>>();
               alloc::dealloc(current.0 as *mut u8,layout);
            }
        }
    }

 
    pub fn clear(&mut self) {
        let root = self.root;
        self.root = Node::null();
        self.clear_recurse(root);
    }

    /// Empties the `RBTree` without freeing objects in it.

    fn fast_clear(&mut self) {
        self.root = Node::null();
    }

    pub fn remove(&mut self, k: &K) -> Option<V> {
        let node = self.find_node(k);
        if node.is_null() {
            return None;
        }
        unsafe { Some(self.delete(node).1) }
    }

    
    unsafe fn delete_fixup(&mut self, mut node: Node<K, V>, mut parent: Node<K, V>) {
        let mut other;
        while node != self.root && node.is_black_color() {
            if parent.left() == node {
                other = parent.right();
                //x的兄弟w是红色的
                if other.is_red_color() {
                    other.set_black_color();
                    parent.set_red_color();
                    self.left_rotate(parent);
                    other = parent.right();
                }

                //x的兄弟w是黑色，且w的俩个孩子也都是黑色的
                if other.left().is_black_color() && other.right().is_black_color() {
                    other.set_red_color();
                    node = parent;
                    parent = node.parent();
                } else {
                    //x的兄弟w是黑色的，并且w的左孩子是红色，右孩子为黑色。
                    if other.right().is_black_color() {
                        other.left().set_black_color();
                        other.set_red_color();
                        self.right_rotate(other);
                        other = parent.right();
                    }
                    //x的兄弟w是黑色的；并且w的右孩子是红色的，左孩子任意颜色。
                    other.set_color(parent.get_color());
                    parent.set_black_color();
                    other.right().set_black_color();
                    self.left_rotate(parent);
                    node = self.root;
                    break;
                }
            } else {
                other = parent.left();
                //x的兄弟w是红色的
                if other.is_red_color() {
                    other.set_black_color();
                    parent.set_red_color();
                    self.right_rotate(parent);
                    other = parent.left();
                }

                //x的兄弟w是黑色，且w的俩个孩子也都是黑色的
                if other.left().is_black_color() && other.right().is_black_color() {
                    other.set_red_color();
                    node = parent;
                    parent = node.parent();
                } else {
                    //x的兄弟w是黑色的，并且w的左孩子是红色，右孩子为黑色。
                    if other.left().is_black_color() {
                        other.right().set_black_color();
                        other.set_red_color();
                        self.left_rotate(other);
                        other = parent.left();
                    }
                    //x的兄弟w是黑色的；并且w的右孩子是红色的，左孩子任意颜色。
                    other.set_color(parent.get_color());
                    parent.set_black_color();
                    other.left().set_black_color();
                    self.right_rotate(parent);
                    node = self.root;
                    break;
                }
            }
        }

        node.set_black_color();
    }

  
    unsafe fn delete(&mut self, node: Node<K, V>) -> (K, V) {
        let mut child;
        let mut parent;
        let color;

        self.len -= 1;
        // 被删除节点的"左右孩子都不为空"的情况。
        if !node.left().is_null() && !node.right().is_null() {
            // 被删节点的后继节点。(称为"取代节点")
            // 用它来取代"被删节点"的位置，然后再将"被删节点"去掉。
            let mut replace = node.right().min_node();
            if node == self.root {
                self.root = replace;
            } else {
                if node.parent().left() == node {
                    node.parent().set_left(replace);
                } else {
                    node.parent().set_right(replace);
                }
            }

            // child是"取代节点"的右孩子，也是需要"调整的节点"。
            // "取代节点"肯定不存在左孩子！因为它是一个后继节点。
            child = replace.right();
            parent = replace.parent();
            color = replace.get_color();
            if parent == node {
                parent = replace;
            } else {
                if !child.is_null() {
                    child.set_parent(parent);
                }
                parent.set_left(child);
                replace.set_right(node.right());
                node.right().set_parent(replace);
            }

            replace.set_parent(node.parent());
            replace.set_color(node.get_color());
            replace.set_left(node.left());
            node.left().set_parent(replace);

            if color == Color::Black {
                self.delete_fixup(child, parent);
            }

  
            let data=ptr::read(node.0 as *const RedBlackTreeNode<K,V>);
            let layout=Layout::new::<RedBlackTreeNode<K,V>>();
            alloc::dealloc(node.0 as *mut u8,layout);
            return data.data;
        }

        if !node.left().is_null() {
            child = node.left();
        } else {
            child = node.right();
        }

        parent = node.parent();
        color = node.get_color();
        if !child.is_null() {
            child.set_parent(parent);
        }

        if self.root == node {
            self.root = child
        } else {
            if parent.left() == node {
                parent.set_left(child);
            } else {
                parent.set_right(child);
            }
        }

        if color == Color::Black {
            self.delete_fixup(child, parent);
        }

  
        let data=ptr::read(node.0 as *const RedBlackTreeNode<K,V>);
        let layout=Layout::new::<RedBlackTreeNode<K,V>>();
        alloc::dealloc(node.0 as *mut u8,layout);
        return data.data;
    }
}

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
enum Color {
    Red,
    Black,
}


pub struct RedBlackTreeNode<K:Ord,V> {
    data:(K,V),
    color:Color,
    left:Node<K,V>,
    right:Node<K,V>,
    parent:Node<K,V>,
 }
 

 impl<K, V> Debug for RedBlackTreeNode<K, V>
     where
         K: Ord + Debug,
         V: Debug,
 {
     fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
         write!(f, "k:{:?} v:{:?} c:{:?}", self.data.0, self.data.1, self.color)
     }
 }
 

//节点指针封装，便于以后计算

#[derive(Debug)]
struct Node<K:Ord,V>(*mut RedBlackTreeNode<K,V>);

impl<K: Ord, V> Clone for Node<K, V> {
    fn clone(&self) -> Node<K, V> {
        Node(self.0)
    }
}

impl<K: Ord, V> Copy for Node<K, V> {}

impl<K: Ord, V> Ord for Node<K, V> {
    fn cmp(&self, other: &Node<K, V>) -> Ordering {
        unsafe { (*self.0).data.0.cmp(&(*other.0).data.0) }
    }
}

impl<K: Ord, V> PartialOrd for Node<K, V> {
    fn partial_cmp(&self, other: &Node<K, V>) -> Option<Ordering> {
        unsafe { Some((*self.0).data.0.cmp(&(*other.0).data.0)) }
    }
}

impl<K: Ord, V> PartialEq for Node<K, V> {

    fn eq(&self, other: &Node<K, V>) -> bool {
        self.0 == other.0
    }
}


impl<K: Ord, V> Eq for Node<K, V> {}


impl<K: Ord, V> Node<K, V> {

    fn new(k: K, v: V) -> Self {

        let layout=Layout::new::<RedBlackTreeNode<K,V>>();


        let node= unsafe {

            let c= alloc::alloc(layout) as *mut RedBlackTreeNode<K,V>;
          //  (*c).data=t;
          let mut data_pointer=&mut ((*c).data) as *mut (K,V);
          ptr::write(data_pointer,(k,v));
      
        
            (*c).left=Node::null();
            (*c).right=Node::null();
            (*c).parent=Node::null();
            (*c).color=Color::Black;
     
            c
         };
   


        Node(node)
    }


    fn set_color(&mut self, color: Color) {
        if self.is_null() {
            return;
        }
        unsafe {
            // Color is Copy,
            (*self.0).color = color;
        }
    }

    
    fn set_red_color(&mut self) {
        self.set_color(Color::Red);
    }

    fn set_black_color(&mut self) {
        self.set_color(Color::Black);
    }


    fn get_color(&self) -> Color {
        if self.is_null() {
            return Color::Black;
        }
        unsafe { (*self.0).color }
    }

 
    fn is_red_color(&self) -> bool {
        if self.is_null() {
            return false;
        }
        unsafe { (*self.0).color == Color::Red }
    }

    
    fn is_black_color(&self) -> bool {
        if self.is_null() {
            return true;
        }
        unsafe { (*self.0).color == Color::Black }
    }

    
    fn is_left_child(&self) -> bool {
        self.parent().left() == *self
    }


    fn is_right_child(&self) -> bool {
        self.parent().right() == *self
    }


    fn min_node(self) -> Node<K, V> {
        let mut temp = self.clone();
        while !temp.left().is_null() {
            temp = temp.left();
        }
        return temp;
    }


    fn max_node(self) -> Node<K, V> {
        let mut temp = self.clone();
        while !temp.right().is_null() {
            temp = temp.right();
        }
        return temp;
    }

    
    fn next(self) -> Node<K, V> {
        if !self.right().is_null() {
            self.right().min_node()
        } else {
            let mut temp = self;
            loop {
                if temp.parent().is_null() {
                    return Node::null();
                }
                if temp.is_left_child() {
                    return temp.parent();
                }
                temp = temp.parent();
            }
        }
    }

    
    fn prev(self) -> Node<K, V> {
        if !self.left().is_null() {
            self.left().max_node()
        } else {
            let mut temp = self;
            loop {
                if temp.parent().is_null() {
                    return Node::null();
                }
                if temp.is_right_child() {
                    return temp.parent();
                }
                temp = temp.parent();
            }
        }
    }


    fn set_parent(&mut self, parent: Node<K, V>) {
        if self.is_null() {
            return;
        }
        unsafe { (*self.0).parent = parent }
    }

    
    fn set_left(&mut self, left: Node<K, V>) {
        if self.is_null() {
            return;
        }
        unsafe { (*self.0).left = left }
    }

    
    fn set_right(&mut self, right: Node<K, V>) {
        if self.is_null() {
            return;
        }
        unsafe { (*self.0).right = right }
    }


    
    fn parent(&self) -> Node<K, V> {
        if self.is_null() {
            return Node::null();
        }
        unsafe { (*self.0).parent.clone() }
    }


    fn left(&self) -> Node<K, V> {
        if self.is_null() {
            return Node::null();
        }
        unsafe { (*self.0).left.clone() }
    }

    
    fn right(&self) -> Node<K, V> {
        if self.is_null() {
            return Node::null();
        }
        unsafe { (*self.0).right.clone() }
    }

    fn null() -> Node<K, V> {
        Node(ptr::null_mut())
    }

    
    fn is_null(&self) -> bool {
        self.0.is_null()
    }
}


impl<K: Ord + Clone, V: Clone> Node<K, V> {
    unsafe fn deep_clone(&self) -> Self {
        let mut node = Node::new((*self.0).data.0.clone(),(*self.0).data.1.clone());
        if !self.left().is_null() {
            node.set_left(self.left().deep_clone());
            node.left().set_parent(node);
        }
        if !self.right().is_null() {
            node.set_right(self.right().deep_clone());
            node.right().set_parent(node);
        }
        node
    }
}



#[test]
fn test_insert() {
    let mut m = RedBlackTree::new();
    assert_eq!(m.len(), 0);
    m.insert(1, 2);
    assert_eq!(m.len(), 1);
    m.insert(2, 3);
    assert_eq!(m.len(), 2);
    m.insert(2, 8);
    assert_eq!(m.len(), 3);
    assert_eq!(*m.get(&1).unwrap(), 2);
    assert_eq!(*m.get(&2).unwrap(), 3);
    assert_eq!(*m.get(&2).unwrap(), 3);
}


#[test]
fn test_replace() {
    let mut m = RedBlackTree::new();
    assert_eq!(m.len(), 0);
    m.insert(2, 4);
    assert_eq!(m.len(), 1);
    assert_eq!(m.replace_or_insert(2, 6).unwrap(), 4);
    assert_eq!(m.len(), 1);
    assert_eq!(*m.get(&2).unwrap(), 6);
}



#[test]
fn test_empty_remove() {
    let mut m: RedBlackTree<isize, bool> = RedBlackTree::new();
    assert_eq!(m.remove(&0), None);
}



#[test]
fn test_find_mut() {
    let mut m = RedBlackTree::new();
    m.insert(1, 14);
    m.insert(3, 88);
    m.insert(5, 14);
    let new = 100;
    match m.get_mut(&5) {
        None => panic!(),
        Some(x) => *x = new,
    }
    assert_eq!(m.get(&5), Some(&new));
}


#[test]
fn test_remove() {
    let mut m = RedBlackTree::new();
    m.insert(1, 2);
    assert_eq!(*m.get(&1).unwrap(), 2);

    m.insert(5, 3);
    m.insert(10,15);
    assert_eq!(*m.get(&1).unwrap(), 2);
    assert_eq!(*m.get(&5).unwrap(), 3);
    assert_eq!(*m.get(&10).unwrap(), 15);
    assert_eq!(m.remove(&1).unwrap(), 2);
    assert_eq!(m.remove(&5).unwrap(), 3);
    assert_eq!(m.remove(&10).unwrap(), 15);
    assert_eq!(m.len(), 0);
}


#[test]
fn test_is_empty() {
    let mut m = RedBlackTree::new();
    m.insert(1, 2);
    assert!(!m.is_empty());
    assert!(m.remove(&1).is_some());
    assert!(m.is_empty());
}

#[test]
fn test_pop() {
    let mut m = RedBlackTree::new();
    m.insert(3, 14);
    m.insert(1, 2);
    m.insert(4, 5);
    m.insert(2,6);
    assert_eq!(m.len(), 4);
    assert_eq!(m.pop_first(), Some((1, 2)));
    assert_eq!(m.len(), 3);
    assert_eq!(m.pop_last(), Some((4, 5)));
    assert_eq!(m.len(), 2);
    assert_eq!(m.get_first(), Some((&2, &6)));
    assert_eq!(m.get_last(), Some((&3, &14)));
}


#[test]
fn test_find() {
    let mut m = RedBlackTree::new();
    assert!(m.get(&1).is_none());
    m.insert(1, 2);
    match m.get(&1) {
        None => panic!(),
        Some(v) => assert_eq!(*v, 2),
    }
}







