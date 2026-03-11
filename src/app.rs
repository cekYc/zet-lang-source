
#![allow(dead_code, unused_imports, unused_variables, unused_parens, unused_mut, non_snake_case)]
use std::time::Duration;
use std::io::{self, Write};
use serde_json::Value;

const RESET: &str = "\x1b[0m";
const CYAN: &str = "\x1b[36m";   
const GREEN: &str = "\x1b[32m";  
const MAGENTA: &str = "\x1b[35m";
const YELLOW: &str = "\x1b[33m"; 
const BLUE: &str = "\x1b[34m";
const RED: &str = "\x1b[31m";

/// Zet v0.3 — Untrusted: Dış dünyadan gelen lekeli veri sarmalayıcısı.
#[derive(Clone, Debug)]
struct Untrusted(String);

impl Untrusted {
    fn validate(self) -> Result<String, String> {
        let s = self.0.trim().to_string();
        if s.is_empty() {
            Err("Dogrulama basarisiz: bos girdi.".to_string())
        } else {
            Ok(s)
        }
    }
}

struct Console;
impl Console {
    async fn read(prompt: String) -> Untrusted {
        print!("  {}[Console] {}: {}", BLUE, prompt, RESET);
        io::stdout().flush().unwrap();
        let mut buffer = String::new();
        io::stdin().read_line(&mut buffer).unwrap();
        Untrusted(buffer.trim().to_string())
    }
}

struct DB;
impl DB {
    async fn log(msg: String) {
        println!("  {}[LOG] {}{}", GREEN, msg, RESET);
    }
}

struct Util;
impl Util {
    #[inline(always)]
    async fn to_int(s: String) -> i64 { s.trim().parse::<i64>().unwrap_or(0) }
    #[inline(always)]
    async fn to_float(s: String) -> f64 { s.trim().parse::<f64>().unwrap_or(0.0) }
    #[inline(always)]
    async fn now() -> i64 {
        std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .unwrap()
            .as_millis() as i64
    }
}

struct HTTP;
impl HTTP {
    async fn get(url: String) -> Untrusted {
        let client = reqwest::Client::builder().user_agent("ZetLang/0.3").build().unwrap();
        match client.get(&url).send().await {
            Ok(res) => Untrusted(res.text().await.unwrap_or_else(|e| format!("Error: {}", e))),
            Err(e) => Untrusted(format!("Error: {}", e))
        }
    }
}

async fn input(prompt: String) -> Untrusted {
    print!("  {}[>] {}{}", CYAN, prompt, RESET);
    io::stdout().flush().unwrap();
    let mut buffer = String::new();
    let _ = io::stdin().read_line(&mut buffer);
    Untrusted(buffer.trim().to_string())
}

async fn inputln(prompt: String) -> Untrusted {
    println!("  {}[>] {}{}", CYAN, prompt, RESET);
    io::stdout().flush().unwrap();
    let mut buffer = String::new();
    let _ = io::stdin().read_line(&mut buffer);
    Untrusted(buffer.trim().to_string())
}

