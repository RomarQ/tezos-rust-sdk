pub mod balance;
pub mod counter;
pub mod delegate;
pub mod entrypoints;
pub mod manager_key;
pub mod script;

use {
    crate::client::TezosRPCContext, crate::error::Error, crate::models::contract::ContractInfo,
    crate::protocol_rpc::block::BlockID,
};

fn path(chain_id: &String, block_id: &BlockID, contract: &String) -> String {
    format!("{}/contracts/{}", super::path(chain_id, block_id), contract)
}

/// A builder to construct the properties of a request to access the counter of a contract.
#[derive(Clone, Copy)]
pub struct RPCRequestBuilder<'a> {
    ctx: &'a TezosRPCContext,
    chain_id: &'a String,
    block_id: &'a BlockID,
    contract: &'a String,
    normalize_types: Option<bool>,
}

impl<'a> RPCRequestBuilder<'a> {
    pub fn new(ctx: &'a TezosRPCContext, contract: &'a String) -> Self {
        RPCRequestBuilder {
            ctx,
            chain_id: &ctx.chain_id,
            block_id: &BlockID::Head,
            contract: contract,
            normalize_types: None,
        }
    }

    /// Modify chain identifier to be used in the request.
    pub fn chain_id(&mut self, chain_id: &'a String) -> &mut Self {
        self.chain_id = chain_id;

        self
    }

    /// Modify the block identifier to be used in the request.
    pub fn block_id(&mut self, block_id: &'a BlockID) -> &mut Self {
        self.block_id = block_id;

        self
    }

    /// Whether types should be normalized (annotations removed, combs flattened) or kept as they appeared in the original script.
    pub fn normalize_types(&mut self, normalize_types: bool) -> &mut Self {
        self.normalize_types = Some(normalize_types);

        self
    }

    pub async fn send(self) -> Result<ContractInfo, Error> {
        let mut query: Vec<(&str, String)> = vec![];

        if let Some(normalize_types) = self.normalize_types {
            // Add `normalize_types` query parameter
            query.push(("normalize_types", normalize_types.to_string()));
        }

        let path = self::path(self.chain_id, self.block_id, self.contract);

        self.ctx
            .http_client
            .get_with_query(path.as_str(), &Some(query))
            .await
    }
}

/// Access the complete status of a contract.
///
/// * `address` : A contract identifier encoded in b58check. e.g. `KT1HxgqnVjGy7KsSUTEsQ6LgpD5iKSGu7QpA`
///
/// Optional query arguments :
/// * `normalize_types` : Whether types should be normalized (annotations removed, combs flattened) or kept as they appeared in the original script.
///
/// [`GET ../<block_id>/context/contracts/<contract_id>?[normalize_types]`](https://tezos.gitlab.io/jakarta/rpc.html#get-block-id-context-contracts-contract-id)
pub fn get<'a>(ctx: &'a TezosRPCContext, address: &'a String) -> RPCRequestBuilder<'a> {
    RPCRequestBuilder::new(ctx, address)
}

#[cfg(test)]
mod tests {

    use {
        crate::{
            client::TezosRPC, constants::DEFAULT_CHAIN_ALIAS, error::Error,
            protocol_rpc::block::BlockID,
        },
        httpmock::prelude::*,
        num_bigint::BigInt,
    };

    #[tokio::test]
    async fn test_get_contract() -> Result<(), Error> {
        let server = MockServer::start();
        let rpc_url = server.base_url();

        let block_id = BlockID::Level(1);
        let contract_address = "KT1HxgqnVjGy7KsSUTEsQ6LgpD5iKSGu7QpA";
        let normalize_types = true;

        server.mock(|when, then| {
            when.method(GET)
                .path(super::path(
                    &DEFAULT_CHAIN_ALIAS.to_string(),
                    &block_id,
                    &contract_address.to_string(),
                ))
                .query_param("normalize_types", normalize_types.to_string());
            then.status(200)
                .header("content-type", "application/json")
                .body(include_str!("contract/__TEST_DATA__/contract.json"));
        });

        let client = TezosRPC::new(rpc_url.as_str());
        let contract = client
            .get_contract(&contract_address.to_string())
            .normalize_types(normalize_types)
            .block_id(&block_id)
            .send()
            .await?;

        assert_eq!(contract.counter, None);
        assert_eq!(contract.delegate, None);
        assert_eq!(contract.balance, BigInt::from(0));

        let contract_script = contract.script.expect("Script exists");
        assert!(contract_script.code.is_micheline_sequence());
        assert!(contract_script.storage.is_micheline_primitive_application());

        Ok(())
    }
}
