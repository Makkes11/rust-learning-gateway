# Rust Learning Gateway

Dieses Repository dokumentiert meine Rust-Lernreise, inklusive Übungen, Mini-Projekten und einem eigenen kleinen IoT-Gateway. Ziel ist es, Rust-Konzepte praktisch zu erlernen und dabei sauberen, idiomatischen Code zu schreiben.

Die Ordnerstruktur ist wie folgt:

--- 

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

---

## Übersicht der Lern-Tage

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

## Hauptprojekt: `gateway`

Der Ordner `gateway/` enthält mein kleines Mini-IoT-Gateway.  
Hier werden die Rust-Konzepte aus den Übungs-Tagen praktisch angewendet.  

- Kommunikation mit Devices simulieren  
- Verarbeitung von Events und Sensor-Daten  
- Fehler- und Statusbehandlung  
- Pattern Matching, Borrowing und Ownership in realistischen Szenarien  

---

## Hinweise

- Jedes Tagesprojekt (`days/dayXX_*`) ist ein eigenständiges Cargo-Projekt (`cargo new`)  
- Tests sind, wenn vorhanden, in `src/lib.rs` oder `tests/` enthalten  
- Die Readme enthält nur kurze Beschreibungen; Details und Code sind in den jeweiligen Ordnern  

---

> Dieses Repo zeigt meinen Lernprozess in Rust und dient gleichzeitig als Showcase für sauberen, idiomatischen Rust-Code.