# tonicr
Rustのtonicを利用し、protoファイルを解析して、gRpcファイルに生成して、チャットのサーバ側とクライアント側を実現するのtonicrです。

> Image

![chat](https://s1.imagehub.cc/images/2022/10/13/chat.gif)



> Proto

```
syntax = "proto3";

package api;

message LoginRequest {
    string name = 1;
    string pwd = 2;
}

message Token {
    string data = 1;
}

message SendMessageRequest  {
    string room = 1;
    string content = 2;
}

message SendMessageResponse{}

message GetMessageRequest{}

message GetMessageResponse {
    string room = 1;
    string content = 2;
    string sender = 3;
    int64 timestamp = 4;

}

service Chat {
    rpc Login(LoginRequest) returns (Token);
    rpc SendMessage(SendMessageRequest) returns (SendMessageResponse);
    rpc GetMessage(GetMessageRequest) returns (stream GetMessageResponse);
}
```
