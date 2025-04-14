use rocket::get;
use rocket_okapi::settings::UrlObject;
use rocket_okapi::{openapi, openapi_get_routes, rapidoc::*, swagger_ui::*};

// OpenAPI: https://github.com/GREsau/okapi/blob/master/examples/json-web-api/src/main.rs

#[openapi]
#[get("/hello/<name>/<age>")]
fn hello(name: &str, age: u8) -> String {
    format!("Hello, {} year old named {}!", age, name)
}

// TODO(toms): 'test' endpoints
//   * POST /api/health-check
// TODO(toms): 'isolate' endpoints
//   * POST /api/isolate/transaction - execute a transaction in a given state/environment
//   * POST /api/isolate/code - execute raw (EVM) code

#[rocket::launch]
fn rocket() -> _ {
    rocket::build()
        .mount("/", openapi_get_routes![hello])
        .mount(
            "/swagger-ui/",
            make_swagger_ui(&SwaggerUIConfig {
                url: "../openapi.json".to_owned(),
                ..Default::default()
            }),
        )
        .mount(
            "/rapidoc/",
            make_rapidoc(&RapiDocConfig {
                general: GeneralConfig {
                    spec_urls: vec![UrlObject::new("General", "../openapi.json")],
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
