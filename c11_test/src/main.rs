

#[cfg(test)]
mod tests {
    #[test]
    fn two_plus_two_in_main() {
        // This test won't be run during "cargo test".
        // Explicitly running "cargo test two_plus_two_in_main" will run it.
        // This function can be invoked in main().
        assert_eq!(2 + 2, 4);
    }
}

fn main() {
    println!("Hello, world!");
}
