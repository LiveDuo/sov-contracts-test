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

#[cfg_attr(feature = "native", derive(ModuleCallJsonSchema))]
#[derive(ModuleInfo)]
pub struct ExampleModule<C: Context> {
    #[address]
    pub address: C::Address,

    #[state]
    pub code: StateMap<Vec<u8>, Vec<u8>>,
    
    #[state]
    pub result: StateValue<i32>,
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
            call::CallMessage::CallContract { wasm_id, method_name } => {
                Ok(self.call_contract(wasm_id, method_name, context, working_set)?)
            },
            call::CallMessage::DeployContract { wasm_code } => {
                Ok(self.deploy_contract(wasm_code, context, working_set)?)
            }
        }
    }
}