// ─── ZetAdd Trait (Polimorfik Toplama) ──────────────────────────
trait ZetAdd<Rhs> { type Output; fn z_add(self, rhs: Rhs) -> Self::Output; }
impl ZetAdd<i64> for i64 { type Output = i64; #[inline(always)] fn z_add(self, rhs: i64) -> i64 { self + rhs } }
impl ZetAdd<f64> for f64 { type Output = f64; #[inline(always)] fn z_add(self, rhs: f64) -> f64 { self + rhs } }
impl ZetAdd<String> for String { type Output = String; #[inline(always)] fn z_add(self, rhs: String) -> String { self + &rhs } }
impl<'a> ZetAdd<&'a str> for String { type Output = String; #[inline(always)] fn z_add(self, rhs: &'a str) -> String { self + rhs } }
impl ZetAdd<i64> for String { type Output = String; #[inline(always)] fn z_add(self, rhs: i64) -> String { format!("{}{}", self, rhs) } }
impl ZetAdd<f64> for String { type Output = String; #[inline(always)] fn z_add(self, rhs: f64) -> String { format!("{}{}", self, rhs) } }
impl ZetAdd<bool> for String { type Output = String; #[inline(always)] fn z_add(self, rhs: bool) -> String { format!("{}{}", self, rhs) } }
impl ZetAdd<char> for String { type Output = String; #[inline(always)] fn z_add(self, rhs: char) -> String { format!("{}{}", self, rhs) } }

// ─── ZetMul Trait (Polimorfik Çarpma) ───────────────────────────
trait ZetMul<Rhs> { type Output; fn z_mul(self, rhs: Rhs) -> Self::Output; }
impl ZetMul<i64> for i64 { type Output = i64; #[inline(always)] fn z_mul(self, rhs: i64) -> i64 { self * rhs } }
impl ZetMul<f64> for f64 { type Output = f64; #[inline(always)] fn z_mul(self, rhs: f64) -> f64 { self * rhs } }
impl ZetMul<i64> for String { type Output = String; fn z_mul(self, rhs: i64) -> String { self.repeat(rhs as usize) } }
impl<'a> ZetMul<i64> for &'a str { type Output = String; fn z_mul(self, rhs: i64) -> String { self.repeat(rhs as usize) } }
fn topla(a: i64, b: i64) -> i64 {
    return a.clone().z_add(b.clone());
}

fn cikar(a: i64, b: i64) -> i64 {
    return (a - b);
}

fn carp(a: i64, b: i64) -> i64 {
    return a.clone().z_mul(b.clone());
}

fn bol(a: i64, b: i64) -> i64 {
    return (a / b);
}

fn esit_mi(a: i64, b: i64) -> i64 {
    if (a == b) {
        return 1;
    }
    return 0;
}

fn farkli_mi(a: i64, b: i64) -> i64 {
    if (a != b) {
        return 1;
    }
    return 0;
}

fn buyuk_mu(a: i64, b: i64) -> i64 {
    if (a > b) {
        return 1;
    }
    return 0;
}

fn kucuk_mu(a: i64, b: i64) -> i64 {
    if (a < b) {
        return 1;
    }
    return 0;
}

fn buyuk_esit_mi(a: i64, b: i64) -> i64 {
    if (a >= b) {
        return 1;
    }
    return 0;
}

fn kucuk_esit_mi(a: i64, b: i64) -> i64 {
    if (a <= b) {
        return 1;
    }
    return 0;
}

fn fibonacci(n: i64) -> i64 {
    if (n <= 1) {
        return n;
    }
    let mut a = fibonacci((n - 1));
    let mut b = fibonacci((n - 2));
    return a.clone().z_add(b.clone());
}

fn faktoriyel(n: i64) -> i64 {
    if (n <= 1) {
        return 1;
    }
    return n.clone().z_mul(faktoriyel((n - 1)));
}

fn us_al(taban: i64, us: i64) -> i64 {
    if (us == 0) {
        return 1;
    }
    return taban.clone().z_mul(us_al(taban.clone(), (us - 1)));
}

fn gcd(a: i64, b: i64) -> i64 {
    if (b == 0) {
        return a;
    }
    let mut kalan = (a - (a / b).z_mul(b.clone()));
    return gcd(b.clone(), kalan.clone());
}

fn mutlak_deger(n: i64) -> i64 {
    if (n < 0) {
        return (0 - n);
    } else {
        return n;
    }
}

fn maks(a: i64, b: i64) -> i64 {
    if (a >= b) {
        return a;
    }
    return b;
}

fn min_deger(a: i64, b: i64) -> i64 {
    if (a <= b) {
        return a;
    }
    return b;
}

fn isaret(n: i64) -> i64 {
    if (n > 0) {
        return 1;
    } else {
        if (n < 0) {
            return (0 - 1);
        } else {
            return 0;
        }
    }
}

fn siniflandir(puan: i64) -> i64 {
    if (puan >= 90) {
        return 5;
    } else {
        if (puan >= 80) {
            return 4;
        } else {
            if (puan >= 70) {
                return 3;
            } else {
                if (puan >= 60) {
                    return 2;
                } else {
                    return 1;
                }
            }
        }
    }
}

fn det_print_testi() -> () {
    println!("{}", "  [DET] print/println det fonksiyonda calisiyor!".to_string());
    print!("{}", "  [DET] Bu satirda ".to_string());
    println!("{}", "devam ediyor.".to_string());
}

fn selamla(isim: String) -> String {
    return "Merhaba, ".to_string().z_add(isim.clone());
}

fn tekrarla_sayi(n: i64) -> String {
    return "Sayi: ".to_string().z_add(n.clone());
}

fn kare_al(n: i64) -> i64 {
    return n.clone().z_mul(n.clone());
}

fn kup_al(n: i64) -> i64 {
    return n.clone().z_mul(n.clone()).z_mul(n.clone());
}

fn ortalama_iki(a: i64, b: i64) -> i64 {
    return (a.clone().z_add(b.clone()) / 2);
}

fn asal_mi(n: i64) -> i64 {
    if (n <= 1) {
        return 0;
    }
    if (n <= 3) {
        return 1;
    }
    let mut kalan2 = (n - (n / 2).z_mul(2));
    if (kalan2 == 0) {
        return 0;
    }
    let mut kalan3 = (n - (n / 3).z_mul(3));
    if (kalan3 == 0) {
        return 0;
    }
    let mut kalan5 = (n - (n / 5).z_mul(5));
    if (kalan5 == 0) {
        if (n != 5) {
            return 0;
        }
    }
    let mut kalan7 = (n - (n / 7).z_mul(7));
    if (kalan7 == 0) {
        if (n != 7) {
            return 0;
        }
    }
    return 1;
}

fn karmasik_hesap(x: i64) -> i64 {
    let mut a = topla(x.clone(), 10);
    let mut b = carp(a.clone(), 2);
    let mut c = cikar(b.clone(), 5);
    return c;
}

fn zincirleme(n: i64) -> i64 {
    return topla(carp(n.clone(), 3), cikar(n.clone(), 1));
}

async fn zaman_damgasi() -> i64 {
    let mut t = Util::now().await;
    return t;
}

async fn hesap_ve_yazdir() -> () {
    let mut sonuc = topla(100, 200);
    println!("{}", "  [NONDET] Det fonksiyon sonucu: ".to_string().z_add(sonuc.clone()));
    let mut fib_sonuc = fibonacci(10);
    println!("{}", "  [NONDET] Fibonacci(10) = ".to_string().z_add(fib_sonuc.clone()));
    let mut fak_sonuc = faktoriyel(6);
    println!("{}", "  [NONDET] 6! = ".to_string().z_add(fak_sonuc.clone()));
}

async fn zaman_olc() -> i64 {
    let mut baslangic = Util::now().await;
    let mut bitis = Util::now().await;
    return (bitis - baslangic);
}

async fn metin_to_sayi() -> () {
    let mut sayi = Util::to_int("42".to_string()).await;
    println!("{}", "  [NONDET] String -> i64 donusumu: ".to_string().z_add(sayi.clone()));
    let mut sayi2 = Util::to_int("100".to_string()).await;
    let mut toplam = sayi.clone().z_add(sayi2.clone());
    println!("{}", "  [NONDET] 42 + 100 = ".to_string().z_add(toplam.clone()));
}

async fn for_testi() -> () {
    println!("{}", "  [FOR] 0'dan 5'e kadar:".to_string());
    {
        let mut i = 0;
        let _zet_end = 5;
        let _zet_step = 1;
        '_zet_for_0: while (_zet_step > 0 && i < _zet_end) || (_zet_step < 0 && i > _zet_end) {
            '_zet_body_0: {
                println!("{}", "    i = ".to_string().z_add(i.clone()));
            }
            i = i.z_add(_zet_step);
        }
    }
}

async fn for_step_testi() -> () {
    println!("{}", "  [FOR-BY] 0'dan 20'ye, 3'er 3'er:".to_string());
    {
        let mut i = 0;
        let _zet_end = 20;
        let _zet_step = 3;
        '_zet_for_1: while (_zet_step > 0 && i < _zet_end) || (_zet_step < 0 && i > _zet_end) {
            '_zet_body_1: {
                println!("{}", "    i = ".to_string().z_add(i.clone()));
            }
            i = i.z_add(_zet_step);
        }
    }
}

async fn while_testi() -> () {
    println!("{}", "  [WHILE] Geri sayim:".to_string());
    let mut x = 5;
    while (x > 0) {
        println!("{}", "    ".to_string().z_add(x.clone()));
        x = (x - 1);
    }
    println!("{}", "    Bitti!".to_string());
}

async fn ic_ice_dongu() -> () {
    println!("{}", "  [NESTED] Carpim tablosu (1-3):".to_string());
    {
        let mut i = 1;
        let _zet_end = 4;
        let _zet_step = 1;
        '_zet_for_2: while (_zet_step > 0 && i < _zet_end) || (_zet_step < 0 && i > _zet_end) {
            '_zet_body_2: {
                {
                    let mut j = 1;
                    let _zet_end = 4;
                    let _zet_step = 1;
                    '_zet_for_3: while (_zet_step > 0 && j < _zet_end) || (_zet_step < 0 && j > _zet_end) {
                        '_zet_body_3: {
                            let mut sonuc = i.clone().z_mul(j.clone());
                            println!("{}", "    ".to_string().z_add(i.clone()).z_add(" x ".to_string()).z_add(j.clone()).z_add(" = ".to_string()).z_add(sonuc.clone()));
                        }
                        j = j.z_add(_zet_step);
                    }
                }
            }
            i = i.z_add(_zet_step);
        }
    }
}

async fn karisik_kontrol() -> () {
    println!("{}", "  [MIX] Cift sayilar (0-9):".to_string());
    {
        let mut i = 0;
        let _zet_end = 10;
        let _zet_step = 1;
        '_zet_for_4: while (_zet_step > 0 && i < _zet_end) || (_zet_step < 0 && i > _zet_end) {
            '_zet_body_4: {
                let mut kalan = (i - (i / 2).z_mul(2));
                if (kalan == 0) {
                    println!("{}", "    Cift: ".to_string().z_add(i.clone()));
                }
            }
            i = i.z_add(_zet_step);
        }
    }
}

async fn while_if_testi() -> () {
    println!("{}", "  [WHILE-IF] Collatz serisi (n=13):".to_string());
    let mut n = 13;
    let mut adim = 0;
    while (n != 1) {
        println!("{}", "    n=".to_string().z_add(n.clone()));
        let mut kalan = (n - (n / 2).z_mul(2));
        if (kalan == 0) {
            n = (n / 2);
        } else {
            n = n.clone().z_mul(3).z_add(1);
        }
        adim = adim.clone().z_add(1);
    }
    println!("{}", "    n=1 (toplam ".to_string().z_add(adim.clone()).z_add(" adim)".to_string()));
}

async fn dizi_testi() -> () {
    println!("{}", "  [ARRAY] Dizi islemleri:".to_string());
    let mut sayilar = vec![10, 20, 30, 40, 50];
    println!("{}", "    ilk eleman: ".to_string().z_add(sayilar[(0 as usize)]));
    println!("{}", "    ikinci eleman: ".to_string().z_add(sayilar[(1 as usize)]));
    println!("{}", "    son eleman: ".to_string().z_add(sayilar[(4 as usize)]));
}

async fn dizi_dongu_testi() -> () {
    println!("{}", "  [ARRAY-FOR] Dizi elemanlari:".to_string());
    let mut veriler = vec![100, 200, 300, 400, 500];
    {
        let mut i = 0;
        let _zet_end = 5;
        let _zet_step = 1;
        '_zet_for_5: while (_zet_step > 0 && i < _zet_end) || (_zet_step < 0 && i > _zet_end) {
            '_zet_body_5: {
                println!("{}", "    veriler[".to_string().z_add(i.clone()).z_add("] = ".to_string()).z_add(veriler[(i as usize)]));
            }
            i = i.z_add(_zet_step);
        }
    }
}

fn dizi_toplam_hesapla() -> i64 {
    let mut dizi = vec![5, 10, 15, 20, 25];
    let mut toplam = dizi[(0 as usize)].z_add(dizi[(1 as usize)]).z_add(dizi[(2 as usize)]).z_add(dizi[(3 as usize)]).z_add(dizi[(4 as usize)]);
    return toplam;
}

async fn string_birlestirme_testi() -> () {
    println!("{}", "  [STRING] Birlestirme testleri:".to_string());
    println!("{}", "    ".to_string().z_add("Zet".to_string()).z_add(" ".to_string()).z_add("Lang".to_string()));
    println!("{}", "    Versiyon: ".to_string().z_add(2));
    let mut x = 42;
    println!("{}", "    Cevap: ".to_string().z_add(x.clone()));
    println!("{}", "    ".to_string().z_add(10).z_add(" + ".to_string()).z_add(20).z_add(" = ".to_string()).z_add(30));
}

