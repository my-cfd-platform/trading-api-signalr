mod app;
mod signalr;
mod grpc_clients;

pub mod accounts_manager {
    tonic::include_proto!("accounts_manager");
}
pub mod trading_executor {
    tonic::include_proto!("trading_executor");
}

pub use app::*;
pub use signalr::*;
pub use grpc_clients::*;