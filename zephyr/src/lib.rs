use serde::{Deserialize, Serialize};
use zephyr_sdk::{prelude::{Limits, WriteXdr}, AgnosticRequest, EnvClient, Method};

const CONTRACT: [u8; 32] = [ 55, 39, 173, 99, 8, 157, 92, 140, 101, 5, 218, 54, 140, 54, 63, 73, 186, 46, 55, 232, 23, 166, 86, 22, 112, 140, 180, 187, 232, 51, 187, 131 ];

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