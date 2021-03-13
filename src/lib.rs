pub mod scholar;

#[cfg(test)]
mod tests {
    use crate::scholar;
    #[test]
    fn new_scholar_query() {
        let sc = scholar::new(
            "abcd", None, None, None, None, None, 
            None, None, None, None, None, None, None);
        assert_eq!(sc.query, "abcd");
    }
}
