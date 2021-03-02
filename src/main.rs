use rayon::prelude::*;
use std::{collections::HashMap, time::Instant};

type Solution = fn() -> String;

mod data;
mod helpers;
use data::*;
use helpers::*;

fn main() {
  let solutions: Vec<(u16, Solution, &str)> = vec![
    (1, problem_1, "233168"),
    (2, problem_2, "4613732"),
    (3, problem_3, "6857"),
    (4, problem_4, "906609"),
    (5, problem_5, "232792560"),
    (6, problem_6, "25164150"),
    (7, problem_7, "104743"),
    (8, problem_8, "23514624000"),
    (9, problem_9, "31875000"),
    (10, problem_10, "142913828922"),
    (11, problem_11, "70600674"),
    (12, problem_12, "76576500"),
    (13, problem_13, "5537376230"),
    (14, problem_14, "837799"),
    (15, problem_15, "137846528820"),
    (16, problem_16, "1366"),
    (17, problem_17, ""),
  ];

  let sol_len = solutions.len();

  let g_start = Instant::now();

  let mut output: Vec<(u16, String, u128)> = solutions
    .into_par_iter()
    .map(|(name, solution, ans)| {
      let start = Instant::now();
      let res = solution();
      assert!(res == ans || ans == "");
      let elapsed = start.elapsed();

      let d = elapsed.as_millis();
      (name, res, d)
    })
    .collect();

  // let mut unsolved = 0;
  // for (name, solution, ans) in solutions {
  //   let start = Instant::now();

  //   let res = solution();
  //   assert!(res == ans || ans == "");
  //   let elapsed = start.elapsed();
  //   println!("{: >11} [ {:.4} sec ] | Answer -> {} ", name, elapsed.as_secs_f64(), res)
  // }

  //format!("{:?} = {} - {} seconds", name, res, d)

  let g_elapsed = g_start.elapsed();
  output.sort_by(|a, b| a.0.partial_cmp(&b.0).unwrap());

  let total_time: u128 = output.clone().into_iter().map(|o| o.2).sum();

  output.into_iter().for_each(|(name, result, time)| {
    let percent = ((time as f64 / total_time as f64) * 100.0).round();
    println!(
      "Problem {: >2} [ {: >4} ms | {: >3} % ] | Answer -> {} ",
      name, time, percent, result
    );
  });

  println!(
    "{} solutions in {:?} ms ({} ms raw time)",
    sol_len,
    g_elapsed.as_millis(),
    total_time
  );
}

fn problem_1() -> String {
  // find the sum of the multiples of 3 or 5 less than 1000
  let mut result = 0;

  for num in 1..1000 {
    if num % 3 == 0 || num % 5 == 0 {
      result += num;
    }
  }

  result.to_string()
}

fn problem_2() -> String {
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

  sum.to_string()
}

fn problem_3() -> String {
  // What is the largest prime factor of the number 600851475143
  const N: u64 = 600851475143;

  let lim: u64 = 600851475143_f64.sqrt().round() as u64;

  let primes = gen_primes(lim);

  for prime in primes.into_iter().rev() {
    if N % prime == 0 {
      return prime.to_string();
    }
  }
  return String::from("None?");
}

fn problem_4() -> String {
  // Find the largest palindrome made from the product of two 3-digit numbers
  let mut max_prod = 0;

  for i in 100..=999 {
    for j in 100..=999 {
      let prod = i * j;
      if is_palin(prod) && prod > max_prod {
        max_prod = prod;
      }
    }
  }
  max_prod.to_string()
}

fn problem_5() -> String {
  // What is the smallest positive number that is evenly divisible by all of the numbers from 1 to 20?
  let mut result = 1;
  for i in 1..=20 {
    result = lcm(result, i);
  }
  result.to_string()
}

fn problem_6() -> String {
  // Find the difference between the sum of the squares of the first one hundred natural numbers and the square of the sum.
  let mut sqr_sum = 0;
  let mut sum_sqr = 0;

  for i in 1..=100 {
    sqr_sum += i * i;
    sum_sqr += i;
  }

  sum_sqr = sum_sqr * sum_sqr;

  (sum_sqr - sqr_sum).to_string()
}

fn problem_7() -> String {
  // What is the 10 001st prime number?
  let mut prime_count = 0;
  let mut i = 2;
  loop {
    if is_prime(i) {
      prime_count += 1;
    }
    if prime_count == 10_001 {
      return i.to_string();
    }
    i += 1
  }
}

fn problem_8() -> String {
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
  max_prod.to_string()
}

fn problem_9() -> String {
  // There exists exactly one Pythagorean triplet for which a + b + c = 1000. Find the product abc.
  for a in 1..=1000 {
    for b in a + 1..=1000 {
      let c = 1000 - a - b;

      if a >= b || b >= c {
        continue;
      }
      if a + b + c == 1000 {
        if a * a + b * b == c * c {
          return (a * b * c).to_string();
        }
      }
    }
  }
  "<not found>".to_string()
}

fn problem_10() -> String {
  // Find the sum of all the primes below two million.
  gen_primes(2_000_000).into_iter().sum::<u64>().to_string()
}

fn problem_11() -> String {
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

  max_prod.to_string()
}

fn problem_12() -> String {
  // What is the value of the first triangle number to have over five hundred divisors?
  let mut num = 1;
  let mut next = 2;

  let mut max_f = 0;

  let primes = gen_primes(12_500);

  loop {
    // let f_count = factor_count(num);
    let f_count = factor_count_v2(num, &primes);

    if f_count > max_f {
      max_f = f_count;
    }

    if f_count > 500 {
      return num.to_string();
    }

    num += next;
    next += 1;
  }
}

fn problem_13() -> String {
  // Work out the first ten digits of the sum of the following one-hundred 50-digit numbers.
  let nums = PROBLEM_13_DATA;

  let mut sum = add_large_nums(nums[0].to_string(), nums[1].to_string());

  for i in 2..nums.len() {
    sum = add_large_nums(sum, nums[i].to_string());
  }

  sum[..10].to_string()
}

fn problem_14() -> String {
  // Which starting number, under one million, produces the longest chain?
  let mut max_chain = 0;
  let mut max_chain_num = 0;

  let mut chain_lens: HashMap<u64, u64> = HashMap::new();

  for num in 1..1_000_000 {
    let c_len = collatz_seq_len(num);

    if c_len > max_chain {
      max_chain = c_len;
      max_chain_num = num;
    }
  }

  max_chain_num.to_string()
}

fn problem_15() -> String {
  // How many such routes are there through a 20×20 grid?
  const N: u64 = 20;
  binomial_coefficient(2 * N, N).to_string()
}

fn problem_16() -> String {
  let mut val = String::from("1");
  let mut sum: u128 = 0;

  for _ in 0..=1000 {
    sum = val.bytes().map(|d| (d - 48) as u128).sum();
    val = add_large_nums(val.clone(), val);
  }

  sum.to_string()
}

fn problem_17() -> String {
  // If all the numbers from 1 to 1000 (one thousand) inclusive were written out in words, how many letters would be used?
  let mut total_letters = 0;

  for i in 1..=1000 {
    let word = num_to_word(i);
    total_letters += word.len();
  }

  total_letters.to_string()
}