async fn string_carpim_testi() -> () {
    println!("{}", "  [STRING-MUL] Tekrarlama testi:".to_string());
    let mut yildiz = "* ".to_string().z_mul(10);
    println!("{}", "    ".to_string().z_add(yildiz.clone()));
    let mut tire = "- ".to_string().z_mul(20);
    println!("{}", "    ".to_string().z_add(tire.clone()));
    let mut emoji = "Zet! ".to_string().z_mul(3);
    println!("{}", "    ".to_string().z_add(emoji.clone()));
}

async fn det_string_testi() -> () {
    let mut mesaj = selamla("Dunya".to_string());
    println!("{}", "  [DET-STR] ".to_string().z_add(mesaj.clone()));
    let mut sayi_str = tekrarla_sayi(99);
    println!("{}", "  [DET-STR] ".to_string().z_add(sayi_str.clone()));
}

async fn basit_scope_testi() -> () {
    println!("{}", "  [SCOPE] Basit scope testi:".to_string());
    // Scope: BasitScope
    {
        let mut _zet_handles: Vec<tokio::task::JoinHandle<()>> = Vec::new();
        _zet_handles.push(tokio::spawn(async move { println!("{}", "    [spawn 1] Merhaba scope'dan!".to_string()) }));
        _zet_handles.push(tokio::spawn(async move { println!("{}", "    [spawn 2] Ben de buradayim!".to_string()) }));
        _zet_handles.push(tokio::spawn(async move { println!("{}", "    [spawn 3] Paralel calisiyoruz!".to_string()) }));
        for _h in _zet_handles { _h.await.ok(); }
    }
    println!("{}", "  [SCOPE] Tum spawn'lar bitti.".to_string());
}

async fn scope_hesap_testi() -> () {
    println!("{}", "  [SCOPE-CALC] Scope ile hesaplama:".to_string());
    let mut sonuc1 = fibonacci(8);
    let mut sonuc2 = faktoriyel(5);
    // Scope: HesapScope
    {
        let mut _zet_handles: Vec<tokio::task::JoinHandle<()>> = Vec::new();
        _zet_handles.push(tokio::spawn(async move { println!("{}", "    [spawn] Fibonacci(8) = ".to_string().z_add(sonuc1.clone())) }));
        _zet_handles.push(tokio::spawn(async move { println!("{}", "    [spawn] 5! = ".to_string().z_add(sonuc2.clone())) }));
        for _h in _zet_handles { _h.await.ok(); }
    }
    println!("{}", "  [SCOPE-CALC] Scope bitti.".to_string());
}

async fn coklu_scope_testi() -> () {
    println!("{}", "  [MULTI-SCOPE] Birden fazla scope:".to_string());
    // Scope: Birinci
    {
        let mut _zet_handles: Vec<tokio::task::JoinHandle<()>> = Vec::new();
        _zet_handles.push(tokio::spawn(async move { println!("{}", "    [Scope1-A] Ilk scope, ilk gorev".to_string()) }));
        _zet_handles.push(tokio::spawn(async move { println!("{}", "    [Scope1-B] Ilk scope, ikinci gorev".to_string()) }));
        for _h in _zet_handles { _h.await.ok(); }
    }
    println!("{}", "    --- Birinci scope tamamlandi ---".to_string());
    // Scope: Ikinci
    {
        let mut _zet_handles: Vec<tokio::task::JoinHandle<()>> = Vec::new();
        _zet_handles.push(tokio::spawn(async move { println!("{}", "    [Scope2-A] Ikinci scope, ilk gorev".to_string()) }));
        _zet_handles.push(tokio::spawn(async move { println!("{}", "    [Scope2-B] Ikinci scope, ikinci gorev".to_string()) }));
        for _h in _zet_handles { _h.await.ok(); }
    }
    println!("{}", "    --- Ikinci scope tamamlandi ---".to_string());
}

async fn call_testi() -> () {
    println!("{}", "  [CALL] Util.now() testi:".to_string());
    let mut t1 = Util::now().await;
    let mut sonuc = fibonacci(25);
    let mut t2 = Util::now().await;
    let mut gecen = (t2 - t1);
    println!("{}", "    Fibonacci(25) = ".to_string().z_add(sonuc.clone()));
    println!("{}", "    Gecen sure: ".to_string().z_add(gecen.clone()).z_add(" ms".to_string()));
}

async fn veri_hazirla() -> i64 {
    let mut t = Util::now().await;
    return t;
}

async fn call_nondet_testi() -> () {
    println!("{}", "  [CALL-NONDET] Nondet fonksiyon cagrisi:".to_string());
    let mut zaman = veri_hazirla().await;
    println!("{}", "    Zaman damgasi: ".to_string().z_add(zaman.clone()));
}

async fn to_int_testi() -> () {
    println!("{}", "  [TO_INT] String -> i64 donusumu:".to_string());
    let mut a = Util::to_int("123".to_string()).await;
    let mut b = Util::to_int("456".to_string()).await;
    let mut toplam = a.clone().z_add(b.clone());
    println!("{}", "    123 + 456 = ".to_string().z_add(toplam.clone()));
}

async fn validate_parametre_testi(veri: Untrusted) -> () {
    println!("{}", "  [VALIDATE] Parametre testi:".to_string());
    match veri.validate() {
        Ok(veri) => {
            println!("{}", "    Dogrulandi: ".to_string().z_add(veri.clone()));
        }
        Err(_zet_err) => {
            eprintln!("  {}[VALIDATE FAIL] {}{}", RED, _zet_err, RESET);
        }
    }
}

async fn validate_hesap_testi(girdi: Untrusted) -> () {
    println!("{}", "  [VALIDATE-CALC] Validate icinde hesaplama:".to_string());
    match girdi.validate() {
        Ok(girdi) => {
            let mut sayi = Util::to_int(girdi.clone()).await;
            let mut kare = sayi.clone().z_mul(sayi.clone());
            println!("{}", "    Girdi: ".to_string().z_add(girdi.clone()));
            println!("{}", "    Karesi: ".to_string().z_add(kare.clone()));
        }
        Err(_zet_err) => {
            eprintln!("  {}[VALIDATE FAIL] {}{}", RED, _zet_err, RESET);
        }
    }
}

async fn validate_isim_testi(isim: Untrusted) -> () {
    println!("{}", "  [VALIDATE-NAME] Isim dogrulama:".to_string());
    match isim.validate() {
        Ok(isim) => {
            println!("{}", "    Hosgeldiniz, ".to_string().z_add(isim.clone()).z_add("!".to_string()));
        }
        Err(_zet_err) => {
            eprintln!("  {}[VALIDATE FAIL] {}{}", RED, _zet_err, RESET);
        }
    }
}

async fn json_testi() -> () {
    println!("{}", "  [JSON] JSON okuma testi:".to_string());
    let mut veri = "{\"isim\":\"Zet\",\"versiyon\":2,\"aktif\":true}".to_string();
    let mut isim = serde_json::from_str::<serde_json::Value>(&veri).ok().and_then(|v| v.get("isim").map(|x| if x.is_string() { x.as_str().unwrap().to_string() } else { x.to_string() })).unwrap_or("HATA".to_string());
    let mut versiyon = serde_json::from_str::<serde_json::Value>(&veri).ok().and_then(|v| v.get("versiyon").map(|x| if x.is_string() { x.as_str().unwrap().to_string() } else { x.to_string() })).unwrap_or("HATA".to_string());
    let mut aktif = serde_json::from_str::<serde_json::Value>(&veri).ok().and_then(|v| v.get("aktif").map(|x| if x.is_string() { x.as_str().unwrap().to_string() } else { x.to_string() })).unwrap_or("HATA".to_string());
    println!("{}", "    Isim: ".to_string().z_add(isim.clone()));
    println!("{}", "    Versiyon: ".to_string().z_add(versiyon.clone()));
    println!("{}", "    Aktif: ".to_string().z_add(aktif.clone()));
}

