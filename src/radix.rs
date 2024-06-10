#![allow(clippy::needless_range_loop)]

use std::mem::swap;

const BUCKETS: usize = 256;

fn radix_sort_sign(a: &mut [i32]) {
    let mut histogram = [[0; BUCKETS]; 4];
    for e in &*a {
        let mut num = (*e ^ (-0x8000_0000)) as usize;

        for i in 0..4 {
            let byte = num & 0xFF;
            histogram[i][byte] += 1;
            num >>= 8;
        }
    }

    for i in 0..4 {
        let mut temp = histogram[i][0];
        histogram[i][0] = 0;
        for j in 1..BUCKETS {
            swap(&mut histogram[i][j], &mut temp);
            histogram[i][j] += histogram[i][j - 1];
        }
    }
    let mut sort_from = a.to_vec();
    let mut sort_to: Vec<i32> = vec![0; a.len()];
    for i in 0..3 {
        let mut offsets = histogram[i];
        for e in sort_from.iter() {
            let byte = ((e >> (8 * i)) & 0xFF) as usize;
            let pos = offsets[byte];
            offsets[byte] += 1;
            sort_to[pos] = *e;
        }
        swap(&mut sort_to, &mut sort_from);
    }
    for e in sort_from.into_iter() {
        // casting i32 to u32 so that we do a logical bitshift instead of an arithmetic one
        let byte = ((e as u32 ^ 0x8000_0000) >> 24) as usize;
        let pos = histogram[3][byte];
        histogram[3][byte] += 1;
        a[pos] = e;
    }
}
fn radix_sort(a: &mut [u32]) {
    let mut histogram = [[0; BUCKETS]; 4];
    for e in &*a {
        let mut num = *e as usize;

        for i in 0..4 {
            let byte = num & 0xFF;
            histogram[i][byte] += 1;
            num >>= 8;
        }
    }

    for i in 0..4 {
        let mut temp = histogram[i][0];
        histogram[i][0] = 0;
        for j in 1..BUCKETS {
            swap(&mut histogram[i][j], &mut temp);
            histogram[i][j] += histogram[i][j - 1];
        }
    }
    let mut sort_from = a.to_vec();
    let mut sort_to: Vec<u32> = vec![0; a.len()];
    for i in 0..4 {
        let mut offsets = histogram[i];
        for e in sort_from.iter() {
            let byte = ((e >> (8 * i)) & 0xFF) as usize;
            let pos = offsets[byte];
            offsets[byte] += 1;
            sort_to[pos] = *e;
        }
        swap(&mut sort_to, &mut sort_from);
    }
    for (val, target) in sort_from.into_iter().zip(a) {
        *target = val;
    }
}

type Pt = (u32, u32);
fn radix_sort_by_fst(a: &mut [Pt]) {
    let mut histogram = [[0; BUCKETS]; 4];
    for e in &*a {
        let mut num = e.0 as usize;

        for i in 0..4 {
            let byte = num & 0xFF;
            histogram[i][byte] += 1;
            num >>= 8;
        }
    }

    for i in 0..4 {
        let mut temp = histogram[i][0];
        histogram[i][0] = 0;
        for j in 1..BUCKETS {
            swap(&mut histogram[i][j], &mut temp);
            histogram[i][j] += histogram[i][j - 1];
        }
    }
    let mut sort_from = a.to_vec();
    let mut sort_to: Vec<Pt> = vec![(0, 0); a.len()];
    for i in 0..4 {
        let mut offsets = histogram[i];
        for e in sort_from.iter() {
            let byte = ((e.0 >> (8 * i)) & 0xFF) as usize;
            let pos = offsets[byte];
            offsets[byte] += 1;
            sort_to[pos] = *e;
        }
        swap(&mut sort_to, &mut sort_from);
    }
    for (val, target) in sort_from.into_iter().zip(a) {
        *target = val;
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    fn is_sorted<T: PartialOrd>(a: &[T]) -> bool {
        a.windows(2).all(|w| w[0] <= w[1])
    }
    #[test]
    fn sort_4_values() {
        let mut a = [0xcccccccc, 0x88888888, 0x44444444, 0x00000000];
        radix_sort(&mut a);
        assert!(is_sorted(&a));
    }
    #[test]
    fn sort_16_values() {
        let mut a = [0; 16];
        for i in 0..16 {
            a[15 - i] = (i as u32) * 0x1111_1111;
        }
        radix_sort(&mut a);
        assert!(is_sorted(&a));
    }

    #[test]
    fn sort_same_values() {
        let mut a = [0xdeadbeef; 1 << 15];
        radix_sort(&mut a);
        assert!(is_sorted(&a));
    }
    #[test]
    fn sort_different_values() {
        let mut a = [0xdeadbeefu32; 1 << 15];
        a.iter_mut().enumerate().for_each(|(i, e)| {
            *e = 32768 - (i as u32);
        });
        println!("{:?}", &a);
        radix_sort(&mut a);
        println!("{:?}", &a);
        assert!(is_sorted(&a));
    }
    #[test]
    fn sort_pairs() {
        let mut a = [0xffff; 1 << 15];
        a.iter_mut().enumerate().for_each(|(i, e)| {
            *e -= (i / 2) as u32;
        });
        radix_sort(&mut a);
        assert!(is_sorted(&a));
    }

    #[test]
    fn sort_same_neg() {
        let mut a = [-1; 1 << 10];
        radix_sort_sign(&mut a);
        assert!(is_sorted(&a));
    }
    #[test]
    fn sort_dif_neg() {
        let mut a = [-1; 1 << 15];
        for (i, e) in a.iter_mut().enumerate() {
            *e = (i as i32) - 16000;
        }
        radix_sort_sign(&mut a);
        assert!(is_sorted(&a));
    }
}
