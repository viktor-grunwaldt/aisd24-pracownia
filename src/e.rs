// use crate::{println, scanf};
use std::cmp::{max, min, Ordering};
use std::mem::{replace, swap};

#[derive(Debug, PartialEq, Clone)]
pub struct AvlNode<T: Ord> {
    pub value: T,
    pub left: AvlTree<T>,
    pub right: AvlTree<T>,
    pub height: usize,
}

pub type AvlTree<T> = Option<Box<AvlNode<T>>>;

impl<'a, T: 'a + Ord> AvlNode<T> {
    pub fn left_height(&self) -> usize {
        self.left.as_ref().map_or(0, |left| left.height)
    }

    pub fn right_height(&self) -> usize {
        self.right.as_ref().map_or(0, |right| right.height)
    }

    pub fn update_height(&mut self) {
        self.height = 1 + max(self.left_height(), self.right_height());
    }

    pub fn balance_factor(&self) -> i8 {
        let left_height = self.left_height();
        let right_height = self.right_height();

        if left_height >= right_height {
            (left_height - right_height) as i8
        } else {
            -((right_height - left_height) as i8)
        }
    }

    pub fn rotate_left(&mut self) -> bool {
        if self.right.is_none() {
            return false;
        }

        let right_node = self.right.as_mut().unwrap();
        let right_left_tree = right_node.left.take();
        let right_right_tree = right_node.right.take();

        let mut new_left_tree = replace(&mut self.right, right_right_tree);
        swap(&mut self.value, &mut new_left_tree.as_mut().unwrap().value);
        let left_tree = self.left.take();

        let new_left_node = new_left_tree.as_mut().unwrap();
        new_left_node.right = right_left_tree;
        new_left_node.left = left_tree;
        self.left = new_left_tree;

        if let Some(node) = self.left.as_mut() {
            node.update_height();
        }

        self.update_height();

        true
    }

    pub fn rotate_right(&mut self) -> bool {
        if self.left.is_none() {
            return false;
        }

        let left_node = self.left.as_mut().unwrap();
        let left_right_tree = left_node.right.take();
        let left_left_tree = left_node.left.take();

        let mut new_right_tree = replace(&mut self.left, left_left_tree);
        swap(&mut self.value, &mut new_right_tree.as_mut().unwrap().value);
        let right_tree = self.right.take();

        let new_right_node = new_right_tree.as_mut().unwrap();
        new_right_node.left = left_right_tree;
        new_right_node.right = right_tree;
        self.right = new_right_tree;

        if let Some(node) = self.right.as_mut() {
            node.update_height();
        }

        self.update_height();

        true
    }

    pub fn rebalance(&mut self) -> bool {
        match self.balance_factor() {
            -2 => {
                let right_node = self.right.as_mut().unwrap();

                if right_node.balance_factor() == 1 {
                    right_node.rotate_right();
                }

                self.rotate_left();

                true
            }

            2 => {
                let left_node = self.left.as_mut().unwrap();

                if left_node.balance_factor() == -1 {
                    left_node.rotate_left();
                }

                self.rotate_right();

                true
            }
            _ => false,
        }
    }
}
#[derive(Debug, PartialEq, Clone)]
pub struct AvlTreeSet<T: Ord> {
    root: AvlTree<T>,
}

impl<'a, T: 'a + Ord> Default for AvlTreeSet<T> {
    fn default() -> Self {
        Self { root: None }
    }
}

impl<'a, T: 'a + Ord> AvlTreeSet<T> {
    pub fn new() -> Self {
        Self { root: None }
    }

    pub fn insert(&mut self, value: T) -> bool {
        let mut prev_ptrs = Vec::<*mut AvlNode<T>>::new();
        let mut current_tree = &mut self.root;

        while let Some(current_node) = current_tree {
            prev_ptrs.push(&mut **current_node);

            match current_node.value.cmp(&value) {
                Ordering::Less => current_tree = &mut current_node.right,
                Ordering::Equal => {
                    return false;
                }
                Ordering::Greater => current_tree = &mut current_node.left,
            }
        }

        *current_tree = Some(Box::new(AvlNode {
            value,
            left: None,
            right: None,
            height: 1,
        }));

        for node_ptr in prev_ptrs.into_iter().rev() {
            let node = unsafe { &mut *node_ptr };
            node.update_height();
            node.rebalance();
        }

        true
    }

