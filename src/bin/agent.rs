use tungstenite::{connect, Message};
use url::Url;
use x_gpu_agent::read_uuid;

fn main() {
    // read uuid
    let uuid = read_uuid().unwrap();

    let (mut socket, response) =
        connect(Url::parse("ws://localhost:3000/ws/53a5000a-3c83-4e4d-9fad-e843942854a5").unwrap())
            .expect("Can't connect");

    socket.write_message(Message::Text(
        r#"{
        "action": "authenticate",
        "data": {
            "key_id": "API-KEY",
            "secret_key": "SECRET-KEY"
        }
    }"#
        .into(),
    ));

    socket.write_message(Message::Text(
        r#"{
        "action": "listen",
        "data": {
            "streams": ["AM.SPY"]
        }
    }"#
        .into(),
    ));

    loop {
        let msg = socket.read_message().expect("Error reading message");
        println!("Received: {}", msg);
    }
}
