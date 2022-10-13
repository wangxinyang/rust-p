use std::{ops::Deref, sync::Arc};

use arc_swap::ArcSwap;
use dashmap::DashMap;
use lazy_static::lazy_static;
use tonic::{
    codegen::InterceptedService, metadata::AsciiMetadataValue, service::Interceptor,
    transport::Channel, Request, Status,
};
use tracing::info;

use crate::tosei::{
    chat_client::ChatClient, GetMessageRequest, GetMessageResponse, LoginRequest,
    SendMessageRequest, Token,
};

lazy_static! {
    static ref TOKEN: ArcSwap<Token> = ArcSwap::from(Arc::new(Token {
        data: "".to_string(),
    }));
}

#[derive(Default, Clone)]
struct Rooms(Arc<DashMap<String, Vec<GetMessageResponse>>>);

impl Rooms {
    fn insert_message(&mut self, msg: GetMessageResponse) {
        let room = msg.room.clone();
        let mut room_messages = self.entry(room).or_insert_with(Vec::new);
        room_messages.push(msg);
    }
}

impl Deref for Rooms {
    type Target = DashMap<String, Vec<GetMessageResponse>>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

pub struct Client {
    username: String,
    conn: ChatClient<InterceptedService<Channel, AuthInterceptor>>,
    rooms: Rooms,
}

impl Client {
    pub async fn new(username: impl Into<String>) -> Self {
        let channel = Channel::from_static("http://127.0.0.1:3200")
            .connect()
            .await
            .unwrap();
        let conn = ChatClient::with_interceptor(channel, AuthInterceptor);
        Self {
            username: username.into(),
            conn,
            rooms: Default::default(),
        }
    }

    /// login from client
    pub async fn login(&mut self) -> anyhow::Result<()> {
        info!("Login self: {:?}", self.username);
        let login = LoginRequest::new(&self.username, "testpassword");
        let token = self.conn.login(Request::new(login)).await?.into_inner();
        // store the token
        TOKEN.store(Arc::new(token));
        Ok(())
    }

    /// send message from client
    pub async fn send_messaeg(
        &mut self,
        room: impl Into<String>,
        content: impl Into<String>,
    ) -> anyhow::Result<()> {
        let msg = SendMessageRequest::new(room, content);
        self.conn.send_message(Request::new(msg)).await?;
        Ok(())
    }

    /// get message from client
    pub async fn get_message(&mut self, username: impl Into<String>) -> anyhow::Result<()> {
        let request = GetMessageRequest::new(username);
        let mut stream = self
            .conn
            .get_message(Request::new(request))
            .await?
            .into_inner();
        let mut rooms = self.rooms.clone();
        tokio::spawn(async move {
            while let Some(msg) = stream.message().await? {
                info!(
                    "get message from :{} content is: {:?}",
                    msg.sender, msg.content
                );
                rooms.insert_message(msg);
            }
            Ok::<_, Status>(())
        });

        Ok(())
    }
}

struct AuthInterceptor;

impl Interceptor for AuthInterceptor {
    fn call(
        &mut self,
        mut request: tonic::Request<()>,
    ) -> Result<tonic::Request<()>, tonic::Status> {
        let token = TOKEN.load();
        if token.is_valid() {
            let header = format!("Bearer {}", token.data);
            let val = AsciiMetadataValue::try_from(header).unwrap();
            request.metadata_mut().insert("authorization", val);
        }

        Ok(request)
    }
}
