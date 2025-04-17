use engine::{Engine, Event, Tracer, TracerDelegate};
use revm::{
    bytecode::Bytecode,
    context::TxEnv,
    primitives::{Bytes, TxKind, address},
    state::AccountInfo,
};
use rocket::{
    fs::{FileServer, Options},
    serde::{Serialize, json::Json},
};
use rocket_okapi::{rapidoc::*, settings::UrlObject, swagger_ui::*};
use std::str::FromStr;

#[derive(Default)]
struct Delegate {
    events: Vec<Event>,
}

impl TracerDelegate for Delegate {
    fn emit(&mut self, event: Event) {
        self.events.push(event);
    }
}

#[derive(Serialize)]
struct Response {
    events: Vec<Event>,
}

#[rocket::post("/api/isolate/eval/<code>")]
fn eval(code: &str) -> Result<Json<Response>, String> {
    let mut engine = Engine::new(Tracer::new(Delegate::default()));

    engine.create_account(
        address!("ffffffffffffffffffffffffffffffffffffffff"),
        AccountInfo::from_bytecode(Bytecode::new_raw(
            Bytes::from_str(code).map_err(|err| err.to_string())?,
        )),
    );

    let _ = engine
        .execute(TxEnv {
            kind: TxKind::Call(address!("ffffffffffffffffffffffffffffffffffffffff")),
            gas_limit: 0x1000000,
            ..Default::default()
        })
        .map_err(|err| err.to_string())?;

    Ok(Json(Response {
        events: engine.inspector().get().events.split_off(0),
    }))
}

// TODO(toms): 'test' endpoints
//   * POST /api/health-check
// TODO(toms): 'isolate' endpoints
//   * POST /api/isolate/transaction - execute a transaction in a given state/environment
//     * prestate - block environment?

#[rocket::launch]
fn rocket() -> _ {
    rocket::build()
        .mount("/", rocket::routes![eval])
        .mount("/res", FileServer::new("res", Options::default()))
        .mount(
            "/swagger-ui/",
            make_swagger_ui(&SwaggerUIConfig {
                url: "/res/openapi.json".to_owned(),
                ..Default::default()
            }),
        )
        .mount(
            "/rapidoc/",
            make_rapidoc(&RapiDocConfig {
                general: GeneralConfig {
                    spec_urls: vec![UrlObject::new("General", "/openapi.json")],
                    ..Default::default()
                },
                hide_show: HideShowConfig {
                    allow_spec_url_load: false,
                    allow_spec_file_load: false,
                    ..Default::default()
                },
                ..Default::default()
            }),
        )
}
