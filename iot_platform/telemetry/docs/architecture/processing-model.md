# Processing Model

## Overview

The processor is the central decision-making component of the system.

It transforms incoming events into domain-level intents by applying
the rules defined in the domain model.

The processor is:
- deterministic
- side-effect free
- independent of storage and infrastructure

---

## Input Events

The processor supports the following events:

- DeviceCreated
- DeviceValueObserved
- DeviceRemoved

### Event Characteristics

All events are assumed to be:

- potentially out-of-order
- possibly duplicated
- potentially delayed

The processor must handle these conditions gracefully.

---

## Output Intents

The processor produces the following intents:

- EnsureDeviceExists
- UpsertDeviceMetadata
- RecordMeasurement
- MarkDeviceRemoved
- ReactivateDevice

### Notes

- Intents represent **domain-level actions**
- Intents must NOT contain any storage-specific logic
- Intents are later executed by the storage adapter

---

## Core Processing Principle

The processor derives system state from events.

Because event ordering is not guaranteed:

- telemetry data is always accepted
- explicit events override implicit state
- latest meaningful signal wins

---

## Processing Rules

### DeviceValueObserved

On receiving a `DeviceValueObserved` event:

- Ensure the device exists (implicit creation allowed)
- Record the measurement (append-only)
- If the device was previously removed:
  - Reactivate the device

---

### DeviceCreated

On receiving a `DeviceCreated` event:

#### If the device does not exist:
- Create the device with explicit metadata
- Set status to active
- Set `created_at` from event timestamp

#### If the device exists implicitly:
- Upgrade device to explicit state
- Update metadata
- Set status to active
- Set `created_at` from event timestamp

#### If the device already exists explicitly:
- Update metadata if necessary

---

### DeviceRemoved

On receiving a `DeviceRemoved` event:

- Mark the device as removed
- Set `removed_at` from event timestamp

---

## Device State Model

Devices can have the following states:

- implicit   (created from telemetry only)
- active     (explicitly created or reactivated)
- removed    (logically deleted)

### State Transitions

- implicit → active   (on DeviceCreated)
- removed  → active   (on DeviceValueObserved)

Device state is derived from the latest meaningful signal.

---

## Execution Model

Each incoming event is transformed into a list of intents:

```
Event → [Intent]
```

Examples:

- DeviceValueObserved →
  - EnsureDeviceExists
  - RecordMeasurement
  - (optional) ReactivateDevice

- DeviceCreated →
  - UpsertDeviceMetadata
  - SetStatus(active)

- DeviceRemoved →
  - MarkDeviceRemoved

---

## Storage Interaction

The processor does NOT interact with storage directly.

Instead:

- It produces intents
- The storage adapter executes them

### Storage Adapter Responsibilities

- Execute SQL statements
- Enforce data integrity
- Remain free of business logic

---

## Related Decisions

- ADR-001: Device Auto Creation
- ADR-002: Device Reactivation