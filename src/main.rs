use rayon::prelude::*;
use std::{fmt::Display, time::Instant};

type Solution<T> = fn() -> T;
type SolutionResult<T> = Option<(u16, T, u128)>;

mod data;
mod helpers;
use data::*;
use helpers::*;

struct Problem<A: Display + Eq> {
  pub number: u16,
  pub answer: Option<A>,
  pub solution: Option<Solution<A>>,
}

impl<A: Display + Eq> Problem<A> {
  pub fn new(number: u16, answer: Option<A>, solution: Option<Solution<A>>) -> Self {
    Problem {
      number,
      answer,
      solution,
    }
  }

  fn time_solution(s: Solution<A>) -> (A, u128) {
    let start = Instant::now();
    let result = s();
    let elapsed = start.elapsed().as_millis();
    (result, elapsed)
  }

  pub fn run_timed(&self) -> SolutionResult<A> {
    if self.solution.is_none() {
      println!("No solution to run");
      return None;
    }

    let (result, elapsed) = Problem::time_solution(self.solution.unwrap());

    if let Some(ans) = &self.answer {
      if *ans != result {
        println!(
          "Problem {: >3}: Incorrect Answer: Expetced {} found {}",
          self.number, ans, result
        );
      }
    } else {
      println!("Problem {: >3}: No answer given to check.", self.number);
    }

    Some((self.number, result, elapsed))
  }
}

fn main() {
  let solutions: Vec<Problem<u64>> = vec![
    Problem::new(1, Some(233168), Some(problem_1)),
    Problem::new(2, Some(4613732), Some(problem_2)),
    Problem::new(3, Some(6857), Some(problem_3)),
    Problem::new(4, Some(906609), Some(problem_4)),
    Problem::new(5, Some(232792560), Some(problem_5)),
    Problem::new(6, Some(25164150), Some(problem_6)),
    Problem::new(7, Some(104743), Some(problem_7)),
    Problem::new(8, Some(23514624000), Some(problem_8)),
    Problem::new(9, Some(31875000), Some(problem_9)),
    Problem::new(10, Some(142913828922), Some(problem_10)),
    Problem::new(11, Some(70600674), Some(problem_11)),
    Problem::new(12, Some(76576500), Some(problem_12)),
    Problem::new(13, Some(5537376230), Some(problem_13)),
    Problem::new(14, Some(837799), Some(problem_14)),
    Problem::new(15, Some(137846528820), Some(problem_15)),
    Problem::new(16, Some(1366), Some(problem_16)),
    Problem::new(17, Some(21124), Some(problem_17)),
    Problem::new(18, Some(1074), Some(problem_18)),
  ];

  let sol_len = solutions.len();

  let g_start = Instant::now();
  println!("-----------------------------------------------");
  let output: Vec<SolutionResult<_>> = solutions
    .into_par_iter()
    .map(|problem| problem.run_timed())
    .collect();
  println!("-----------------------------------------------");
  let g_elapsed = g_start.elapsed();

  let mut success_res: Vec<(u16, _, u128)> = output
    .into_iter()
    .filter(|r| r.is_some())
    .map(|r| r.unwrap())
    .collect();

  success_res.sort_by(|a, b| (a.0).partial_cmp(&b.0).unwrap());

  let total_time: u128 = success_res.iter().map(|o| o.2).sum();

  println!("| Problem   # | Time ms |  ms % | Result ");
  println!("-----------------------------------------------");
  success_res.iter().for_each(|(number, result, duration)| {
    let percent = ((*duration as f64 / total_time as f64) * 100.0).round();
    println!(
      "| Problem {: >3} | {: >4} ms | {: >3} % | {: >12}",
      number, duration, percent, result
    );
  });
  println!("-----------------------------------------------");

  println!(
    "{} solutions in {:?} ms ({} ms raw time)",
    sol_len,
    g_elapsed.as_millis(),
    total_time
  );
}

fn problem_1() -> u64 {
  // find the sum of the multiples of 3 or 5 less than 1000
  let mut result = 0;

  for num in 1..1000 {
    if num % 3 == 0 || num % 5 == 0 {
      result += num;
    }
  }

  result
}

