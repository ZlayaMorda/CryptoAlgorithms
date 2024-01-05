use rand::Rng;
use std::sync::Once;

static INIT: Once = Once::new();
const GOPPA_N_MIN: usize = 3;
const GOPPA_N_MAX: usize = 256 + 1;

const GOPPA_N: usize = 0; // Pick a nonzero value to set code length
const GOPPA_T: usize = 0; // Pick a nonzero value to set code correction capacity

pub fn div_ceil(n: usize, d: usize) -> usize {
    n / d + if n % d == 0 { 0 } else { 1 }
}

pub fn log_setup() {
    INIT.call_once(|| {
        env_logger::init();
    });
}

// Picks a random length between GOPPA_N_MIN and GOPPA_N_MAX.
// Picks a random correction capacity which is valid for this length.
pub fn goppa_setup() -> (usize, usize, usize) {
    log_setup();
    let mut rng = rand::thread_rng();
    let (mut m, mut n, mut t) = (1, 0, 1);
    while m as usize * t >= n {
        n = match GOPPA_N {
            0 => rng.gen_range(GOPPA_N_MIN, GOPPA_N_MAX),
            value => value,
        };
        let q = n.next_power_of_two();
        m = q.trailing_zeros();
        t = match GOPPA_T {
            0 => {
                let t_min = 1;
                let t_max = div_ceil(n, m as usize);
                rng.gen_range(t_min, t_max)
            }
            value => value,
        };
        if n == q && t == 1 {
            m += 1;
        }
    }
    (1 << m, n, t)
}
