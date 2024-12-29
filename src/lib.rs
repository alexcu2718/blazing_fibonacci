use num_bigint::BigUint;
use num_traits::{One, Zero};
use rayon::join;

pub struct Matrix2x2 {
    a11: BigUint,
    a12: BigUint,
    a21: BigUint,
    a22: BigUint,
}

impl Matrix2x2 {
    pub fn multiply(&self, other: &Matrix2x2) -> Matrix2x2 {
        Matrix2x2 {
            a11: &self.a11 * &other.a11 + &self.a12 * &other.a21,
            a12: &self.a11 * &other.a12 + &self.a12 * &other.a22,
            a21: &self.a21 * &other.a11 + &self.a22 * &other.a21,
            a22: &self.a21 * &other.a12 + &self.a22 * &other.a22,
        }
    }
    pub fn parallel_multiply(&self, other: &Matrix2x2) -> Matrix2x2 {
        let ((a11, a12), (a21, a22)) = join(
            || join(
                || &self.a11 * &other.a11 + &self.a12 * &other.a21,
                || &self.a11 * &other.a12 + &self.a12 * &other.a22,
            ),
            || join(
                || &self.a21 * &other.a11 + &self.a22 * &other.a21,
                || &self.a21 * &other.a12 + &self.a22 * &other.a22,
            ),
        );

        Matrix2x2 {
            a11,
            a12,
            a21,
            a22,
        }
    }
}
//1,2,3,4,5
//1,1,2,3,5




pub fn fast_fib(n: usize) -> BigUint {
  
    if n == 1 || n==2 {
        return BigUint::one();
    }
    if n ==3{
        return BigUint::from(2u32); 
    }
    

    let mut base: Matrix2x2 = Matrix2x2 {
        a11: BigUint::one(),
        a12: BigUint::one(),
        a21: BigUint::one(),
        a22: BigUint::zero(),
    };
    
    let mut result: Matrix2x2 = Matrix2x2 {
        a11: BigUint::one(),
        a12: BigUint::zero(),
        a21: BigUint::zero(),
        a22: BigUint::one(),
    };
    
    let mut exp = n - 1;
    while exp > 0 {
        if exp & 1 == 1 {
            result = result.parallel_multiply(&base);
        }
        base = base.parallel_multiply(&base);
        exp >>= 1;
    }
    
    result.a11}



    pub fn fast_double(n: usize) -> BigUint {

        if 10>n{return fast_fib(n)}

        if n % 2 == 0 {
            let big_2:BigUint=BigUint::one()+BigUint::one();
            let f_n: BigUint = fast_fib(n/2);
            f_n.clone() * (big_2*fast_fib((n/2) + 1) - f_n)
        } else {
            let k = (n - 1) / 2;
            let f_k = fast_fib(k);
            let f_k_plus_1 = fast_fib(k + 1);
            f_k.clone() * f_k + f_k_plus_1.clone() * f_k_plus_1
        }
    }
    
    
    
