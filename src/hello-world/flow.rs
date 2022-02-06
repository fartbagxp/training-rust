fn main() {
  for n in 1..200 {
    if n % 15 == 0 {
      println!("fizzbuzz");
    } else if n % 3 == 0 {
      println!("fizz");
    } else if n % 5 == 0 {
      println!("buzz");
    } else {
      println!("{}", n);
    }
  }

  let mut n = 1u32;
  while n < 15 {
    println!("{}", n);
    n=n+1;
  }
}