async fn json_karmasik_testi() -> () {
    println!("{}", "  [JSON-ADV] Karmasik JSON:".to_string());
    let mut json_str = "{\"kullanici\":\"admin\",\"yetki\":\"root\",\"seviye\":5}".to_string();
    let mut kullanici = serde_json::from_str::<serde_json::Value>(&json_str).ok().and_then(|v| v.get("kullanici").map(|x| if x.is_string() { x.as_str().unwrap().to_string() } else { x.to_string() })).unwrap_or("HATA".to_string());
    let mut yetki = serde_json::from_str::<serde_json::Value>(&json_str).ok().and_then(|v| v.get("yetki").map(|x| if x.is_string() { x.as_str().unwrap().to_string() } else { x.to_string() })).unwrap_or("HATA".to_string());
    let mut seviye = serde_json::from_str::<serde_json::Value>(&json_str).ok().and_then(|v| v.get("seviye").map(|x| if x.is_string() { x.as_str().unwrap().to_string() } else { x.to_string() })).unwrap_or("HATA".to_string());
    println!("{}", "    Kullanici: ".to_string().z_add(kullanici.clone()).z_add(", Yetki: ".to_string()).z_add(yetki.clone()).z_add(", Seviye: ".to_string()).z_add(seviye.clone()));
}

fn basit_return() -> i64 {
    return 42;
}

fn kosullu_return(n: i64) -> i64 {
    if (n > 0) {
        return n;
    }
    return (0 - n);
}

fn void_return_testi() -> () {
    println!("{}", "  [RETURN] Bu fonksiyon Void donduruyor.".to_string());
    return;
}

fn not_hesapla(puan: i64) -> String {
    if (puan >= 90) {
        return "AA".to_string();
    } else {
        if (puan >= 80) {
            return "BA".to_string();
        } else {
            if (puan >= 70) {
                return "BB".to_string();
            } else {
                if (puan >= 60) {
                    return "CB".to_string();
                } else {
                    return "FF".to_string();
                }
            }
        }
    }
}

async fn benchmark_testi() -> () {
    println!("{}", "  [BENCH] Fibonacci(30) benchmark:".to_string());
    let mut t1 = Util::now().await;
    let mut sonuc = fibonacci(30);
    let mut t2 = Util::now().await;
    let mut sure = (t2 - t1);
    println!("{}", "    Sonuc: ".to_string().z_add(sonuc.clone()));
    println!("{}", "    Sure: ".to_string().z_add(sure.clone()).z_add(" ms".to_string()));
}

async fn faktoriyel_tablosu() -> () {
    println!("{}", "  [TABLE] Faktoriyel tablosu (1-10):".to_string());
    {
        let mut i = 1;
        let _zet_end = 11;
        let _zet_step = 1;
        '_zet_for_6: while (_zet_step > 0 && i < _zet_end) || (_zet_step < 0 && i > _zet_end) {
            '_zet_body_6: {
                let mut sonuc = faktoriyel(i.clone());
                println!("{}", "    ".to_string().z_add(i.clone()).z_add("! = ".to_string()).z_add(sonuc.clone()));
            }
            i = i.z_add(_zet_step);
        }
    }
}

async fn fibonacci_serisi() -> () {
    println!("{}", "  [FIB-SERIES] Fibonacci(0-15):".to_string());
    {
        let mut i = 0;
        let _zet_end = 16;
        let _zet_step = 1;
        '_zet_for_7: while (_zet_step > 0 && i < _zet_end) || (_zet_step < 0 && i > _zet_end) {
            '_zet_body_7: {
                let mut sonuc = fibonacci(i.clone());
                println!("{}", "    F(".to_string().z_add(i.clone()).z_add(") = ".to_string()).z_add(sonuc.clone()));
            }
            i = i.z_add(_zet_step);
        }
    }
}

async fn asal_sayi_testi() -> () {
    println!("{}", "  [PRIME] Asal sayi testi (1-30):".to_string());
    {
        let mut i = 1;
        let _zet_end = 31;
        let _zet_step = 1;
        '_zet_for_8: while (_zet_step > 0 && i < _zet_end) || (_zet_step < 0 && i > _zet_end) {
            '_zet_body_8: {
                let mut sonuc = asal_mi(i.clone());
                if (sonuc == 1) {
                    println!("{}", "    ".to_string().z_add(i.clone()).z_add(" asal!".to_string()));
                }
            }
            i = i.z_add(_zet_step);
        }
    }
}

async fn karsilastirma_testi() -> () {
    println!("{}", "  [CMP] Karsilastirma operatorleri:".to_string());
    println!("{}", "    5 == 5 ? ".to_string().z_add(esit_mi(5, 5)));
    println!("{}", "    5 != 3 ? ".to_string().z_add(farkli_mi(5, 3)));
    println!("{}", "    7 > 3  ? ".to_string().z_add(buyuk_mu(7, 3)));
    println!("{}", "    2 < 8  ? ".to_string().z_add(kucuk_mu(2, 8)));
    println!("{}", "    5 >= 5 ? ".to_string().z_add(buyuk_esit_mi(5, 5)));
    println!("{}", "    4 <= 4 ? ".to_string().z_add(kucuk_esit_mi(4, 4)));
    println!("{}", "    3 == 7 ? ".to_string().z_add(esit_mi(3, 7)));
    println!("{}", "    6 > 9  ? ".to_string().z_add(buyuk_mu(6, 9)));
}

async fn matematik_testi() -> () {
    println!("{}", "  [MATH] Ileri matematik:".to_string());
    println!("{}", "    2^10 = ".to_string().z_add(us_al(2, 10)));
    println!("{}", "    3^5  = ".to_string().z_add(us_al(3, 5)));
    println!("{}", "    GCD(48, 18)  = ".to_string().z_add(gcd(48, 18)));
    println!("{}", "    GCD(100, 75) = ".to_string().z_add(gcd(100, 75)));
    println!("{}", "    |(-42)| = ".to_string().z_add(mutlak_deger((0 - 42))));
    println!("{}", "    max(15, 27) = ".to_string().z_add(maks(15, 27)));
    println!("{}", "    min(15, 27) = ".to_string().z_add(min_deger(15, 27)));
    println!("{}", "    isaret(-5) = ".to_string().z_add(isaret((0 - 5))));
    println!("{}", "    isaret(0)  = ".to_string().z_add(isaret(0)));
    println!("{}", "    isaret(3)  = ".to_string().z_add(isaret(3)));
}

async fn not_testi() -> () {
    println!("{}", "  [GRADE] Not hesaplama:".to_string());
    let mut puanlar = vec![95, 85, 75, 65, 45];
    {
        let mut i = 0;
        let _zet_end = 5;
        let _zet_step = 1;
        '_zet_for_9: while (_zet_step > 0 && i < _zet_end) || (_zet_step < 0 && i > _zet_end) {
            '_zet_body_9: {
                let mut puan = puanlar[(i as usize)];
                let mut not_val = not_hesapla(puan.clone());
                let mut sinif = siniflandir(puan.clone());
                println!("{}", "    Puan: ".to_string().z_add(puan.clone()).z_add(" -> Not: ".to_string()).z_add(not_val.clone()).z_add(" (Sinif: ".to_string()).z_add(sinif.clone()).z_add(")".to_string()));
            }
            i = i.z_add(_zet_step);
        }
    }
}

async fn zincir_testi() -> () {
    println!("{}", "  [CHAIN] Fonksiyon zincirleri:".to_string());
    let mut a = karmasik_hesap(5);
    println!("{}", "    karmasik_hesap(5) = ".to_string().z_add(a.clone()));
    let mut b = zincirleme(7);
    println!("{}", "    zincirleme(7) = ".to_string().z_add(b.clone()));
    let mut c = topla(carp(3, 4), cikar(10, 5));
    println!("{}", "    (3*4) + (10-5) = ".to_string().z_add(c.clone()));
    let mut d = maks(fibonacci(7), faktoriyel(4));
    println!("{}", "    max(fib(7), 4!) = ".to_string().z_add(d.clone()));
    let mut e = kare_al(kup_al(2));
    println!("{}", "    (2^3)^2 = ".to_string().z_add(e.clone()));
}

