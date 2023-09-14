#[my_grpc_client_macros::generate_grpc_client(
    proto_file: "./proto/AccountsManagerGrpcService.proto",
    crate_ns: "crate::accounts_manager_grpc",
    retries: 3,
    request_timeout_sec: 1,
    ping_timeout_sec: 1,
    ping_interval_sec: 3,
)]
pub struct AccountsManagerGrpcClient;
