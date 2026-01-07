//! # GriKod
//! A safe Rust library implementing the standard Gray code conversion.
//! Standart Gray kod dönüşümünü uygulayan güvenli bir Rust kütüphanesi.

use std::error::Error;
use std::fmt;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum InvalidBinaryError {
    BosGiris,
    GecersizKarakter,
    FazlaUzun,
    DonusturmeHatasi,
}

impl fmt::Display for InvalidBinaryError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::BosGiris => write!(f, "Hata: Giriş boş olamaz."),
            Self::GecersizKarakter => write!(f, "Hata: Sadece '0' ve '1' kullanın."),
            Self::FazlaUzun => write!(f, "Hata: Maksimum 64 bit."),
            Self::DonusturmeHatasi => write!(f, "Hata: İkili parse edilemedi."),
        }
    }
}

impl Error for InvalidBinaryError {}

/// Orijinal Python koduna **tam eşdeğer**
pub fn ikili_2_gri_kod(i2grik: &str) -> Result<String, InvalidBinaryError> {
    if i2grik.is_empty() {
        return Err(InvalidBinaryError::BosGiris);
    }

    if !i2grik.chars().all(|c| c == '0' || c == '1') {
        return Err(InvalidBinaryError::GecersizKarakter);
    }

    if i2grik.len() > 64 {
        return Err(InvalidBinaryError::FazlaUzun);
    }

    // ✔️ DOĞRU: binary parse
    let n = u64::from_str_radix(i2grik, 2).map_err(|_| InvalidBinaryError::DonusturmeHatasi)?;

    let gray = n ^ (n >> 1);

    // ✔️ KIRPMA YOK, SADECE GEREKİRSE SOLA SIFIR EKLE
    let mut out = format!("{:b}", gray);
    while out.len() < i2grik.len() {
        out.insert(0, '0');
    }

    Ok(out)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_basit_donusumler() {
        assert_eq!(ikili_2_gri_kod("0"), Ok("0".to_string()));
        assert_eq!(ikili_2_gri_kod("1"), Ok("1".to_string()));
        assert_eq!(ikili_2_gri_kod("10"), Ok("11".to_string()));
        assert_eq!(ikili_2_gri_kod("11"), Ok("10".to_string()));
        assert_eq!(ikili_2_gri_kod("1010"), Ok("1111".to_string()));
        assert_eq!(ikili_2_gri_kod("101010"), Ok("111111".to_string()));
        assert_eq!(ikili_2_gri_kod("1100"), Ok("1010".to_string()));
    }

    #[test]
    fn test_hata_durumlari() {
        assert!(ikili_2_gri_kod("").is_err());
        assert!(ikili_2_gri_kod("12a3").is_err());
        assert!(ikili_2_gri_kod("abc").is_err());
        let uzun = "1".repeat(65);
        assert!(ikili_2_gri_kod(&uzun).is_err());
    }

    #[test]
    fn test_uzun_string_zerofill() {
        let input = "10101010101010101010"; // 20 chars
        let sonuc = ikili_2_gri_kod(input).unwrap();
        assert_eq!(sonuc.len(), input.len());
    }
}
