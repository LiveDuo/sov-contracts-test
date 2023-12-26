use sov_modules_api::prelude::*;
use sov_modules_api::{WorkingSet, Context};

use super::ExampleModule;

use jsonrpsee::core::RpcResult;

use sov_modules_api::macros::rpc_gen;

#[derive(Clone, serde::Serialize, serde::Deserialize, Debug, Eq, PartialEq)]
pub struct Response {
    pub value: Option<i32>,
}

#[rpc_gen(client, server, namespace = "contracts")]
impl<C: Context> ExampleModule<C> {
    
    #[rpc_method(name = "queryContract")]
    pub fn query_contract(&self, wasm_id: Vec<u8>, index: u32, working_set: &mut WorkingSet<C>) -> RpcResult<Response> {
        let value = self.storage.get(&wasm_id, working_set).map(|s| s.get(&index, working_set)).flatten();
        Ok(Response { value })
    }
}
