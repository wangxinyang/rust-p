use std::pin::Pin;

use futures::Stream;
use tokio::sync::{broadcast, mpsc::unbounded_channel};
use tokio_stream::wrappers::UnboundedReceiverStream;
use tonic::{transport::Server, Code, Extensions, Request, Response, Status};
use tracing::{info, warn};

use crate::tosei::{
    chat_server::{Chat, ChatServer},
    GetMessageRequest, GetMessageResponse, LoginRequest, SendMessageRequest, SendMessageResponse,
    Token,
};

const MAX_CHANNEL_NUM: usize = 1024;

struct ChatService {
    sender: broadcast::Sender<GetMessageResponse>,
}

type ChatResult<T> = Result<Response<T>, Status>;

#[tonic::async_trait]
impl Chat for ChatService {
    /// login method
    async fn login(&self, request: Request<LoginRequest>) -> ChatResult<Token> {
        let info = request.into_inner();
        info!("login: {info:?}");
        let token = info.get_token();
        Ok(Response::new(token))
    }

    /// send message method
    async fn send_message(
        &self,
        request: Request<SendMessageRequest>,
    ) -> ChatResult<SendMessageResponse> {
        // assumes get the sender from the head data
        let sender = get_user_name(request.extensions())?;
        let info = request.into_inner();
        info!("send message request: {info:?}");
        // store the send message request into something
        // how to publish the message??
        let msg = info.to_get_msg_res(sender);
        self.sender.send(msg).unwrap();
        Ok(Response::new(SendMessageResponse {}))
    }

    /// get message method
    type GetMessageStream =
        Pin<Box<dyn Stream<Item = Result<GetMessageResponse, Status>> + Send + 'static>>;
    async fn get_message(
        &self,
        request: Request<GetMessageRequest>,
    ) -> ChatResult<Self::GetMessageStream> {
        let info = request.into_inner();
        info!("get message request: {info:?}");
        let mut rx = self.sender.subscribe();
        let (utx, urx) = unbounded_channel();
        tokio::spawn(async move {
            // filter message if i am not sub
            while let Ok(msg) = rx.recv().await {
                if msg.sender != info.sender {
                    if let Err(e) = utx.send(Ok(msg)) {
                        warn!("Error sending message: {:?}", e)
                    }
                }
            }
        });
        let stream = UnboundedReceiverStream::new(urx);
        Ok(Response::new(Box::pin(stream)))
    }
}

impl Default for ChatService {
    fn default() -> Self {
        let (tx, _rx) = broadcast::channel(MAX_CHANNEL_NUM);
        Self { sender: tx }
    }
}

/// サーバを開始する
pub async fn start() {
    let service = ChatServer::with_interceptor(ChatService::default(), check_auth);
    let addr = "0.0.0.0:3200";
    info!("the server is listening on {:?}", addr);
    Server::builder()
        .add_service(service)
        .serve(addr.parse().unwrap())
        .await
        .unwrap();
}

/// インターセプタ　権限をチェックする
fn check_auth(mut req: Request<()>) -> Result<Request<()>, Status> {
    // metedataからauthorizationの情報を取得する
    let token = match req.metadata().get("authorization") {
        Some(val) => {
            // MetadataValue -> &str
            let data = val
                .to_str()
                .map_err(|_| Status::new(Code::Unauthenticated, "Invalid token format"))?;
            info!("token data is : {data:?}");
            Token::new(data.strip_prefix("Bearer ").unwrap())
        }
        None => Token::default(),
    };
    req.extensions_mut().insert(token);
    Ok(req)
}

fn get_user_name(ext: &Extensions) -> Result<String, Status> {
    let token = ext
        .get::<Token>()
        .ok_or_else(|| Status::unauthenticated("No Token"))?;

    if token.is_valid() {
        Ok(token.into_username())
    } else {
        Err(Status::unauthenticated("Invalid Token"))
    }
}
