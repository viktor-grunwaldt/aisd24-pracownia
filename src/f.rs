// use crate::{println, scanf};

type Str2d = Vec<Vec<u8>>;

fn solve() {
    let (a, b, c, d) = scanf!(usize, usize, usize, usize);
    // skip newline
    scanf!(char);

    let mut needle = vec![vec![0; b]; a];
    let mut haystack = vec![vec![0; d]; c];
    for row in needle.iter_mut() {
        for e in row.iter_mut() {
            *e = scanf!(char) as u8;
        }
        scanf!(char);
    }
    for row in haystack.iter_mut() {
        for e in row.iter_mut() {
            *e = scanf!(char) as u8;
        }
        // newline
        scanf!(char);
    }
    let res = rk2dv2(needle, haystack);
    println!("{}", res);
}
#[allow(dead_code)]
fn brute_force(needle: Str2d, haystack: Str2d) -> usize {
    let needle_width = needle[0].len();
    let mut acc = 0;
    for (i, row) in haystack.iter().enumerate() {
        for (j, _e) in row.iter().enumerate() {
            if j + needle_width > row.len() || i + needle.len() > haystack.len() {
                break;
            }

            if needle
                .iter()
                .zip(haystack[i..].iter())
                .all(|(nr, hr)| nr.eq(&hr[j..(j + needle_width)]))
            {
                acc += 1;
            }
        }
    }
    acc
}

/// 1st version - Rabin-Karp for 2d patterns
/// https://www.researchgate.net/publication/322765507_Efficient_Algorithm_for_Two_Dimensional_Pattern_Matching_Problem_Square_Pattern
/// those guys had the same idea as I did:
/// Make hash out of needle and move it around the haystack in one direction at a time. We're going
/// from the left, to the right, then one down and then back to left. With this approach, the number of
/// comparisons will be equal to the size of our haystack (plus number of times the hashes are equal)
/// Picture:
/// 1.
/// [ [] ->    ]
/// [          ]
/// 2.
/// [        []]
/// [        vv]
/// 1.
/// [          ]
/// [      <-[]]
/// problem: how the f**k do you implement the rolling hash function?
/// one way:
/// needle:
///  4 1 3 2 \n 2 2 4 4
///x 7 6 5 4    3 2 1 0 (powers of 31)
///= hash
/// RIGHT
/// 7 and 3 fall out of scope:
/// hash -= p[7]*haystack[0][0] + p[3]*haystack[1][0] + ...
///  0 1 3 2 \n 0 2 4 4
///x 7 6 5 4    3 2 1 0 (powers of 31)
/// now, shift all elements left
/// hash *= p[1]
///
///   1 3 2 0 \n 2 4 4 0
/// x 7 6 5 4    3 2 1 0 (powers of 31)
/// add values from the new column
/// hash += p[7]*haystack[0][4] + p[3]*haystack[1][4] + ...
/// 1 3 2 1 \n 2 4 4 4
/// LEFT
/// easy part: we switch the side of the columns we add and subtract
/// hard part: to shift all elements to the right, we have to divide?
/// division in modular arithmetic is not allowed :<
/// idea: calculate the inverse of 31 in mod m, then multiply by it
/// DOWN
/// remove highest row,
/// multiply by p[needle_width]
/// add new row
/// probably easiest to execute
/// rolling hash done~
///
/// how to move with rolling hash:
/// dist = haystack_width - needle_width
/// (RIGHT*dist, DOWN, LEFT*dist) * height_diff/2
///
/// version 2 additional memory approach
/// hashes = [hash.*LEFT*i for i in 0..dist]
/// hashes.*DOWN*j for j in 0..(height_diff)
/// [[] [] [] []]
/// [vv vv vv vv]
///
/// knobs to tweak when optimizing:
/// 1. according to cp-algo p = 31 is solid, but we could play with it
/// 2. store powers of p mod m (upto nw*nh+1)
/// 3. having m = u64::MAX is nice  for modulo arithmetic, but then you can construct
///    a nasty input with many hash collisions. Two solutions:
///        1. have two hashes with different p values
///        2. use a prime m (ex. 10e9 + 9)
/// 4. consider rotating needle and haystack (for example if h>w)
/// version2
/// 1. think about going each row by row, col by col or doing segments (testing)
/// 2. if going row by row, we can 1d hash roll the removed and added rows
/// 3. if going col by col, those values can also be stored
/// another algo?
/// the original RK paper gives also an algorithm, however it was explained in a high level math
/// language making implementing it difficult [Karp, R., Rabin, M. (1987). Efficient randomized pattern-matching algorithms. IBM, 31 (2) 249-260.]
///
/// 1110 Communications of the ACM, September 1989, Volume 32, Number 9
/// Zhu-Takaoka propose two algorithms which use RK for columns and KMP for rows
type Hash = u64;
const P: Hash = 31;
// for now using m = u64::MAX i.e. a+b = a.wrapping_add(b)

