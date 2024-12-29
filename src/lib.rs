use num_bigint::BigUint;
use num_traits::{One, Zero};

#[derive(Clone, Debug)]
pub struct Matrix2x2 {
    a11: BigUint,
    a12: BigUint,
    a21: BigUint,
    a22: BigUint,
}

impl Matrix2x2 {
    pub fn new(a11: BigUint, a12: BigUint, a21: BigUint, a22: BigUint) -> Self {
        Matrix2x2 { a11, a12, a21, a22 }
    }

    pub fn identity() -> Self {
        Matrix2x2 {
            a11: BigUint::one(),
            a12: BigUint::zero(),
            a21: BigUint::zero(),
            a22: BigUint::one(),
        }
    }

    pub fn multiply(&self, other: &Matrix2x2) -> Matrix2x2 {
        Matrix2x2 {
            a11: &self.a11 * &other.a11 + &self.a12 * &other.a21,
            a12: &self.a11 * &other.a12 + &self.a12 * &other.a22,
            a21: &self.a21 * &other.a11 + &self.a22 * &other.a21,
            a22: &self.a21 * &other.a12 + &self.a22 * &other.a22,
        }
    }
}

pub fn matrix_power(matrix: &Matrix2x2, n: usize) -> Matrix2x2 {
    if n == 0 {
        return Matrix2x2::identity();
    }
    if n == 1 {
        return matrix.clone();
    }

    let half = matrix_power(matrix, n / 2);
    let result = half.multiply(&half);
    
    if n % 2 == 0 {
        result
    } else {
        result.multiply(matrix)
    }
}

pub fn fibonacci(n: usize) -> BigUint {
    if n == 0 {
        return BigUint::zero();
    }

    let base = Matrix2x2::new(
        BigUint::one(),
        BigUint::one(),
        BigUint::one(),
        BigUint::zero(),
    );
    
    matrix_power(&base, n - 1).a11
}

pub fn fast_double(n: usize) -> BigUint {
    if n == 0 {
        return BigUint::zero();
    }

    fn fib_pair(n: usize) -> (BigUint, BigUint) {
        if n == 0 {
            return (BigUint::zero(), BigUint::one());
        }

        let (a, b) = fib_pair(n >> 1);
        let two = BigUint::from(2u32);
        let c = &a * (&two * &b - &a);
        let d = &a * &a + &b * &b;

        if n & 1 == 1 {
            (d.clone(), c + d)
        } else {
            (c, d)
        }
    }

    fib_pair(n).0
}
    
    
