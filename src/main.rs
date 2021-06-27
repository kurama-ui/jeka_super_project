fn main() {
    println!("Hello from super project");
    println!("Super function: {}", get_four());
}

fn get_four() -> i32 {
    2 + 2
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn super_test() {
        assert_eq!(get_four(), 4);
    }
}
