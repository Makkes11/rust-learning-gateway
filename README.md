# 🚀 Rust Learning Gateway

**Rust lernen durch Bauen - von den Grundlagen bis zum produktionsreifen IoT-Gateway**

Dieses Repository dokumentiert meinen Lernweg in Rust: 29 Tage Fundamentals durch tägliche Übungen, danach die Entwicklung eines echten Industrial IoT Gateways.

**Hintergrund:** Ich komme aus TypeScript/Java und lerne Rust, um Backend- und Industrial-IoT-Projekte auf professionellem Level umzusetzen.

---

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


**→ [Gateway Dokumentation ansehen](gateway/README.md)**

---

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

## 🌐 Industrial IoT Gateway (Hauptprojekt)

Ab Tag 30 arbeite ich am Gateway - ein event-getriebenes System für industrielle IoT-Anwendungen.

**Aktueller Stand:**
- ✅ Event-basierte Architektur (Single-Writer-Pattern)
- ✅ REST API mit Axum
- ✅ Async Runtime mit Tokio
- ✅ Background-Tasks für Polling
- ✅ Thread-sichere State-Verwaltung

**Geplant:**
- MQTT Publisher/Subscriber
- Modbus TCP Client
- OPC UA Adapter
- Config-Management
- Docker Deployment

**→ [Vollständige Dokumentation und Architektur](gateway/README.md)**

### 💻 Gateway starten
```bash
cd gateway
cargo run

# Testen
curl http://127.0.0.1:3000/devices
```

---

## ⚡ Was dieses Repo zeigt

- **Tag 1-29:** Rust Fundamentals durch praktische Übungen
  - Ownership & Borrowing
  - Pattern Matching (einfach bis komplex)
  - Fehlerbehandlung mit `Result` und `Option`
  - State Machines und komplexe Datenstrukturen
  
- **Ab Tag 30:** Produktionsreifes IoT-Gateway
  - Event-driven Architecture
  - Async Rust in der Praxis
  - REST APIs und Background-Tasks
  - Vorbereitung für Modbus/OPC-UA/MQTT

---

## 🔗 Hinweise

- Dieses Repository ist **öffentlich** und dokumentiert kontinuierlichen Lernfortschritt
- Tages-Übungen (Tag 1-29) sind abgeschlossen und können einzeln ausgeführt werden
- Gateway-Entwicklung läuft kontinuierlich weiter (ab Tag 30)
- Jeder Commit zeigt echte Lernschritte - mit Fehlern, Refactorings und Verbesserungen