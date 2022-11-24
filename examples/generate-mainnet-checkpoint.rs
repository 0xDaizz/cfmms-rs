use std::{error::Error, str::FromStr, sync::Arc};

use ethers::{
    providers::{Http, Provider},
    types::H160,
};

use pool_sync::{
    checkpoint::generate_checkpoint,
    dex::{Dex, DexVariant},
};

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    //Add rpc endpoint here:
    let rpc_endpoint = "";
    let provider = Arc::new(Provider::<Http>::try_from(rpc_endpoint).unwrap());

    let mut dexes = vec![];

    //Add UniswapV2
    dexes.push(Dex::new(
        H160::from_str("0x5C69bEe701ef814a2B6a3EDD4B1652CB9cc5aA6f").unwrap(),
        DexVariant::UniswapV2,
        10000835,
    ));

    //Add Sushiswap
    dexes.push(Dex::new(
        H160::from_str("0xC0AEe478e3658e2610c5F7A4A2E1777cE9e4f2Ac").unwrap(),
        DexVariant::UniswapV2,
        10794229,
    ));

    //Add UniswapV3
    dexes.push(Dex::new(
        H160::from_str("0x1F98431c8aD98523631AE4a59f267346ea31F984").unwrap(),
        DexVariant::UniswapV3,
        12369621,
    ));

    //Sync pools and generate checkpoint
    generate_checkpoint(dexes, provider, String::from("pool_sync_checkpoint")).await?;

    Ok(())
}
