use rand::{self, Rng};

pub fn add(left: usize, right: usize) -> usize {
    left + right
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        assert_eq!(3, add_one(2));
    }
}

pub fn add_one(x: i32) -> i32 {
    x + 1
}

pub fn random_number_between_1_and_100() -> i32 {
    rand::thread_rng().gen_range(1..=100)
}
