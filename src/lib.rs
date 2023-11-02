pub mod class;
pub mod executor;

pub fn add(left: usize, right: usize) -> usize {
    left + right
}
mod classloader;
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = add(2, 2);
        assert_eq!(result, 4);
    }
}