async fn dizi_hesaplama_testi() -> () {
    println!("{}", "  [ARRAY-CALC] Dizi ile hesaplamalar:".to_string());
    let mut dizi = vec![3, 7, 2, 9, 1, 8, 4, 6, 5, 10];
    let mut toplam = dizi[(0 as usize)].z_add(dizi[(1 as usize)]).z_add(dizi[(2 as usize)]).z_add(dizi[(3 as usize)]).z_add(dizi[(4 as usize)]).z_add(dizi[(5 as usize)]).z_add(dizi[(6 as usize)]).z_add(dizi[(7 as usize)]).z_add(dizi[(8 as usize)]).z_add(dizi[(9 as usize)]);
    println!("{}", "    Dizi toplami: ".to_string().z_add(toplam.clone()));
    let mut en_buyuk = maks(maks(maks(dizi[(0 as usize)], dizi[(1 as usize)]), maks(dizi[(2 as usize)], dizi[(3 as usize)])), maks(maks(dizi[(4 as usize)], dizi[(5 as usize)]), maks(dizi[(6 as usize)], dizi[(7 as usize)])));
    println!("{}", "    En buyuk (ilk 8): ".to_string().z_add(en_buyuk.clone()));
    println!("{}", "    Kareler:".to_string());
    {
        let mut i = 0;
        let _zet_end = 5;
        let _zet_step = 1;
        '_zet_for_10: while (_zet_step > 0 && i < _zet_end) || (_zet_step < 0 && i > _zet_end) {
            '_zet_body_10: {
                let mut eleman = dizi[(i as usize)];
                let mut karesi = kare_al(eleman.clone());
                println!("{}", "      ".to_string().z_add(eleman.clone()).z_add("^2 = ".to_string()).z_add(karesi.clone()));
            }
            i = i.z_add(_zet_step);
        }
    }
}

async fn dongu_karisim_testi() -> () {
    println!("{}", "  [LOOP-MIX] Dongu karisimi:".to_string());
    {
        let mut i = 1;
        let _zet_end = 6;
        let _zet_step = 1;
        '_zet_for_11: while (_zet_step > 0 && i < _zet_end) || (_zet_step < 0 && i > _zet_end) {
            '_zet_body_11: {
                let mut j = 1;
                let mut satir = "    ".to_string().z_add(i.clone()).z_add(": ".to_string());
                while (j <= i) {
                    satir = satir.clone().z_add("* ".to_string());
                    j = j.clone().z_add(1);
                }
                println!("{}", satir);
            }
            i = i.z_add(_zet_step);
        }
    }
}

async fn boolean_testi() -> () {
    println!("{}", "  [BOOL] Boolean testi:".to_string());
    let mut dogru = true;
    let mut yanlis = false;
    println!("{}", "    true degeri: ".to_string().z_add(dogru.clone()));
    println!("{}", "    false degeri: ".to_string().z_add(yanlis.clone()));
}

async fn scope_karmasik_testi() -> () {
    println!("{}", "  [SCOPE-ADV] Karmasik scope:".to_string());
    let mut mesaj1 = "Gorev A tamamlandi".to_string();
    let mut mesaj2 = "Gorev B tamamlandi".to_string();
    let mut mesaj3 = "Gorev C tamamlandi".to_string();
    // Scope: GorevYonetici
    {
        let mut _zet_handles: Vec<tokio::task::JoinHandle<()>> = Vec::new();
        _zet_handles.push(tokio::spawn(async move { println!("{}", "    [A] ".to_string().z_add(mesaj1.clone())) }));
        _zet_handles.push(tokio::spawn(async move { println!("{}", "    [B] ".to_string().z_add(mesaj2.clone())) }));
        _zet_handles.push(tokio::spawn(async move { println!("{}", "    [C] ".to_string().z_add(mesaj3.clone())) }));
        for _h in _zet_handles { _h.await.ok(); }
    }
    println!("{}", "  [SCOPE-ADV] Tum gorevler bitti!".to_string());
}

async fn validate_karmasik_testi(veri: Untrusted) -> () {
    println!("{}", "  [VALIDATE-ADV] Karmasik validate:".to_string());
    match veri.validate() {
        Ok(veri) => {
            let mut uzunluk = Util::to_int(veri.clone()).await;
            if (uzunluk > 0) {
                println!("{}", "    Pozitif deger: ".to_string().z_add(uzunluk.clone()));
                let mut kare = kare_al(uzunluk.clone());
                println!("{}", "    Karesi: ".to_string().z_add(kare.clone()));
                let mut kup = kup_al(uzunluk.clone());
                println!("{}", "    Kupu: ".to_string().z_add(kup.clone()));
            } else {
                println!("{}", "    Sifir veya negatif.".to_string());
            }
        }
        Err(_zet_err) => {
            eprintln!("  {}[VALIDATE FAIL] {}{}", RED, _zet_err, RESET);
        }
    }
}

async fn coklu_validate_testi(a: Untrusted, b: Untrusted) -> () {
    println!("{}", "  [MULTI-VALIDATE] Coklu validate:".to_string());
    match a.validate() {
        Ok(a) => {
            println!("{}", "    Birinci veri: ".to_string().z_add(a.clone()));
            match b.validate() {
                Ok(b) => {
                    println!("{}", "    Ikinci veri: ".to_string().z_add(b.clone()));
                    println!("{}", "    Her iki veri de dogrulandi!".to_string());
                }
                Err(_zet_err) => {
                    eprintln!("  {}[VALIDATE FAIL] {}{}", RED, _zet_err, RESET);
                }
            }
        }
        Err(_zet_err) => {
            eprintln!("  {}[VALIDATE FAIL] {}{}", RED, _zet_err, RESET);
        }
    }
}

async fn buyuk_dongu_testi() -> () {
    println!("{}", "  [BIG-LOOP] 1'den 100'e kadar toplam:".to_string());
    let mut toplam = 0;
    {
        let mut i = 1;
        let _zet_end = 101;
        let _zet_step = 1;
        '_zet_for_12: while (_zet_step > 0 && i < _zet_end) || (_zet_step < 0 && i > _zet_end) {
            '_zet_body_12: {
                toplam = toplam.clone().z_add(i.clone());
            }
            i = i.z_add(_zet_step);
        }
    }
    println!("{}", "    1+2+...+100 = ".to_string().z_add(toplam.clone()));
}

async fn for_negatif_step() -> () {
    println!("{}", "  [FOR-NEG] 10'dan 0'a, -2 adimla:".to_string());
    {
        let mut i = 10;
        let _zet_end = 0;
        let _zet_step = (0 - 2);
        '_zet_for_13: while (_zet_step > 0 && i < _zet_end) || (_zet_step < 0 && i > _zet_end) {
            '_zet_body_13: {
                println!("{}", "    i = ".to_string().z_add(i.clone()));
            }
            i = i.z_add(_zet_step);
        }
    }
}

fn det_dongu_testi() -> () {
    println!("{}", "  [DET-LOOP] Det fonksiyonda for dongusu:".to_string());
    {
        let mut i = 0;
        let _zet_end = 5;
        let _zet_step = 1;
        '_zet_for_14: while (_zet_step > 0 && i < _zet_end) || (_zet_step < 0 && i > _zet_end) {
            '_zet_body_14: {
                println!("{}", "    Det sayac: ".to_string().z_add(i.clone()));
            }
            i += _zet_step;
        }
    }
}

fn det_while_testi() -> () {
    println!("{}", "  [DET-WHILE] Det fonksiyonda while:".to_string());
    let mut n = 10;
    while (n > 0) {
        print!("{}", "    ".to_string().z_add(n.clone()).z_add(" ".to_string()));
        n = (n - 1);
    }
    println!("{}", "".to_string());
}

fn det_dizi_testi() -> () {
    println!("{}", "  [DET-ARRAY] Det fonksiyonda dizi:".to_string());
    let mut dizi = vec![100, 200, 300];
    let mut toplam = dizi[(0 as usize)].z_add(dizi[(1 as usize)]).z_add(dizi[(2 as usize)]);
    println!("{}", "    Toplam: ".to_string().z_add(toplam.clone()));
    if (toplam > 500) {
        println!("{}", "    Toplam 500'den buyuk!".to_string());
    } else {
        println!("{}", "    Toplam 500 veya daha kucuk.".to_string());
    }
}

fn sezar_sifrele_sayi(sayi: i64, kayma: i64) -> i64 {
    return sayi.clone().z_add(kayma.clone());
}

async fn sezar_testi() -> () {
    println!("{}", "  [CAESAR] Sezar sifreleme (sayisal):".to_string());
    let mut orijinal = vec![1, 5, 9, 14, 20];
    let mut kayma = 3;
    println!("{}", "    Orijinal ve sifrelenmis:".to_string());
    {
        let mut i = 0;
        let _zet_end = 5;
        let _zet_step = 1;
        '_zet_for_15: while (_zet_step > 0 && i < _zet_end) || (_zet_step < 0 && i > _zet_end) {
            '_zet_body_15: {
                let mut sifre = sezar_sifrele_sayi(orijinal[(i as usize)], kayma.clone());
                println!("{}", "    ".to_string().z_add(orijinal[(i as usize)]).z_add(" -> ".to_string()).z_add(sifre.clone()));
            }
            i = i.z_add(_zet_step);
        }
    }
}

