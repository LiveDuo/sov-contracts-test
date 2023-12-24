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

  // call module
  let update_message = CallMessage::SetValue(100);
  module.call(update_message, &sender_context, &mut working_set).unwrap();

  // check response
  let response = module.query_value(&mut working_set);
  dbg!(response.unwrap());

}

