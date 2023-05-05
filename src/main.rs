use std::{io::Write, ops::SubAssign};

use rug::{ops::Pow, Integer};

pub struct LucasLehmer {
    pub n: Integer,
    value: Integer,
}

impl LucasLehmer {
    pub fn new(n: Integer) -> Self {
        LucasLehmer {
            n: n.clone(),
            value: Integer::from(4) % n,
        }
    }
}

impl Iterator for LucasLehmer {
    type Item = Integer;

    fn next(&mut self) -> Option<Self::Item> {
        let result = self.value.clone();
        self.value = self
            .value
            .clone()
            .pow_mod(&Integer::from(2), &self.n)
            .unwrap();
        self.value.sub_assign(2);
        Some(result)
    }
}

fn is_prime(n: u32) -> bool {
    for i in 2..(n as f64).sqrt().ceil() as u32 {
        if n % i == 0 {
            return false;
        }
    }
    return true;
}

fn ll_is_prime(e: u32) -> bool {
    if !is_prime(e) {
        return false;
    }

    let mut lucas_lehmer = LucasLehmer::new(Integer::from(2).pow(e) - 1);
    let mut acc = Integer::from(1);

    for _ in 0..(e - 1) {
        acc = lucas_lehmer.next().unwrap();
    }

    acc == Integer::new()
}

fn main() {
    let num_threads: usize = std::thread::available_parallelism().unwrap().into();
    println!("Available OS Threads: {num_threads}");

    let mut handles = Vec::with_capacity(num_threads);
    for t in 0..num_threads {
        handles.push(std::thread::spawn(move || {
            for x in 2u32..u32::MAX {
                let i = x * num_threads as u32 + t as u32 - 30;
                if i % 2 == 0 {
                    continue;
                }

                if ll_is_prime(i) {
                    let mut stdout = std::io::stdout().lock();
                    stdout
                        .write(
                            format!("2^{i} - 1 = {} is prime!\n\n", Integer::from(2).pow(i) - 1)
                                .as_bytes(),
                        )
                        .unwrap();
                }
            }
        }));
    }

    for h in handles {
        h.join().expect("Couldn't join thread!");
    }
}
