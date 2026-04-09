-- gateways table
CREATE TABLE IF NOT EXISTS gateways (
    id TEXT PRIMARY KEY,
    name TEXT NOT NULL
);

-- devices table
CREATE TABLE IF NOT EXISTS devices (
    gateway_id TEXT NOT NULL,
    id TEXT NOT NULL,
    name TEXT,
    created_at TIMESTAMP,
    removed_at TIMESTAMP,
    last_seen TIMESTAMP,
    PRIMARY KEY (gateway_id, id),
    FOREIGN KEY (gateway_id) REFERENCES gateways(id)
);

-- device_values table
CREATE TABLE IF NOT EXISTS device_values (
    gateway_id TEXT NOT NULL,
    device_id TEXT NOT NULL,
    "timestamp" TIMESTAMP NOT NULL,
    value DOUBLE PRECISION NOT NULL,
    PRIMARY KEY (gateway_id, device_id, "timestamp"),
    FOREIGN KEY (gateway_id, device_id)
        REFERENCES devices(gateway_id, id)
);