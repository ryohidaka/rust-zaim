mod mock;

#[cfg(test)]
mod tests {
    use crate::mock::create_mock_server;
    use zaim::types::me::{Me, MeResponse};
    use zaim::zaim::me;

    #[tokio::test]
    async fn test_fetch_me() {
        // Arrange
        let mut mock_server = create_mock_server();
        let expected_me: Me = Default::default();
        let expected_response: MeResponse = Default::default();
        let path = "/home/user/verify";
        let body = serde_json::to_string(&expected_response)
            .expect("Failed to serialize MeResponse to JSON");

        let _mock = mock_server
            .mock("GET", path)
            .with_status(201)
            .with_header("content-type", "application/json")
            .with_header("x-api-key", "1234")
            .with_body(body)
            .create();

        // Act
        let me = me::fetch_me().await.me;

        // Assert
        assert_eq!(me, expected_me);
    }
}
