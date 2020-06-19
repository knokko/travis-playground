fn main() {
    println!("Hello, world!");
}

#[cfg(test)]
mod tests {

  fn simple_test() {
    assert_eq!(5, 2 + 3);
  }
}
