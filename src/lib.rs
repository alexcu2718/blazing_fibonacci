use num_bigint::BigUint;
use num_traits::{One, Zero};

pub struct Matrix2x2 {
    a11: BigUint,
    a12: BigUint,
    a21: BigUint,
    a22: BigUint,
}

impl Matrix2x2 {
    fn multiply(&self, other: &Matrix2x2) -> Matrix2x2 {
        Matrix2x2 {
            a11: &self.a11 * &other.a11 + &self.a12 * &other.a21,
            a12: &self.a11 * &other.a12 + &self.a12 * &other.a22,
            a21: &self.a21 * &other.a11 + &self.a22 * &other.a21,
            a22: &self.a21 * &other.a12 + &self.a22 * &other.a22,
        }
    }
}


pub fn fast_fib(n: usize) -> BigUint {
    if n == 0 {
        return BigUint::zero();
    }
    if n == 1 {
        return BigUint::one();
    }
    if n==2 {
        return BigUint::one()+BigUint::one();
    }

    let mut base = Matrix2x2 {
        a11: BigUint::one(),
        a12: BigUint::one(),
        a21: BigUint::one(),
        a22: BigUint::zero(),
    };
    
    let mut result = Matrix2x2 {
        a11: BigUint::one(),
        a12: BigUint::zero(),
        a21: BigUint::zero(),
        a22: BigUint::one(),
    };
    
    let mut exp = n - 1;
    while exp > 0 {
        if exp & 1 == 1 {
            result = result.multiply(&base);
        }
        base = base.multiply(&base);
        exp >>= 1;
    }
    
    result.a11
}