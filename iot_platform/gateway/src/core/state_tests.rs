#[cfg(test)]
mod tests {

    use crate::core::events::GatewayEvent;
    use crate::core::state::{GatewayState, StateChange};
    use chrono::Utc;

    #[tokio::test]
    async fn test_device_created_applies_timestamp() {
        let mut state = GatewayState::new();
        let ts = Utc::now().timestamp_millis();

        let event = GatewayEvent::DeviceCreated {
            id: 1,
            timestamp: ts,
        };
        let change = state.apply_event(event).unwrap();

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
        let ts_created = Utc::now().timestamp_millis();
        state
            .apply_event(GatewayEvent::DeviceCreated {
                id: 1,
                timestamp: ts_created,
            })
            .unwrap();

        let ts_update = Utc::now().timestamp_millis();
        let event = GatewayEvent::DeviceValueObserved {
            id: 1,
            value: 42.0,
            timestamp: ts_update,
        };
        let change = state.apply_event(event).unwrap();

        assert_eq!(
            change,
            StateChange::DeviceUpdated {
                id: 1,
                value: 42.0,
                timestamp: ts_update
            }
        );
        assert_eq!(state.devices[0].value, Some(42.0));
        assert_eq!(state.devices[0].timestamp, ts_update);
    }

    #[tokio::test]
    async fn test_device_removed_applies_timestamp() {
        let mut state = GatewayState::new();
        let ts_created = Utc::now().timestamp_millis();
        state
            .apply_event(GatewayEvent::DeviceCreated {
                id: 1,
                timestamp: ts_created,
            })
            .unwrap();

        let ts_removed = Utc::now().timestamp_millis();
        let event = GatewayEvent::DeviceRemoved {
            id: 1,
            timestamp: ts_removed,
        };
        let change = state.apply_event(event).unwrap();

        assert_eq!(
            change,
            StateChange::DeviceRemoved {
                id: 1,
                timestamp: ts_removed
            }
        );
        assert!(state.devices.is_empty());
    }
}
