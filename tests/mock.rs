use std::env;

use mockito::{Server, ServerGuard};

pub fn create_mock_server() -> ServerGuard {
    // Set up mock server
    let mock_server = setup_mock_server();
    
    // Set BASE_URL environment variable
    set_base_url_env(&mock_server);

    mock_server
}

fn setup_mock_server() -> ServerGuard {
    Server::new()
}

fn set_base_url_env(mock_server: &ServerGuard) {
    let url = mock_server.url() + "/";
    env::set_var("BASE_URL", url);
}
