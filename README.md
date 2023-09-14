# Flux

|Termin|Opis|Wykonane|
|--|--|--|
|04.04 | Przygotowanie wstępnej koncepcji gry| Zrobione
|11.04| Stworzenie zorientowanego komponentowo podstawowego systemu objektów umozliwiającego serializacje I data-driven design| Serializacja została zarzucona ze względu na stopień skompilkowania - został użyty system komponentów (`ECS`)
|18.04| Dodanie komunikacji za pomocą eventów pomiędzy obiektami gry, systemu Scen oraz  ResourceManagera, podstawowa pętla gry | Zrobione
|25.04| Dodanie podstawowego renderera oraz kamery, proste elementy UI – przyciski, listy, itp| Renderer dodany, elementy UI stworzone w uproszczonej, nieinteraktywnej formie - celownik, tekst etc.
|02.05| Zaawansowany renderer – sky cube, siatki , instancjonowanie,efekty cząsteczkowe | Zrobione
|09.05| Wykrywanie kolizji pomiędzy prostymi simplexami, podstawowa fizyka| Kolizje oparte na sferycznych colliderach - stały krok fizyczny pozwala na stabilną symulacje fizyki. Kolizje wykorzystują algorytm przewidujący kolizje zanim wystąpią (brak siły kompensującej), kolizje są doskonale elastyczne, jednak nie została zaimplementowana fizyka związana z momentem obrotowym
|16.05| Rozbudowanie silnika fizyki o bardziej skomplikowane kształty| Możliwe jest ładowanie siatek 3D w formacie glb
|23.05| Refaktor| Wykonywany ciągle w trakcie projektu
|30.05| Stworzenie przeciwników gracza – proste AI poruszające się w chmarze  (swarm)| Obecnie brak żadnych agentów
|06.06| Złożenie gry w całość – zamiana placeholderów na prawdziwe assety, dopracowanie UI, oraz przejść między scenami| Zrobione

