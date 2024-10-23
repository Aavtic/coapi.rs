//use coapi_rs;
mod language_client; 

fn main() {
    //coapi_rs::server();
    language_client::python_client::python_client::get_output();
}
