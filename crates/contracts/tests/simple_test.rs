use contracts_module::{ ExampleModule, CallMessage, ExampleModuleConfig };
use sov_modules_api::utils::generate_address;
use sov_modules_api::{Context, WorkingSet, Module, Spec};
use sov_modules_api::default_context::DefaultContext;
use sov_modules_api::digest::Digest;
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
  let response = module.query_result(&mut working_set).unwrap();
  dbg!(response);

  // get wasm
  let wat = "(module (func (export \"inc\") (param i32) (result i32) local.get 0 i32.const 1 i32.add))";
  let wasm = wat::parse_str(wat).unwrap();
  
  // deploy wasm
  let update_message = CallMessage::DeployContract(wasm.clone());
  module.call(update_message, &sender_context, &mut working_set).unwrap();

  // call contract
  let wasm_id: [u8; 32] = <DefaultContext as Spec>::Hasher::digest(&wasm).into();
  let update_message = CallMessage::CallContract(wasm_id.to_vec());
  module.call(update_message, &sender_context, &mut working_set).unwrap();

  // check response
  let response = module.query_result(&mut working_set).unwrap();
  dbg!(response);

}

