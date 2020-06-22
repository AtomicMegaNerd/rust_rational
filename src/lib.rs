use std::fmt;
use std::fmt::Display;
use std::ops;

#[derive(Debug, Eq, PartialEq, Copy, Clone)]
pub struct Rational {
    pub numerator: i64,
    pub denominator: i64,
}

impl Rational {
    pub fn new(numer: i64, denom: i64) -> Rational {
        if denom == 0 {
            panic!("Cannot specify a Rational object with 0 as the denominator");
        }

        let gcd = gcd(numer, denom);
        Rational {
            numerator: numer / gcd,
            denominator: denom / gcd,
        }
    }

    pub fn reciprocal(&self) -> Rational {
        Rational::new(self.denominator, self.numerator)
    }
}

// Rust does support operator overloading
impl ops::Add<Rational> for Rational {
    type Output = Self;
    fn add(self, other: Rational) -> Rational {
        let l = lcm(self.denominator, self.denominator);
        Rational::new(
            (self.numerator * l / self.denominator) + (other.numerator * l / other.denominator),
            l,
        )
    }
}

impl ops::Sub<Rational> for Rational {
    type Output = Self;
    fn sub(self, other: Rational) -> Rational {
        let l = lcm(self.denominator, other.denominator);
        Rational::new(
            (self.numerator * l / self.denominator) - (other.numerator * l / other.denominator),
            l,
        )
    }
}

impl ops::Mul<Rational> for Rational {
    type Output = Self;
    fn mul(self, other: Rational) -> Rational {
        Rational::new(
            self.numerator * other.numerator,
            self.denominator * other.denominator,
        )
    }
}

// We are using * here on purpose to make the code simpler so silence
// this warning from Clippy.
#[allow(clippy::suspicious_arithmetic_impl)]
impl ops::Div<Rational> for Rational {
    type Output = Self;
    fn div(self, other: Rational) -> Rational {
        self * other.reciprocal()
    }
}

impl Display for Rational {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        if self.denominator == 1 {
            write!(f, "{}", self.numerator)
        } else {
            write!(f, "{}/{}", self.numerator, self.denominator)
        }
    }
}

// Private utility functions

fn gcd(x: i64, y: i64) -> i64 {
    if y == 0 {
        x
    } else {
        gcd(y, x % y)
    }
}

fn lcm(x: i64, y: i64) -> i64 {
    (x * y) / gcd(x, y)
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_rational_reduction_works() {
        let r1 = Rational::new(1, 2);
        let r2 = Rational::new(2, 4);
        assert_eq!(r1, r2);
    }

    #[test]
    fn test_rational_addition_works() {
        let r1 = Rational::new(1, 4);
        let r2 = Rational::new(1, 4);
        assert_eq!(Rational::new(1, 2), r1 + r2);
    }

    #[test]
    fn test_rational_subtraction_works() {
        let r1 = Rational::new(4, 4);
        let r2 = Rational::new(1, 4);
        assert_eq!(Rational::new(3, 4), r1 - r2);
    }

    #[test]
    fn test_rational_multiplication_works() {
        let r1 = Rational::new(1, 2);
        let r2 = Rational::new(1, 2);
        assert_eq!(Rational::new(1, 4), r1 * r2);
    }

    #[test]
    fn test_rational_division_works() {
        let r1 = Rational::new(1, 2);
        let r2 = Rational::new(1, 2);
        assert_eq!(Rational::new(1, 1), r1 / r2);
    }

    #[test]
    #[should_panic(expected = "Cannot specify a Rational object with 0 as the denominator")]
    fn test_should_panic_with_denominator_of_0() {
        let _r1 = Rational::new(2, 0);
    }

    #[test]
    fn reciprocal_should_work() {
        assert_eq!(Rational::new(3, 1), Rational::new(1, 3).reciprocal());
    }
}
