Title: Device reactivation on telemetry after removal

Decision:
A device is reactivated automatically when new telemetry is received after a removal event.

Reason:
- Event ordering is not guaranteed
- Telemetry is considered a stronger signal than lifecycle events
- Avoids data loss and stale states

Consequences:
- Device lifecycle is not strictly enforced
- Devices may oscillate between states
- System becomes resilient to late events