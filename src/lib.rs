pub mod exercise_test; 

pub fn abc() {
    println!("abc");
    exercise_test::lib_test();
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        let result = 2 + 2;
        assert_eq!(result, 4)
    }
}