use serde::{Deserialize, Serialize};
use zephyr_sdk::{prelude::{Limits, WriteXdr}, AgnosticRequest, EnvClient, Method};

const CONTRACT: [u8; 32] = [ 207, 245, 120, 180, 65, 171, 176, 234, 132, 81, 210, 173, 68, 135, 13, 79, 37, 209, 58, 171, 89, 75, 69, 60, 33, 117, 59, 194, 240, 128, 73, 78 ];

#[derive(Serialize, Deserialize)]
pub struct Body {
    pub topics: Vec<String>,
    pub data: String
}

#[no_mangle]
pub extern "C" fn on_close() {
    let env = EnvClient::new();

    for event in env.reader().pretty().soroban_events() {
        if event.contract == CONTRACT {
            env.log().debug(format!("{:?}", "fired 1"), None);

            let mut topics: Vec<String> = Vec::default();

            for topic in event.topics.iter() {
                topics.push(topic.to_xdr_base64(Limits::none()).unwrap());
            }

            let event_body = Body {
                topics,
                data: event.data.to_xdr_base64(Limits::none()).unwrap(),
            };

            env.log().debug(format!("{:?}", "fired 2"), None);

            let body = serde_json::to_string(&event_body).unwrap();

            env.log().debug(format!("{:?}", body), None);

            env.send_web_request(AgnosticRequest {
                body: Some(body),
                url: "https://smol-be.sdf-ecosystem.workers.dev/zephyr".into(), // TODO make this an env var
                method: Method::Post,
                headers: vec![("Content-Type".into(), "application/json".into())],
            });
        }
    }
}