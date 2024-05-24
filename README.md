# rust-fortgeschrittene
Schulungsunterlagen für den Kurs "Rust für Fortgeschrittene"


# Vorbereitungen am Tag 1
```
# installieren/updaten von rust mit rustup
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh

#für manche Beispiele Brauchen wir nightly features
rustup toolchain install nightly
rustup default nightly
rustup component add clippy # bester rust linter

# alle example Projekte mit abhängigkeiten holen und bauen
cargo build

```

* installieren von vscode
  * siehe https://code.visualstudio.com/
  * plugin rust-analyzer (The Rust Programming Language)
  * plugin crates (Seray Uzgur)
  * plugin Remote Development (Microsoft; damit können wir in einem Container entwickeln)


# Agenda

## Asynchrones Rust

- Asynchrone Funktionen in Rust
- Tokio vs async-std vs smol
- [Tokio im Einsatz](./tokio-example/)
- [Join, select, await…](./join-example/)
- Asynchrones Data-Handling und Streams
- Praxisbeispiele

## Ownership Deep Dive

- Speicherverwaltung (Heap und Stack)
- [Pointers, Box und Dereferenzierung](./pointers-example/)
- [Ownership, Primitive und Non-Premitive typen](./ownership-example/)
- [Referenzen zu Mutable und Immutable](./mutable-example)

## Erweiterte Programmierung

- [(Berechnete) Konstanten (consts)](./const-function/)
- [Laufzeit-Typen (Any und TypeId)](./any-example/)
- [Nicht beweglicher Speicher](./pin-example/)
- [Enums](./enum-example/)
- [Der match Operator](./match-example/)
- [Pattern-Matching](./pattern-matching-example/)
- [Macros](./macro-example/)
- [Derive Macros](./derive-macro-example/)
- [Unsicherer Code](./unsafe-example/)
- [Rekursion in Rust](./recursion-example/)
- RefCell & [Smart Pointer](./smart-pointer-example/)
- [Regular Expressions in Rust](./regex-example/)
- [Traits](./trait-example/)
- Komplexe [Traits](./complex-trait-example/)

## Eigene Bibliotheken in Rust

- [Basis-Aufbau](./lib-example/)
- Standard-Implementierungen
- [Generics](./generic-example/)

## Error Handling

- [Richtiges Error-Handling in Rust](./error-example/)
- Error Propagation
- Panic! und Result

## Testing in Rust

- [Unit- und Integration-Tests in Rust](./tests-example/)
- Das Rust Testing Framework
- Test Setup (cargo test)
- Assertions

## Sonstiges

- Tipps zur effizienten Entwicklung
  - clippy
  - cargo machete for dependencies
  - cargo bench
  - github copilot
- Projektaufbau