fn collatz_adim(n: i64) -> i64 {
    if (n == 1) {
        return 0;
    }
    let mut kalan = (n - (n / 2).z_mul(2));
    if (kalan == 0) {
        return 1.z_add(collatz_adim((n / 2)));
    } else {
        return 1.z_add(collatz_adim(n.clone().z_mul(3).z_add(1)));
    }
}

async fn collatz_testi() -> () {
    println!("{}", "  [COLLATZ] Collatz adim sayilari:".to_string());
    {
        let mut i = 1;
        let _zet_end = 21;
        let _zet_step = 1;
        '_zet_for_16: while (_zet_step > 0 && i < _zet_end) || (_zet_step < 0 && i > _zet_end) {
            '_zet_body_16: {
                let mut adim = collatz_adim(i.clone());
                println!("{}", "    n=".to_string().z_add(i.clone()).z_add(" -> ".to_string()).z_add(adim.clone()).z_add(" adim".to_string()));
            }
            i = i.z_add(_zet_step);
        }
    }
}

async fn buyuk_benchmark() -> () {
    println!("{}", "  [BENCH2] Buyuk hesaplama:".to_string());
    let mut t1 = Util::now().await;
    let mut toplam = 0;
    {
        let mut i = 0;
        let _zet_end = 1000;
        let _zet_step = 1;
        '_zet_for_17: while (_zet_step > 0 && i < _zet_end) || (_zet_step < 0 && i > _zet_end) {
            '_zet_body_17: {
                toplam = toplam.clone().z_add(i.clone());
            }
            i = i.z_add(_zet_step);
        }
    }
    let mut t2 = Util::now().await;
    println!("{}", "    0+1+...+999 = ".to_string().z_add(toplam.clone()));
    println!("{}", "    Sure: ".to_string().z_add((t2 - t1)).z_add(" ms".to_string()));
}

fn topla_f(a: f64, b: f64) -> f64 {
    return a.clone().z_add(b.clone());
}

fn carp_f(a: f64, b: f64) -> f64 {
    return a.clone().z_mul(b.clone());
}

fn daire_alani(r: f64) -> f64 {
    return 3.14159f64.z_mul(r.clone()).z_mul(r.clone());
}

async fn f64_testi() -> () {
    println!("{}", "  [F64] Ondalikli sayi testleri:".to_string());
    let mut a = 3.14f64;
    let mut b = 2.71f64;
    println!("{}", "    a = ".to_string().z_add(a.clone()));
    println!("{}", "    b = ".to_string().z_add(b.clone()));
    println!("{}", "    topla_f(a, b) = ".to_string().z_add(topla_f(a.clone(), b.clone())));
    println!("{}", "    carp_f(a, b) = ".to_string().z_add(carp_f(a.clone(), b.clone())));
    println!("{}", "    daire_alani(5.0) = ".to_string().z_add(daire_alani(5.0f64)));
    let mut c = (10.5f64 - 3.2f64);
    println!("{}", "    10.5 - 3.2 = ".to_string().z_add(c.clone()));
    let mut d = (10.0f64 / 3.0f64);
    println!("{}", "    10.0 / 3.0 = ".to_string().z_add(d.clone()));
}

async fn char_testi() -> () {
    println!("{}", "  [CHAR] Karakter testleri:".to_string());
    let mut harf = 'A';
    let mut rakam = '7';
    let mut ozel = '!';
    println!("{}", "    harf = ".to_string().z_add(harf.clone()));
    println!("{}", "    rakam = ".to_string().z_add(rakam.clone()));
    println!("{}", "    ozel = ".to_string().z_add(ozel.clone()));
    let mut mesaj = "Karakter: ".to_string().z_add(harf.clone());
    println!("{}", "    ".to_string().z_add(mesaj.clone()));
}

fn ve_islemi(a: bool, b: bool) -> bool {
    return (a && b);
}

fn veya_islemi(a: bool, b: bool) -> bool {
    return (a || b);
}

fn degil_islemi(a: bool) -> bool {
    return (!a);
}

async fn bool_testi_v2() -> () {
    println!("{}", "  [BOOL-V2] Bool testi:".to_string());
    let mut d = true;
    let mut y = false;
    println!("{}", "    true && true  = ".to_string().z_add(ve_islemi(d.clone(), d.clone())));
    println!("{}", "    true && false = ".to_string().z_add(ve_islemi(d.clone(), y.clone())));
    println!("{}", "    false || true = ".to_string().z_add(veya_islemi(y.clone(), d.clone())));
    println!("{}", "    false || false= ".to_string().z_add(veya_islemi(y.clone(), y.clone())));
    println!("{}", "    !true  = ".to_string().z_add(degil_islemi(d.clone())));
    println!("{}", "    !false = ".to_string().z_add(degil_islemi(y.clone())));
}

fn mod_testi_fn(a: i64, b: i64) -> i64 {
    return (a % b);
}

async fn modulo_testi() -> () {
    println!("{}", "  [MOD] Modulo (%) testi:".to_string());
    println!("{}", "    10 % 3 = ".to_string().z_add(mod_testi_fn(10, 3)));
    println!("{}", "    17 % 5 = ".to_string().z_add(mod_testi_fn(17, 5)));
    println!("{}", "    100 % 7 = ".to_string().z_add(mod_testi_fn(100, 7)));
    println!("{}", "    20 % 4 = ".to_string().z_add(mod_testi_fn(20, 4)));
}

async fn mantiksal_operator_testi() -> () {
    println!("{}", "  [LOGIC] Mantiksal operator testleri:".to_string());
    let mut a = true;
    let mut b = false;
    if (a && a) {
        println!("{}", "    true && true = true (dogru)".to_string());
    }
    if (!(a && b)) {
        println!("{}", "    true && false = false (dogru)".to_string());
    }
    if (a || b) {
        println!("{}", "    true || false = true (dogru)".to_string());
    }
    if (!(b || b)) {
        println!("{}", "    false || false = false (dogru)".to_string());
    }
    if (!b) {
        println!("{}", "    !false = true (dogru)".to_string());
    }
    if (!(!a)) {
        println!("{}", "    !(!true) = true (dogru)".to_string());
    }
    if ((a && (!b)) || (b && (!a))) {
        println!("{}", "    XOR benzeri: (a&&!b)||(b&&!a) = true (dogru)".to_string());
    }
}

fn bit_and(a: i64, b: i64) -> i64 {
    return (a & b);
}

fn bit_or(a: i64, b: i64) -> i64 {
    return (a | b);
}

fn bit_xor(a: i64, b: i64) -> i64 {
    return (a ^ b);
}

fn bit_shl(a: i64, n: i64) -> i64 {
    return (a << n);
}

fn bit_shr(a: i64, n: i64) -> i64 {
    return (a >> n);
}

async fn bitwise_testi() -> () {
    println!("{}", "  [BIT] Bitwise operator testleri:".to_string());
    println!("{}", "    12 & 10  = ".to_string().z_add(bit_and(12, 10)));
    println!("{}", "    12 | 10  = ".to_string().z_add(bit_or(12, 10)));
    println!("{}", "    12 ^ 10  = ".to_string().z_add(bit_xor(12, 10)));
    println!("{}", "    1 << 4   = ".to_string().z_add(bit_shl(1, 4)));
    println!("{}", "    256 >> 3 = ".to_string().z_add(bit_shr(256, 3)));
    println!("{}", "    0xFF & 0x0F = ".to_string().z_add(bit_and(255, 15)));
}

async fn break_testi() -> () {
    println!("{}", "  [BREAK] Break testi:".to_string());
    let mut i = 0;
    while (i < 100) {
        if (i == 5) {
            println!("{}", "    i=5'te break!".to_string());
            break;
        }
        println!("{}", "    i = ".to_string().z_add(i.clone()));
        i = i.clone().z_add(1);
    }
}

async fn continue_testi() -> () {
    println!("{}", "  [CONTINUE] Continue testi (cift sayilari atla):".to_string());
    {
        let mut i = 0;
        let _zet_end = 10;
        let _zet_step = 1;
        '_zet_for_18: while (_zet_step > 0 && i < _zet_end) || (_zet_step < 0 && i > _zet_end) {
            '_zet_body_18: {
                if ((i % 2) == 0) {
                    break '_zet_body_18;
                }
                println!("{}", "    tek: ".to_string().z_add(i.clone()));
            }
            i = i.z_add(_zet_step);
        }
    }
}