    pub fn take(&mut self, value: &T) -> Option<T> {
        let mut prev_ptrs = Vec::<*mut AvlNode<T>>::new();
        let mut current_tree = &mut self.root;
        let mut target_value = None;

        while let Some(current_node) = current_tree {
            match current_node.value.cmp(value) {
                Ordering::Less => {
                    prev_ptrs.push(&mut **current_node);
                    current_tree = &mut current_node.right;
                }
                Ordering::Equal => {
                    target_value = Some(&mut **current_node);
                    break;
                }
                Ordering::Greater => {
                    prev_ptrs.push(&mut **current_node);
                    current_tree = &mut current_node.left;
                }
            };
        }

        target_value.as_ref()?;

        let target_node = target_value.unwrap();

        let taken_value = if target_node.left.is_none() || target_node.right.is_none() {
            if let Some(left_node) = target_node.left.take() {
                replace(target_node, *left_node).value
            } else if let Some(right_node) = target_node.right.take() {
                replace(target_node, *right_node).value
            } else if let Some(prev_ptr) = prev_ptrs.pop() {
                let prev_node = unsafe { &mut *prev_ptr };

                let inner_value = if let Some(left_node) = prev_node.left.as_ref() {
                    if left_node.value == target_node.value {
                        prev_node.left.take().unwrap().value
                    } else {
                        prev_node.right.take().unwrap().value
                    }
                } else {
                    prev_node.right.take().unwrap().value
                };

                prev_node.update_height();
                prev_node.rebalance();

                inner_value
            } else {
                self.root.take().unwrap().value
            }
        } else {
            let right_tree = &mut target_node.right;

            if right_tree.as_ref().unwrap().left.is_none() {
                let mut right_node = right_tree.take().unwrap();

                let inner_value = replace(&mut target_node.value, right_node.value);
                let _ = replace(&mut target_node.right, right_node.right.take());

                target_node.update_height();
                target_node.rebalance();

                inner_value
            } else {
                let mut next_tree = right_tree;
                let mut inner_ptrs = Vec::<*mut AvlNode<T>>::new();

                while let Some(next_left_node) = next_tree {
                    if next_left_node.left.is_some() {
                        inner_ptrs.push(&mut **next_left_node);
                    }
                    next_tree = &mut next_left_node.left;
                }

                let parent_left_node = unsafe { &mut *inner_ptrs.pop().unwrap() };
                let mut leftmost_node = parent_left_node.left.take().unwrap();

                let inner_value = replace(&mut target_node.value, leftmost_node.value);
                let _ = replace(&mut parent_left_node.left, leftmost_node.right.take());

                parent_left_node.update_height();
                parent_left_node.rebalance();

                for node_ptr in inner_ptrs.into_iter().rev() {
                    let node = unsafe { &mut *node_ptr };
                    node.update_height();
                    node.rebalance();
                }

                target_node.update_height();
                target_node.rebalance();

                inner_value
            }
        };

        for node_ptr in prev_ptrs.into_iter().rev() {
            let node = unsafe { &mut *node_ptr };
            node.update_height();
            node.rebalance();
        }

        Some(taken_value)
    }

    pub fn remove(&mut self, value: &T) -> bool {
        self.take(value).is_some()
    }

    pub fn lower_bound(&self, value: &T) -> Option<&T> {
        let mut current_tree = &self.root;
        let mut candidate = None;
        while let Some(current_node) = current_tree {
            let v = &current_node.value;
            match v.cmp(value) {
                Ordering::Less => {
                    candidate = Some(candidate.map_or(v, |c| max(v, c)));
                    current_tree = &current_node.right;
                }
                Ordering::Equal => {
                    return Some(&current_node.value);
                }
                Ordering::Greater => {
                    current_tree = &current_node.left;
                }
            };
        }

        candidate
    }
    pub fn upper_bound(&self, value: &T) -> Option<&T> {
        let mut current_tree = &self.root;
        let mut candidate = None;

        while let Some(current_node) = current_tree {
            let v = &current_node.value;
            match v.cmp(value) {
                Ordering::Less => {
                    current_tree = &current_node.right;
                }
                Ordering::Equal => {
                    return Some(&current_node.value);
                }
                Ordering::Greater => {
                    candidate = Some(candidate.map_or(v, |c| min(v, c)));
                    current_tree = &current_node.left;
                }
            };
        }

        candidate
    }
}

fn eval(op: char, val: i64, set: &mut AvlTreeSet<i64>) -> Option<String> {
    match op {
        'I' => {
            set.insert(val);
            None
        }
        'D' => {
            let res = set.remove(&val);
            Some(if res {
                "OK".to_owned()
            } else {
                "BRAK".to_owned()
            })
        }
        'L' => {
            let res = set.lower_bound(&val);
            // dbg!(&res.peek_prev(), &res.peek_next());
            Some(res.map_or("BRAK".to_string(), |p| p.to_string()))
        }
        'U' => {
            let res = set.upper_bound(&val);
            // dbg!(&res.peek_prev(), &res.peek_next());
            Some(res.map_or("BRAK".to_string(), |p| p.to_string()))
        }
        _ => unreachable!("niedozwolony input {:?}", op),
    }
}

fn solve() {
    let mut set = AvlTreeSet::new();
    let n = scanf!(usize);
    scanf!(char);
    for _ in 0..n {
        let (op, val, _newline) = scanf!(char, i64, char);
        let res = eval(op, val, &mut set);
        if let Some(ans) = res {
            println!("{}", ans);
        }
    }
}
