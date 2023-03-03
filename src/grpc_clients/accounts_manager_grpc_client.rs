use std::{sync::Arc, time::Duration};

use my_grpc_extensions::{GrpcChannel, GrpcClientSettings};
use tonic::transport::Channel;

use crate::{
    accounts_manager::{
        accounts_manager_grpc_service_client::AccountsManagerGrpcServiceClient,
        AccountManagerGetClientAccountsGrpcRequest,
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

        let accounts = accounts
            .iter()
            .map(|acc| AccountSignalRModel {
                id: acc.id.clone(),
                balance: acc.balance,
                bonus: 0.0,
                currency: acc.currency.clone(),
                is_live: true,
                digits: 2,
                symbol: "USD".to_string(),
                timestamp: acc.last_update_date,
                invest_amount: 1000.0,
                achievement_status: "".to_string(),
                free_to_withdrawal: 0,
            })
            .collect();

        return accounts;
    }
}
