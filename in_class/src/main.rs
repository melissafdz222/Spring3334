fn add(a: i32, b: i32) -> i32 {
    a + b
}

fn main() {
    println!("2 + 2 = {}", add(2, 2));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_add() {
        assert_eq!(add(2, 2), 4);
    }

    #[test]
    fn test_add_negative() {
    assert_eq!(add(-2, -2), -4);
    }

    #[test]
    fn test_add_zero() {
    assert_eq!(add(0, 0), 0);
    }
}