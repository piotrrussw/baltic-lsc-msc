### Analiza porównawcza technologii frontendowych w oparciu o frontend BalticLSC  

#### [Praca dyplomowa (LINK)](https://docs.google.com/document/d/1QeaRgBcWZ-nMwDhP0VeAvC3ENhXhrXamVlzptIaHvMY/edit?usp=sharing)

Analiza porównawcza projektowanie frontendu z wykorzystaniem klasycznej technologii Javascript oraz WebAssembly. 
Porównanie wydajności zarówno po stronie serwera, jak i klienta. Wyniki mogą być interesujące, bo o ile sam WebAssembly i Rust są znane z bardzo dobrej wydajności, użycia wielowątkowości, co daje przewagę po stronie serwera, to po stronie klienta WebAssembly nie jest jeszcze tak szybki i wymaga przetworzenia dużo większej ilości kodu przez przeglądarkę.
Porównanie projektowania z uwzględnieniem kluczowych elementów do tworzenia Frontendu:
* Tworzenie komponentów
* Zarządzanie stanem aplikacji
* Stylowanie aplikacji
* Pobieranie danych z backendu


###### Składowe projektu

* [Proxy](https://github.com/piotrrussw/baltic-lsc-msc/blob/main/packages/cors-proxy/README.md) - lokalny serwer na potrzeby projektowania przekierujowujący zapytania do backendu BalticLSC z localhosta (ze względu na mechanizm CORS). Dodatkowo funkcja "in memory cache" dla zapytań przydatna do testów wydajnościowych.
* [BalticLSC CSR WebAssembly](https://github.com/piotrrussw/baltic-lsc-msc/blob/main/packages/yew-csr/README.md) - aplikacja BalticLSC napisana w rust (z wykorzystaniem biblioteki Yew) renderowana po stronie klienta
* [BalticLSC SSR WebAssembly](https://github.com/piotrrussw/baltic-lsc-msc/blob/main/packages/yew-ssr/README.md) - aplikacja BalticLSC napisana w rust (z wykorzystaniem biblioteki Yew) renderowana po stronie serwera
* [BalticLSC SSR Javascript](https://github.com/piotrrussw/baltic-lsc-msc/blob/main/packages/react-csr/README.md) - aplikacja BalticLSC napisana w javascript (z wykorzystaniem biblioteki React) renderowana po stronie klienta
* [BalticLSC SSR Javascript](https://github.com/piotrrussw/baltic-lsc-msc/blob/main/packages/react-ssr/README.md) - aplikacja BalticLSC napisana w javascript (z wykorzystaniem biblioteki React) renderowana po stronie serwera
* [Load tests](https://github.com/piotrrussw/baltic-lsc-msc/blob/main/packages/ssr-perf/README.md) - skrypt do load testów po stronie serwera