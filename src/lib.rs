mod app;
mod bg;
mod grpc_clients;
mod services;
mod settings;
mod signalr;

pub mod accounts_manager_grpc {
    tonic::include_proto!("accounts_manager");
}
pub mod trading_executor_grpc {
    tonic::include_proto!("trading_executor");
}

pub use app::*;
pub use bg::*;
pub use grpc_clients::*;
pub use services::*;
pub use signalr::*;
