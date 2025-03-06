use super::util::{current_date_and_time, magic_number};
use colored::*;
use ethers::{
    abi::ParamType,
    middleware::SignerMiddleware,
    prelude::*,
    providers::Provider,
    signers::{LocalWallet, Signer},
    types::Address,
};
use futures::future::join_all;
use std::{
    collections::HashMap,
    sync::Arc,
    time::{SystemTime, UNIX_EPOCH},
};
use tokio::{
    task,
    time::{sleep, Duration},
};

#[derive(Clone)]
pub struct Monitor {
    pub http_provider: Arc<Provider<Http>>,
    pub ws_provider: Arc<Provider<Ws>>,
    pub pair: Address,
    pub hope: Address,
    pub nft: Address,
    pub current_block: u64,
    pub current_price: f64,
    pub threshold: f64, // 价格阈值(默认为100U，此处设为99.99U)
}

impl Monitor {
    pub async fn new(rpc_url: &str, pair: &str, hope: &str, nft: &str) -> Self {
        let http_provider = Arc::new(Self::get_http_provider(rpc_url).await);
        let ws_provider = Arc::new(Self::get_ws_provider(rpc_url).await);

        let pair: Address = pair.parse().expect("Invalid pair address");
        let hope: Address = hope.parse().expect("Invalid HOPE address");
        let nft: Address = nft.parse().expect("Invalid NFT address");

        let current_block = 0;
        let current_price = 0.0;
        let threshold = 99.99;

        Self {
            http_provider,
            ws_provider,
            pair,
            hope,
            nft,
            current_block,
            current_price,
            threshold,
        }
    }

    pub async fn run(&self) -> eyre::Result<()> {
        println!("👀 Monitoring the NFT:Claim & Swap:Buy event of $HOPE in Coinfair...\n\n");

        let event_trading_start = "TradingStart()";
        let event_swap = "Swap(address,uint256,uint256,uint256,uint256,address)";

        let filter = Filter::new()
            .address(vec![self.hope, self.pair])
            .events(vec![event_trading_start, event_swap]);

        let mut stream = self.ws_provider.subscribe_logs(&filter).await.unwrap();
        while let Some(log) = stream.next().await {
            println!("Log received: {:?}\n", log);
            println!("Log received\n");

            if let Some(topics) = log.topics.first() {
                match *topics {
                    x if x == magic_number(event_trading_start) => {
                        // 解码 TradingStart 参数
                        println!("TradingStart Event");

                        // self.batch_buy(99, true).await?;

                        println!("{:?}", current_date_and_time());
                    }

                    x if x == magic_number(event_swap) => {
                        // 解码 Swap 参数

                        let decoded = ethers::abi::decode(
                            &[
                                ParamType::Uint(256), // amount0In
                                ParamType::Uint(256), // amount1In
                                ParamType::Uint(256), // amount0Out
                                ParamType::Uint(256), // amount1Out
                            ],
                            &log.data,
                        )
                        .unwrap();

                        let sender: Address = H256::into(log.topics[1].into());
                        let to: Address = H256::into(log.topics[2].into());
                        let amount0_in: U256 = decoded[0].clone().into_uint().unwrap();
                        let amount1_in: U256 = decoded[1].clone().into_uint().unwrap();
                        let amount0_out: U256 = decoded[2].clone().into_uint().unwrap();
                        let amount1_out: U256 = decoded[3].clone().into_uint().unwrap();

                        if self.is_buy(amount0_in, amount1_in, amount0_out, amount1_out) {
                            println!(
                                "{:?} Buy Event: {} -> {}",
                                current_date_and_time(),
                                sender,
                                to
                            );
                        } else {
                            println!("Sell Event");
                        }

                        // println!(
                        //     "Sender: {:?}, To: {:?}, Amount0In: {:?}, Amount1In: {:?}, Amount0Out: {:?}, Amount1Out: {:?}\n",
                        //     sender, to, amount0_in, amount1_in, amount0_out, amount1_out
                        // );
                    }
                    _ => println!("Unknown event received"),
                }
            }
        }

        Ok(())
    }
}

impl Monitor {
    // async fn get_latest_block(&self) -> u64 {
    //     let provider = self.provider.clone();
    //     provider.get_block_number().await.unwrap()
    // }

    async fn get_http_provider(rpc_url: &str) -> Provider<Http> {
        Provider::<Http>::try_from(rpc_url).expect("Cannot establish http connection")
    }

    async fn get_ws_provider(rpc_url: &str) -> Provider<Ws> {
        Provider::<Ws>::connect(rpc_url)
            .await
            .expect("Cannot establish ws connection")
    }

    fn is_buy(
        &self,
        amount0_in: U256,
        amount1_in: U256,
        amount0_out: U256,
        amount1_out: U256,
    ) -> bool {
        amount1_in > U256::zero()
            && amount0_out > U256::zero()
            && amount0_in.is_zero()
            && amount1_out.is_zero()
    }
}
