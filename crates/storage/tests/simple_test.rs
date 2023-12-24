use storage_module::{ ExampleModule, CallMessage, ExampleModuleConfig };
use sov_modules_api::utils::generate_address;
use sov_modules_api::{Context, WorkingSet, Module};
use sov_modules_api::default_context::DefaultContext;
use sov_state::ProverStorage;

#[test]
fn simple_test() {

  // get working set
  let tmpdir = tempfile::tempdir().unwrap();
  let mut working_set = WorkingSet::new(ProverStorage::with_path(tmpdir.path()).unwrap());
  
  // sender context
  let sender_address = generate_address::<DefaultContext>("sender");
  let sender_context = DefaultContext::new(sender_address, 0);

  // init module
  let module: ExampleModule<DefaultContext> = ExampleModule::default();
  module.genesis(&ExampleModuleConfig {}, &mut working_set).unwrap();

  // check response
  let response = module.query_value(&mut working_set).unwrap();
  dbg!(response);

  // get wasm
  let wat = r#"
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
  let wasm = wat::parse_str(wat).expect("Failed to parse_str");
  
  // call module
  let update_message = CallMessage::RunWasm(wasm);
  module.call(update_message, &sender_context, &mut working_set).unwrap();

  // check response
  let response = module.query_value(&mut working_set).unwrap();
  dbg!(response);

}

