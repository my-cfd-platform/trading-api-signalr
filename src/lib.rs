mod app;
mod signalr;
mod grpc_clients;
mod services;
mod bg;

pub mod accounts_manager {
    tonic::include_proto!("accounts_manager");
}
pub mod trading_executor {
    tonic::include_proto!("trading_executor");
}

pub use app::*;
pub use bg::*;
pub use signalr::*;
pub use services::*;
pub use grpc_clients::*;