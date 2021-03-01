use std::time::Instant;

type Solution = fn() -> String;

fn main() {
  let solutions: Vec<(&str, Solution)> = vec![
    ("Problem 1", problem_1),
    ("Problem 2", problem_2),
    ("Problem 3", problem_3),
    ("Problem 4", problem_4),
    ("Problem 5", problem_5),
    ("Problem 6", problem_6),
    ("Problem 7", problem_7),
    ("Problem 8", problem_8),
    ("Problem 9", problem_9),
    ("Problem 10", problem_10),
    ("Problem 11", problem_11),
  ];

  let sol_len = solutions.len();

  let g_start = Instant::now();
  for (name, solution) in solutions {
    let start = Instant::now();

    let res = solution();
    let elapsed = start.elapsed();
    println!("{:?} = {} - {} seconds", name, res, elapsed.as_secs_f64())
  }
  let g_elapsed = g_start.elapsed();
  println!("{} solutions in {:?} seconds", sol_len, g_elapsed.as_secs_f64())
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

fn is_prime(num: u64) -> bool {
  if num == 2 || num == 3 {
    return true;
  }

  let mod6 = num % 6;
  if num % 2 == 0 || (mod6 != 1 && mod6 != 5) {
    return false;
  }

  let mut i = 3;
  while i * i <= num {
    if num % i == 0 {
      return false;
    }
    i += 2;
  }
  true
}

fn gen_primes(lim: u64) -> Vec<u64> {
  let mut result = Vec::new();
  for num in 2..lim {
    if is_prime(num) {
      result.push(num);
    }
  }
  result
}

fn problem_3() -> String {
  // What is the largest prime factor of the number 600851475143
  let lim: u64 = 600851475143_f64.sqrt().round() as u64;
  let primes = gen_primes(lim);

  const N: u64 = 600851475143;

  for prime in primes.into_iter().rev() {
    if N % prime == 0 {
      return prime.to_string();
    }
  }
  return String::from("None?");
}

fn is_palin<T: ToString>(s: T) -> bool {
  let s = s.to_string();

  let len = s.len();
  let mid = len >> 1;

  let first_half = s[..=mid].as_bytes();
  let second_half = s[mid..].as_bytes();

  for i in 0..mid {
    if first_half[i] != second_half[mid - (i + 1)] {
      return false;
    }
  }
  true
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

fn gcd(a: i128, b: i128) -> i128 {
  if b == 0 {
    return a;
  }

  gcd(b, a % b)
}

fn lcm(a: i128, b: i128) -> i128 {
  let abs_ab = (a * b).abs();
  abs_ab / gcd(a, b)
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
  const KILO_DIG: &str = "7316717653133062491922511967442657474235534919493496983520312774506326239578318016984801869478851843858615607891129494954595017379583319528532088055111254069874715852386305071569329096329522744304355766896648950445244523161731856403098711121722383113622298934233803081353362766142828064444866452387493035890729629049156044077239071381051585930796086670172427121883998797908792274921901699720888093776657273330010533678812202354218097512545405947522435258490771167055601360483958644670632441572215539753697817977846174064955149290862569321978468622482839722413756570560574902614079729686524145351004748216637048440319989000889524345065854122758866688116427171479924442928230863465674813919123162824586178664583591245665294765456828489128831426076900422421902267105562632111110937054421750694165896040807198403850962455444362981230987879927244284909188845801561660979191338754992005240636899125607176060588611646710940507754100225698315520005593572972571636269561882670428252483600823257530420752963450";

  let mut max_prod = 0;
  let lim = KILO_DIG.len() - 12;
  for i in 0..lim {
    let prod = &KILO_DIG[i..i + 13]
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
      for c in b + 1..=1000 {
        if a >= b || b >= c {
          continue;
        } else if a * a + b * b == c * c {
          if a + b + c == 1000 {
            return (a * b * c).to_string();
          }
        }
      }
    }
  }
  "<not found>".to_string()
}

fn problem_10() -> String {
  // Find the sum of all the primes below two million.
  let mut prime_sum = 0;
  for num in 2..2_000_000 {
    if is_prime(num) {
      prime_sum += num;
    }
  }
  prime_sum.to_string()
}

mod matrix;
use matrix::Matrix;

fn problem_11() -> String {
  // What is the greatest product of four adjacent numbers in the same direction (up, down, left, right, or diagonally) in the 20×20 grid?
  let matrix: Matrix<u8> = Matrix::from_vec(
    20,
    vec![
      08, 02, 22, 97, 38, 15, 00, 40, 00, 75, 04, 05, 07, 78, 52, 12, 50, 77, 91, 08, 49, 49, 99, 40, 17, 81, 18, 57, 60, 87, 17, 40, 98,
      43, 69, 48, 04, 56, 62, 00, 81, 49, 31, 73, 55, 79, 14, 29, 93, 71, 40, 67, 53, 88, 30, 03, 49, 13, 36, 65, 52, 70, 95, 23, 04, 60,
      11, 42, 69, 24, 68, 56, 01, 32, 56, 71, 37, 02, 36, 91, 22, 31, 16, 71, 51, 67, 63, 89, 41, 92, 36, 54, 22, 40, 40, 28, 66, 33, 13,
      80, 24, 47, 32, 60, 99, 03, 45, 02, 44, 75, 33, 53, 78, 36, 84, 20, 35, 17, 12, 50, 32, 98, 81, 28, 64, 23, 67, 10, 26, 38, 40, 67,
      59, 54, 70, 66, 18, 38, 64, 70, 67, 26, 20, 68, 02, 62, 12, 20, 95, 63, 94, 39, 63, 08, 40, 91, 66, 49, 94, 21, 24, 55, 58, 05, 66,
      73, 99, 26, 97, 17, 78, 78, 96, 83, 14, 88, 34, 89, 63, 72, 21, 36, 23, 09, 75, 00, 76, 44, 20, 45, 35, 14, 00, 61, 33, 97, 34, 31,
      33, 95, 78, 17, 53, 28, 22, 75, 31, 67, 15, 94, 03, 80, 04, 62, 16, 14, 09, 53, 56, 92, 16, 39, 05, 42, 96, 35, 31, 47, 55, 58, 88,
      24, 00, 17, 54, 24, 36, 29, 85, 57, 86, 56, 00, 48, 35, 71, 89, 07, 05, 44, 44, 37, 44, 60, 21, 58, 51, 54, 17, 58, 19, 80, 81, 68,
      05, 94, 47, 69, 28, 73, 92, 13, 86, 52, 17, 77, 04, 89, 55, 40, 04, 52, 08, 83, 97, 35, 99, 16, 07, 97, 57, 32, 16, 26, 26, 79, 33,
      27, 98, 66, 88, 36, 68, 87, 57, 62, 20, 72, 03, 46, 33, 67, 46, 55, 12, 32, 63, 93, 53, 69, 04, 42, 16, 73, 38, 25, 39, 11, 24, 94,
      72, 18, 08, 46, 29, 32, 40, 62, 76, 36, 20, 69, 36, 41, 72, 30, 23, 88, 34, 62, 99, 69, 82, 67, 59, 85, 74, 04, 36, 16, 20, 73, 35,
      29, 78, 31, 90, 01, 74, 31, 49, 71, 48, 86, 81, 16, 23, 57, 05, 54, 01, 70, 54, 71, 83, 51, 54, 69, 16, 92, 33, 48, 61, 43, 52, 01,
      89, 19, 67, 48,
    ],
  );
  // horizontal

  println!("{:?}", matrix[(19, 19)]);
  "".to_string()
}
