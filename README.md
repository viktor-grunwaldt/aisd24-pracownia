# aisd24-pracownia

Pracownia z AiSD (2024), Instytut Informatyki, Uniwersytet Wrocławski

## używanie i kompilacja lokalnie

Aby wykonać pracownię X, importujemy solve z pliku X.rs do main.rs

Aby skompilować: `cargo build --release`

## kompilacja na sprawdzaczce

Programy mają być kompilowane i uruchamiane w 64-bitowym środowisku Linux na komputerze PC.
Pamięć cache procesora sprawdzaczki to 3 MB.

Rust, kompilator rustc 1.63.0

komenda do kompilowania: `rustc –edition=2021 -C opt-level=2 -C target-feature=+crt-static`

Wymaganiem jest fakt że kompletny program musi być w jednym pliku, żadne moduły spoza biblioteki standardowej nie będą importowane.
Aby spreparować taki samodzielny plik, każdy z osobnych plików-modułów można przekleić do pliku main.rs wewnątrz bloku `mod (nazwa pliku) { zawartość pliku }`

<details>
  
<summary>Przykład takiego złączonego pliku:</summary>

- `lazy_static, buf, io, scan, sync` pochodzą z lib.rs
- `macros` pochodzi z macros.rs
- `radix` pochodzi z radix.rs

```rust
    #[macro_use]
    mod lazy_static {
        // ...
    }

    mod buf {
        // ...
    }

    mod io {
        // ...
    }

    mod scan {
        // ...
    }

    mod sync {
        // ...
    }

    mod radix {
        // ...
    }
    pub use io::{scan, stdout};

    mod macros {
        #[macro_export]
        macro_rules! scan {
            ($t:ty) => { scan::<$t>() };
            ($($t:ty),+) => { ($(scan::<$t>(),)*) };
        }

        /// Macros for writing to stdout
        #[macro_export]
        macro_rules! println {
            ($($arg:tt)*) => { {
                use std::io::Write;
                writeln!($crate::stdout(), $($arg)*).unwrap();
            } }
        }
        #[macro_export]
        macro_rules! print {
            ($($arg:tt)*) => { {
                use std::io::Write;
                write!($crate::stdout(), $($arg)*).unwrap();
            } }
        }
    }
    use radix::sort_by_x;

    fn main() {
      // body here
    }
```

</details>

## Opis zadań

### A. Genealogia

#### Dostępna pamięć: 32 MB

Na przestrzeni wieków w pewnym kraju żyło wiele kobiet; wszystkie były potomkiniami królowej Genowefy Pierwszej. Zachowały się zapisy określające, kto był matką każdej z nich. Do ustalania praw spadkowych potrzebne jest szybkie określenie, czy dana kobieta jest przodkiem innej.

#### Specyfikacja danych wejściowych

W pierwszym wierszu wejścia znajdują się dwie liczby naturalne oddzielone spacją: liczba kobiet $n \in [2,10^6]$ i liczba zapytań $q \in [1,10^5]$. Zakładamy, że kobiety są numerowane liczbami od 1 do n, a królowa Genowefa Pierwsza ma numer 1. Numeracja nie musi być chronologiczna: przykładowo kobieta o numerze 3 może być matką kobiety o numerze 2. W kolejnych n - 1 wierszach wejścia znajduje się opis drzewa genealogicznego; w i-tym z nich znajduje się jedna liczba naturalna będąca numerem matki kobiety i + 1. Następnie w każdym z kolejnych q wierszy znajdują się dwie liczby a i b oddzielone spacją, takie że 1 <= a != b <= n . Jest to zapytanie „czy kobieta a jest przodkinią kobiety b?”. Poniższe drzewo genealogiczne zostało opisane w przykładzie A.

#### Specyfikacja danych wyjściowych

Twój program powinien wypisać q wierszy. W i-tym wierszu powinna znaleźć się odpowiedź na i-te zapytanie,
będąca napisem TAK lub NIE.

### B. Trójkąty

#### Dostępna pamięć: 128 MB

Dla danego zbioru 𝐴 składającego się z 𝑛 parami różnych punktów na płaszczyźnie znajdź trzy będące wierzchołkami trójkąta o najmniejszym obwodzie. Za trójkąt uważamy również trójkąt zdegenerowany, którego wszystkie wierzchołki leżą na jednej prostej

#### Specyfikacja danych wejściowych

