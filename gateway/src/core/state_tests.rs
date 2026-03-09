#[cfg(test)]
mod tests {

    use crate::core::events::GatewayEvent;
    use crate::core::state::{GatewayState, StateChange, StateError};
    use chrono::Utc;

    #[tokio::test]
    async fn test_device_created_applies_timestamp() {
        let mut state = GatewayState::new();
        let ts = Utc::now();

        let event = GatewayEvent::DeviceCreated {
            id: 1,
            timestamp: ts,
        };
        let change = state.apply_event(event).unwrap().unwrap();

        assert_eq!(
            change,
            StateChange::DeviceCreated {
                id: 1,
                timestamp: ts
            }
        );
        assert_eq!(state.devices.len(), 1);
        assert_eq!(state.devices[0].id, 1);
        assert_eq!(state.devices[0].timestamp, ts);
    }

    #[tokio::test]
    async fn test_device_value_observed_applies_timestamp() {
        let mut state = GatewayState::new();
        let ts_created = Utc::now();
        state
            .apply_event(GatewayEvent::DeviceCreated {
                id: 1,
                timestamp: ts_created,
            })
            .unwrap();

        let ts_update = Utc::now();
        let event = GatewayEvent::DeviceValueObserved {
            id: 1,
            value: Some(42.0),
            timestamp: ts_update,
        };
        let change = state.apply_event(event).unwrap().unwrap();

        assert_eq!(
            change,
            StateChange::DeviceUpdated {
                id: 1,
                value: Some(42.0),
                timestamp: ts_update
            }
        );
        assert_eq!(state.devices[0].value, Some(42.0));
        assert_eq!(state.devices[0].timestamp, ts_update);
    }

    #[tokio::test]
    async fn test_device_removed_applies_timestamp() {
        let mut state = GatewayState::new();
        let ts_created = Utc::now();
        state
            .apply_event(GatewayEvent::DeviceCreated {
                id: 1,
                timestamp: ts_created,
            })
            .unwrap();

        let ts_removed = Utc::now();
        let event = GatewayEvent::DeviceRemoved {
            id: 1,
            timestamp: ts_removed,
        };
        let change = state.apply_event(event).unwrap().unwrap();

        assert_eq!(
            change,
            StateChange::DeviceRemoved {
                id: 1,
                timestamp: ts_removed
            }
        );
        assert!(state.devices.is_empty());
    }

    #[tokio::test]
    async fn test_device_not_found_error() {
        let mut state = GatewayState::new();
        let ts = Utc::now();

        let event = GatewayEvent::DeviceValueObserved {
            id: 999,
            value: Some(10.0),
            timestamp: ts,
        };
        let err = state.apply_event(event).unwrap_err();

        match err {
            StateError::DeviceNotFound(id) => assert_eq!(id, 999),
        }
    }
}
