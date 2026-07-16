
#![allow(dead_code, unused_imports, unused_variables, unused_parens, unused_mut, non_snake_case)]
use ::std::time::Duration;
use ::std::io::{self, Write};
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
        ::std::time::SystemTime::now()
            .duration_since(::std::time::UNIX_EPOCH)
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
pub mod std {
    use super::*;
    pub mod http {
        use super::*;
        pub async fn serve(port: String) -> () {
            
                let app = crate::_zet_build_router();
                let listener = ::tokio::net::TcpListener::bind(&port).await.unwrap();
                ::axum::serve(listener, app).await.unwrap();
            ;
        }
        
    }
    
}

pub async fn bolme(a: i64, b: i64) -> Result<i64, String> {
    if (b == 0) {
        return Err("Sifira bolme hatasi!".to_string().to_string());
    }
    return Ok((a / b));
}

pub async fn islem_yap() -> String {
    let mut sonuc1 = bolme(10, 0).await.unwrap_or_else(|_| { 999 });
    let mut sonuc2 = bolme(20, 2).await.unwrap_or_else(|_| { 0 });
    let mut total = sonuc1.clone().z_add(sonuc2.clone());
    return "Basarili Islem. Total: ".to_string().z_add(total.clone());
}

pub async fn hata_firlat() -> Result<String, String> {
    let mut result = bolme(10, 0).await?;
    return Ok("Bu satir hic calismayacak: ".to_string().z_add(result.clone()));
}

pub async fn hata_yakala() -> String {
    let mut sonuc = bolme(5, 0).await.unwrap_or_else(|_| { 404 });
    if (sonuc == 404) {
        return "Bolme isleminde hata oldu, catch calisti!".to_string();
    }
    return "Sonuc: ".to_string().z_add(sonuc.clone());
}

pub async fn user_main() -> () {
    println!("{}", "Error handling test sunucusu baslatiliyor...".to_string());
    std::http::serve("0.0.0.0:8081".to_string()).await;
}

#[tokio::main] async fn main() {
    user_main().await;
}

pub fn _zet_build_router() -> ::axum::Router {
    let mut app = ::axum::Router::new();
    app = app.route("/islem", ::axum::routing::get(|req: ::axum::extract::Request| async move {
        let res = islem_yap().await;
        ::axum::response::IntoResponse::into_response(res.to_string())
    }));
    app = app.route("/hata-firlat", ::axum::routing::get(|req: ::axum::extract::Request| async move {
        let res = hata_firlat().await;
        let res_str = match res { Ok(v) => v.to_string(), Err(e) => format!("Error: {}", e) };
        ::axum::response::IntoResponse::into_response(res_str)
    }));
    app = app.route("/hata-yakala", ::axum::routing::get(|req: ::axum::extract::Request| async move {
        let res = hata_yakala().await;
        ::axum::response::IntoResponse::into_response(res.to_string())
    }));
    app
}
