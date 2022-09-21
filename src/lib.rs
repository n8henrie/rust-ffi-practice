include!(concat!(env!("OUT_DIR"), "/bindings.rs"));

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_main() {
        let result = unsafe { barfunc() };
        assert_eq!(result, 42);
    }
}
