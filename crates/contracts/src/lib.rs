mod call;
mod genesis;
#[cfg(feature = "native")]
mod query;
pub use call::CallMessage;
#[cfg(feature = "native")]
pub use query::*;
use serde::{Deserialize, Serialize};
use sov_modules_api::*;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ExampleModuleConfig {}

#[derive(borsh::BorshDeserialize, borsh::BorshSerialize)]
pub struct Contract {
    pub code: Vec<u8>,
    pub storage: StateMap<u32, i32>,
}

type ContractId = Vec<u8>;

#[cfg_attr(feature = "native", derive(ModuleCallJsonSchema))]
#[derive(ModuleInfo)]
pub struct ExampleModule<C: Context> {
    #[address]
    pub address: C::Address,

    #[state]
    pub contracts: StateMap<ContractId, Contract>,
}

impl<C: Context> Module for ExampleModule<C> {
    type Context = C;

    type Config = ExampleModuleConfig;

    type CallMessage = call::CallMessage;

    type Event = ();

    fn genesis(&self, config: &Self::Config, working_set: &mut WorkingSet<C>) -> Result<(), Error> {
        Ok(self.init_module(config, working_set)?)
    }

    fn call(
        &self,
        msg: Self::CallMessage,
        context: &Self::Context,
        working_set: &mut WorkingSet<C>,
    ) -> Result<CallResponse, Error> {
        match msg {
            call::CallMessage::DeployContract { wasm_code } => {
                Ok(self.deploy_contract(wasm_code, context, working_set)?)
            },
            call::CallMessage::CallContract { wasm_id, method_name, method_param, fuel_limit } => {
                Ok(self.call_contract(wasm_id, method_name, method_param, fuel_limit, context, working_set)?)
            }
        }
    }
}
