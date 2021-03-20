pub mod scholar;

#[cfg(test)]
mod tests {
    use crate::scholar;
    #[test]
    fn new_scholar_query() {
        let sc = scholar::ScholarArgs{
            query: "machine-learning",
            cite_id: None,
            from_year: None,
            to_year: None,
            sort_by: None,
            cluster_id: None,
            lang: None,
            lang_limit: None,
            limit: Some(3),
            offset: Some(0),
            adult_filtering: None,
            include_similar_results: None,
            include_citations: None,
        };
        assert_eq!(sc.query, "machine-learning");
    }
}