W pierwszym wierszu danych wejściowych znajduje się dodatnia liczba całkowita $ n\in[3, 500'000]$, będąca liczbą punktów w zbiorze 𝐴. W każdym z kolejnych 𝑛 wierszy znajdują się współrzędne kolejnego punktu, będące parą liczb całkowitych x[i] , y[i] oddzielonych pojedynczą spacją, gdzie −107 ⩽ x[i] , y[i] ⩽ 107.

#### Specyfikacja danych wyjściowych

Twój program powinien wypisać opis trójkąta o najmniejszym obwodzie, tj. trzy wiersze opisujące jego wierzchołki. W każdym wierszu powinny znaleźć się współrzędne jednego wierzchołka trójkąta oddzielone spacją (jak w danych wejściowych). Jeśli istnieje wiele trójkątów o najmniejszym obwodzie, Twój program może wypisać wierzchołki dowolnego z nich.

### C. Monety

#### Dostępna pamięć: 32 MB

W pudełku znajduje się pewna liczba monet o sumarycznej masie F gramów. Czy można bez otwierania pudełka stwierdzić, ile warte są pieniądze w środku? Przykładowo załóżmy, że dostępne na rynku monety to moneta 1-groszowa ważąca 1 gram oraz moneta 30-groszowa ważąca 50 gramów, zaś całość waży F = 100 gramów. Wtedy minimalna możliwa wartość monet w pudełku to 60 groszy (2 monety 30-groszowe), zaś maksymalna — 100 groszy (100 monet jednogroszowych).

#### Specyfikacja danych wejściowych

W pierwszym wierszu wejścia znajduje się dodatnia liczba całkowita F ⩽ 106 , będąca sumaryczną masą monet w pudełku w gramach. W drugim wierszu wejścia znajduje się dodatnia liczba całkowita C <= 100, będąca
liczbą dostępnych na rynku monet. W każdym z kolejnych C wierszy wejścia znajduje się opis 𝑖-tej monety, gdzie $i \in \{1, \dotsc , C\}$. Opis monety jest parą dodatnich liczb całkowitych oddzielonych spacją: p[i] <= 10^5 będąca nominałem w groszach i w[i] <= 10^5 będąca wagą w gramach. Może istnieć wiele monet o takim samym nominale, ale różnych wagach i wiele monet o takiej samej wadze, ale różnych nominałach.

#### Specyfikacja danych wyjściowych

Pierwszy wiersz wyjścia powinien zawierać słowo TAK, jeśli masa 𝐹 jest możliwa do uzyskania za pomocą dostępnych na rynku monet, zaś słowo NIE w przeciwnym przypadku. W przypadku odpowiedzi pozytywnej Twój program powinien wypisać cztery dodatkowe wiersze. W drugim wierszu wyjścia powinna znajdować się wtedy liczba P_min będąca możliwą sumaryczną minimalną wartością monet (w groszach) znajdujących się w pudełku. Trzeci wiersz wyjścia powinien zawierać opis uzyskania wartości P_min: C liczb naturalnych x[1..C] oddzielonych pojedynczymi spacjami, oznaczających że i-tą monetę bierzemy x[i] razy.

W czwartym wierszu wyjścia powinna znajdować się liczba P_max będąca możliwą sumaryczną maksymalną wartością monet w pudełku, zaś piąty wiersz powinien zawierać opis uzyskania P_max w identycznym formacie jak w przypadku wiersza trzeciego. Jeśli istnieje wiele możliwych sposobów uzyskania wartości P_min lub P_max , Twój program powinien opisać dowolną z nich.

### D. Chińska komórka

#### Dostępna pamięć: 32 MB

Niezbyt dawno temu wpisywanie tekstu na telefonie komórkowym wyglądało następująco. 𝐿 liter napisanych w kolejności alfabetycznej było podzielone pomiędzy 𝐾 klawiszy, tj. każdy klawisz zawierał spójny fragment alfabetu. Aby wpisać określoną literę, należało znaleźć klawisz z zadaną literą; jeśli stała ona na nim na i-tej pozycji, należało nacisnąć ten klawisz i razy. Przykładowo na tandardowej komórce 𝐿 = 26 liter było podzielonych między 𝐾 = 8 klawiszy. Na klawiszu „7” znajdowały się litery pqrs. Wprowadzenie litery r wymagało zatem naciśnięcia tego klawisza 3 razy.

Ostatnio Rząd Chińskiej Republiki Ludowej postanowił wprowadzić na rynek podobną komórkę, tylko nieco większą, żeby zmieściły się na niej wszystkie chińskie symbole (dalej nazywane literami). Litery zostały ponumerowane od 1 do 𝐿, pozostało je tylko podzielić pomiędzy 𝐾 klawiszy. Rząd ChRL ustalił, że układ klawiszy na komórce ma zostać tak zoptymalizowany, żeby pewien określony komunistyczny pamflet dało się wpisać naciskając klawisze minimalną liczbę razy.

#### Specyfikacja danych wejściowych

W pierwszym wierszu wejścia znajdują się dwie liczby naturalne 𝐾 i 𝐿 oddzielone pojedynczą spacją, spełniające
warunki: $K \in [1,100],\, L \in [1, 10'000], K \le L$. W drugim wierszu znajduje się 𝐿 liczb naturalnych f[1..L] oddzielonych pojedynczym odstępem. Liczba $f[i] \in [1,1000]$ jest liczbą wystąpień litery 𝑖 w pamflecie. W siedmiu punktowanych testach zachodzi dodatkowo $L\le 1000$

#### Specyfikacja danych wyjściowych

W pierwszym wierszu wyjścia Twój program powinien wypisać jedną liczbę naturalną będącą minimalną liczbą naciśnięć klawiszy konieczną do wpisania pamfletu na najlepszej możliwej klawiaturze składającej się z K klawiszy. W drugim wierszu wyjścia Twój program powinien wypisać opis takiej klawiatury: 𝐾 liczb naturalnych k[1..K] oddzielonych pojedynczymi odstępami, gdzie k[j] jest liczbą liter przypisanych do klawisza j. Jeśli istnieje wiele optymalnych rozmieszczeń liter na klawiszach, należy wybrać takie, które maksymalizuje liczbę liter na ostatnim klawiszu, wśród nich to, które maksymalizuje liczbę liter na przedostatnim klawiszu itd.

### E. Liczby

#### Dostępna pamięć: 8 MB

Zaprojektuj i zaimplementuj strukturę, która umożliwi przechowywanie zbioru liczb całkowitych P i wykonywanie na nim opisanych poniżej operacji.

1. insert(x). Dodaje liczbę całkowitą x do zbioru P. Jeśli x już należy do P, nic się nie dzieje.
2. delete(x). Usuwa liczbę całkowitą x ze zbioru P.
3. upper(x) Zwraca liczbę $\min_y y\in P,\, y\geqslant x$ (x jeśli jest w P lub najmniejszą liczbę większą od x)
4. lower(x) Zwraca liczbę $\max_y y\in P,\, y\leqslant x$ (x jeśli jest w P lub największą liczbę mniejszą od x)

(dla P = [1,3] upper(2) = 3, lower(2) = 1)

#### Specyfikacja danych wejściowych

W pierwszym wierszu danych wejściowych znajduje się liczba naturalna $N \in [1, 10^6 ]$, oznaczająca liczbę operacji na zbiorze P. Początkowo zbiór P jest pusty. W każdym z kolejnych N wierszy znajduje się opis jednej operacji wykonywanej na zbiorze P. Każdy z wierszy składa się z dużej litery ze zbioru {I, D, U, L}, pojedynczego odstępu i liczby całkowitej $x \in [−10^{18} , 10^{18}]$. Podana litera jest pierwszą literą operacji zdefiniowanych powyżej. Operacje są tak dobrane, że po każdej z nich rozmiar zbioru wynosi co najwyżej 50 000.

#### Specyfikacja danych wyjściowych

Twój program powinien wypisać jeden wiersz dla każdej operacji insert, delete, upper, lower. Zawartość tego wiersza powinna być następująca: dla operacji delete(x) należy wypisać słowo BRAK, jeśli $x \notin P$,w przeciwnym przypadku OK. Dla operacji upper lub lower należy wypisać znalezioną liczbę, a jeśli taka nie istnieje — słowo BRAK.

### F. Wzorzec

#### Dostępna pamięć: 64 MB

Oblicz, ile razy prostokątny dwuwymiarowy wzorzec występuje w prostokątnej tabeli. Zarówno tabela jak i wzorzec składają się z dużych liter alfabetu. Przykładowo wzorzec

| x\y | 0   | 1   | 2   |
| --- | --- | --- | --- |
| 0   | B   | C   | B   |
| 1   | C   | B   | C   |

występuje w poniższej tabeli w dwóch miejscach:

| x\y | 0   | 1   | 2   | 3   | 4   |
| --- | --- | --- | --- | --- | --- |
| 0   | B   | C   | B   | C   | B   |
| 1   | C   | B   | C   | B   | C   |
| 2   | A   | A   | B   | A   | A   |

(x:0,y:0) i (x:2,y:0)

#### Specyfikacja danych wejściowych

W pierwszym wierszu wejścia znajdują się cztery liczby naturalne a b c d oddzielone pojedynczym odstępem, spełniające warunki $a,b \in [1,200];\>\> c,d \in [1,2000];\>\> a \leqslant c;\>\> b \leqslant d$ Oznaczają one odpowiednio: wysokość wzorca, szerokość wzorca, wysokość tabeli i szerokość tabeli. W kolejnych a wierszach znajduje się opis wzorca: każdy wiersz składa się z b dużych liter alfabetu łacińskiego ('A' – 'Z'). W kolejnych c wierszach znajduje się
opis tabeli: każdy wiersz składa się z d dużych liter alfabetu łacińskiego. Litery nie są oddzielone odstępami.
Dodatkowo wiadomo, że zadany wzorzec występuje w tabeli co najwyżej 200 razy.

#### Specyfikacja danych wyjściowych

W pierwszym i jedynym wierszu wyjścia powinna pojawić się liczba wystąpień wzorca w tabeli.