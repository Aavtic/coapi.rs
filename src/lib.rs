pub mod web_server;
pub mod runner;

use web_server::axum_serve;


pub fn server() {
    axum_serve::code_output_api("0.0.0.0:8081", "/api/v1");
}
