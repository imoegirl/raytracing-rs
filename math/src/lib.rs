pub mod vector;

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn it_works() {
        let pos = vector::Vector3::default();

        assert_eq!(pos, vector::Vector3::new(0.1, 0.0, 0.0))
        
    }
}
