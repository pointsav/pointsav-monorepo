// SPDX-License-Identifier: Apache-2.0 OR MIT

//! Three compute tiers per `conventions/llm-substrate-decision.md`.
//!
//! The Doorman holds optional instances of each tier client. Missing
//! tiers are not errors at boot; per the Optional Intelligence principle
//! (`conventions/three-ring-architecture.md`), the system functions
//! without them and the router routes around them.

mod external;
mod local;
mod yoyo;

pub use external::{ExternalAllowlist, ExternalTierClient, ExternalTierConfig};
pub use local::{LocalTierClient, LocalTierConfig};
pub use yoyo::{BearerTokenProvider, PricingConfig, StaticBearer, YoYoTierClient, YoYoTierConfig};
