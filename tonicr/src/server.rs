use std::pin::Pin;

use futures::Stream;
use tonic::{Request, Response, Status};
use tracing::info;

use crate::tosei::{
    chat_server::Chat, GetMessageRequest, GetMessageResponse, LoginRequest, SendMessageRequest,
    SendMessageResponse, Token,
};

struct ChatService;

type ChatResult<T> = Result<Response<T>, Status>;

#[tonic::async_trait]
impl Chat for ChatService {
    async fn login(&self, request: Request<LoginRequest>) -> ChatResult<Token> {
        let info = request.into_inner();
        info!("login: {info:?}");
        let token = info.get_token();
        Ok(Response::new(token))
    }

    async fn send_message(
        &self,
        request: Request<SendMessageRequest>,
    ) -> ChatResult<SendMessageResponse> {
        // assumes get the sender from the head data
        let sender = "tosei";
        let info = request.into_inner();
        info!("send message request: {info:?}");
        // store the send message request into something
        // how to publish the message??
        let _ = info.to_get_msg_res(sender);
        Ok(Response::new(SendMessageResponse {}))
    }

    type GetMessageStream =
        Pin<Box<dyn Stream<Item = Result<GetMessageResponse, Status>> + Send + 'static>>;
    async fn get_message(
        &self,
        request: Request<GetMessageRequest>,
    ) -> ChatResult<Self::GetMessageStream> {
        let info = request.into_inner();
        info!("get message request: {info:?}");
        unimplemented!()
    }
}