/// hashes the square 0x0 -> h x w
#[inline]
fn init_hash(data: &Str2d, w: usize, h: usize, ps: &[Hash]) -> Hash {
    let len = w * h;
    data[..h]
        .iter()
        .flat_map(|row| row[..w].iter())
        .enumerate()
        .fold(Hash::default(), |acc, (i, k)| {
            (*k as Hash).wrapping_mul(ps[len - 1 - i]).wrapping_add(acc)
        })
}

#[inline]
fn roll_hash_right(
    hash: Hash,
    ps: &[Hash],
    haystack: &Str2d,
    x: usize,
    y: usize,
    nw: usize,
    nh: usize,
) -> Hash {
    let mut hash = hash;
    let len = nw * nh;
    let mut subtrahend = Hash::default();
    let mut addend = Hash::default();
    for j in y..y + nh {
        let pow = len - 1 - nw * (j - y);
        let temp = ps[pow].wrapping_mul(haystack[j][x] as Hash);
        subtrahend = subtrahend.wrapping_add(temp);
        let temp = ps[pow + 1 - nw].wrapping_mul(haystack[j][x + nw] as Hash);
        addend = addend.wrapping_add(temp);
    }
    roll_hash(&mut hash, subtrahend, addend, P);
    hash
}
#[inline]
fn roll_hash(h: &mut Hash, l: Hash, r: Hash, shift: Hash) {
    let mut hash = *h;
    hash = hash.wrapping_sub(l);
    hash = hash.wrapping_mul(shift);
    hash = hash.wrapping_add(r);
    *h = hash;
}
#[inline]
fn roll_hash_down(
    hash: Hash,
    ps: &[Hash],
    haystack: &Str2d,
    x: usize,
    y: usize,
    nw: usize,
    nh: usize,
) -> Hash {
    let len = nh * nw;
    let mut hash = hash;
    let mut subtrahend = Hash::default();
    let mut addend = Hash::default();
    for j in 0..nw {
        let pow = len - 1 - j;
        let temp = ps[pow].wrapping_mul(haystack[y][j + x] as Hash);
        subtrahend = subtrahend.wrapping_add(temp);
        let temp = ps[nw - 1 - j].wrapping_mul(haystack[y + nh][j + x] as Hash);
        addend = addend.wrapping_add(temp);
    }
    roll_hash(&mut hash, subtrahend, addend, ps[nw]);
    hash
}
fn precompute_hashes(needle: &Str2d, haystack: &Str2d, ps: &[Hash]) -> Vec<Hash> {
    let (_hh, hw) = (haystack.len(), haystack[0].len());
    let (nh, nw) = (needle.len(), needle[0].len());
    // calc first hash
    let mut hash = init_hash(haystack, nw, nh, ps);

    // now time for the hash to roll right:
    let mut hashes = vec![hash];
    for i in 0..hw - nw {
        hash = roll_hash_right(hash, ps, haystack, i, 0, nw, nh);
        hashes.push(hash);
    }
    hashes
}
/// precomputes powers of p, up to len inclusive
#[inline]
fn init_ps(len: usize) -> Vec<Hash> {
    (0..=len)
        .scan(1, |acc, _| {
            let old = *acc;
            *acc = P.wrapping_mul(old);
            Some(old)
        })
        .collect::<Vec<_>>()
}
// find uses Two-way string-matching algorithm
fn str_count(needle: String, haystack: String) -> usize {
    let mut ans = 0;
    let mut pos = 0;
    while let Some(npos) = haystack[pos..].find(&needle) {
        ans += 1;
        pos += npos + 1;
    }
    ans
}

