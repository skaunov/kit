use crate::hyperware::process::chat::{
    Request as ChatRequest, Response as ChatResponse, SendRequest,
};
use hyperware_process_lib::{
    await_next_message_body, call_init, println, Address, Message, Request,
};

wit_bindgen::generate!({
    path: "../target/wit",
    world: "chat-template-dot-os-v0",
    generate_unused_types: true,
    additional_derives: [serde::Deserialize, serde::Serialize],
});

call_init!(init);
fn init(our: Address) {
    if let Ok(body) = await_next_message_body() {
        if let Some((target, message)) = String::from_utf8(body).unwrap_or_default().split_once(" ")
        {
            if let Ok(Ok(Message::Response { body, .. })) =
                Request::to((our.node(), ("chat", "chat", "template.os")))
                    .body(
                        serde_json::to_vec(&ChatRequest::Send(SendRequest {
                            target: target.into(),
                            message: message.into(),
                        }))
                        .unwrap(),
                    )
                    .send_and_await_response(5)
            {
                if Ok(ChatResponse::Send) != serde_json::from_slice(&body) {
                    println!("did not receive expected Ack from chat:chat:template.os");
                }
            } else {
                println!("did not receive expected `Response` from chat:chat:template.os");
            }
        } else {
            println!("usage:\nsend:chat:template.os target message");
        }
    } else {
        println!("failed to get args!");
    }
}
