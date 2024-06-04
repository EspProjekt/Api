use crate::state::AppState;
use super::prelude::*;
use actix::AsyncContext;
use uuid::Uuid;
use actix::WrapFuture;


pub struct Session {
    pub id: Uuid,
    pub addr: Option<Addr<Session>>,
    pub app_state: AppState,
}


impl Session {
    pub fn new(app_state: AppState) -> Self {
        Self {
            id: Uuid::new_v4(),
            addr: None,
            app_state
        }
    }
}


impl Actor for Session {
    type Context = WebsocketContext<Self>;

    fn started(&mut self, ctx: &mut Self::Context) {
        let app_state = self.app_state.clone();
        let session_id = self.id;
        let addr = ctx.address();

        ctx.spawn(
            async move {
                let mut state = app_state.lock().await;
                state.ws_manager.add_address(session_id, addr);
            }
            .into_actor(self)
        );
    }


    fn stopping(&mut self, ctx: &mut Self::Context) -> Running {
        let app_state = self.app_state.clone();
        let session_id = self.id;

        ctx.spawn(
            async move {
                let mut state = app_state.lock().await;
                state.ws_manager.remove_address(session_id);
            }
            .into_actor(self)
        );

        Running::Stop
    }
}


impl StreamHandler<Result<Message, ProtocolError>> for Session {
    fn handle(&mut self, _item: Result<Message, ProtocolError>, _ctx: &mut Self::Context) {}
}