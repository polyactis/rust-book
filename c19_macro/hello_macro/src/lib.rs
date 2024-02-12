pub trait HelloMacroTrait {
    fn hello_macro();
}

pub fn add(left: usize, right: usize) -> usize {
    left + right
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn hello_macro_test() {
        let result = add(2, 2);
        assert_eq!(result, 4);
    }
}