#[allow(dead_code)]
fn rk2d(needle: Str2d, haystack: Str2d) -> usize {
    // ugh, edge case
    let (hh, _hw) = (haystack.len(), haystack[0].len());
    let (nh, nw) = (needle.len(), needle[0].len());
    let len = nh * nw;
    if nw.min(nh) == 1 {
        let needle: String = needle.into_iter().flatten().map(|c| c as char).collect();
        let haystack: String = haystack.into_iter().flatten().map(|c| c as char).collect();
        return str_count(needle, haystack);
    }
    // phase 1 precompute hashes
    let ps = init_ps(len);
    let mut hashes = precompute_hashes(&needle, &haystack, &ps);
    let hashed_needle: Hash = init_hash(&needle, nw, nh, &ps);
    // check and down
    let mut acc = 0;
    for i in 0..=hh - nh {
        for (j, h) in hashes.iter_mut().enumerate() {
            // verify
            if *h == hashed_needle
                && needle
                    .iter()
                    .zip(haystack[i..].iter())
                    .all(|(nr, hr)| *nr == hr[j..(j + nw)])
            {
                acc += 1;
            }
            // rollover if not last row
            if i < hh - nh {
                *h = roll_hash_down(*h, &ps, &haystack, j, i, nw, nh);
            }
        }
    }

    acc
}
fn rk2dv2(needle: Str2d, haystack: Str2d) -> usize {
    // ugh, edge case
    let (hh, _hw) = (haystack.len(), haystack[0].len());
    let (nh, nw) = (needle.len(), needle[0].len());
    let len = nh * nw;
    if nw.min(nh) == 1 {
        let needle: String = needle.into_iter().flatten().map(|c| c as char).collect();
        let haystack: String = haystack.into_iter().flatten().map(|c| c as char).collect();
        return str_count(needle, haystack);
    }
    if 4 * nh + nw <= 20 {
        return brute_force(needle, haystack);
    }
    // phase 1 precompute hashes
    let ps = init_ps(len);
    let mut hashes = precompute_hashes(&needle, &haystack, &ps);
    let hashed_needle: Hash = init_hash(&needle, nw, nh, &ps);
    // check and down
    let mut acc = 0;
    for i in 0..=hh - nh {
        let mut cached_roll: Option<(Hash, Hash)> = None;
        for (j, h) in hashes.iter_mut().enumerate() {
            // verify
            if *h == hashed_needle
                && needle
                    .iter()
                    .zip(haystack[i..].iter())
                    .all(|(nr, hr)| *nr == hr[j..(j + nw)])
            {
                acc += 1;
            }
            // rollover if not last row
            if i < hh - nh {
                match cached_roll {
                    Some((mut sub, mut add)) => {
                        // opt: addend and subtrahend can be rolled over
                        let pj = j - 1;
                        let top_left = ps[len - 1].wrapping_mul(haystack[i][pj] as Hash);
                        let top_right = ps[len - nw].wrapping_mul(haystack[i][pj + nw] as Hash);
                        let bottom_left = ps[nw - 1].wrapping_mul(haystack[i + nh][pj] as Hash);
                        let bottom_right = haystack[i + nh][pj + nw] as Hash;
                        roll_hash(&mut sub, top_left, top_right, P);
                        roll_hash(&mut add, bottom_left, bottom_right, P);
                        roll_hash(h, sub, add, ps[nw]);
                        cached_roll = Some((sub, add));
                    }
                    None => {
                        let mut subtrahend = Hash::default();
                        let mut addend = Hash::default();
                        for j in 0..nw {
                            let temp = ps[len - 1 - j].wrapping_mul(haystack[i][j] as Hash);
                            subtrahend = subtrahend.wrapping_add(temp);
                            let temp = ps[nw - 1 - j].wrapping_mul(haystack[i + nh][j] as Hash);
                            addend = addend.wrapping_add(temp);
                        }
                        roll_hash(h, subtrahend, addend, ps[nw]);
                        cached_roll = Some((subtrahend, addend));
                    }
                }
            }
        }
    }

    acc
}

