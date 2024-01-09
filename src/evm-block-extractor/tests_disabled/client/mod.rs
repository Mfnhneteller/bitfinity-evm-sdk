use gcp_bigquery_client::Client;
use tempfile::NamedTempFile;
use testcontainers::clients::Cli;
use testcontainers::core::WaitFor;
use testcontainers::{Container, GenericImage};

use self::auth_mock::GoogleAuthMock;

pub async fn new_bigquery_client<'a>(
    docker: &'a Cli,
    project_id: &str,
) -> (
    Client,
    Container<'a, GenericImage>,
    NamedTempFile,
    GoogleAuthMock,
) {
    let emulator_image = GenericImage::new("ghcr.io/goccy/bigquery-emulator", "0.4.4")
        .with_exposed_port(9050)
        .with_wait_for(WaitFor::message_on_stdout(
            "[bigquery-emulator] gRPC server listening at 0.0.0.0:9060",
        ));

    let node = docker.run((emulator_image, vec![format!("--project={project_id}")]));
    let port = node.get_host_port_ipv4(9050);

    let google_auth = auth_mock::GoogleAuthMock::start().await;
    google_auth.mock_token(1).await;

    let google_config = dummy_configuration(&google_auth.uri());
    // Write google configuration to file.
    let temp_file = tempfile::NamedTempFile::new().unwrap();
    std::fs::write(
        temp_file.path(),
        serde_json::to_string_pretty(&google_config).unwrap(),
    )
    .unwrap();

    let gcp_client = gcp_bigquery_client::client_builder::ClientBuilder::new()
        .with_auth_base_url(google_auth.uri())
        .with_v2_base_url(format!("http://localhost:{port}"))
        .build_from_service_account_key_file(temp_file.path().to_str().unwrap())
        .await
        .unwrap();

    (gcp_client, node, temp_file, google_auth)
}

pub mod auth_mock {
    use std::ops::Deref;

    use serde::Serialize;
    use wiremock::matchers::{method, path};
    use wiremock::{Mock, MockServer, ResponseTemplate, Times};

    pub const AUTH_TOKEN_ENDPOINT: &str = "/:o/oauth2/token";

    pub struct GoogleAuthMock {
        server: MockServer,
    }

    impl Deref for GoogleAuthMock {
        type Target = MockServer;

        fn deref(&self) -> &Self::Target {
            &self.server
        }
    }

    impl GoogleAuthMock {
        pub async fn start() -> Self {
            Self {
                server: MockServer::builder().start().await,
            }
        }
    }

    #[derive(Eq, PartialEq, Serialize, Debug, Clone)]
    pub struct Token {
        access_token: String,
        token_type: String,
        expires_in: u32,
    }

    impl Token {
        fn fake() -> Self {
            Self {
                access_token: "aaaa".to_string(),
                token_type: "bearer".to_string(),
                expires_in: 9999999,
            }
        }
    }

    impl GoogleAuthMock {
        /// Mock token, given how many times the endpoint will be called.
        pub async fn mock_token<T: Into<Times>>(&self, n_times: T) {
            let response = ResponseTemplate::new(200).set_body_json(Token::fake());
            Mock::given(method("POST"))
                .and(path(AUTH_TOKEN_ENDPOINT))
                .respond_with(response)
                .named("mock token")
                .expect(n_times)
                .mount(self)
                .await;
        }
    }
}

