#[my_grpc_client_macros::generate_grpc_client(
    proto_file: "./proto/AccountsManagerGrpcService.proto",
    crate_ns: "crate::accounts_manager_grpc",
    retries: 3,
    request_timeout_sec: 1,
    ping_timeout_sec: 1,
    ping_interval_sec: 3,
)]
pub struct AccountsManagerGrpcClient {
    channel: my_grpc_extensions::GrpcChannel<TGrpcService>,
}

/*

use std::{sync::Arc, time::Duration};

use my_grpc_extensions::{GrpcChannel, GrpcClientSettings};
use tonic::transport::Channel;

use crate::{
    accounts_manager::{
        accounts_manager_grpc_service_client::AccountsManagerGrpcServiceClient,
        AccountManagerGetClientAccountGrpcRequest, AccountManagerGetClientAccountsGrpcRequest, AccountGrpcModel,
    },
    AccountSignalRModel,
};



struct AccountsManagerSettingsGrpcUrl(String);

impl AccountsManagerSettingsGrpcUrl {
    pub fn new(url: String) -> Self {
        Self(url)
    }
}

#[tonic::async_trait]
impl GrpcClientSettings for AccountsManagerSettingsGrpcUrl {
    async fn get_grpc_url(&self, _: &'static str) -> String {
        self.0.clone()
    }
}

pub struct AccountsManagerGrpcClient {
    channel: GrpcChannel,
    timeout: Duration,
}

impl AccountsManagerGrpcClient {
    pub async fn new(grpc_address: String) -> Self {
        Self {
            channel: GrpcChannel::new(
                Arc::new(AccountsManagerSettingsGrpcUrl::new(grpc_address)),
                "accounts_manager",
                Duration::from_secs(10),
            ),
            timeout: Duration::from_secs(2),
        }
    }

    async fn create_grpc_service(&self) -> AccountsManagerGrpcServiceClient<Channel> {
        return AccountsManagerGrpcServiceClient::new(self.channel.get_channel().await.unwrap());
    }

    pub async fn get_client_account(
        &self,
        trader_id: &str,
        account_id: &str,
    ) -> Option<AccountGrpcModel> {
        let mut grpc_client = self.create_grpc_service().await;

        let request = AccountManagerGetClientAccountGrpcRequest {
            trader_id: trader_id.to_string(),
            account_id: account_id.to_string(),
        };

        let response = grpc_client
            .get_client_account(request)
            .await
            .unwrap()
            .into_inner();

        match response.account {
            Some(account) => Some(account),
            None => None,
        }
    }

    pub async fn get_client_accounts(&self, trader_id: &str) -> Vec<AccountSignalRModel> {
        let mut grpc_client = self.create_grpc_service().await;
        let request = AccountManagerGetClientAccountsGrpcRequest {
            trader_id: trader_id.to_string(),
        };

        let accounts = my_grpc_extensions::read_grpc_stream::as_vec(
            grpc_client
                .get_client_accounts(tonic::Request::new(request))
                .await
                .unwrap()
                .into_inner(),
            self.timeout,
        )
        .await
        .unwrap();

        let accounts = match accounts {
            Some(acc) => acc,
            None => vec![],
        };

        let accounts = accounts.iter().map(|acc| acc.to_owned().into()).collect();

        return accounts;
    }
}
 */
