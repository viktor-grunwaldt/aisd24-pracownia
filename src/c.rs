// use crate::{println, scanf};

const C: usize = 100;
const MAX: u32 = 1 << 31;
const HIGHEST_WV: u32 = 10_000;

// weight, value
type Coin = (u32, u32);
type Coins<'a> = &'a [Coin];
type Hist = [u32; C];
struct Ans {
    p_min: u32,
    min_coins: Hist,
    p_max: u32,
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
    let mut last_coin_max = vec![(C + 1) as u8; m];
    let mut last_coin_min = vec![(C + 1) as u8; m];
    dp_max[0] = 0;
    dp_min[0] = 0;
    for i in 0..m {
        if dp_max[i] == MAX {
            continue;
        }
        let rem = m - i;
        let last_coin = last_coin_max[i] as usize;

        let current_smallest_weight = if last_coin == C + 1 {
            cs[last_coin].0
        } else {
            HIGHEST_WV
        };
        for (ic, &(_w, v)) in cs.iter().enumerate() {
            let w = _w as usize;
            if w >= rem {
                continue;
            }
            if _w > current_smallest_weight {
                continue;
            }
            let next_w = i + w;
            let cand = dp_max[i] + v;
            // max
            if dp_max[next_w] == MAX || dp_max[next_w] < cand {
                dp_max[next_w] = cand;
                last_coin_max[next_w] = ic as u8;
            }
            // min
            let cand = dp_min[i] + v;
            if dp_min[next_w] > cand {
                dp_min[next_w] = cand;
                last_coin_min[next_w] = ic as u8;
            }
        }
    }
    if dp_max[masa] == MAX {
        return None;
    }
    let max_coins = get_coins(last_coin_max, masa, cs);
    let min_coins = get_coins(last_coin_min, masa, cs);
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
pub fn solve() {
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

    fn dot_prod(cs: Coins, hist: &Hist) -> u32 {
        cs.iter().zip(hist).map(|(&c, b)| c.1 * b).sum()
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

    #[test]
    fn many_random_tests() {
        let mut rng = ChaCha8Rng::seed_from_u64(6969 + 420);
        for i in 0..10 {
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
    #[test]
    fn ascending_weights() {
        let mut coins = [(0, 0); C];
        coins.iter_mut().enumerate().take(10).for_each(|(i, e)| {
            *e = (1000 - i as u32, 1000);
        });
        let w = 9955;
        let o_res = knapsack(w, &coins);
        assert!(o_res.is_some());
        assert_eq!(o_res.unwrap().p_max, 10 * 1000);
    }
}
