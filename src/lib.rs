use rug::Assign;
use rug::Integer;

#[derive(Clone, Debug)]
pub struct Matrix2x2 {
    a11: Integer,
    a12: Integer,
    a21: Integer,
    a22: Integer,
}

impl Matrix2x2 {
    pub fn new(a11: Integer, a12: Integer, a21: Integer, a22: Integer) -> Self {
        Matrix2x2 { a11, a12, a21, a22 }
    }

    pub fn identity() -> Self {
        Matrix2x2 {
            a11: Integer::from(1),
            a12: Integer::from(0),
            a21: Integer::from(0),
            a22: Integer::from(1),
        }
    }

    pub fn multiply(&self, other: &Matrix2x2) -> Matrix2x2 {
        // Use fully owned operations to get complete Integer results.
        Matrix2x2 {
            a11: Integer::from(
                self.a11.clone() * other.a11.clone() + self.a12.clone() * other.a21.clone(),
            ),
            a12: Integer::from(
                self.a11.clone() * other.a12.clone() + self.a12.clone() * other.a22.clone(),
            ),
            a21: Integer::from(
                self.a21.clone() * other.a11.clone() + self.a22.clone() * other.a21.clone(),
            ),
            a22: Integer::from(
                self.a21.clone() * other.a12.clone() + self.a22.clone() * other.a22.clone(),
            ),
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

pub fn fibonacci(n: usize) -> Integer {
    if n == 0 {
        return Integer::from(0);
    }

    let base = Matrix2x2::new(
        Integer::from(1),
        Integer::from(1),
        Integer::from(1),
        Integer::from(0),
    );

    matrix_power(&base, n - 1).a11
}

/// Highly optimized iterative fast doubling Fibonacci using in-place arithmetic with rug::Integer.
/// This version reuses scratch variables and in-place assignment to minimize allocations and cloning.
pub fn fast_double(n: usize) -> Integer {
    if n == 0 {
        return Integer::from(0);
    }
    let mut a = Integer::from(0);
    let mut b = Integer::from(1);
    let bits = (usize::BITS - n.leading_zeros()) as usize;

    // Preallocate scratch variables to reuse their memory across iterations.
    let mut t = Integer::new(); // for computing 2*b - a
    let mut c = Integer::new(); // for computing a * t
    let mut aa = Integer::new(); // for computing a^2
    let mut bb = Integer::new(); // for computing b^2
    let mut d = Integer::new(); // for computing a^2 + b^2

    for i in (0..bits).rev() {
        // t = 2*b - a (in-place)
        t.assign(&b);
        t <<= 1; // t = b * 2
        t -= &a; // t = 2*b - a

        // c = a * t
        c.assign(&a);
        c *= &t; // c = a * (2*b - a)

        // d = a^2 + b^2 using in-place operations
        aa.assign(&a);
        aa.square_mut(); // aa = a^2
        bb.assign(&b);
        bb.square_mut(); // bb = b^2
        d.assign(&aa);
        d += &bb; // d = a^2 + b^2

        // Update (a, b) based on the current bit.
        if ((n >> i) & 1) == 1 {
            a.assign(&d);
            b.assign(&c);
            b += &d;
        } else {
            a.assign(&c);
            b.assign(&d);
        }
    }
    a
}
