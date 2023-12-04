pub mod globals;

use rand::random;

pub fn get_random_id() -> u16 {
    let mut t: u16 = random();

    while t == 0 {
        t = random();
    }

    t
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_get_random_id() {
        for _ in 1..11 {
            let id = get_random_id();
            assert!((1..u16::MAX).contains(&id));
        }
    }
}
