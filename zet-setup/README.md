# ZET PROGRAMLAMA DİLİ — v0.4

> **"Zero Trust — Güvenmediğin Kodu Derleme"**

---

## 🚀 v0.4 Sürümündeki Yenilikler
* **Hata Yönetimi (Error Handling):** `T!` dönüş tipleri, `error("msg")`, `?` (propagate) operatörü ve `catch` (fallback) bloğu.
* **Backend First Sınıf:** Dahili HTTP Router (`@get`, `@post`) ve Zero-Trust ile ağ verisi denetimi (SQLite entegrasyonu).
* **Modül Sistemi (`import`):** Projeleri parçalara bölme ve `std.http`, `std.db`, `std.file` kütüphanelerine erişim.
* **Veri Tipleri & Sözdizimi:** Özel struct tanımlamaları (`struct`), array mutasyonları, pattern matching (`match`) ve `+=`, `-=` gibi hızlı atamalar.
* **Dil Sunucusu (LSP):** Editor/IDE kullanımı için anlık Taint Analizi ve yazım hataları gösterimi (`zet-compiler --lsp`).

---

## Nasıl Kurulur?

1. `kurulum.bat` dosyasına sağ tıklayıp **"Yönetici Olarak Çalıştır"** deyin.
2. Kurulum bitince terminali kapatın.
3. Yeni bir terminal (CMD veya PowerShell) açın.

> **Not:** Bilgisayarınızda [Rust](https://rustup.rs/) kurulu olmalıdır.

## Nasıl Çalıştırılır?

### Kurulumdan sonra (herhangi bir klasörden):
```
zet dosyaniz.zt
```

### Kurulum yapmadan (ZetSetup klasöründen):
```
zet.bat dosyaniz.zt
```

### Çift tıklama:
`zet.bat` dosyasına çift tıklayıp dosya yolunu girebilir veya `.zt` dosyasını sürükleyebilirsiniz.

---

## Dil Rehberi

### 1. Değişkenler ve Tipler

```
let x = 10                   // i64 (Tamsayı)
let pi = 3.14                // f64 (Ondalıklı sayı)
let aktif = true             // bool
let harf = 'A'               // char (Karakter)
let bayt: u8 = 255           // u8 (Byte)
let isim = "Zet"             // String
let mut y = 20               // Değiştirilebilir değişken
y = 30                       // Değer atama
const MAX = 100              // Sabit (değiştirilemez)
let sayilar = [1, 2, 3]     // Dizi
let ilk = sayilar[0]        // Dizi erişimi
let cift = (10, "on")       // Tuple
let ilk_el = cift.0         // Tuple erişimi (index ile)

// Hızlı Atamalar (v0.4)
let mut sayi = 10
sayi += 5                    // +=, -=, *=, /=
```

### 2. Dizi (Array) İşlemleri

```
let mut dizi = [10, 20, 30]
let ilk = dizi[0]            // 10
dizi[1] = 99                 // Dizi elemanı güncelleme (v0.4)
```

### 3. Operatörler

```
// Aritmetik
let a = 10 + 3               // 13
let b = 10 - 3               // 7
let c = 10 * 3               // 30
let d = 10 / 3               // 3
let e = 10 % 3               // 1 (Mod)

// Karşılaştırma
let f = 10 > 3               // true
let g = 10 == 10             // true
let h = 10 != 5              // true
let i = 10 >= 10             // true

// Mantıksal
let j = true && false         // false
let k = true || false         // true
let l = !true                 // false

// Bitwise
let m = 5 & 3                // 1  (AND)
let n = 5 | 3                // 7  (OR)
let o = 5 ^ 3                // 6  (XOR)
let p = 1 << 3               // 8  (Sola kaydırma)
let q = 8 >> 2               // 2  (Sağa kaydırma)
```

### 3. Kontrol Yapıları

```
if x > 10 {
    println("Buyuk")
} else {
    println("Kucuk")
}

for i in 0..10 {             // 0'dan 9'a
    println(i)
}

for i in 0..100 by 5 {       // 5'er 5'er adım
    println(i)
}

while x < 50 {
    x = x + 1
}
```

### 4. Döngü Kontrolü (`break` / `continue`)

```
for i in 0..100 {
    if i == 5 {
        break                 // Döngüden çık
    }
}

for i in 0..10 {
    if i % 2 == 0 {
        continue              // Bu adımı atla
    }
    println(i)                // Sadece tek sayılar
}
```

### 5. String Interpolation

```
let isim = "Zet"
let yas = 1
println("Merhaba ${isim}, yas: ${yas}")
// Çıktı: Merhaba Zet, yas: 1
```

### 6. Fonksiyon Tipleri

Zet'te iki tip fonksiyon vardır:

**A) Deterministik:** Saf matematik, yan etkisi yok. Daima aynı girdi → aynı çıktı.

```
deterministic fn topla(a: i64, b: i64) -> i64 {
    return a + b
}
```

> Det fonksiyonlar içinde `print`, `println`, döngü, dizi, if/else kullanılabilir.  
> Ancak I/O (input, HTTP, Util.now) **kullanılamaz**.

