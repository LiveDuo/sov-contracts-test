use std::fmt::Debug;

use anyhow::Result;
use sov_modules_api::prelude::*;
use sov_modules_api::{CallResponse, WorkingSet};
#[cfg(feature = "native")]
use sov_modules_api::macros::CliWalletArg;
use thiserror::Error;

use wasmi::{Engine, Linker, Module, Store};

use crate::ExampleModule;

/// This enumeration represents the available call messages for interacting with
/// the `ExampleModule` module.
/// The `derive` for [`schemars::JsonSchema`] is a requirement of
/// [`sov_modules_api::ModuleCallJsonSchema`].
#[cfg_attr(
    feature = "native",
    derive(serde::Serialize),
    derive(serde::Deserialize),
    derive(CliWalletArg),
    derive(schemars::JsonSchema)
)]
#[derive(borsh::BorshDeserialize, borsh::BorshSerialize, Debug, PartialEq)]
pub enum CallMessage {
    SetValue(i32),
}

/// Example of a custom error.
#[derive(Debug, Error)]
enum SetValueError {}

impl<C: sov_modules_api::Context> ExampleModule<C> {
    /// Sets `value` field to the `new_value`
    pub(crate) fn set_value(
        &self,
        _new_value: i32,
        _context: &C,
        working_set: &mut WorkingSet<C>,
    ) -> Result<sov_modules_api::CallResponse> {

        let _wat = r#"
            (module
                (export "fib" (func $fib))
                (func $fib (; 0 ;) (param $0 i32) (result i32)
                    (local $1 i32)
                    (local $2 i32)
                    (local $3 i32)
                    (local $4 i32)
                    (set_local $4 (i32.const 1))
                    (block $label$0
                        (br_if $label$0 (i32.lt_s (get_local $0) (i32.const 1)))
                        (set_local $3 (i32.const 0))
                        (loop $label$1
                        (set_local $1
                        (i32.add (get_local $3) (get_local $4))
                    )
                    (set_local $2 (get_local $4))
                    (set_local $3 (get_local $4))
                    (set_local $4 (get_local $1))
                    (br_if $label$1 (tee_local $0 (i32.add (get_local $0) (i32.const -1)))))
                    (return (get_local $2))
                )
                (i32.const 0)
            )
        )
        "#;

        // NOTE this line makes compiling hang
        // let wasm = wat::parse_str(wat).expect("Failed to parse_str");

        // let engine = Engine::default();

        // // Derived from the wasmi example: https://docs.rs/wasmi/0.29.0/wasmi/#example
        // let module = Module::new(&engine, &mut &wasm[..]).expect("Failed to create module");
        // type HostState = u32;

        // let linker = <Linker<HostState>>::new(&engine);
        // let mut store = Store::new(&engine, 42);
        // let instance = linker
        //     .instantiate(&mut store, &module)
        //     .expect("failed to instantiate")
        //     .start(&mut store)
        //     .expect("Failed to start");

        // let fib = instance
        //     .get_typed_func::<i32, i32>(&store, "fib")
        //     .expect("Failed to get typed_func");
        // let res = fib.call(&mut store, 2).expect("Failed to call");
        let res = 2;
        
        self.value.set(&res, working_set);
        working_set.add_event("set", &format!("value_set: {res:?}"));

        Ok(CallResponse::default())
    }
}