#[cfg(test)]
mod tests {
    use super::*;
    #[inline]
    fn str2str2d(s: &str) -> Str2d {
        s.split_ascii_whitespace()
            .map(|r| r.to_string().into_bytes())
            .collect::<Str2d>()
    }
    // hahah I have no internet, time to implement my own random number generator
    fn random_u64(seed: &mut u64) -> u64 {
        let mut x = *seed;
        x ^= x >> 12;
        x ^= x << 25;
        x ^= x >> 27;
        *seed = x;
        x.wrapping_mul(0x2545F4914F6CDD1D)
    }
    // returns infinite iterator which generates random numbers between 0..26
    fn char_gen(seed: &mut u64) -> impl Iterator<Item = u8> {
        (0..)
            .scan(*seed, |acc, _| {
                let rand = random_u64(acc);
                Some((0..(u64::BITS / 5)).scan(rand, |r, _| {
                    let rem = (*r) & 0b11111;
                    *r >>= 5;
                    Some(rem as u8)
                }))
            })
            .flatten()
            .filter(|c| *c < 26)
    }
    fn gen_str2d(
        w: usize,
        h: usize,
        rng: &mut impl Iterator<Item = u8>,
        lambda: impl Fn(u8) -> u8,
    ) -> Str2d {
        rng.take(h * w)
            .map(lambda)
            .collect::<Vec<_>>()
            .chunks_exact(w)
            .map(|row| row.to_owned())
            .collect()
    }

    #[test]
    fn ex_a() {
        let needle = str2str2d("BCB\nCBC");
        let haystack = str2str2d("BCBCB\nCBCBC\nAABAA");
        assert_eq!(brute_force(needle, haystack), 2);
    }
    #[test]
    fn ex_b() {
        let needle = str2str2d("AA\nAA");
        let haystack = str2str2d("BAAAB\nBAAAB\nBAAAB");
        assert_eq!(brute_force(needle, haystack), 4);
    }
    #[test]
    fn ex_c() {
        let needle = str2str2d("ZZZ\nZZZ\nZZZ");
        let haystack = str2str2d("AAA\nAAA\nAAA");
        assert_eq!(brute_force(needle, haystack), 0);
    }

