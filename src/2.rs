  // Generate a random number between 1 and 10
  let mut rng = rand::thread_rng();
  let random_number = rng.gen_range(1..=10);
  
  // Print the random number to the console
  println!("Random Number: {}", random_number);
  