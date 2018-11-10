use std::fs;

use base64;
use jsonrpc_client::{
    header::{HeaderMap, HeaderValue, AUTHORIZATION},
    ClientError, HTTPClient, JsonRpcVersion, RpcClient, RpcError, RpcRequest,
};
use serde::{de::DeserializeOwned, ser::Serialize};
use std::fmt::Debug;
use rpc::*;
use TransactionId;
use KomodoRpcApi;
use assetchains::Assetchain;
use dirs;

use arguments::AddressList;
use std::collections::HashMap;

pub struct Client {
    client: RpcClient,
}

impl Client {
    pub fn new(username: &str, password: &str) -> Self {
        let mut headers = HeaderMap::new();

        headers.insert(
            AUTHORIZATION,
            HeaderValue::from_str(&format!(
                "Basic {}",
                base64::encode(&format!("{}:{}", username, password))
            )).unwrap(),
        );

        // todo: show helpful error when credentials are false

        let client = HTTPClient::builder()
            .default_headers(headers)
            .build()
            .expect("unable to create http client");

        let rpc_client = RpcClient::new(client, "http://127.0.0.1:7771");

        Client {
            client: rpc_client,
        }
    }

    pub fn new_assetchain(ac: Assetchain) -> Self {

        let config = Config::get(ac);

        let mut headers = HeaderMap::new();

        headers.insert(
            AUTHORIZATION,
            HeaderValue::from_str(&format!(
                "Basic {}",
                base64::encode(&format!("{}:{}", config.rpc_user, config.rpc_password))
            )).unwrap(),
        );

        // todo: show helpful error when credentials are false

        let client = HTTPClient::builder()
            .default_headers(headers)
            .build()
            .expect("unable to create http client");

        let rpc_client = RpcClient::new(
            client,
            &format!("http://127.0.0.1:{}", config.rpc_port)
        );

        Client {
            client: rpc_client,
        }
    }

    fn send<R: DeserializeOwned + Debug, P: Serialize + Debug>(
        &self,
        request: &RpcRequest<P>
    ) -> Result<Result<R, RpcError>, ClientError> {
        let result = self.client.send::<R, P>(request);

        match result {
            Ok(Err(ref rpc_error)) if rpc_error.code == -28 => {
                println!("komodod is still booting, try again")
            }
            _ => return result
        }
        self.client.send(request)
    }
}

impl KomodoRpcApi for Client {
    fn get_transaction(
        &self,
        tx: &TransactionId,
    ) -> Result<Result<Transaction, RpcError>, ClientError> {
        self.send(&RpcRequest::new1(
            JsonRpcVersion::V1,
            "777",
            "gettransaction",
            tx,
        ))
    }

    fn get_info(
        &self,
    ) -> Result<Result<Info, RpcError>, ClientError> {
        self.send(&RpcRequest::new0(
            JsonRpcVersion::V1,
            "curltest",
            "getinfo"
        ))
    }

    fn get_best_block_hash(&self) -> Result<Result<BlockHash, RpcError>, ClientError> {
        self.send(&RpcRequest::new0(
            JsonRpcVersion::V1,
            "777",
            "getbestblockhash",
        ))
    }

    fn get_new_address(&self) -> Result<Result<String, RpcError>, ClientError> {
        self.send(&RpcRequest::new0(
            JsonRpcVersion::V1,
            "777",
            "getnewaddress",
        ))
    }

    fn get_difficulty(&self) -> Result<Result<f64, RpcError>, ClientError> {
        self.send(&RpcRequest::new0(
            JsonRpcVersion::V1,
            "777",
            "getdifficulty",
        ))
    }

    fn dump_privkey(&self, address: &str) -> Result<Result<String, RpcError>, ClientError> {
        self.send(&RpcRequest::new1(
            JsonRpcVersion::V1,
            "777",
            "dumpprivkey",
            address
        ))
    }

    fn get_address_balance(&self, addresses: &arguments::AddressList) -> Result<Result<AddressBalance, RpcError>, ClientError> {
        self.send(&RpcRequest::new1(
            JsonRpcVersion::V1,
            "777",
            "getaddressbalance",
            addresses
        ))
    }

    fn get_address_deltas(&self, addresses: &AddressList) -> Result<Result<AddressDeltas, RpcError>, ClientError> {
        self.send(&RpcRequest::new1(
            JsonRpcVersion::V1,
            "777",
            "getaddressdeltas",
            addresses
        ))
    }

    fn get_address_mempool(&self, addresses: &AddressList) -> Result<Result<AddressMempool, RpcError>, ClientError> {
        self.send(&RpcRequest::new1(
            JsonRpcVersion::V1,
            "777",
            "getaddressmempool",
            addresses
        ))
    }

    fn get_address_tx_ids(&self, addresses: &AddressList) -> Result<Result<AddressTxIDs, RpcError>, ClientError> {
        self.send(&RpcRequest::new1(
            JsonRpcVersion::V1,
            "777",
            "getaddresstxids",
            addresses
        ))
    }

    fn get_address_utxos(&self, addresses: &AddressList) -> Result<Result<AddressUtxos, RpcError>, ClientError> {
        self.send(&RpcRequest::new1(
            JsonRpcVersion::V1,
            "777",
            "getaddressutxos",
            addresses
        ))
    }

    fn get_snapshot_max(&self, n: u32) -> Result<Result<Snapshot, RpcError>, ClientError> {
        // parameter must be string:
        let n = n.to_string();
        self.send(&RpcRequest::new1(
            JsonRpcVersion::V1,
            "777",
            "getsnapshot",
            n
        ))
    }

    fn get_snapshot(&self) -> Result<Result<Snapshot, RpcError>, ClientError> {
        self.send(&RpcRequest::new0(
            JsonRpcVersion::V1,
            "777",
            "getsnapshot"
        ))
    }
}

struct Config {
    rpc_user: String,
    rpc_password: String,
    rpc_port: u16,
}

impl Config {
    pub fn get(ac: Assetchain) -> Self {
        let config_file_path;
        if let Some(mut path) = dirs::home_dir() {
            path.push(".komodo/");
            path.push(ac.to_string());
            path.push(format!("{}.conf", ac.to_string()));
            config_file_path = path.to_str().unwrap().to_owned();
        } else {
            config_file_path = String::new();
        }

        let contents = fs::read_to_string(config_file_path).expect("unable to open config file");

        let map: HashMap<String, String> = contents.as_str()
            .split('\n')
            .map(|line| line.splitn(2, '=').collect::<Vec<&str>>())
            .filter(|vec| vec.len() == 2)
            .map(|vec| (
                vec[0].to_string(),
                vec[1].to_string()
            ))
            .collect::<HashMap<String, String>>();

        let _rpc_user = map.get("rpcuser").expect("no rpcuser in config file");
        let _rpc_password = map.get("rpcpassword").expect("no rpcpassword in config file");
        let _rpc_port = map.get("rpcport").expect("no rpcport in config file");

        Config {
            rpc_user:       _rpc_user.to_owned(),
            rpc_password:   _rpc_password.to_owned(),
            rpc_port:       _rpc_port.parse::<u16>().unwrap()
        }
    }
}