**B) Nondeterministik:** I/O, network, zaman gibi dış dünya ile etkileşim.

```
nondeterministic fn main() -> Void {
    println("Merhaba Dunya!")
}
```

### 7. Fonksiyon Çağırma (`call` vs normal)

**Normal çağrı** — Deterministik fonksiyonlar:
```
let sonuc = topla(10, 20)
```

**`call` ile çağrı** — Nondeterministik (I/O) işlemler:
```
let veri = call HTTP.get("https://...")
let isim = call input("Adiniz: ")
let zaman = call Util.now()
```

> `call` komutu işlem bitene kadar programı o satırda bekletir.

### 8. Kullanıcı Girdisi (`input` / `inputln`)

```
let isim = call input("Adin nedir? ")     // Aynı satırda bekler
let sehir = call inputln("Sehriniz?")     // Alt satırda bekler
```

> `input`/`inputln` her zaman `Untrusted` döner!  
> Kullanmadan önce `validate` etmelisiniz.

### 9. Güvenlik (`validate`) — Zero Trust

Dışarıdan gelen her veri (input, HTTP, args) **"Untrusted"** kabul edilir.  
`validate` bloğuna almadan kullanamazsınız — derleyici bunu **derleme zamanında** zorlar.

```
let girdi = call input("Bir sayi girin: ")

validate girdi {
    success: {
        // Burada 'girdi' artık güvenli String
        println("Girdiniz: " + girdi)
    }
}
```

### 10. Eşzamanlılık — Structured Concurrency

`spawn` **yalnızca** `scope` içinde kullanılabilir — dışarıda derleyici hata verir.

```
scope IsimAdSoyadToplanti {
    spawn println("Gorev A")
    spawn println("Gorev B")
    spawn println("Gorev C")
}
// Scope kapanmadan program ilerlemez.
// Zombie process oluşması yapısal olarak imkansızdır.
```

### 11. JSON İşlemleri

```
let veri = "{\"isim\":\"Zet\",\"versiyon\":3}"
validate veri {
    success: {
        let isim = json(veri, "isim")
        println(isim)
    }
}
```

### 12. Yerleşik Kütüphaneler (stdlib)

| Kütüphane | Kullanım | Açıklama |
|-----------|----------|----------|
| **print** | `print("Mesaj")` | Ekrana yazar (satır sonu yok) |
| **println** | `println("Mesaj")` | Ekrana yazar (satır sonu var) |
| **input** | `call input("Soru?")` | Kullanıcıdan girdi alır (Untrusted döner) |
| **inputln** | `call inputln("Soru?")` | Kullanıcıdan girdi alır, alt satırda bekler |
| **Util.now** | `call Util.now()` | Şu anki zaman (ms cinsinden, i64) |
| **Util.to_int** | `call Util.to_int("123")` | String → i64 dönüşümü |
| **Util.to_float** | `call Util.to_float("3.14")` | String → f64 dönüşümü |
| **HTTP.get** | `call HTTP.get("url")` | HTTP GET isteği (Untrusted döner) |
| **json** | `json(veri, "key")` | JSON içinden veri çeker |

---

### 13. String İşlemleri

```
let a = "Merhaba" + " " + "Dunya"     // Birleştirme
let b = "Sayi: " + 42                  // String + i64
let c = "-" * 20                       // String tekrarlama
let d = "Aktif: " + true               // String + bool
let e = "Pi: " + 3.14                  // String + f64
let f = "Harf: " + 'A'                 // String + char
let g = "Ad: ${isim}, Yas: ${yas}"    // String interpolation
```

### 14. Struct (Veri Yapıları)

```
struct Kullanici {
    id: i64,
    isim: String,
    aktif: bool
}

let k = Kullanici { id: 1, isim: "Ali", aktif: true }
println(k.isim)
```

### 15. Pattern Matching (`match`)

```
let x = 10
match x {
    5 => println("Bes"),
    10 => println("On"),
    _ => println("Diger")
}
```

### 16. Hata Yönetimi (`!`, `?`, `catch`)

```
// ! işareti fonksiyonun hata dönebileceğini belirtir
nondet fn bolme(a: i64, b: i64) -> i64! {
    if b == 0 {
        return error("Sifira bolme!")
    }
    return a / b
}

nondet fn test() -> Void {
    // catch: Hata gelirse varsayılan değer dön
    let x = bolme(10, 0) catch 999 
    
    // ?: Hata gelirse direkt üst fonksiyona aktar
    let y = bolme(10, 2)? 
}
```

### 17. Modül Sistemi (`import`)

Projelerinizi çoklu dosyalara bölebilirsiniz.
```
import std.http
import myproject.utils.math
```

### 18. Backend İlk-Sınıf Yönlendirme (HTTP Router)

```
import std.http

@get("/hello")
nondet fn merhaba() -> String {
    return "Merhaba Dünya!"
}

@post("/data")
nondet fn data_al(payload: Untrusted) -> String {
    validate payload {
        success: { return "Güvenli Veri!" }
        fail: { return "Geçersiz İstek!" }
    }
}
```

---

*Hazırlayan: Ceky*
