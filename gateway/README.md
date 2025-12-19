# IoT Gateway in Rust

Ein event-getriebenes Gateway für industrielle IoT-Anwendungen. Gebaut mit Rust, Tokio und Axum während meiner Rust-Lernreise.

## Was macht das Gateway?

Das Gateway verarbeitet Gerätedaten über verschiedene Kanäle:
- REST API zum Erstellen und Aktualisieren von Geräten
- Hintergrund-Task simuliert Sensor-Polling (später echte Protokolle)
- Zentraler State mit thread-sicherem Zugriff
- Event-basierte Architektur ohne Race Conditions

## Schnellstart

```bash
# Gateway starten
cargo run

# Testen (in anderem Terminal)
curl http://127.0.0.1:3000/devices

# Gerät erstellen
curl -X POST http://127.0.0.1:3000/devices \
  -H "Content-Type: application/json" \
  -d '{"id": 42, "value": 100}'
```

## API Endpoints

**GET /devices**  
Gibt alle Geräte als JSON-Array zurück.

**POST /devices**  
Erstellt oder aktualisiert ein Gerät.

Beispiel Body:
```json
{"id": 1, "value": 50}
```

## Architektur

Das Gateway nutzt ein Single-Writer-Pattern:

```
REST API ──────┐
               ├──> mpsc::channel ──> Event Loop ──> GatewayState
Background ────┘                                     (Arc<Mutex>)
```

### Event-Typen

- `Update{id, value}` - Gerät anlegen/ändern
- `Remove(id)` - Gerät löschen
- `Tick(delta)` - Alle Geräte um Wert ändern

### Warum Events?

Der Event-Loop ist der einzige Ort, an dem der State geändert wird. Das verhindert Race Conditions und macht alle Änderungen nachvollziehbar. Später kann ich hier einfach Logging oder Persistierung einbauen.

## Technologien

- **Tokio** - Async Runtime für nebenläufige Tasks
- **Axum** - HTTP Server und Routing
- **Serde** - JSON Serialisierung
- **mpsc** - Asynchrone Channels für Events

## Nächste Schritte

**Kurzfristig (Tag 36-40):**
- MQTT Publisher (Updates automatisch publishen)
- Logging mit `tracing`
- Config-Datei statt hardcoded Werte

**Mittelfristig (Tag 41-50):**
- Modbus TCP Client für echte PLCs
- OPC UA Adapter für SCADA-Systeme
- Docker Container

## Kontext

Das ist Tag 34 meiner [Rust-Lernreise](../README.md). Ich baue hier ein produktionsreifes IoT-Gateway, um zu lernen:
- Event-getriebenes Design
- Async Rust in der Praxis
- Industrielle Protokolle
- Backend-Architektur

Der Code wird kontinuierlich erweitert - aktuell ist die Core-Architektur stabil und bereit für echte Protokoll-Integrationen.