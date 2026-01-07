use grikod::ikili_2_gri_kod;

fn main() {
    let uzun_65 = "1".repeat(65);

    let testler = [
        "101010",   // → 111111
        "1100",     // → 1010
        "11111111", // → 10000000
        "",         // HATA
        "12a3",     // HATA
    ];

    println!("GriKod v0.1.0 - Testler");
    println!("========================");

    for &test in testler.iter() {
        match ikili_2_gri_kod(test) {
            Ok(sonuc) => println!("'{}' → {}", test, sonuc),
            Err(e) => println!("'{}' → {}", test, e),
        }
    }

    println!("\n📏 Uzun test: '{}'", &uzun_65[..65]);
    println!("Sonuç: {}", ikili_2_gri_kod(&uzun_65[..64]).unwrap().len());
}
