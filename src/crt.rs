use cargo_snippet::snippet;

#[snippet("_ext_gcd")]
pub fn ext_gcd(a: i64, b: i64) -> (i64, i64, i64) {
    if b == 0 {
        return (a, 1, 0);
    }
    let (q, r) = (a.div_euclid(b), a.rem_euclid(b));
    let (d, s, t) = ext_gcd(b, r);
    let x = t;
    let y = s - q * t;
    (d, x, y)
}

#[snippet("_crt", include = "_ext_gcd")]
pub fn crt(b: Vec<i64>, m: Vec<i64>) -> Option<(i64, i64)> {
    let (mut r, mut lcm) = (0, 1);
    let len = b.len();
    for i in 0..len {
        let (d, p, _q) = ext_gcd(lcm, m[i]);
        if (b[i] - r) % d != 0 {
            return None;
        }
        let s = (b[i] - r) / d;
        r = r + (s * p % (m[i] / d)) * lcm;
        lcm = lcm * (m[i] / d);
    }
    Some((r.rem_euclid(lcm), lcm))
}
