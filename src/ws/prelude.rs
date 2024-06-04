pub use actix::{Actor, StreamHandler};
pub use actix_web_actors::ws;
pub use ws::{Message, ProtocolError, WebsocketContext};
pub use actix::prelude::Running;
pub use std::collections::HashMap;
pub use actix::Addr;
pub use serde::{Deserialize, Serialize};
pub use uuid::Uuid;
pub use crate::{data::{device::PublicDevice, device_list::DeviceList}, redis::Redis};