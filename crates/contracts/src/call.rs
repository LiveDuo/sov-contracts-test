use std::fmt::Debug;
use std::cell::RefCell;
use std::sync::Arc;

use anyhow::Result;

use sov_modules_api::prelude::*;
use sov_modules_api::*;
#[cfg(feature = "native")]
use sov_modules_api::macros::CliWalletArg;
use sov_modules_api::digest::Digest;
use sov_state::Prefix;

use wasmi::{Engine, Linker, Module, Store, Caller, Config};

use crate::ExampleModule;

struct HostState<'a, C: Context> {
    storage: &'a StateMap<Vec<u8>, StateMap<u32, i32>>,
    working_set: &'a mut WorkingSet<C>
}

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
    CallContract { wasm_id: Vec<u8>, method_name: String, method_param: i32 }
}

// NOTE: compiling with the prover takes ~6m
impl<C: Context> ExampleModule<C> {

    pub(crate) fn deploy_contract(
        &self,
        wasm: Vec<u8>,
        _context: &C,
        working_set: &mut WorkingSet<C>,
    ) -> Result<CallResponse> {

        let wasm_id: [u8; 32] = <C as Spec>::Hasher::digest(&wasm).into();
        self.code.set(&wasm_id.to_vec(), &wasm, working_set);

        Ok(CallResponse::default())
    }

    // https://docs.rs/wasmi/0.29.0/wasmi/#example
    pub(crate) fn call_contract(
        &self,
        wasm_id: Vec<u8>,
        method_name: String,
        method_param: i32,
        _context: &C,
        working_set: &mut WorkingSet<C>,
    ) -> Result<CallResponse> {

        let wasm = self.code.get(&wasm_id, working_set).unwrap();

        let mut config = Config::default();
        config.consume_fuel(true);
        
        let engine = Engine::new(&config);
        let module = Module::new(&engine, &mut &wasm[..]).unwrap();

        let mut linker = Linker::new(&engine);
        linker.func_wrap("host", "store_param", move |caller: Caller<'_, Arc<RefCell<HostState<C>>>>, index: i32, param: i32| {
            
            println!("Store {} to storage slot {}", param, index);
            
            let mut state = caller.data().borrow_mut();
            let contract_storage = state.storage.get(&wasm_id, state.working_set)
                .unwrap_or_else(|| StateMap::<u32, i32>::new(Prefix::new(wasm_id.clone())));
            contract_storage.set(&0, &param, state.working_set);
            state.storage.set(&wasm_id, &contract_storage, state.working_set);
        }).unwrap();

        let state = Arc::new(RefCell::new(HostState { storage: &self.storage, working_set }));
        let mut store = Store::new(&engine, state);
        store.add_fuel(100).unwrap();
        
        let instance = linker.instantiate(&mut store, &module).unwrap().start(&mut store).unwrap();
        let func = instance.get_typed_func::<i32, i32>(&store, &method_name).unwrap();
        func.call(&mut store, method_param).unwrap();
        
        let consumed = store.fuel_consumed().unwrap();
        println!("Fuel consumed: {}", consumed);
        
        Ok(CallResponse::default())
    }
}
