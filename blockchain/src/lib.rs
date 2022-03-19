// Setup code should go here. Maybe import to export?
mod block;
mod mine;
mod node;

pub static DIFFICULTY_PREFIX: &str = "00";

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        let result = 2 + 2;
        assert_eq!(result, 4);
    }
}
