mod apis;
mod thai_bath_text;
mod thai_encoding;
mod thai_text_number_utility;

use actix_web::{App, Error, HttpServer, web};
use actix_files as fs;
use actix_files::NamedFile;
use actix_web::middleware::Logger;
use logs::info;
use opentelemetry::global::shutdown_tracer_provider;
use actix_web_opentelemetry::RequestTracing;

use crate::apis::*;
include!(concat!(env!("OUT_DIR"),"/thai_charset.rs"));

pub async fn index() -> Result<NamedFile, Error> {
    Ok(NamedFile::open("./static/index.html")?)
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    std::env::set_var("RUST_LOG", "actix_web=info");
    env_logger::init();


    info!("Start opentelemetry application insights");
    let instrumentation_key = String::from("33b6362c-87a9-4359-bff9-ad493aadb60e");//std::env::var("INSTRUMENTATION_KEY").unwrap();
    //let tracer = opentelemetry_application_insights::new_pipeline(instrumentation_key)
    //.with_client(reqwest::blocking::Client::new())
    //.install_simple();
    let tracer = opentelemetry_application_insights::new_pipeline(instrumentation_key)
        //.with_endpoint("https://centralus-2.in.applicationinsights.azure.com")
        .with_client(reqwest::Client::new())
        .with_endpoint("https://centralus-2.in.applicationinsights.azure.com").unwrap()
        .install_batch(opentelemetry::runtime::TokioCurrentThread);
    info!("Get Tracer already");

    //InstrumentationKey=447d328d-a287-426d-a5f2-c03f2a7c909e;IngestionEndpoint=https://centralus-2.in.applicationinsights.azure.com/;LiveEndpoint=https://centralus.livediagnostics.monitor.azure.com/



    HttpServer::new(|| {
        App::new()
            .wrap(Logger::default())
            .wrap(Logger::new("%a %{User-Agent}i"))
            .wrap(RequestTracing::new())
            .service(apis::do_baht_text_handler)
            .service(apis::do_text_sorting_handler)
            //.service(fs::Files::new("/", ".").prefer_utf8(true).index_file("index.html"))
            .route("/", web::get().to(index))
    }).workers(20)
        .bind("0.0.0.0:8000")?
        .run()
        .await?;

    // wait until all pending spans get exported.
    shutdown_tracer_provider();
    Ok(())
}
