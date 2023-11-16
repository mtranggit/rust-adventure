pub fn add(left: usize, right: usize) -> usize {
    left + right
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        assert_eq!(6, add_five(1));
    }
}

pub fn add_five(x: i32) -> i32 {
    x + 5
}
