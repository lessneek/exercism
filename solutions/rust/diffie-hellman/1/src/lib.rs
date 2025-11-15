use num_bigint::BigUint;
use rand::Rng;

pub fn private_key(p: u64) -> u64 {
    let mut rng = rand::thread_rng();
    rng.gen_range(2..p)
}

fn modexp(b: u64, e: u64, m: u64) -> u64 {
    let e = BigUint::from(e);
    let b = BigUint::from(b);
    let m = BigUint::from(m);

    let r = b.modpow(&e, &m);
    r.to_u64_digits()[0]
}

pub fn public_key(p: u64, g: u64, priv_key: u64) -> u64 {
    modexp(g, priv_key, p)
}

pub fn secret(p: u64, pub_key: u64, priv_key: u64) -> u64 {
    modexp(pub_key, priv_key, p)
}
