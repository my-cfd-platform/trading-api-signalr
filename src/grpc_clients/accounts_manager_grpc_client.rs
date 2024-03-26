use service_sdk::async_trait;
use service_sdk::my_grpc_extensions;
use service_sdk::my_telemetry;


#[service_sdk::my_grpc_extensions::client::generate_grpc_client(
    proto_file: "./proto/AccountsManagerGrpcService.proto",
    crate_ns: "crate::accounts_manager_grpc",
    retries: 3,
    request_timeout_sec: 30,
    ping_timeout_sec: 1,
    ping_interval_sec: 3,
)]
pub struct AccountsManagerGrpcClient;