    #[test]
    fn hash_test() {
        let needle = str2str2d("BCB\nCBC");
        let haystack = str2str2d("BCBCB\nCBCBC\nAABAA");
        let (nw, nh) = (needle[0].len(), needle.len());
        let ps = init_ps(6);
        let h1 = init_hash(&needle, nw, nh, &ps);
        let mut h2 = init_hash(&haystack, nw, nh, &ps);
        assert_eq!(h1, h2);
        h2 = roll_hash_right(h2, &ps, &haystack, 0, 0, nw, nh);
        h2 = roll_hash_right(h2, &ps, &haystack, 1, 0, nw, nh);
        assert_eq!(h1, h2);
    }
    #[test]
    fn roll_right_test() {
        let needle = vec![vec![1, 2, 3], vec![4, 5, 6]];
        let haystack = vec![vec![5, 1, 2, 3], vec![9, 4, 5, 6], vec![13, 13, 13, 13]];
        let (nw, nh) = (needle[0].len(), needle.len());
        let ps = init_ps(6);
        let h1 = init_hash(&needle, nw, nh, &ps);
        let h2 = init_hash(&haystack, nw, nh, &ps);
        assert_eq!(h1, roll_hash_right(h2, &ps, &haystack, 0, 0, nw, nh));
    }
    #[test]
    fn roll_down_test() {
        let needle1 = vec![vec![11, 12, 14], vec![1, 2, 3]];
        let needle2 = vec![vec![1, 2, 3], vec![4, 5, 6]];
        let haystack = vec![
            vec![11, 12, 14, 9],
            vec![11, 12, 14, 9],
            vec![1, 2, 3, 7],
            vec![4, 5, 6, 13],
        ];
        let (nw, nh) = (needle1[0].len(), needle1.len());
        let ps = init_ps(6);
        let hn1 = init_hash(&needle1, nw, nh, &ps);
        let hn2 = init_hash(&needle2, nw, nh, &ps);
        let mut hhay = init_hash(&haystack, nw, nh, &ps);
        assert_ne!(hn1, hhay);
        assert_ne!(hn2, hhay);
        hhay = roll_hash_down(hhay, &ps, &haystack, 0, 0, nw, nh);
        assert_eq!(hn1, hhay);
        assert_ne!(hn2, hhay);
        hhay = roll_hash_down(hhay, &ps, &haystack, 0, 1, nw, nh);
        assert_ne!(hn1, hhay);
        assert_eq!(hn2, hhay);
    }
    #[test]
    fn ex_a_rk() {
        let needle = str2str2d("BCB\nCBC");
        let haystack = str2str2d("BCBCB\nCBCBC\nAABAA");
        assert_eq!(rk2d(needle, haystack), 2);
    }
    #[test]
    fn ex_b_rk() {
        let needle = str2str2d("AA\nAA");
        let haystack = str2str2d("BAAAB\nBAAAB\nBAAAB");
        assert_eq!(rk2d(needle, haystack), 4);
    }
    #[test]
    fn ex_a_rk2() {
        let needle = str2str2d("BCB\nCBC");
        let haystack = str2str2d("BCBCB\nCBCBC\nAABAA");
        assert_eq!(rk2dv2(needle, haystack), 2);
    }
    #[test]
    fn ex_b_rk2() {
        let needle = str2str2d("AA\nAA");
        let haystack = str2str2d("BAAAB\nBAAAB\nBAAAB");
        assert_eq!(rk2dv2(needle, haystack), 4);
    }
    #[test]
    fn ex_b_manual_rk2() {
        let needle = str2str2d("AA\nAA");
        let haystack = str2str2d("BAAAB\nBAAAB\nBAAAB");
        let (hh, _hw) = (haystack.len(), haystack[0].len());
        let (nh, nw) = (needle.len(), needle[0].len());
        let len = nh * nw;
        // phase 1 precompute hashes
        let ps = init_ps(len);
        let mut hashes = precompute_hashes(&needle, &haystack, &ps);
        let hashed_needle: Hash = init_hash(&needle, nw, nh, &ps);
        // check and down
        let mut acc = 0;
        for i in 0..=hh - nh {
            let mut cached_roll: Option<(Hash, Hash)> = None;
            for (j, h) in hashes.iter_mut().enumerate() {
                // verify
                if *h == hashed_needle
                    && needle
                        .iter()
                        .zip(haystack[i..].iter())
                        .all(|(nr, hr)| *nr == hr[j..(j + nw)])
                {
                    acc += 1;
                }
                // rollover if not last row
                if i < hh - nh {
                    match cached_roll {
                        Some((mut sub, mut add)) => {
                            // opt: addend and subtrahend can be rolled over
                            let pj = j - 1;
                            let top_left = ps[len - 1].wrapping_mul(haystack[i][pj] as Hash);
                            let top_right = ps[len - nw].wrapping_mul(haystack[i][pj + nw] as Hash);
                            let bottom_left = ps[nw - 1].wrapping_mul(haystack[i + nh][pj] as Hash);
                            let bottom_right = haystack[i + nh][pj + nw] as Hash;
                            roll_hash(&mut sub, top_left, top_right, P);
                            roll_hash(&mut add, bottom_left, bottom_right, P);
                            roll_hash(h, sub, add, ps[nw]);
                            cached_roll = Some((sub, add));
                        }
                        None => {
                            let mut subtrahend = Hash::default();
                            let mut addend = Hash::default();
                            for j in 0..nw {
                                let temp = ps[len - 1 - j].wrapping_mul(haystack[i][j + j] as Hash);
                                subtrahend = subtrahend.wrapping_add(temp);
                                let temp =
                                    ps[nw - 1 - j].wrapping_mul(haystack[i + nh][j + j] as Hash);
                                addend = addend.wrapping_add(temp);
                            }
                            roll_hash(h, subtrahend, addend, ps[nw]);
                            cached_roll = Some((subtrahend, addend));
                        }
                    }
                }
            }
        }
        assert_eq!(acc, 4);
    }
    #[test]
    fn manual_ex_b_rk() {
        let needle = str2str2d("AA\nAA");
        let haystack = str2str2d("BAAAB\nBAAAB\nBAAAB");
        // assert_eq!(rk2d(needle, haystack), 4);
        let (nw, nh) = (needle[0].len(), needle.len());
        let ps = init_ps(6);
        let h = init_hash(&needle, nw, nh, &ps);
        let mut rolling = init_hash(&haystack, nw, nh, &ps);
        assert_ne!(h, rolling);
        rolling = roll_hash_right(rolling, &ps, &haystack, 0, 0, nw, nh);
        let mut right_rolling = roll_hash_right(rolling, &ps, &haystack, 1, 0, nw, nh);
        assert_eq!(h, rolling);
        assert_eq!(h, right_rolling);
        right_rolling = roll_hash_down(right_rolling, &ps, &haystack, 2, 0, nw, nh);
        rolling = roll_hash_down(rolling, &ps, &haystack, 1, 0, nw, nh);
        assert_eq!(h, rolling);
        assert_eq!(h, right_rolling);
    }
    #[test]
    fn ex_c_rk() {
        let needle = str2str2d("ZZZ\nZZZ\nZZZ");
        let haystack = str2str2d("AAA\nAAA\nAAA");
        assert_eq!(rk2d(needle, haystack), 0);
    }
    #[test]
    fn ex_c_rk2() {
        let needle = str2str2d("ZZZ\nZZZ\nZZZ");
        let haystack = str2str2d("AAA\nAAA\nAAA");
        assert_eq!(rk2dv2(needle, haystack), 0);
    }
    #[ignore]
    #[test]
    fn rk_random() {
        let mut seed: u64 = 0xdeadbeefdeadbeef;
        let mut rng = char_gen(&mut seed);
        let (nw, nh) = (200, 200);
        let (hw, hh) = (2000, 2000);
        let needle = gen_str2d(nw, nh, &mut rng, |c| c + 0x41);
        let haystack = gen_str2d(hw, hh, &mut rng, |c| c + 0x41);
        let ba = brute_force(needle.clone(), haystack.clone());
        let rk2 = rk2dv2(needle.clone(), haystack.clone());
        let rk = rk2d(needle, haystack);
        assert_eq!(ba, rk);
        assert_eq!(ba, rk2);
    }
    #[ignore]
    #[test]
    fn rk_random_grid() {
        let mut seed: u64 = 0xdeadbeefdeadbeef;
        let mut rng = char_gen(&mut seed);
        let scale = 200;
        let reps = 10;
        let (nw, nh) = (scale, scale);
        let (hw, hh) = (scale * reps, scale * reps);
        let needle = gen_str2d(nw, nh, &mut rng, |c| c + 1);
        let mut haystack = vec![];
        for _ in 0..reps {
            for row in needle.iter() {
                let hr: Vec<_> = row.iter().cycle().take(hw).cloned().collect();
                haystack.push(hr);
            }
        }
        assert_eq!((hw, hh), (haystack[0].len(), haystack.len()));
        let ba = brute_force(needle.clone(), haystack.clone());
        let rk2 = rk2dv2(needle.clone(), haystack.clone());
        let rk = rk2d(needle, haystack);
        assert_eq!(reps * reps, ba);
        assert_eq!(reps * reps, rk);
        assert_eq!(reps * reps, rk2);
    }
    #[ignore]
    #[test]
    fn rk_worst_case() {
        let mut needle: Str2d = (0..200)
            .map(|_| (0..200).map(|_| 0x41_u8).collect::<Vec<_>>())
            .collect();
        needle[199][199] = 0x42;
        let haystack: Str2d = (0..200).map(|_| (0..200).map(|_| 0x41).collect()).collect();
        let ba = brute_force(needle.clone(), haystack.clone());
        let rk2 = rk2dv2(needle.clone(), haystack.clone());
        let rk = rk2d(needle, haystack);
        assert_eq!(0, ba);
        assert_eq!(0, rk);
        assert_eq!(0, rk2);
    }

    #[test]
    fn str_count_test() {
        let hs =
            "This is a test. This testest is only a test. If this were a real emergency...".into();
        let nd = "test".into();
        assert_eq!(str_count(nd, hs), 4);
        assert_eq!(str_count("aaa".into(), "aaaaaaaa".into()), 6);
    }
}
