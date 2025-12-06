# 🚀 Rust Learning Gateway

Dieses Repository dokumentiert meine praktische Lernreise in Rust – bestehend aus täglichen Übungen (jeweils in eigenen Tagesordnern) und einem wachsenden Mini-IoT-Gateway-Projekt.  
Alle Übungen finden sich im Ordner `days/`, das Hauptprojekt liegt unter `gateway/`.


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

## 📚 Übersicht der Übungstage

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


## 🌐 Mini IoT-Gateway

Im Ordner `gateway/` entsteht ein kleines Industrial/IoT-Gateway, das Sensordaten als Events verarbeitet, einen internen Zustand hält und künftig um weitere Schnittstellen (REST, MQTT, Modbus, OPC UA) erweitert wird.

### 💻 Ausführen

```bash
cd gateway
cargo run
```

## ⚡ Features

- Sammlung praktischer Rust-Übungen von Grundlagen bis fortgeschrittenen Themen
- Mini-IoT-Gateway zur Anwendung echter Backend- und IoT-Konzepte
- Beispiele zu:
  - Ownership & Borrowing
  - Pattern Matching (einfach bis komplex)
  - Fehlerbehandlung mit `Result` und `Option`
  - Modularisierung und Strukturierung größerer Projekte
- Jede Tagesübung als eigenständiges Cargo-Projekt ausführbar
- Gateway-Projekt als Basis für weitere Integrationen (REST, MQTT, Modbus, OPC UA)

---

## 🔗 Hinweise

- Dieses Repository ist **öffentlich** und dokumentiert kontinuierlichen Lernfortschritt.
- Jede Tagesübung kann isoliert ausgeführt werden.
- Das Gateway-Projekt wird iterativ erweitert (Async, REST, MQTT, Modbus, OPC UA).