pub fn dummy_configuration(oauth_server: &str) -> serde_json::Value {
    let oauth_endpoint = format!("{oauth_server}/:o/oauth2");
    serde_json::json!({
      "type": "service_account",
      "project_id": "dummy",
      "private_key_id": "dummy",
      "private_key": "-----BEGIN PRIVATE KEY-----\nMIIEvwIBADANBgkqhkiG9w0BAQEFAASCBKkwggSlAgEAAoIBAQDNk6cKkWP/4NMu\nWb3s24YHfM639IXzPtTev06PUVVQnyHmT1bZgQ/XB6BvIRaReqAqnQd61PAGtX3e\n8XocTw+u/ZfiPJOf+jrXMkRBpiBh9mbyEIqBy8BC20OmsUc+O/YYh/qRccvRfPI7\n3XMabQ8eFWhI6z/t35oRpvEVFJnSIgyV4JR/L/cjtoKnxaFwjBzEnxPiwtdy4olU\nKO/1maklXexvlO7onC7CNmPAjuEZKzdMLzFszikCDnoKJC8k6+2GZh0/JDMAcAF4\nwxlKNQ89MpHVRXZ566uKZg0MqZqkq5RXPn6u7yvNHwZ0oahHT+8ixPPrAEjuPEKM\nUPzVRz71AgMBAAECggEAfdbVWLW5Befkvam3hea2+5xdmeN3n3elrJhkiXxbAhf3\nE1kbq9bCEHmdrokNnI34vz0SWBFCwIiWfUNJ4UxQKGkZcSZto270V8hwWdNMXUsM\npz6S2nMTxJkdp0s7dhAUS93o9uE2x4x5Z0XecJ2ztFGcXY6Lupu2XvnW93V9109h\nkY3uICLdbovJq7wS/fO/AL97QStfEVRWW2agIXGvoQG5jOwfPh86GZZRYP9b8VNw\ntkAUJe4qpzNbWs9AItXOzL+50/wsFkD/iWMGWFuU8DY5ZwsL434N+uzFlaD13wtZ\n63D+tNAxCSRBfZGQbd7WxJVFfZe/2vgjykKWsdyNAQKBgQDnEBgSI836HGSRk0Ub\nDwiEtdfh2TosV+z6xtyU7j/NwjugTOJEGj1VO/TMlZCEfpkYPLZt3ek2LdNL66n8\nDyxwzTT5Q3D/D0n5yE3mmxy13Qyya6qBYvqqyeWNwyotGM7hNNOix1v9lEMtH5Rd\nUT0gkThvJhtrV663bcAWCALmtQKBgQDjw2rYlMUp2TUIa2/E7904WOnSEG85d+nc\norhzthX8EWmPgw1Bbfo6NzH4HhebTw03j3NjZdW2a8TG/uEmZFWhK4eDvkx+rxAa\n6EwamS6cmQ4+vdep2Ac4QCSaTZj02YjHb06Be3gptvpFaFrotH2jnpXxggdiv8ul\n6x+ooCffQQKBgQCR3ykzGoOI6K/c75prELyR+7MEk/0TzZaAY1cSdq61GXBHLQKT\nd/VMgAN1vN51pu7DzGBnT/dRCvEgNvEjffjSZdqRmrAVdfN/y6LSeQ5RCfJgGXSV\nJoWVmMxhCNrxiX3h01Xgp/c9SYJ3VD54AzeR/dwg32/j/oEAsDraLciXGQKBgQDF\nMNc8k/DvfmJv27R06Ma6liA6AoiJVMxgfXD8nVUDW3/tBCVh1HmkFU1p54PArvxe\nchAQqoYQ3dUMBHeh6ZRJaYp2ATfxJlfnM99P1/eHFOxEXdBt996oUMBf53bZ5cyJ\n/lAVwnQSiZy8otCyUDHGivJ+mXkTgcIq8BoEwERFAQKBgQDmImBaFqoMSVihqHIf\nDa4WZqwM7ODqOx0JnBKrKO8UOc51J5e1vpwP/qRpNhUipoILvIWJzu4efZY7GN5C\nImF9sN3PP6Sy044fkVPyw4SYEisxbvp9tfw8Xmpj/pbmugkB2ut6lz5frmEBoJSN\n3osZlZTgx+pM3sO6ITV6U4ID2Q==\n-----END PRIVATE KEY-----\n",
      "client_email": "dummy@developer.gserviceaccount.com",
      "client_id": "dummy",
      "auth_uri": format!("{oauth_endpoint}/auth"),
      "token_uri": format!("{}{}", oauth_server, auth_mock::AUTH_TOKEN_ENDPOINT),
      "auth_provider_x509_cert_url": format!("{oauth_endpoint}/v1/certs"),
      "client_x509_cert_url": format!("{oauth_server}/robot/v1/metadata/x509/457015483506-compute%40developer.gserviceaccount.com")
    })
}
