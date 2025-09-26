
# rAIce

Edukacyjny eksperyment, w którym samochody uczą się pokonywać tor, korzystając z prostego algorytmu ewolucyjnego oraz sieci neuronowych.

## Pomysł gry
- Populacja aut startuje równocześnie, każde sterowane własnym MLP.
- Obserwacje: raycasty, prędkość, odchylenie od środka toru.
- Akcje: skręt, gaz, hamulec.
- Po sesji wybieramy najlepszy genom i tworzymy nową generację mutacji.

## Tech stack
- **Rust (2024 Edition)**
- **Bevy 0.13.x** - render, ECS, input
- **bevy_rapier2d 0.26.x** - fizyka, kolizje, (docelowo) raycasty
- **Serde + JSON** - serializacja genomów i torów
- **Rand** - mutacje i losowość
- *(opcjonalnie)* **bevy_egui** - debugowe UI
- *(opcjonalnie)* **Rayon** - równoległa ewaluacja fitnessu

## Struktura
```
src/
  main.rs          # entrypoint, konfiguracja Bevy
  brain.rs         # MLP / genomy
  control.rs       # komponent sterowania autem
  evolution.rs     # populacja, selekcja, mutacje (placeholder)
  fitness.rs       # scoring przejazdów
  sensors.rs       # konfiguracja i odczyty sensorów
  track.rs         # definicja toru, segmenty, ściany
  vehicle.rs       # komponent auta + spawn helpery
assets/
tracks/
````

## Uruchamianie
```bash
# dev
cargo run

# release
cargo run --release
````

## Obecny stan

* Placeholderowy tor generuje widoczne segmenty + kolidery ścian.
* Kilku zawodników spawnuje się na siatce startowej i otrzymuje bryły Rapiera.
* Brak sterowania AI (auta stoją) - pozostałe moduły pełnią rolę stubów do dalszej implementacji.

## Roadmap

* [ ] Rysowalne tory → wygładzanie (Catmull-Rom) → generowanie band i colliderów
* [ ] MLP + sterowanie pojazdu
* [ ] Fitness + prosty GA (selekcja, mutacje, checkpointy JSON)
* [ ] Tryb headless (przyspieszone treningi), replay najlepszego przejazdu
* [ ] Refaktoryzacja zgodnie z zasadami clean code
