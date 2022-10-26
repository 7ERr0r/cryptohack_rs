
use num_bigint::BigInt;
use crate::big;





// from https://en.wikipedia.org/wiki/Tonelli%E2%80%93Shanks_algorithm
pub fn my_tonelli_shanks(p: &BigInt, n: &BigInt) -> BigInt {
    use num_traits::One;
    use num_traits::Zero;

    let p_1 = p - 1;

    let (q, s) = my_factor_powers_2(&p_1);

    println!("q: {}", q);
    println!("s: {}", s);

    //let a1 =

    let z = my_find_non_residue(p);
    println!("z: {}", z);

    let mut m = s.clone();
    let mut c = z.modpow(&q, p);
    let mut t = n.modpow(&q, p);
    let mut r = n.modpow(&((&q + 1) / 2), p);

    for iter in 0..1000 {
        println!("--- iteration: {}", iter);
        println!("t: {}", t);
        println!("r: {}", r);
        println!("m: {}", m);
        if t.is_zero() {
            return BigInt::zero();
        } else if t.is_one() {
            return r;
        } else {
            let i = my_find_t_1_squaring(&m, &t, p);
            
            let b = (&m - &i - 1) % p;
            let b = (big!(2)).modpow(&b, p);
            //println!("2**(m - i - 1) == {}", &b);
            let b = &c.modpow(&b, p);
            //println!("b <- {}", b);

            m = i;
            c = b.modpow(&big!(2), p);
            //println!("c <- {}", c);
            
            t *= &c;
            t %= p;

            r *= b;
            r %= p;
        }
    }
    let root = BigInt::default();
    root
}

// returns i, 0 < i < m, such that t**(2**i) == 1
pub fn my_find_t_1_squaring(m: &BigInt, t: &BigInt, p: &BigInt) -> BigInt {
    use num_traits::One;

    println!("use repeated squaring to find the least i, 0 < i < M, such that t**(2**i) == 1");
    println!("M: {}", m);

    let mut i = BigInt::one();
    while &i < m {
        let squared = (&big!(2)).modpow(&i, p);
        let squared = t.modpow(&squared, p);

        if squared.is_one() {
            println!("found i: {}", i);

            break;
        }

        i += 1;
    }

    i
}

// returns (q, s)
// where x = q * 2**s
pub fn my_factor_powers_2(x: &BigInt) -> (BigInt, BigInt) {
    use num_integer::Integer;
    use num_traits::identities::Zero;

    let mut q = x.clone();
    let mut s = BigInt::zero();

    while q.is_even() {
        q = q / 2;

        s += 1;
    }
    //let s = BigInt::default();

    (q, s)
}

// return z, which is non-residue
pub fn my_find_non_residue(p: &BigInt) -> BigInt {
    let mut test: BigInt = big!(2);
    let _minus1 = &(p - 1);
    let legendre_exp = _minus1 / 2;

    loop {
        test += 1;

        let legendre = test.modpow(&legendre_exp, p);

        if &legendre == _minus1 {
            break;
        }
    }

    test
}