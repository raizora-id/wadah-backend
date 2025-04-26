use std::num::NonZeroU32;
use std::time::Duration;
use tower_governor::{
    governor::GovernorConfigBuilder,
    key_extractor::{KeyExtractor, SmartIpKeyExtractor},
    GovernorLayer,
};

pub fn create_rate_limit_middleware() -> GovernorLayer<SmartIpKeyExtractor> {
    let config = Box::new(
        GovernorConfigBuilder::default()
            .per_second(2)
            .burst_size(NonZeroU32::new(5).unwrap())
            .finish()
            .unwrap(),
    );

    GovernorLayer::with_config(config)
}
