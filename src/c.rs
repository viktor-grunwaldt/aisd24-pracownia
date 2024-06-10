use crate::macros::{println, scanf};
const C: usize = 100;
// limit is 10^{5+6} <= 2^37
const MAX: u64 = 1 << 38;
// weight, value
type Coin = (u32, u32);
type Coins<'a> = &'a [Coin];
type Hist = [u32; C];
struct Ans {
    p_min: u64,
    min_coins: Hist,
    p_max: u64,
    max_coins: Hist,
}

fn get_coins(arr: Vec<u8>, masa: usize, cs: Coins) -> Hist {
    let mut coins = [0; C];
    let mut prev_weight = masa;
    while prev_weight > 0 {
        let prev_coin = arr[prev_weight] as usize;
        coins[prev_coin] += 1;
        prev_weight -= cs[prev_coin].0 as usize;
    }
    coins
}

fn knapsack(masa: usize, cs: Coins) -> Option<Ans> {
    let m = masa + 1;
    let mut dp_max = vec![MAX; m];
    let mut dp_min = vec![MAX; m];
    let mut max_coin_added = vec![(C + 1) as u8; m];
    let mut min_coin_added = vec![(C + 1) as u8; m];
    dp_max[0] = 0;
    dp_min[0] = 0;
    for i in 0..m {
        let rem: u32 = (m - i).try_into().unwrap();
        if dp_max[i] == MAX {
            continue;
        }
        for (ic, &(_w, v)) in cs.iter().enumerate().filter(|c| c.1 .0 < rem) {
            let w = _w as usize;
            let next_w = i + w;
            let cand = dp_max[i] + v as u64;
            // max
            if dp_max[next_w] == MAX || dp_max[next_w] < cand {
                dp_max[next_w] = cand;
                max_coin_added[next_w] = ic as u8;
            }
            // min
            let cand = dp_min[i] + v as u64;
            if dp_min[next_w] > cand {
                dp_min[next_w] = cand;
                min_coin_added[next_w] = ic as u8;
            }
        }
    }
    if dp_max[masa] == MAX {
        return None;
    }
    let max_coins = get_coins(max_coin_added, masa, cs);
    let min_coins = get_coins(min_coin_added, masa, cs);
    Some(Ans {
        p_min: dp_min[masa],
        min_coins,
        p_max: dp_max[masa],
        max_coins,
    })
}

fn vec_to_str(cs: Hist, n: usize) -> String {
    cs.iter()
        .take(n)
        .map(|ct| ct.to_string())
        .collect::<Vec<_>>()
        .join(" ")
}
fn main() {
    let masa_monet = scanf!(usize);
    let mut monety = [(0, 0); C];
    let n = scanf!(usize);
    for e in monety.iter_mut().take(n) {
        let (val, wt) = scanf!(u32, u32);
        *e = (wt, val);
    }

    match knapsack(masa_monet, &monety[..n]) {
        Some(res) => {
            println!(
                "TAK\n{}\n{}\n{}\n{}",
                res.p_min,
                vec_to_str(res.min_coins, n),
                res.p_max,
                vec_to_str(res.max_coins, n),
            )
        }
        None => {
            println!("NIE");
        }
    }
}

#[cfg(test)]
mod tests {
    use rand::{distributions::Uniform, Rng, SeedableRng};
    use rand_chacha::ChaCha8Rng;

    use super::*;
    const HIGHEST_WV: u32 = 100_000;

    fn dot_prod(cs: Coins, hist: &Hist) -> u64 {
        cs.iter().zip(hist).map(|(&c, b)| (c.1 * b) as u64).sum()
    }
    #[test]
    fn ex_a() {
        let w = 100;
        let cs = vec![(1, 1), (50, 30)];
        let o_res = knapsack(w, &cs);
        assert!(o_res.is_some());
        let res = o_res.unwrap();
        assert_eq!(res.p_max, 100);
        assert_eq!(res.p_min, 60);
        assert_eq!(dot_prod(&cs, &res.max_coins), 100);
        assert_eq!(dot_prod(&cs, &res.min_coins), 60);
    }

    #[test]
    fn ex_b() {
        let w = 10;
        let cs = vec![(1, 1), (4, 2), (16, 4)];
        let o_res = knapsack(w, &cs);
        assert!(o_res.is_some());
        let res = o_res.unwrap();
        assert_eq!(res.p_max, 10);
        assert_eq!(res.p_min, 6);
        assert_eq!(dot_prod(&cs, &res.max_coins), 10);
        assert_eq!(dot_prod(&cs, &res.min_coins), 6);
    }

    #[test]
    fn ex_c() {
        let w = 5;
        let cs = vec![(2, 1), (4, 1), (4, 2)];
        let res = knapsack(w, &cs);
        assert!(res.is_none());
    }

    #[test]
    fn random_test() {
        let coins = {
            let mut rng = ChaCha8Rng::seed_from_u64(6969);
            let uni = Uniform::from(1..=HIGHEST_WV);
            let mut coins = [(0, 0); C];
            coins.iter_mut().take(100).for_each(|e| {
                *e = (rng.sample(uni), rng.sample(uni));
            });
            coins
        };
        let w = 4 * coins[0].0;
        let o_res = knapsack(w as usize, &coins);
        assert!(o_res.is_some());
        let res = o_res.unwrap();
        assert_eq!(res.p_max, dot_prod(&coins, &res.max_coins));
        assert_eq!(res.p_min, dot_prod(&coins, &res.min_coins));
    }
    #[ignore = "slow"]
    #[test]
    fn many_random_tests() {
        let mut rng = ChaCha8Rng::seed_from_u64(6969 + 420);
        for i in 0..100 {
            dbg!(i);
            let coins = {
                let uni = Uniform::from(1..=HIGHEST_WV);
                let mut coins = [(0, 0); C];
                coins.iter_mut().take(100).for_each(|e| {
                    *e = (rng.sample(uni), rng.sample(uni));
                });
                coins
            };
            let w = 4 * coins[0].0;
            let o_res = knapsack(w as usize, &coins);
            assert!(o_res.is_some());
            let res = o_res.unwrap();
            assert_eq!(res.p_max, dot_prod(&coins, &res.max_coins));
            assert_eq!(res.p_min, dot_prod(&coins, &res.min_coins));
        }
    }
}
