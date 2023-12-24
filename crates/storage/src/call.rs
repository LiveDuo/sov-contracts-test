use std::fmt::Debug;

use anyhow::Result;
use sov_modules_api::prelude::*;
use sov_modules_api::{CallResponse, WorkingSet};
#[cfg(feature = "native")]
use sov_modules_api::macros::CliWalletArg;

use wasmi::{Engine, Linker, Module, Store};

use crate::ExampleModule;

#[cfg_attr(
    feature = "native",
    derive(serde::Serialize),
    derive(serde::Deserialize),
    derive(CliWalletArg),
    derive(schemars::JsonSchema)
)]
#[derive(borsh::BorshDeserialize, borsh::BorshSerialize, Debug, PartialEq)]
pub enum CallMessage {
    RunWasm(Vec<u8>),
}

// NOTE compiling takes too long
impl<C: sov_modules_api::Context> ExampleModule<C> {

    pub(crate) fn run_wasm(
        &self,
        wasm: Vec<u8>,
        _context: &C,
        working_set: &mut WorkingSet<C>,
    ) -> Result<sov_modules_api::CallResponse> {

        let engine = Engine::default();

        // https://docs.rs/wasmi/0.29.0/wasmi/#example
        let module = Module::new(&engine, &mut &wasm[..]).unwrap();

        let linker = <Linker<u32>>::new(&engine);
        let mut store = Store::new(&engine, 42);
        let instance = linker.instantiate(&mut store, &module).unwrap().start(&mut store).unwrap();

        let fib = instance.get_typed_func::<i32, i32>(&store, "fib").unwrap();
        let res = fib.call(&mut store, 5).unwrap();
        
        self.value.set(&res, working_set);

        Ok(CallResponse::default())
    }
}
