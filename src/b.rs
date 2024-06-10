use crate::macros;

pub const N: usize = 500_000;
// Window Capacity for the rotating array
const WC: usize = 16;

type Pt = (i32, i32);
type Triangle = (Pt, Pt, Pt);

fn dist(slf: &Pt, oth: &Pt) -> f32 {
    let dx = (slf.0 - oth.0) as i64;
    let dy = (slf.1 - oth.1) as i64;
    let sum = dx * dx + dy * dy;
    (sum as f32).sqrt()
}

fn perim(a: &Pt, b: &Pt, c: &Pt) -> f32 {
    dist(a, b) + dist(a, c) + dist(b, c)
}

fn rec(n: usize, p_by_x: &[Pt], p_by_y: &[Pt], min_t: &mut Triangle, min_p: &mut f32) {
    if n < 3 {
        return;
    }
    if n < 7 {
        // using the criterion benchmark, I've found out that <= 6 is faster when using brute force attempt,
        // reducing the recursion depth by 1 or 2
        for i in 0..(n - 2) {
            for j in (i + 1)..(n - 1) {
                let bok = dist(&p_by_x[i], &p_by_x[j]);
                for k in (j + 1)..n {
                    let perim = bok + dist(&p_by_x[i], &p_by_x[k]) + dist(&p_by_x[j], &p_by_x[k]);
                    if perim < *min_p {
                        *min_p = perim;
                        *min_t = (p_by_x[i], p_by_x[j], p_by_x[k]);
                    }
                }
            }
        }
        return;
    }
    let left = n / 2;
    let right = n - left;
    let mid_x = (p_by_x[left].0 + p_by_x[left + 1].0) / 2;
    // we have to allocate new arrays for points sorted by y for the left and right call
    // some nasty <1% error occurs when the arrays don't split nicely in two
    // setting split size on Vec len instead of n / 2 seems to fix the issue
    // could be if points have same coordinate
    let mut left_rec = Vec::with_capacity(left + 1);
    let mut right_rec = Vec::with_capacity(right);
    let mut distributor = true;
    for &e in p_by_y.iter() {
        match e.0.cmp(&mid_x) {
            std::cmp::Ordering::Less => left_rec.push(e),
            std::cmp::Ordering::Equal => {
                if distributor {
                    left_rec.push(e);
                } else {
                    right_rec.push(e);
                }
                distributor = !distributor;
            }
            std::cmp::Ordering::Greater => right_rec.push(e),
        }
    }
    let (pbxl, pbxr) = p_by_x.split_at(left_rec.len());
    rec(left_rec.len(), pbxl, &left_rec, min_t, min_p);
    rec(right_rec.len(), pbxr, &right_rec, min_t, min_p);
    let radius = (*min_p / 2.0).ceil() as i32;
    let candidates = p_by_y.iter().filter(|(x, _)| (x - mid_x).abs() <= radius);
    // there is a proof flying around (don't ask me, I've just ran some tests) that there can not be more than 16 points
    // at once. Thus we can cut down by not allocating the candidates
    // oh and I just assume that the compiler knows that '% 16' is eqv to '& 0xF'
    let mut rotating_window = [(0, 0); WC];
    let mut w_size = 0;
    let mut pos = 0;
    for p in candidates {
        while w_size > 0 && p.1 - rotating_window[pos % WC].1 > radius {
            // pop();
            w_size -= 1;
            pos += 1;
        }

        for i in 0..w_size {
            // I have no idea if the two lines below would be optimized by LLVM, but it's good practice
            let b = rotating_window[(pos + i) % WC];
            let edge = dist(p, &b);
            for j in (i + 1)..w_size {
                let c = rotating_window[(pos + j) % WC];
                let temp_obw = edge + dist(p, &c) + dist(&b, &c);
                if temp_obw < *min_p {
                    *min_p = temp_obw;
                    *min_t = (*p, b, c);
                }
            }
        }
        // push(p);
        rotating_window[(pos + w_size) % WC] = *p;
        w_size += 1;
        // not needed
        // assert!(w_size < WC);
    }
}

pub fn div_and_conq(p: &mut [Pt], n: usize) -> Triangle {
    let mut min_t = (p[0], p[1], p[2]);
    let mut min_p = perim(&p[0], &p[1], &p[2]);
    p[..n].sort_unstable_by_key(|&e| e.1);
    let p_by_y = p.to_vec();
    p[..n].sort_unstable();

    rec(n, &p[..n], &p_by_y[..n], &mut min_t, &mut min_p);
    min_t
}
pub fn solve() {
    let n = macros::scanf!(usize);
    assert!(n <= N, "input has exceeded size specifed in task");

    let mut punkty = [(0, 0); N];
    for e in punkty.iter_mut().take(n) {
        *e = macros::scanf!(i32, i32);
    }
    let (a, b, c) = div_and_conq(&mut punkty, n);
    println!("{} {}", a.0, a.1);
    println!("{} {}", b.0, b.1);
    println!("{} {}", c.0, c.1);
}
