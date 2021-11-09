use serde_json::{json, Value};
use yew::{
    format::Json,
    html,
    prelude::*,
    services::{
        fetch::{FetchTask, Request, Response},
        ConsoleService, FetchService,
    },
};

use crate::components::ws::WebsocketMsg;
use crate::{
    components,
    protocal::{common::*, vnc::VncHandler},
    utils::WeakComponentLink,
};

pub struct PageRemote {
    link: ComponentLink<Self>,
    target: (String, u16),
    error_msg: String,
    fetch_task: Option<FetchTask>,
    connected: bool,
    handler: ProtocalHandler<VncHandler>,
    ws_link: WeakComponentLink<components::ws::WebsocketCtx>,
}

#[derive(Clone, PartialEq, Properties)]
pub struct RemoteProps {}

pub enum RemoteMsg {
    Connect((String, u16)),
    ConnectResp(Result<Value, anyhow::Error>),
    Connected,
    Recv(Vec<u8>),
    Send(Vec<u8>),
}

impl Component for PageRemote {
    type Message = RemoteMsg;
    type Properties = RemoteProps;

    fn create(_: Self::Properties, link: ComponentLink<Self>) -> Self {
        PageRemote {
            link,
            target: (String::from(""), 0),
            error_msg: String::from(""),
            fetch_task: None,
            connected: false,
            handler: ProtocalHandler::new(),
            ws_link: WeakComponentLink::default(),
        }
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        match msg {
            RemoteMsg::Connect(target) => {
                self.target = target;
                // ConsoleService::log(&self.target);
                let to_post = json!({
                    "ip": self.target.0,
                    "port": self.target.1,
                });

                // 1. build the request
                let request = Request::post("/target/remote")
                    .header("Content-Type", "application/json")
                    .body(Json(&to_post))
                    .expect("Could not build auth request.");
                // 2. construct a callback
                let callback =
                    self.link
                        .callback(|response: Response<Json<Result<Value, anyhow::Error>>>| {
                            // ConsoleService::error(&format!("{:?}", response));
                            let Json(data) = response.into_body();
                            RemoteMsg::ConnectResp(data)
                        });
                // 3. pass the request and callback to the fetch service
                let task = FetchService::fetch(request, callback).expect("failed to start request");
                // 4. store the task so it isn't canceled immediately
                self.fetch_task = Some(task);
                true
            }
            RemoteMsg::ConnectResp(response) => {
                if let Ok(response) = response {
                    self.error_msg = response["status"].to_string();

                    if "\"success\"" == self.error_msg {
                        self.link.send_message(RemoteMsg::Connected);
                    } else {
                        self.error_msg = response["message"].to_string();
                    }
                } else {
                    self.error_msg = String::from("Connect host failed with unknown reason");
                    ConsoleService::error(&format!("{:?}", response.unwrap_err().to_string()));
                }
                // release resources
                self.fetch_task = None;
                true
            }
            RemoteMsg::Connected => {
                self.connected = true;
                true
            }
            RemoteMsg::Recv(v) => {
                let out = self.handler.handle(&v);
                match out {
                    ProtocalHandlerOutput::Err(err) => {
                        self.error_msg = err.clone();
                        true
                    }
                    ProtocalHandlerOutput::Ok => false,
                    ProtocalHandlerOutput::WsBuf(out) => {
                        self.link.send_message(RemoteMsg::Send(out));
                        false
                    }
                }
            }
            RemoteMsg::Send(v) => {
                self.ws_link
                    .borrow()
                    .as_ref()
                    .unwrap()
                    .send_message(WebsocketMsg::Send(Ok(v)));
                false
            }
        }
    }

    fn change(&mut self, _: Self::Properties) -> ShouldRender {
        false
    }

    fn view(&self) -> Html {
        if !self.connected {
            let connect_remote = self.link.callback(RemoteMsg::Connect);
            html! {
                <>
                    <components::host::Host onsubmit=connect_remote/>
                    {self.error_msg.clone()}
                </>
            }
        } else {
            let recv_msg = self.link.callback(|v| RemoteMsg::Recv(v));
            let ws_link = &self.ws_link;
            html! {
                <>
                    <components::ws::WebsocketCtx
                    weak_link=ws_link onrecv=recv_msg/>
                    {self.error_msg.clone()}
                </>
            }
        }
    }
}
