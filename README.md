# rust-zaim

![Crates.io Version](https://img.shields.io/crates/v/zaim)
[![License](https://img.shields.io/badge/license-MIT-blue.svg)](https://opensource.org/licenses/MIT)

rust-zaim is a Rust client library for the [Zaim API](https://dev.zaim.net/home)

## Installation

```sh
cargo add zaim
```

## Methods

### `new()`

Initialize a new Zaim client.
Required secrets should be provided as arguments.

Obtain credentials on the [Zaim Developers Center](https://dev.zaim.net/)

```rs
use dotenv::dotenv;
use std::env;
use zaim::client::Zaim;

#[tokio::main]
async fn main() {
    dotenv().ok(); // Load environment variables

    let zaim = Zaim::new(
        env::var("ZAIM_CUSTOMER_ID").unwrap(),
        env::var("ZAIM_CUSTOMER_SECRET").unwrap(),
        env::var("ZAIM_TOKEN").unwrap(),
        env::var("ZAIM_SECRET").unwrap(),
        env::var("ZAIM_VERIFIER").unwrap(),
    );
}
```

### `fetch_me()`

Fetch user information during authentication.

```rs
match zaim.fetch_me().await {
    Ok(me_response) => {
        let me = me_response.me;
        println!("{:?}", me);
    },
    Err(e) => {
        eprintln!("Error fetching user information: {}", e);
    }
}
```

## License

This project is licensed under the MIT License - see the [LICENSE](LICENSE) file for details.
