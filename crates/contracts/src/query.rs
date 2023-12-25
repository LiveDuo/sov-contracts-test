use sov_modules_api::prelude::*;
use sov_modules_api::{WorkingSet, Context};

use super::ExampleModule;

use jsonrpsee::core::RpcResult;

use sov_modules_api::macros::rpc_gen;

#[derive(Clone, serde::Serialize, serde::Deserialize, Debug, Eq, PartialEq)]
pub struct Response {
    pub result: Option<i32>,
}

#[rpc_gen(client, server, namespace = "contracts")]
impl<C: Context> ExampleModule<C> {
    
    #[rpc_method(name = "getResult")]
    pub fn query_result(&self, working_set: &mut WorkingSet<C>) -> RpcResult<Response> {
        Ok(Response { result: self.result.get(working_set) })
    }
}
