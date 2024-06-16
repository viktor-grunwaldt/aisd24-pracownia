use std::io::{self, BufRead};

fn solve_dp(k: usize, freq: Vec<u32>) -> (u32, Vec<usize>) {
    let n = freq.len();
    let mut dp = vec![vec![u32::MAX; k + 1]; n + 1];
    // n max = 10_000 <= 2^14
    let mut backtrack = vec![vec![u16::MAX; k + 1]; n + 1];
    let mut _a = vec![0; n + 1];
    let mut _b = vec![0; n + 1];
    for i in (1..n).rev() {
        _a[i] = freq[i] + _a[i + 1];
        _b[i] = _a[i] + _b[i + 1];
    }
    dp[n][0] = 0;
    for j in 1..=k {
        for i in (1..=n).rev() {
            for s in i + 1..=n {
                let seg = _b[i] - _b[s] - ((s - i) as u32) * _a[s];
                let new = dp[s][j - 1].saturating_add(seg);
                if new < dp[i][j] {
                    dp[i][j] = new;
                    backtrack[i][j] = s as u16;
                }
            }
        }
    }
    let pos = (1..=k)
        .rev()
        .scan(1, |state, j| {
            let val = backtrack[*state][j] as usize - *state;
            *state = backtrack[*state][j] as usize;
            Some(val)
        })
        .collect();
    (dp[1][k], pos)
}

pub fn solve() {
    let lines = io::stdin()
        .lock()
        .lines()
        .map(|l| l.expect("failed to read line"))
        .collect::<Vec<String>>();
    assert_eq!(lines.len(), 2, "unexpected number of lines");
    let kl = lines[0].split_once(' ').expect("expected 2 numbers");
    let k: usize = kl.0.parse().expect("failed to parse number k");
    let l: usize = kl.1.parse().expect("failed to parse number l");
    let histogram = vec![u32::MAX]
        .into_iter()
        .chain(
            lines[1]
                .split_ascii_whitespace()
                .map(|n| n.parse().expect("failed to parse number")),
        )
        .collect::<Vec<u32>>();
    assert_eq!(l, histogram.len() - 1, "input does not match");
    // assert!(l <= 1002, "this algorithm will be too slow");
    let (min_ct, layout) = solve_dp(k, histogram);
    let out = layout
        .iter()
        .map(|e| e.to_string())
        .collect::<Vec<_>>()
        .join(" ");
    println!("{}", min_ct);
    println!("{}", out);
}