fn problem_2() -> u64 {
  // find the sum of the even valued fibonacci numbers whose values do not exceed 4_000_000
  let mut a = 1;
  let mut b = 2;
  let mut c = a + b;
  let mut sum = 2;

  while c < 4_000_000 {
    a = b;
    b = c;
    c = a + b;
    if c % 2 == 0 {
      sum += c;
    }
  }

  sum
}

fn problem_3() -> u64 {
  // What is the largest prime factor of the number 600851475143
  const N: u64 = 600851475143;

  let mut lim: u64 = 600851475143_f64.sqrt().round() as u64;

  if lim % 2 == 0 {
    lim += 1;
  }

  while lim > 1 {
    if N % lim == 0 && is_prime(lim) {
      return lim;
    }
    lim -= 2;
  }
  u64::MAX
}

fn problem_4() -> u64 {
  // Find the largest palindrome made from the product of two 3-digit numbers

  let mut max_prod = 0;

  for a in 1..=999 {
    for b in 1..=999 {
      let prod = a * b;
      if prod > max_prod && is_palin(prod) {
        max_prod = prod;
      }
    }
  }
  max_prod
}

fn problem_5() -> u64 {
  // What is the smallest positive number that is evenly divisible by all of the numbers from 1 to 20?
  let mut result = 1;
  for i in 1..=20 {
    result = lcm(result, i);
  }
  result
}

fn problem_6() -> u64 {
  // Find the difference between the sum of the squares of the first one hundred natural numbers and the square of the sum.
  let mut sqr_sum = 0;
  let mut sum_sqr = 0;

  for i in 1..=100 {
    sqr_sum += i * i;
    sum_sqr += i;
  }

  sum_sqr = sum_sqr * sum_sqr;

  sum_sqr - sqr_sum
}

fn problem_7() -> u64 {
  // What is the 10 001st prime number?
  let mut prime_count = 0;
  let mut i = 1;
  loop {
    if is_prime(i) {
      prime_count += 1;
    }
    if prime_count == 10_001 {
      return i as u64;
    }
    i += 2
  }
}

fn problem_8() -> u64 {
  // Find the thirteen adjacent digits in the 1000-digit number that have the greatest product. What is the value of this product?
  let kilo_dig = PROBLEM_8_DATA;

  let mut max_prod = 0;
  let lim = kilo_dig.len() - 12;
  for i in 0..lim {
    let prod = &kilo_dig[i..i + 13]
      .as_bytes()
      .into_iter()
      .fold::<u64, _>(1, |a, b| a * ((*b - 48) as u64));

    if *prod > max_prod {
      max_prod = *prod;
    }
  }
  max_prod
}

fn problem_9() -> u64 {
  // There exists exactly one Pythagorean triplet for which a + b + c = 1000. Find the product abc.
  for a in 1..=1000 {
    for b in a + 1..=1000 {
      let c = 1000 - a - b;

      if a >= b || b >= c {
        continue;
      }
      if a + b + c == 1000 {
        if a * a + b * b == c * c {
          return a * b * c;
        }
      }
    }
  }
  0
}

fn problem_10() -> u64 {
  // Find the sum of all the primes below two million.
  gen_primes(2_000_000).into_iter().sum::<u64>()
}

fn problem_11() -> u64 {
  // What is the greatest product of four adjacent numbers in the same direction (up, down, left, right, or diagonally) in the 20×20 grid?
  let grid = PROBLEM_11_DATA;

  let mut max_prod = 0;

  let slice_product = |acc: u64, n: &u8| acc * (*n as u64);

  // horizontal
  for i in 0..grid.len() - 4 {
    let prod = grid[i..i + 4].into_iter().fold(1, slice_product);
    if prod > max_prod {
      max_prod = prod;
    }
  }
  // vertical
  for i in 0..grid.len() - 60 {
    let prod = vec![&grid[i], &grid[i + 20], &grid[i + 40], &grid[i + 60]]
      .into_iter()
      .fold(1, slice_product);

    if prod > max_prod {
      max_prod = prod;
    }
  }
  // diagonally \
  for i in 0..grid.len() - 63 {
    let prod = vec![&grid[i], &grid[i + 21], &grid[i + 42], &grid[i + 63]]
      .into_iter()
      .fold(1, slice_product);

    if prod > max_prod {
      max_prod = prod;
    }
  }
  // diagonally /
  for i in 0..grid.len() - 63 {
    let prod = vec![&grid[i], &grid[i + 19], &grid[i + 38], &grid[i + 57]]
      .into_iter()
      .fold(1, slice_product);

    if prod > max_prod {
      max_prod = prod;
    }
  }

  max_prod
}