async fn break_continue_for() -> () {
    println!("{}", "  [BC-FOR] For'da break+continue:".to_string());
    {
        let mut i = 0;
        let _zet_end = 20;
        let _zet_step = 1;
        '_zet_for_19: while (_zet_step > 0 && i < _zet_end) || (_zet_step < 0 && i > _zet_end) {
            '_zet_body_19: {
                if ((i % 3) == 0) {
                    break '_zet_body_19;
                }
                if (i > 10) {
                    println!("{}", "    i>10, break!".to_string());
                    break '_zet_for_19;
                }
                println!("{}", "    i = ".to_string().z_add(i.clone()));
            }
            i = i.z_add(_zet_step);
        }
    }
}

fn const_testi_fn() -> i64 {
    let PI_INT = 3;
    let MAX = 100;
    return PI_INT.clone().z_add(MAX.clone());
}

async fn const_testi() -> () {
    println!("{}", "  [CONST] Const tanimlama testi:".to_string());
    let BASLIK = "Zet Lang v0.3".to_string();
    let VERSIYON = 3;
    println!("{}", "    BASLIK = ".to_string().z_add(BASLIK.clone()));
    println!("{}", "    VERSIYON = ".to_string().z_add(VERSIYON.clone()));
    println!("{}", "    const_testi_fn() = ".to_string().z_add(const_testi_fn()));
}

async fn interpolation_testi() -> () {
    println!("{}", "  [INTERP] String interpolation testi:".to_string());
    let mut isim = "Dunya".to_string();
    let mut yas = 42;
    let mut mesaj = format!("Merhaba {}, yasin {}!", isim, yas);
    println!("{}", "    ".to_string().z_add(mesaj.clone()));
    let mut a = 10;
    let mut b = 20;
    println!("{}", format!("    {} + {} = {}", a, b, a.clone().z_add(b.clone())));
    let mut puan = 95;
    let mut not_val = not_hesapla(puan.clone());
    println!("{}", format!("    Puan: {} -> Not: {}", puan, not_val));
}

fn tuple_topla(t: (i64, i64)) -> i64 {
    return t.0.z_add(t.1);
}

async fn tuple_testi() -> () {
    println!("{}", "  [TUPLE] Tuple testi:".to_string());
    let mut t = (10, 20);
    println!("{}", "    t.0 = ".to_string().z_add(t.0));
    println!("{}", "    t.1 = ".to_string().z_add(t.1));
    println!("{}", "    tuple_topla(t) = ".to_string().z_add(tuple_topla(t.clone())));
    let mut t2 = (100, 200, 300);
    println!("{}", "    t2 = (".to_string().z_add(t2.0).z_add(", ".to_string()).z_add(t2.1).z_add(", ".to_string()).z_add(t2.2).z_add(")".to_string()));
}

async fn cok_boyutlu_dizi_testi() -> () {
    println!("{}", "  [MATRIX] Cok boyutlu dizi testi:".to_string());
    let mut matris = vec![vec![1, 2, 3], vec![4, 5, 6], vec![7, 8, 9]];
    println!("{}", "    matris[0][0] = ".to_string().z_add(matris[(0 as usize)][(0 as usize)]));
    println!("{}", "    matris[1][1] = ".to_string().z_add(matris[(1 as usize)][(1 as usize)]));
    println!("{}", "    matris[2][2] = ".to_string().z_add(matris[(2 as usize)][(2 as usize)]));
    let mut kosegen = matris[(0 as usize)][(0 as usize)].z_add(matris[(1 as usize)][(1 as usize)]).z_add(matris[(2 as usize)][(2 as usize)]);
    println!("{}", "    Kosegen toplami = ".to_string().z_add(kosegen.clone()));
}

fn negatif_al(n: i64) -> i64 {
    return (-n);
}

async fn unary_testi() -> () {
    println!("{}", "  [UNARY] Unary operator testi:".to_string());
    let mut a = 42;
    println!("{}", "    -42 = ".to_string().z_add((-a)));
    println!("{}", "    negatif_al(7) = ".to_string().z_add(negatif_al(7)));
    println!("{}", "    -(-5) = ".to_string().z_add(negatif_al(negatif_al(5))));
    let mut b = true;
    println!("{}", "    !true = ".to_string().z_add((!b)));
    println!("{}", "    !false = ".to_string().z_add((!(!b))));
}

fn esit_mi_bool(a: i64, b: i64) -> bool {
    return (a == b);
}

fn buyuk_mu_bool(a: i64, b: i64) -> bool {
    return (a > b);
}

fn asal_mi_bool(n: i64) -> bool {
    if (n <= 1) {
        return false;
    }
    if (n <= 3) {
        return true;
    }
    if ((n % 2) == 0) {
        return false;
    }
    if ((n % 3) == 0) {
        return false;
    }
    return true;
}

async fn bool_karsilastirma_testi() -> () {
    println!("{}", "  [BOOL-CMP] Bool donuslu karsilastirma:".to_string());
    println!("{}", "    5 == 5 ? ".to_string().z_add(esit_mi_bool(5, 5)));
    println!("{}", "    5 == 3 ? ".to_string().z_add(esit_mi_bool(5, 3)));
    println!("{}", "    7 > 3  ? ".to_string().z_add(buyuk_mu_bool(7, 3)));
    println!("{}", "    2 > 8  ? ".to_string().z_add(buyuk_mu_bool(2, 8)));
    println!("{}", "    7 asal mi? ".to_string().z_add(asal_mi_bool(7)));
    println!("{}", "    4 asal mi? ".to_string().z_add(asal_mi_bool(4)));
}

