pub mod web_server;
pub mod runner;
pub mod poller; 

use web_server::axum_serve;

pub const WEB_HOST: &str = "127.0.0.1";

pub fn server() {
    axum_serve::code_output_api(&format!("{}:8081", WEB_HOST));
}
