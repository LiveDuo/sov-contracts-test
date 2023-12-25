mod call;
mod genesis;
#[cfg(feature = "native")]
mod query;
pub use call::CallMessage;
#[cfg(feature = "native")]
pub use query::*;
use serde::{Deserialize, Serialize};
use sov_modules_api::{Error, ModuleInfo, WorkingSet};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ExampleModuleConfig {}

#[cfg_attr(feature = "native", derive(sov_modules_api::ModuleCallJsonSchema))]
#[derive(ModuleInfo)]
pub struct ExampleModule<C: sov_modules_api::Context> {
    #[address]
    pub address: C::Address,

    #[state]
    pub value: sov_modules_api::StateValue<i32>,
}

impl<C: sov_modules_api::Context> sov_modules_api::Module for ExampleModule<C> {
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
    ) -> Result<sov_modules_api::CallResponse, Error> {
        match msg {
            call::CallMessage::RunWasm(wasm) => {
                Ok(self.run_wasm(wasm, context, working_set)?)
            }
        }
    }
}