async fn user_main() -> () {
    println!("{}", "============================================================".to_string());
    println!("{}", "  ZET LANG v0.3 — KAPSAMLI OZELLIK TESTI".to_string());
    println!("{}", "============================================================".to_string());
    println!("{}", "".to_string());
    println!("{}", "[BOLUM 1] Deterministik Fonksiyonlar".to_string());
    println!("{}", "------------------------------------------------------------".to_string());
    println!("{}", "  topla(3, 7) = ".to_string().z_add(topla(3, 7)));
    println!("{}", "  cikar(10, 4) = ".to_string().z_add(cikar(10, 4)));
    println!("{}", "  carp(6, 8) = ".to_string().z_add(carp(6, 8)));
    println!("{}", "  bol(20, 4) = ".to_string().z_add(bol(20, 4)));
    println!("{}", "  kare_al(9) = ".to_string().z_add(kare_al(9)));
    println!("{}", "  kup_al(3) = ".to_string().z_add(kup_al(3)));
    println!("{}", "  ortalama_iki(10, 20) = ".to_string().z_add(ortalama_iki(10, 20)));
    det_print_testi();
    println!("{}", "".to_string());
    println!("{}", "[BOLUM 2] Rekursif Fonksiyonlar".to_string());
    println!("{}", "------------------------------------------------------------".to_string());
    println!("{}", "  fibonacci(10) = ".to_string().z_add(fibonacci(10)));
    println!("{}", "  faktoriyel(8) = ".to_string().z_add(faktoriyel(8)));
    println!("{}", "  us_al(2, 10) = ".to_string().z_add(us_al(2, 10)));
    println!("{}", "  gcd(48, 18) = ".to_string().z_add(gcd(48, 18)));
    println!("{}", "".to_string());
    println!("{}", "[BOLUM 3] Kontrol Yapilari".to_string());
    println!("{}", "------------------------------------------------------------".to_string());
    for_testi().await;
    for_step_testi().await;
    while_testi().await;
    ic_ice_dongu().await;
    karisik_kontrol().await;
    while_if_testi().await;
    for_negatif_step().await;
    println!("{}", "".to_string());
    println!("{}", "[BOLUM 4] Diziler".to_string());
    println!("{}", "------------------------------------------------------------".to_string());
    dizi_testi().await;
    dizi_dongu_testi().await;
    println!("{}", "  dizi_toplam_hesapla() = ".to_string().z_add(dizi_toplam_hesapla()));
    println!("{}", "".to_string());
    println!("{}", "[BOLUM 5] String Islemleri".to_string());
    println!("{}", "------------------------------------------------------------".to_string());
    string_birlestirme_testi().await;
    string_carpim_testi().await;
    det_string_testi().await;
    println!("{}", "".to_string());
    println!("{}", "[BOLUM 6] Karsilastirma Operatorleri".to_string());
    println!("{}", "------------------------------------------------------------".to_string());
    karsilastirma_testi().await;
    println!("{}", "".to_string());
    println!("{}", "[BOLUM 7] Ileri Matematik".to_string());
    println!("{}", "------------------------------------------------------------".to_string());
    matematik_testi().await;
    println!("{}", "".to_string());
    println!("{}", "[BOLUM 8] Not Hesaplama".to_string());
    println!("{}", "------------------------------------------------------------".to_string());
    not_testi().await;
    println!("{}", "".to_string());
    println!("{}", "[BOLUM 9] Return Ifadesi".to_string());
    println!("{}", "------------------------------------------------------------".to_string());
    println!("{}", "  basit_return() = ".to_string().z_add(basit_return()));
    println!("{}", "  kosullu_return(5) = ".to_string().z_add(kosullu_return(5)));
    println!("{}", "  kosullu_return(-3) = ".to_string().z_add(kosullu_return((0 - 3))));
    void_return_testi();
    println!("{}", "".to_string());
    println!("{}", "[BOLUM 10] Scope ve Spawn".to_string());
    println!("{}", "------------------------------------------------------------".to_string());
    basit_scope_testi().await;
    scope_hesap_testi().await;
    coklu_scope_testi().await;
    scope_karmasik_testi().await;
    println!("{}", "".to_string());
    println!("{}", "[BOLUM 11] Call Anahtar Kelimesi".to_string());
    println!("{}", "------------------------------------------------------------".to_string());
    call_testi().await;
    call_nondet_testi().await;
    to_int_testi().await;
    metin_to_sayi().await;
    hesap_ve_yazdir().await;
    println!("{}", "".to_string());
    println!("{}", "[BOLUM 12] Zaman Olcumu".to_string());
    println!("{}", "------------------------------------------------------------".to_string());
    let mut gecen = zaman_olc().await;
    println!("{}", "  Bos zaman olcumu: ".to_string().z_add(gecen.clone()).z_add(" ms".to_string()));
    let mut zaman = zaman_damgasi().await;
    println!("{}", "  Zaman damgasi: ".to_string().z_add(zaman.clone()));
    println!("{}", "".to_string());
    println!("{}", "[BOLUM 13] JSON Islemleri".to_string());
    println!("{}", "------------------------------------------------------------".to_string());
    json_testi().await;
    json_karmasik_testi().await;
    println!("{}", "".to_string());
    println!("{}", "[BOLUM 14] Boolean Degerleri".to_string());
    println!("{}", "------------------------------------------------------------".to_string());
    boolean_testi().await;
    println!("{}", "".to_string());
    println!("{}", "[BOLUM 15] Fonksiyon Zincirleri".to_string());
    println!("{}", "------------------------------------------------------------".to_string());
    zincir_testi().await;
    println!("{}", "".to_string());
    println!("{}", "[BOLUM 16] Det Fonksiyonda Dongu ve Dizi".to_string());
    println!("{}", "------------------------------------------------------------".to_string());
    det_dongu_testi();
    det_while_testi();
    det_dizi_testi();
    println!("{}", "".to_string());
    println!("{}", "[BOLUM 17] Buyuk Dongular".to_string());
    println!("{}", "------------------------------------------------------------".to_string());
    buyuk_dongu_testi().await;
    println!("{}", "".to_string());
    println!("{}", "[BOLUM 18] Dizi Hesaplamalari".to_string());
    println!("{}", "------------------------------------------------------------".to_string());
    dizi_hesaplama_testi().await;
    println!("{}", "".to_string());
    println!("{}", "[BOLUM 19] Dongu Karisimi (Yildiz Ucgeni)".to_string());
    println!("{}", "------------------------------------------------------------".to_string());
    dongu_karisim_testi().await;
    println!("{}", "".to_string());
    println!("{}", "[BOLUM 20] Fibonacci Serisi".to_string());
    println!("{}", "------------------------------------------------------------".to_string());
    fibonacci_serisi().await;
    println!("{}", "".to_string());
    println!("{}", "[BOLUM 21] Faktoriyel Tablosu".to_string());
    println!("{}", "------------------------------------------------------------".to_string());
    faktoriyel_tablosu().await;
    println!("{}", "".to_string());
    println!("{}", "[BOLUM 22] Asal Sayilar".to_string());
    println!("{}", "------------------------------------------------------------".to_string());
    asal_sayi_testi().await;
    println!("{}", "".to_string());
    println!("{}", "[BOLUM 23] Sezar Sifreleme".to_string());
    println!("{}", "------------------------------------------------------------".to_string());
    sezar_testi().await;
    println!("{}", "".to_string());
    println!("{}", "[BOLUM 24] Collatz Conjecture".to_string());
    println!("{}", "------------------------------------------------------------".to_string());
    collatz_testi().await;
    println!("{}", "".to_string());
    println!("{}", "[BOLUM 25] Benchmark".to_string());
    println!("{}", "------------------------------------------------------------".to_string());
    benchmark_testi().await;
    buyuk_benchmark().await;
    println!("{}", "".to_string());
    println!("{}", "[BOLUM 29] Yeni Tipler (f64, char, bool)".to_string());
    println!("{}", "------------------------------------------------------------".to_string());
    f64_testi().await;
    char_testi().await;
    bool_testi_v2().await;
    println!("{}", "".to_string());
    println!("{}", "[BOLUM 30] Yeni Operatorler (%, &&, ||, !, &, |, ^, <<, >>)".to_string());
    println!("{}", "------------------------------------------------------------".to_string());
    modulo_testi().await;
    mantiksal_operator_testi().await;
    bitwise_testi().await;
    println!("{}", "".to_string());
    println!("{}", "[BOLUM 31] Break ve Continue".to_string());
    println!("{}", "------------------------------------------------------------".to_string());
    break_testi().await;
    continue_testi().await;
    break_continue_for().await;
    println!("{}", "".to_string());
    println!("{}", "[BOLUM 32] Const Tanimlamalari".to_string());
    println!("{}", "------------------------------------------------------------".to_string());
    const_testi().await;
    println!("{}", "".to_string());
    println!("{}", "[BOLUM 33] String Interpolation".to_string());
    println!("{}", "------------------------------------------------------------".to_string());
    interpolation_testi().await;
    println!("{}", "".to_string());
    println!("{}", "[BOLUM 34] Tuple (Demet)".to_string());
    println!("{}", "------------------------------------------------------------".to_string());
    tuple_testi().await;
    println!("{}", "".to_string());
    println!("{}", "[BOLUM 35] Cok Boyutlu Diziler".to_string());
    println!("{}", "------------------------------------------------------------".to_string());
    cok_boyutlu_dizi_testi().await;
    println!("{}", "".to_string());
    println!("{}", "[BOLUM 36] Unary Operatorler".to_string());
    println!("{}", "------------------------------------------------------------".to_string());
    unary_testi().await;
    println!("{}", "".to_string());
    println!("{}", "[BOLUM 37] Bool Donuslu Fonksiyonlar".to_string());
    println!("{}", "------------------------------------------------------------".to_string());
    bool_karsilastirma_testi().await;
    println!("{}", "".to_string());
    println!("{}", "[BOLUM 26] Validate (Taint Analysis)".to_string());
    println!("{}", "------------------------------------------------------------".to_string());
    let mut test_veri1 = input("Test verisi girin (veya Enter): ".to_string()).await;
    validate_parametre_testi(test_veri1.clone()).await;
    let mut sayi_veri = input("Bir sayi girin: ".to_string()).await;
    validate_hesap_testi(sayi_veri.clone()).await;
    let mut isim_veri = input("Adinizi girin: ".to_string()).await;
    validate_isim_testi(isim_veri.clone()).await;
    println!("{}", "".to_string());
    println!("{}", "[BOLUM 27] Coklu Validate".to_string());
    println!("{}", "------------------------------------------------------------".to_string());
    let mut v1 = input("Birinci deger: ".to_string()).await;
    let mut v2 = input("Ikinci deger: ".to_string()).await;
    coklu_validate_testi(v1.clone(), v2.clone()).await;
    println!("{}", "".to_string());
    println!("{}", "[BOLUM 28] Karmasik Validate".to_string());
    println!("{}", "------------------------------------------------------------".to_string());
    let mut kv = input("Bir sayi girin (karmasik test): ".to_string()).await;
    validate_karmasik_testi(kv.clone()).await;
    println!("{}", "".to_string());
    println!("{}", "============================================================".to_string());
    println!("{}", "  TUM TESTLER TAMAMLANDI!".to_string());
    println!("{}", "  Zet Lang v0.3 - Basariyla calisti.".to_string());
    println!("{}", "============================================================".to_string());
}

#[tokio::main] async fn main() {
    user_main().await;
}