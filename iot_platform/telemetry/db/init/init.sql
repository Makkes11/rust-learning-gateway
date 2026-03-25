-- gateways table
CREATE TABLE IF NOT EXISTS gateways (
    id TEXT PRIMARY KEY,
    name TEXT NOT NULL
);

-- devices table
CREATE TABLE IF NOT EXISTS devices (
    id SERIAL PRIMARY KEY,
    gateway_id TEXT NOT NULL REFERENCES gateways(id),
    name TEXT NOT NULL,
    created_at TIMESTAMP NOT NULL DEFAULT NOW(),
    removed_at TIMESTAMP
);

-- device_values table
CREATE TABLE IF NOT EXISTS device_values (
    device_id INT NOT NULL REFERENCES devices(id),
    timestamp TIMESTAMP NOT NULL,
    value DOUBLE PRECISION NOT NULL,
    PRIMARY KEY (device_id, timestamp)
);