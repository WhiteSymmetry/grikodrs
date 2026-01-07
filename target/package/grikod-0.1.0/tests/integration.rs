#[cfg(test)]
mod tests {
    use grikod::ikili_2_gri_kod;

    #[test]
    fn test_basit_donusumler() {
        assert_eq!(ikili_2_gri_kod("0"), Ok("0".to_string()));
        assert_eq!(ikili_2_gri_kod("1"), Ok("1".to_string()));
        assert_eq!(ikili_2_gri_kod("10"), Ok("11".to_string()));
        assert_eq!(ikili_2_gri_kod("101010"), Ok("111111".to_string()));
    }

    #[test]
    fn test_hata_durumlari() {
        assert!(ikili_2_gri_kod("").is_err());
        assert!(ikili_2_gri_kod("12a3").is_err());
        assert!(ikili_2_gri_kod(&"1".repeat(65)).is_err());
    }

    #[test]
    fn test_uzun_stringler() {
        let uzun = "1010101010101010101010101010101010101010101010101010101010101010";
        let sonuc = ikili_2_gri_kod(uzun).unwrap();
        assert_eq!(sonuc.len(), uzun.len());
    }
}
