use std::fmt::Debug;

use anyhow::Result;

use sov_modules_api::prelude::*;
use sov_modules_api::*;
#[cfg(feature = "native")]
use sov_modules_api::macros::CliWalletArg;
use sov_modules_api::digest::Digest;
use sov_state::Prefix;

use wasmi::{Engine, Linker, Module, Store, Caller};

use crate::ExampleModule;

#[derive(borsh::BorshDeserialize, borsh::BorshSerialize, Debug, PartialEq)]
struct CallContractParams { wasm_id: Vec<u8>, method_name: String, }

#[cfg_attr(
    feature = "native",
    derive(serde::Serialize),
    derive(serde::Deserialize),
    derive(CliWalletArg),
    derive(schemars::JsonSchema)
)]
#[derive(borsh::BorshDeserialize, borsh::BorshSerialize, Debug, PartialEq)]
pub enum CallMessage {
    DeployContract{ wasm_code: Vec<u8> },
    CallContract { wasm_id: Vec<u8>, method_name: String, }
}

// NOTE compiling takes too long
impl<C: sov_modules_api::Context> ExampleModule<C> {

    pub(crate) fn deploy_contract(
        &self,
        wasm: Vec<u8>,
        _context: &C,
        working_set: &mut WorkingSet<C>,
    ) -> Result<sov_modules_api::CallResponse> {

        let wasm_id: [u8; 32] = <C as Spec>::Hasher::digest(&wasm).into();
        self.code.set(&wasm_id.to_vec(), &wasm, working_set);

        Ok(CallResponse::default())
    }

    pub(crate) fn call_contract(
        &self,
        wasm_id: Vec<u8>,
        method_name: String,
        _context: &C,
        working_set: &mut WorkingSet<C>,
    ) -> Result<sov_modules_api::CallResponse> {

        let wasm = self.code.get(&wasm_id, working_set).unwrap();

        let engine = Engine::default();

        // https://docs.rs/wasmi/0.29.0/wasmi/#example
        let module = Module::new(&engine, &mut &wasm[..]).unwrap();

        let mut linker = <Linker<()>>::new(&engine);
        linker.func_wrap("host", "store_param", |_caller: Caller<'_, ()>, param: i32| {
            
            println!("Function params: {}", param);
            
            // println!("Caller data: {:?}", caller.data());
            // TODO self.storage.set(&wasm_id, &param, working_set).unwrap();
        }).unwrap();

        let mut store = Store::new(&engine, ());
        let instance = linker.instantiate(&mut store, &module).unwrap().start(&mut store).unwrap();

        let inc = instance.get_typed_func::<i32, i32>(&store, &method_name).unwrap();
        let res = inc.call(&mut store, 5).unwrap();
        
        let _storage_default = StateMap::<u32, i32>::new(Prefix::new(b"1".to_vec()));
        let _storage = self.storage.get(&wasm_id, working_set).unwrap_or(_storage_default);
        _storage.set(&0, &res, working_set);
        self.storage.set(&wasm_id, &_storage, working_set);

        Ok(CallResponse::default())
    }
}
