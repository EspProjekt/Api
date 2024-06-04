use super::{prelude::*, session::Session};
pub use actix::{Handler, Message};


pub struct WebSocketManager{
    pub adresses: HashMap<String, Addr<Session>>,
    pub redis: Redis,
}


impl WebSocketManager {
    pub fn new(redis: &Redis) -> Self {
        Self { 
            adresses: HashMap::new(),
            redis: redis.clone()
        }
    }


    pub fn add_address(&mut self, key: Uuid, addr: Addr<Session>) {
        self.adresses.insert(key.to_string(), addr);
    }


    pub fn remove_address(&mut self, key: Uuid) {
        self.adresses.remove(&key.to_string());
    }


    pub fn get_addresses(&self) -> Vec<Addr<Session>> {
        self.adresses.values().cloned().collect()
    }


    pub fn send_device_list(&self){
        let device_list = DeviceList::list_devices(&self.redis).unwrap();
        
        for addr in self.get_addresses() {
            addr.do_send(DeviceListMessage::new(device_list.clone()));
        }
    }
}


#[derive(Serialize, Deserialize)]
struct DeviceListMessage{
    pub device_list: Vec<PublicDevice>,
}


impl DeviceListMessage {
    pub fn new(device_list: Vec<PublicDevice>) -> Self {
        Self {
            device_list,
        }
    }
}


impl Message for DeviceListMessage { type Result = (); }
impl Handler<DeviceListMessage> for Session {
    type Result = ();

    fn handle(&mut self, msg: DeviceListMessage, ctx: &mut Self::Context) {
        let msg_str = serde_json::to_string(&msg.device_list).unwrap();
        ctx.text(msg_str);
    }
}