fn problem_12() -> u64 {
  // What is the value of the first triangle number to have over five hundred divisors?
  // let mut num = 1;
  // let mut next = 2;

  // let mut max_f = 0;

  let primes = gen_primes(12_500);

  // loop {
  //   // let f_count = factor_count(num);
  //   let f_count = factor_count_v2(num, &primes);

  //   if f_count > max_f {
  //     max_f = f_count;
  //   }

  //   if f_count > 500 {
  //     return num;
  //   }

  //   num += next;
  //   next += 1;
  // }

  let tn = |n: u64| ((n as f64 * 0.5) * (n + 1) as f64).round() as u64;

  tn(
    (1..12_500u64)
      .into_par_iter()
      .find_first(|&n| factor_count_v2(tn(n), &primes) >= 500)
      .unwrap(),
  )
}

fn problem_13() -> u64 {
  // Work out the first ten digits of the sum of the following one-hundred 50-digit numbers.
  let nums = PROBLEM_13_DATA;

  let mut sum = add_large_nums(nums[0].to_string(), nums[1].to_string());

  for i in 2..nums.len() {
    sum = add_large_nums(sum, nums[i].to_string());
  }

  sum[..10].parse::<u64>().unwrap()
}

fn problem_14() -> u64 {
  // Which starting number, under one million, produces the longest chain?
  (1..1_000_000_u64)
    .into_par_iter()
    .map(|sn| (sn, collatz_seq_len(sn)))
    .max_by(|a, b| a.1.cmp(&b.1))
    .unwrap()
    .0
}

fn problem_15() -> u64 {
  // How many such routes are there through a 20×20 grid?
  const N: u64 = 20;
  binomial_coefficient(2 * N, N)
}

fn problem_16() -> u64 {
  let mut val = String::from("1");
  let mut sum: u64 = 0;

  for _ in 0..=1000 {
    sum = val.bytes().map(|d| (d - 48) as u64).sum();
    val = add_large_nums(val.clone(), val);
  }

  sum
}

fn problem_17() -> u64 {
  // If all the numbers from 1 to 1000 (one thousand) inclusive were written out in words, how many letters would be used?
  let mut total_letters = 0;

  for i in 1..=1000 {
    let word = num_to_word(i);
    total_letters += word.len();
  }

  total_letters as u64
}

/// returns the number of rows in a triangular array of given length
fn n(t: usize) -> usize {
  let t8 = 8.0 * t as f64;
  (0.5 * ((t8 + 1.0).sqrt() - 1.0)).round() as usize
}

fn find_path(mut arr: Vec<u64>) -> u64 {
  let last_row_len = n(arr.len());
  let mut row_len = last_row_len;

  let mut idx_start = arr.len() - last_row_len;
  let mut idx_end = idx_start + row_len;

  while idx_start > 0 {
    let mut prev_row_idx = idx_start - row_len + 1;

    for i in idx_start..idx_end - 1 {
      let max = arr[i].max(arr[i + 1]);
      let new_val = arr[prev_row_idx] + max;

      // let _ = std::mem::replace(&mut arr[prev_row_idx], new_val);
      arr[prev_row_idx] = new_val;

      prev_row_idx += 1;
    }

    row_len -= 1;
    idx_start -= row_len;
    idx_end = idx_start + row_len;
  }

  arr[0]
}

fn problem_18() -> u64 {
  find_path(PROBLEM_18_DATA.to_vec())
}
