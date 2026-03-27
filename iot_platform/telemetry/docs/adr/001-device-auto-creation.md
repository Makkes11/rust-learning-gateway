Title: Device auto-creation on telemetry ingest

Status: accepted

Context:
MQTT events can arrive before device creation event.

Decision:
We allow implicit device creation in the processor layer.

Reason:
- MQTT is unordered
- System must be resilient to missing events
- Avoids data loss

Trade-offs:
- Less strict domain model
- Potential ghost devices

Consequences:

Positive:
- Improved resilience to out-of-order events
- No data loss due to missing device state

Negative:
- Device lifecycle is not strictly event-driven
- Presence of ghost devices with incomplete metadata

Neutral:
- Device state becomes eventually consistent

Definitions:

Ghost Device:
A device implicitly created from telemetry data without a corresponding "created" event.

Handling:
- Treated as valid entity
- Can be enriched later by a "created" event
- Must not block ingestion