# 🚀 Rust Learning Gateway

Dieses Repository dokumentiert meine **Rust-Lernreise** mit praktischen Übungen, Mini-Projekten und einem kleinen IoT-Gateway-Projekt. Alle Übungen sind in **Tagesordnern** organisiert (`days/day01` bis `days/day29`).  
Das Gateway-Projekt befindet sich im Ordner `gateway/`.


## 📂 Projektstruktur

```

rust-learning-gateway/
│
├── days/ # Einzelne Übungs-Tage
│ ├── day01_read_file/
│ ├── day02_process_number_file/
│ ├── ...
│ └── day29_borrowing_complex_datatypes_lifecycle/
│
├── gateway/ # Hauptprojekt / Mini-IoT-Gateway
│
└── README.md

```

## 📚 Übungen

| Tag | Projekt / Übung | Kurzbeschreibung |
|-----|-----------------|-----------------|
| 01  | `read_file` | Einfache Datei einlesen und Inhalt verarbeiten |
| 02  | `process_number_file` | Zahlen aus Datei lesen und verarbeiten |
| 03  | `to_user_message` | Zahlen in benutzerfreundliche Nachrichten umwandeln |
| 04  | `print_description` | Structs und einfache Beschreibungen ausgeben |
| 05  | `lifetimes` | Rust-Lifetimes und Borrowing üben |
| 06  | `ownership_borrowing_enums` | Ownership, Borrowing und Enums vertiefen |
| 07  | `library_book` | Kleine Bibliothek erstellen mit Funktionen und Tests |
| 08  | `lifetimes_borrowing_advanced` | Fortgeschrittenes Borrowing und Lifetimes |
| 09  | `testing` | Unit- und Integrationstests schreiben |
| 10  | `cli` | Einfaches CLI-Projekt |
| 11  | `cli_advanced` | CLI mit mehreren Commands und Fehlerhandling |
| 12  | `cli_advanced` | Weiterführende CLI-Features |
| 13  | `cli_commands` | Implementierung von Befehlen als Library |
| 14  | `cli_commands` | Fortsetzung der CLI-Command-Logik |
| 15  | `cli_commands` | Optimierung und Tests |
| 16  | `cli_commands` | Weitere Verbesserungen und Features |
| 17  | `ownership_borrowing_depth` | Komplexe Ownership- und Borrowing-Beispiele |
| 18  | `fragen_beantworten` | Theoriefragen zu Rust-Konzepten |
| 19  | `enums_result_errors` | Enums und Result für Fehlerhandling |
| 20  | `enums_result_errors` | Weiteres Fehlerhandling und Pattern Matching |
| 21  | `enums_result_errors` | Integration verschiedener Fehlerarten |
| 22  | `enums_payload` | Enums mit komplexen Payloads |
| 23  | `state_machines` | Zustandmaschinen implementieren |
| 24  | `rust_pattern_matching_deep_dive` | Pattern Matching tiefgehend |
| 25  | `advanced_pattern_matching` | Fortgeschrittene Pattern Matching Techniken |
| 26  | `nested_borrowing_and_references_from_structs` | Nested Borrows und Referenzen in Structs |
| 27  | `pattern_matching_complex_data_structures` | Komplexe Datenstrukturen mit Pattern Matching |
| 28  | `advanced_pattern_matching_borrowed_patterns` | Borrowed Patterns und Destructuring |
| 29  | `borrowing_complex_datatypes_lifecycle` | Komplexe Datentypen und Lebenszyklen |

---


## 🌐 Gateway Projekt

Im Ordner `gateway/` befindet sich ein **Mini IoT-Gateway**, das Sensordaten verarbeitet, Events behandelt und den Status von Devices verwaltet.

### 💻 Ausführen

```bash
cd gateway
cargo run
```

## ⚡ Features

- Praxisnahe Rust-Übungen, sortiert nach Tagen
- Mini IoT-Gateway als kleines Projekt zum Anwenden von Rust
- Umfassende Beispiele zu Pattern Matching, Borrowing und Ownership
- Fehlerbehandlung mit `Result` und `Option`
- Jede Tagesübung als eigenständiges Cargo-Projekt

---

## 🔗 Hinweise

- Dieses Repository ist **öffentlich**, ideal zur Dokumentation von Lernfortschritten
- Jede Tagesübung ist eigenständig und kann separat getestet werden
- Das Gateway-Projekt befindet sich im Ordner `gateway/` und kann direkt mit `cargo run` ausgeführt werden
- Für detaillierte Infos zu den Übungen, siehe die Tagesordner `days/day01` bis `days/day29`