use bit_vec::BitVec;
use num_integer::{Integer, Roots};
use num_traits::FromPrimitive;

pub struct Primes {
    data: Vec<u64>
}

impl Primes {
    pub fn from_sieve(n: usize) -> BitVec {
        let mut is_prime = BitVec::from_elem(n + 1, true);

        is_prime.set(0, false);
        is_prime.set(1, false);

        for i in (4..=n).step_by(2) {
            is_prime.set(i, false);
        }

        for i in (3..=n).step_by(2) {
            if is_prime.get(i).unwrap() {
                for j in ((i*i)..=n).step_by(i) {
                    is_prime.set(j, false);
                }
            }
        }

        is_prime
    }

    pub fn until(n: usize) -> Self {
        Self::until_with_sieve(n).0
    }

    pub fn until_with_sieve(n: usize) -> (Self, BitVec) {
        let is_prime = Self::from_sieve(n);
        let mut primes = Vec::new();

        primes.push(2);
        for i in (3..=n).step_by(2) {
            if is_prime.get(i).unwrap() {
                primes.push(i as u64);
            }
        }
        
        (Self { data: primes }, is_prime)
    }

    pub fn iter(&self) -> std::slice::Iter<'_, u64> {
        self.data.iter()
    }

    pub fn check(&self, n: u64) -> bool {
        if n < *self.data.last().unwrap() {
            self.data.binary_search(&n).is_ok()
        } else {
            let limit = (n as f64).sqrt() as u64;

            !self.iter().take_while(|&&p| p < limit).any(|&p| n % p == 0)
        }
        
    }

    pub fn len(&self) -> usize {
        self.data.len()
    }

    
}

pub type Factor<T> = (T, u32);

pub trait Factorize: Integer + FromPrimitive + Clone {
    fn factorize(&self) -> Vec<Factor<Self>>;
    fn factorize_with_prime(&self, primes: &Primes) -> Vec<Factor<Self>>;
    fn num_of_divisor(&self, primes: &Primes) -> u64;
    fn sum_of_divisor(&self) -> u64;
}

static FACTORIZE_INCREMENT: [usize; 8] = [4, 2, 4, 2, 4, 6, 2, 6];

impl Factorize for u64 {
    #[inline]
    fn factorize(&self) -> Vec<Factor<Self>> {
        let mut n = self.clone();
        let mut factors = Vec::new();

        for d in [2, 3, 5] {
            if n % d == 0 {
                let mut factor = (d, 0_u32);
                while n % d == 0 {
                    n /= d;
                    factor.1 += 1;
                }

                factors.push(factor);
            }            
        }

        let mut i = 0;
        let mut d = 7;
        while d * d <= n {
            if n % d == 0 {
                let mut factor = (d, 0_u32);
                while n % d == 0 {
                    n /= d;
                    factor.1 += 1;
                }

                factors.push(factor);
            }

            d += FACTORIZE_INCREMENT[i % 8] as u64;
            i += 1;
        }

        if n > 1 {
            factors.push((n, 1));
        }

        factors
    }

    fn factorize_with_prime(&self, primes: &Primes) -> Vec<Factor<Self>> {
        let mut n = self.clone();
        let mut factors = Vec::new();

        for p in primes.iter() {
            if n % p == 0 {
                let mut factor = (*p, 0_u32);
                while n % p == 0 {
                    n /= p;
                    factor.1 += 1;
                }

                factors.push(factor);
            }
        }

        factors
    }

    fn num_of_divisor(&self, primes: &Primes) -> u64 {
        let mut n = self.clone();
        let mut count = 1;

        for p in primes.iter() {
            if p * p * p > n {
                break;
            }

            let mut cnt = 1;
            while n % p == 0 {
                n /= p;
                cnt += 1;
            }

            count *= cnt;
        }

        if primes.check(n) {
            count *= 2;
        } else if primes.check(n.sqrt()) {
            count *= 3;
        } else if n != 1 {
            count *= 4;
        }
        
        count
    }

    fn sum_of_divisor(&self) -> u64 {
        if self == &0_u64 || self == &1_u64 {
            return self.clone()
        }

        self.factorize()
            .iter()
            .map(|(prime, exp)| {
                let denom = prime - 1;
                
                (prime.pow(exp + 1) - 1) / denom
            })
            .product::<u64>()
    }
}


