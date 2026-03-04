# 👁️ Zet Lang: Dil Felsefesi ve Tasarım Manifestosu

*"Yüksek Seviye Konforu, Düşük Seviye Performansı ve Sıfır Toleranslı Güvenlik."*

Zet Lang (Zero Trust), modern yazılım dünyasındaki temel bir açmazı çözmek için tasarlanmıştır: **Geliştirici hızı ile çalışma zamanı (runtime) güvenliğinin sürekli çatışması.** Geleneksel dillerde ya C/C++ gibi donanıma hükmeder ama bellek hatalarıyla (memory leaks, segfaults) boğuşursunuz; ya da Python/Java/Go gibi dillerde çöp toplayıcılara (Garbage Collector) ve sanal makinelere (VM) boyun eğip performanstan taviz verirsiniz. Ayrıca bu dillerin hiçbiri, dışarıdan gelen verinin güvenliğini dil seviyesinde garanti etmez.

Zet, bu paradigmayı yıkmak için üç temel sütun üzerine inşa edilmiştir.

---

## 1. Sıfır Güven Mimarisi (Zero Trust & Taint Analysis)

Zet'in en katı kuralı şudur: **Dış dünyadan gelen hiçbir veriye güvenilmez.**
Bir ağ isteği (HTTP), bir veritabanı sorgusu veya basit bir klavye girdisi (`Console.read`) her zaman `Untrusted` (Güvensiz/Lekeli) veri tipi olarak doğar. 

* **Derleyici Zorlaması:** Zet derleyicisi, `Untrusted` bir verinin `validate` (doğrulama) bloğundan geçmeden herhangi bir kritik işleme (matematik, veritabanı yazması, sistem çağrısı) girmesine **izin vermez.**
* **Çalışma Zamanı Değil, Derleme Zamanı:** Diğer dillerde güvenlik açıkları (SQL Injection, XSS vb.) program çalışırken (runtime) patlar veya sızar. Zet'te ise kod eğer güvensizse **derlenmez.**

## 2. Otonom Bellek Yönetimi (No GC, No Malloc)

Zet, bellek yönetimini bir "otomatik pilot" edasıyla halleder. 
* **Garbage Collector (Çöpçü) Yoktur:** Arka planda aniden çalışıp programı duraksatan (micro-stuttering) bir çöpçü mekanizması yoktur.
* **Manuel Yönetim Yoktur:** Geliştirici `malloc` veya `free` yazarak bellekle uğraşmaz.
* **Kapsam Temelli Yaşam (RAII & Ownership):** Her değişken, içinde doğduğu kapsamla (`{ ... }` veya `scope` bloğu) yaşar. Kapsam bittiği milisaniye, bellek iade edilir. Bu sayede C/Rust seviyesinde bir hız elde edilirken, Python seviyesinde bir yazım kolaylığı sağlanır.

## 3. Hibrit ve Akıllı Motor (Deterministic vs Nondeterministic)

Zet, yazdığınız fonksiyonun "ne tür bir iş yaptığını" bilir.
* **Deterministic (Saf) Fonksiyonlar:** Sadece CPU ve RAM kullanan (örneğin karmaşık matematik hesaplamaları) fonksiyonlardır. Derleyici bunları araya hiçbir asenkron yük bindirmeden, en saf ve optimize makine koduna çevirir.
* **Nondeterministic (Kirli) Fonksiyonlar:** I/O (Ağ, Disk, Bekleme) yapan fonksiyonlardır. Derleyici bunları otomatik olarak "Green Thread" mimarisine (Async/Await) sarar.
Geliştirici arka planda dönen bu karmaşayı görmez, sadece `call` ve `spawn` kelimeleriyle orkestrayı yönetir.

## 4. Yapısal Eşzamanlılık (Structured Concurrency)

Modern backend sistemlerinde en büyük sorunlardan biri başıboş kalan, unutulan veya çöken arka plan işlemleridir (Zombie Processes).
Zet, eşzamanlılığı `scope` (kapsam) blokları içine hapseder. Bir `scope` içinde başlatılan (`spawn`) hiçbir işlem bitmeden, o `scope` kapanamaz. 

> *"Karanlıkta ateş edip merminin nereye gittiğini unutmazsınız. Zet'te sıktığınız her mermi (thread), şarjöre (scope) hesap vermek zorundadır."*

---

## Sonuç: Neden Zet?

Zet Lang; **"Patron gibi kod yazıp, yarışçı gibi çalışmak"** isteyen backend ve sistem mühendisleri için tasarlanmıştır. Geliştiriciyi hamallıktan (pointer aritmetiği, bellek temizliği) kurtarır, ancak onu güvenlik konusunda zorla disipline eder. 

Zet, size bir hata yapma hakkı tanımaz; çünkü **güvenmediği kodu derlemez.**