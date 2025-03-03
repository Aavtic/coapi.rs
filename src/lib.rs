pub mod web_server;
pub mod runner;
pub mod poller; 

use web_server::axum_serve;

pub const WEB_HOST: &str = "192.168.213.165";
pub const WEB_PORT: i32 = 8081;

pub fn server() {
    axum_serve::code_output_api(&format!("{}:{}", WEB_HOST, WEB_PORT));
}
