use sov_modules_api::prelude::*;
use sov_modules_api::WorkingSet;

use super::ExampleModule;

use jsonrpsee::core::RpcResult;

use sov_modules_api::macros::rpc_gen;

#[derive(Clone, serde::Serialize, serde::Deserialize, Debug, Eq, PartialEq)]
pub struct Response {
    pub value: Option<i32>,
}

#[rpc_gen(client, server, namespace = "contracts")]
impl<C: sov_modules_api::Context> ExampleModule<C> {
    
    #[rpc_method(name = "getValue")]
    pub fn query_value(&self, working_set: &mut WorkingSet<C>) -> RpcResult<Response> {
        Ok(Response { value: self.value.get(working_set) })
    }
}
