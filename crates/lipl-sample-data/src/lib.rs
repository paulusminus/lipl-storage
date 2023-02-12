include!(concat!(env!("OUT_DIR"), "/sample_data.rs"));

#[cfg(test)]
mod tests {
    #[test]
    fn test_daar_bij_die_molen() {
        let db = super::repo_db();
        assert_eq!(
            db.find_lyric_by_title("Daar bij die molen").unwrap().title,
            "Daar bij die molen".to_owned(),
        );
    }
}
