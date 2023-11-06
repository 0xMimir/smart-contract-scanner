use web3::{Web3, transports::Http};

/// Change this to return websocket if url is for websocket and http if url is for https
pub fn create_web3_client(url: &str) -> Web3<Http>{
    let transport = Http::new(url).expect("Error creating transport");
    Web3::new(transport)
}