# IoT Gateway in Rust

Ein event-getriebenes Gateway für industrielle IoT-Anwendungen. Gebaut mit Rust, Tokio und Axum während meiner Rust-Lernreise.

## Was macht das Gateway?

Das Gateway verarbeitet Gerätedaten über verschiedene Kanäle:
- REST API zum Erstellen und Aktualisieren von Geräten
- Hintergrund-Task simuliert Sensor-Polling (später echte Protokolle)
- Zentraler State mit thread-sicherem Zugriff
- Event-basierte Architektur ohne Race Conditions

## 🚀 Quick Start

### Gateway starten
```bash
cargo run

# Server starts on http://127.0.0.1:3000
# ✓ MQTT connected (if mosquitto running)
```

### API testen
```bash
# Alle Geräte abrufen
curl http://127.0.0.1:3000/devices

# Gerät erstellen
curl -X POST http://127.0.0.1:3000/devices \
  -H "Content-Type: application/json" \
  -d '{"id": 42, "value": 100}'

# Gerät löschen
curl -X DELETE http://127.0.0.1:3000/devices/42 -v

# MQTT Messages live sehen
mosquitto_sub -h localhost -t "devices/#" -v
```

## API Endpoints

### `GET /devices`
Gibt alle registrierten Geräte zurück.

**Response:** `200 OK`
```json
[
  {"id": 1, "value": 42},
  {"id": 2, "value": 100}
]
```

### `POST /devices`
Erstellt oder aktualisiert ein Gerät.

**Request Body:**
```json
{"id": 1, "value": 50}
```

**Response:** `200 OK`
```json
{"id": 1, "value": 50}
```

**MQTT:** Publisht automatisch auf `devices/{id}/value`

### `DELETE /devices/{id}`
Löscht ein Gerät.

**Response:** `204 No Content`

**MQTT:** Publisht Delete-Event auf `devices/{id}/deleted`

## 📡 MQTT Integration

Das Gateway publisht alle Änderungen automatisch auf MQTT Topics:

**Topics:**
- `devices/{id}/value` - Device Updates
```json
  {"id": 1, "value": 42, "timestamp": "2024-12-21T00:15:00Z"}
```

- `devices/{id}/deleted` - Device Deletions
```json
  {"id": 1, "timestamp": "2024-12-21T00:15:00Z"}
```

**Subscribe Beispiele:**
```bash
# Alle Device-Updates
mosquitto_sub -h localhost -t "devices/+/value" -v

# Alle Delete-Events
mosquitto_sub -h localhost -t "devices/+/deleted" -v

# Alles von Device 1
mosquitto_sub -h localhost -t "devices/1/#" -v
```

## 🏗️ Architektur

```
REST API ──────┐
               ├──> mpsc::channel ──> Event Loop ──> GatewayState
Background ────┘                          │          (Arc<Mutex>)
                                          ↓
                                    MQTT Publisher
                                          │
                                          ↓
                            devices/{id}/value
                            devices/{id}/deleted
```

**Event-Typen:**
- `Update{id, value}` - Gerät anlegen/ändern → MQTT publish
- `Remove(id)` - Gerät löschen → MQTT delete event
- `Tick(delta)` - Alle Geräte um Wert ändern

**Module:**
- `main.rs` - Server-Setup & Event-Loop
- `api/` - REST Endpoints (GET, POST, DELETE)
- `state/` - GatewayState & Events
- `mqtt/` - MQTT Publisher
- `device.rs` - Device Model

### Event-Typen

- `Update{id, value}` - Gerät anlegen/ändern
- `Remove(id)` - Gerät löschen
- `Tick(delta)` - Alle Geräte um Wert ändern

### Warum Events?

Der Event-Loop ist der einzige Ort, an dem der State geändert wird. Das verhindert Race Conditions und macht alle Änderungen nachvollziehbar. Später kann ich hier einfach Logging oder Persistierung einbauen.

## 🛠️ Tech Stack

- **Tokio** - Async Runtime für nebenläufige Tasks
- **Axum 0.7** - HTTP Server und Routing
- **Serde** - JSON Serialisierung
- **rumqttc** - MQTT Client
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