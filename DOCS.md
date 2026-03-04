# 📘 Zet Lang Resmi Dökümantasyonu (v0.2)

Zet Lang'e hoş geldiniz. Bu dökümantasyon, dilin sözdizimini (syntax), temel konseptlerini, güvenlik modelini ve standart kütüphanesini içerir.

---

## 📑 İçindekiler

1. [Temel Konseptler](#1-temel-konseptler)
2. [Sözdizimi ve Değişkenler](#2-sözdizimi-ve-değişkenler)
3. [Kontrol Yapıları](#3-kontrol-yapıları)
4. [Fonksiyonlar (Saf ve Kirli)](#4-fonksiyonlar-saf-ve-kirli)
5. [Güvenlik Mimarisi (Validation)](#5-güvenlik-mimarisi-validation)
6. [Eşzamanlılık (Concurrency & Scope)](#6-eşzamanlılık-concurrency--scope)
7. [Standart Kütüphane (Stdlib)](#7-standart-kütüphane-stdlib)

---

## 1. Temel Konseptler

Zet, diğer yüksek seviyeli dillerden farklı bir zihniyete sahiptir. Yazdığınız kodu derlemeden önce şu kuralları işletir:

- **Sıfır Güven (Zero Trust):** Dış dünyadan alınan (Ağ, Dosya, Terminal) hiçbir veri doğrudan işlenemez. `validate` bloğu olmadan kullanmaya çalışmak **derleme hatası** verir.
- **Belirleyicilik (Determinism):** Ağ veya I/O işlemi yapan fonksiyonlar ile sadece CPU kullanan fonksiyonlar dil seviyesinde birbirinden ayrılır. `deterministic` fonksiyon içinde I/O çağrısı **derleme hatası** verir.
- **Yapısal Eşzamanlılık:** `spawn` edilen her arka plan görevi bir `scope` bloğu içinde yaşamak zorundadır. Scope dışında `spawn` kullanmak **derleme hatası** verir.

---

## 2. Sözdizimi ve Değişkenler

Zet, statik tipli ancak tip çıkarımı (type inference) yapabilen modern bir sözdizimine sahiptir.

### Değişken Tanımlama

Değişkenler `let` anahtar kelimesi ile tanımlanır.

```zet
let yas = 20
let isim = "Zet"
```

### Veri Tipleri

Şu an desteklenen temel veri tipleri şunlardır:

| Tip | Açıklama |
| --- | --- |
| `i64` | 64-bit Tamsayılar |
| `String` | Metin dizileri |
| `Array<T>` | Aynı tipteki verilerin listesi |
| `Untrusted` | Dışarıdan gelen, henüz doğrulanmamış kirli veri |
| `Void` | Değer döndürmeyen fonksiyonların tipi |

### Diziler (Arrays)

Diziler köşeli parantezlerle tanımlanır ve indeks ile erişilir.

```zet
let sayilar = [10, 20, 30, 40]
let ilk_eleman = sayilar[0]
```

---

## 3. Kontrol Yapıları

### İf / Else İfadeleri

```zet
if yas > 18 {
    spawn DB.log("Giris izni verildi.")
} else {
    spawn DB.log("Giris reddedildi.")
}
```

### Döngüler (Loops)

Zet, aralık (range) tabanlı `for` döngülerini ve koşullu `while` döngülerini destekler.

```zet
// 0'dan 4'e kadar (4 dahil değil) döner
for i in 0..4 {
    spawn DB.log("Sayac: " + i)
}

let x = 0
while x < 5 {
    x = x + 1
}
```

---

## 4. Fonksiyonlar (Saf ve Kirli)

Zet'te fonksiyonlar, I/O (Girdi/Çıktı) yapıp yapmadıklarına göre ikiye ayrılır. Derleyici bu sayede kodunuzu en yüksek hızda optimize eder.

### Deterministic (Saf) Fonksiyonlar

Sadece RAM ve CPU kullanır. Asenkron bir işlem içermez. "Native C/Rust" hızında, hiçbir VM engeline takılmadan çalışır. **I/O çağrısı içerirse derleme hatası verir.**

```zet
deterministic fn topla(a: i64, b: i64) -> i64 {
    return a + b
}
```

### Nondeterministic (Kirli/I-O) Fonksiyonlar

İçerisinde Ağ isteği, konsol girdisi veya bekleme süresi barındıran fonksiyonlardır. Arka planda otomatik olarak Asenkron (Async/Await) hale getirilirler.

```zet
nondeterministic fn veri_cek() -> Void {
    // I/O işlemleri burada yapılır
}
```

### `call` Anahtar Kelimesi

Bir I/O (Nondeterministic) işleminin sonucunu beklemek istiyorsanız `call` kelimesini kullanmalısınız. Bu, işlemi başlatan işçiyi duraklatır ancak tüm programı dondurmaz. **`call` yalnızca nondeterministic fonksiyonlar için kullanılabilir; saf fonksiyona `call` eklemek derleme hatası verir.**

```zet
let zaman = call Util.now()
let web_verisi = call HTTP.get("https://api.ornek.com")
```

---

## 5. Güvenlik Mimarisi (Validation)

Zet'in kalbi **Leke Analizi (Taint Analysis)** sistemidir. Dış dünyadan gelen veriler (`Console.read`, `HTTP.get` vb.) `Untrusted` tipindedir. Bu veriyi standart değişkenlere atayamaz veya işlemlere sokamazsınız. **Derleyici, lekeli verinin `validate` bloğu olmadan kullanılmasını engeller.**

Bunu çözmek için `validate` bloğu kullanılmalıdır:

```zet
let kullanici_girdisi = call Console.read("Adiniz: ")

// Derleyici bu blok olmadan islem yapmaniza izin vermez!
validate kullanici_girdisi {
    success: {
        // kullanici_girdisi burada "String" (Trusted) tipine donusur
        scope Loglama {
            spawn DB.log("Giris yapan: " + kullanici_girdisi)
        }
    }
}
```

---

## 6. Eşzamanlılık (Concurrency & Scope)

Arka planda aynı anda birden fazla iş yapmak (Multi-threading) Zet'te çok kolay ve güvenlidir.

### `spawn` (Ateşle ve Unut)

Bir fonksiyonu veya işlemi ana akışı durdurmadan arka planda başlatır. **`spawn` yalnızca `scope` bloğu içinde kullanılabilir; aksi takdirde derleme hatası verir.**

```zet
scope Islemler {
    spawn ag_istegi_gonder()
    spawn DB.log("Bu yazi aninda ekrana basilir.")
}
```

### `scope` (Kapsam / Şantiye Şefi)

Zombi süreçleri engellemek için, `spawn` edilen tüm işlemler bir `scope` bloğu içinde olmak zorundadır. Scope bloğu, içindeki tüm işçiler görevini bitirmeden kapanmaz ve alt satıra geçilmez.

```zet
scope VeriIslemleri {
    // Bu iki islem ayni anda, paralel olarak baslar
    spawn HTTP.get("https://api.1.com")
    spawn HTTP.get("https://api.2.com")
}
// Kod buraya geldiginde, her iki HTTP isteginin de bittigi garanti altindadir.
```

---

## 7. Standart Kütüphane (Stdlib)

Zet v0.2 ile birlikte gelen yerleşik modüller:

### Konsol (Console)

- `call Console.read(mesaj: String) -> Untrusted` — Kullanıcıdan terminal üzerinden veri okur.

### Veritabanı / Loglama (DB)

- `spawn DB.log(mesaj: String) -> Void` — Ekrana formatlanmış sistem logu basar. *(Gelecekte doğrudan DB bağlantısı sağlayacaktır.)*

### İnternet (HTTP)

- `call HTTP.get(url: String) -> Untrusted` — Belirtilen URL'ye asenkron HTTP GET isteği atar. Sonuç `Untrusted` tipindedir, kullanmadan önce `validate` gerekir.

### Araçlar (Util)

- `call Util.now() -> i64` — Sistem saatini Unix Epoch (milisaniye) cinsinden döndürür. Hız testleri için idealdir.
- `call Util.to_int(veri: String) -> i64` — Metinsel ifadeyi tam sayıya (Integer) çevirir.

### JSON İşlemleri

- `json(veri: String, anahtar: String) -> String` — Verilen JSON metninin içinden, belirtilen anahtara (key) ait değeri çıkarır.