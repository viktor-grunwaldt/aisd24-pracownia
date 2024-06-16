use std::collections::VecDeque;

use crate::{
    macros::{println, scanf},
    radix,
};


fn binary_search_leftmost(arr: &[(u32, u32)], value: &u32) -> Result<usize, usize> {
    let mut left = 0;
    let mut right = arr.len();
    while left < right {
        let mid = left + (right - left) / 2;
        if arr[mid].1 < *value {
            left = mid + 1;
        } else {
            right = mid;
        }
    }

    if left < arr.len() && arr[left].1 == *value {
        Ok(left)
    } else {
        Err(left)
    }
}
#[inline]
fn is_ancestor(u: usize, v: usize, times: &[Option<(u32, u32)>]) -> bool {
    if let Some((l, r)) = times[u] {
        if let Some((li, ri)) = times[v] {
            return l < li && ri <= r;
        }
    }
    false
}
#[inline]
fn unmark(x: u32) -> usize {
    (x & 0x7fff_ffff) as usize
}
#[inline]
fn mark(x: u32) -> u32 {
    x | (1 << 31)
}

fn iterative_dfs_with_timemarks(arr: Vec<(u32, u32)>) -> Vec<Option<(u32, u32)>> {
    let mut stack: VecDeque<u32> = VecDeque::new();
    stack.push_back(1);
    let mut times: Vec<Option<(u32, u32)>> = vec![None; arr.len() + 2];
    let mut timer = 0;
    // while pop returns value
    while let Some(node) = stack.pop_back() {
        // on exit
        if node > (1 << 31) {
            if let Some(Some(x)) = times.get_mut(unmark(node)) {
                x.1 = timer;
            }
            continue;
        }
        // on entry
        // if child exists
        if let Ok(leftmost_child_index) = binary_search_leftmost(&arr, &node) {
            // only put exit mark if we would be putting node's children also
            stack.push_back(mark(node));
            let mut children_index = leftmost_child_index;
            while let Some((l, r)) = arr.get(children_index) {
                if *r != node {
                    break;
                }
                stack.push_back(*l);
                children_index += 1;
            }
        }
        times[node as usize] = Some((timer, timer));
        timer += 1;
    }
    times
}

fn produce_times(n: usize) -> Vec<Option<(u32, u32)>> {
    // let mut daughters: Vec<(u32, u32)> = lines
    //     .take(n - 1)
    //     .map(|s| s.parse::<u32>().unwrap())
    //     .enumerate()
    //     .map(|(i, e)| ((i + 2) as u32, e))
    //     .collect();
    let mut daughters: Vec<(u32, u32)> =
        (0..n - 1).map(|i| ((i + 2) as u32, scanf!(u32))).collect();
    // daughters.sort_unstable_by_key(|e| e.1);
    radix::sort_by_y_uint(daughters.as_mut_slice());
    iterative_dfs_with_timemarks(daughters)
}

pub fn solve() {
    let read_pair = || scanf!(usize, usize);
    let (n, q) = read_pair();
    let times = produce_times(n);
    let print_result = |p: (usize, usize)| {
        if is_ancestor(p.0, p.1, &times) {
            println!("TAK");
        } else {
            println!("NIE");
        }
    };
    (0..q).map(|_| scanf!(usize, usize)).for_each(print_result);
}
