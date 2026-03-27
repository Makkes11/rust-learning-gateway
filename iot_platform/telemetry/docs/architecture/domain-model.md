Domain Decisions:

Device:
- Can be explicitly created via event
- Can be implicitly created from telemetry
- Can be logically removed (soft delete)

Value:
- Append-only time-series data
- Always stored regardless of device state
- Timestamp is authoritative

Removal:
- Soft delete only
- Historical data must remain
- Device may be reactivated by new values