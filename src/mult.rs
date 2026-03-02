use num::{Integer};
// Let's start with a specification of a recursive algorithm for multiplication:
// a * 1 = a
// a * (n + 1) = a * n + a
pub fn mult_spec<T>(a : T, n : T) -> T
where
    T : Integer + Copy
// We will use the `Integer` trait to be generic with
// respect to the instantiated numeric type we are using.
{
    if n == T::one() {
        return a
    }
    return mult_spec(a, n - T::one()) + a
}

pub trait Parity {
    fn odd(&self) -> bool;
    fn half(&self) -> Self;
}

impl Parity for u32 {
    fn odd(&self) -> bool {
        return self & 1 > 0
    }

    fn half(&self) -> u32 {
        return self >> 1
    }
}

// In our first implementation we will compute:
//   a * n = (a + a) * (n / 2) + odd(n) * a
// We can think of this as taking the binary representation of n:
//   n = b_n ... b_0
// and computing:
//   r_0 = (b_n ... b_1) * n
//   r_1 = b_0 * n
// We then have:
//   r = r_0 + r_0 + r_1
pub fn mult1<T>(a : T, n : T) -> T
  where
    T : Integer + Copy + Parity
{
    if n == T::one(){
        return a;
    }
    let r0 = mult1(a + a, n.half());
    if n.odd() {
        return r0 + a;
    }
    return r0;
}



#[cfg(test)]
mod tests {
    use super::*;
    use proptest::prelude::*;

    #[test]
    fn spec_unit1() {
        let result : u32 = mult_spec(19,3);
        assert_eq!(result, 57);
    }

    #[test]
    fn half_spec1() {
        let result : u32 = 10.half();
        assert_eq!(result, 5);
    }    

    proptest!{
        #[test]
        fn mult_spec_commutative(a in 1u32..100, n in 1u32..100){
            let an = mult_spec(a, n);
            let na = mult_spec(n, a);
            prop_assert_eq!(an , na);
        }
    }

    proptest!{
        #[test]
        fn mult1_spec(a in 1u32..50, n in 1u32..50){
            let lhs = mult1(a, n);
            let rhs = mult_spec(a, n);
            prop_assert_eq!(lhs , rhs);
        }
    }
}
