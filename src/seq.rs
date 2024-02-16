use std::{mem, ops::Add};

use num_integer::Integer;
use num_traits::One;

pub struct Fibonacci<T> {
    curr: T,
    next: T
}

impl<T: One> Fibonacci<T> {
    #[inline]
    pub fn new() -> Fibonacci<T> {
        Self {
            curr: One::one(),
            next: One::one(),
        }
    }
}

impl<T: Add<T, Output = T> + Clone> Iterator for Fibonacci<T> {
    type Item = T;

    #[inline]
    fn next(&mut self) -> Option<T> {
        let new_next = self.curr.clone() + self.next.clone();
        let new_curr = mem::replace(&mut self.next, new_next);
        Some(mem::replace(&mut self.curr, new_curr))
    }
}

pub struct Collatz {
    n: u64,
}

impl Collatz {
    pub fn new(n: u64) -> Collatz {
        Collatz { n }
    }
}

impl Iterator for Collatz {
    type Item = u64;
    fn next(&mut self) -> Option<Self::Item> {
        match self.n % 2 {
            0 => {
                self.n = self.n / 2;
                Some(self.n)
            },
            1 => {
                if self.n == 1 {
                    None
                } else {
                    self.n = 3 * self.n + 1;
                    Some(self.n)
                }
            },
            _ => unreachable!()
        }
    }
}

pub struct PythagoreanPrimitiveTriplet {
    n: u64,
    m: u64
}

impl PythagoreanPrimitiveTriplet {
    pub fn new(m: u64) -> Self {
        Self { n: 1 + m % 2, m }
    }
}

impl Iterator for PythagoreanPrimitiveTriplet {
    type Item = (u64, u64, u64);

    fn next(&mut self) -> Option<Self::Item> {
        let m_squared = self.m * self.m;
        let mut n_squared = self.n * self.n;

        let (mut n, m) = (self.n, self.m);

        while n < m {
            let a = m_squared - n_squared;
            let b = 2 * m * n;
            let c = m_squared + n_squared;

            if a.gcd(&b) == 1 {
                self.n += 2;
                return Some((a, b, c))
            }

            n += 2;
            n_squared = n * n;
        }

        None
    }
}
