use smart_contract_scanner::{env, web3::create_web3_client};

#[tokio::main]
async fn main(){
    env::load_env();

    let node_url = env::get("NODE_URL").unwrap();
    let web3 = create_web3_client(&node_url);


    println!("{:#?}", web3.eth().chain_id().await);
}