#[derive(Debug)]
pub enum Intent {
    EnsureGatewayExists {
        gateway_id: String,
        gateway_name: String,
    },

    EnsureDeviceExists {
        device_id: String,
        gateway_id: String,
    },

    UpsertDeviceMetadata {
        device_id: String,
        gateway_id: String,
        gateway_name: String,
        device_name: String,
        timestamp: i64,
    },

    UpdateDeviceLastSeen {
        device_id: String,
        gateway_id: String,
        timestamp: i64,
    },

    RecordMeasurement {
        device_id: String,
        gateway_id: String,
        value: Option<f64>,
        timestamp: i64,
    },

    MarkDeviceRemoved {
        device_id: String,
        gateway_id: String,
        timestamp: i64,
    },

    ReactivateDevice {
        device_id: String,
        gateway_id: String,
        timestamp: i64,
    },
}
