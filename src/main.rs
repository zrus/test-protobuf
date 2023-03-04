mod protos;
mod test;

pub use test::*;

use async_trait::async_trait;
use cqrs_es::{Aggregate, DomainEvent};
use protobuf::MessageDyn;
use serde::{Deserialize, Serialize};
use thiserror::Error;

use protos::test::{
  commands::Command, Commands, Create, DataType, Delete, Update,
};

fn main() {
  let mut type_ = DataType::default();
  type_.set_type1(1);
  let type_ = Some(type_).into();

  let _command_enum = Command::Create(Create {
    data: String::from("test"),
    type_,
    ..Default::default()
  });

  let string = serde_json::json!(_command_enum);
  println!("{string:#}");

  let mut type_ = DataType::default();
  type_.set_type1(1);
  let type_ = Some(type_).into();

  let mut command_struct = Commands::default();
  command_struct.set_create(Create {
    data: String::from("test"),
    type_,
    ..Default::default()
  });

  // if let Some(command) = command_struct.command {
  //   match command {
  //     Command::Create(_) => todo!(),
  //     Command::Read(_) => todo!(),
  //     Command::Update(_) => todo!(),
  //     Command::Delete(_) => todo!(),
  //   }
  // }

  // parse_command(&command_struct);

  let string = serde_json::json!(command_struct);
  println!("{string:#}");
}

fn parse_command(message: &dyn MessageDyn) -> Option<()> {
  let descriptor = message.descriptor_dyn();
  for field in descriptor.fields() {
    if let Some(v) = field.get_singular(message) {
      if let Some(oneof) = field.containing_oneof() {
        println!("{}: {}", oneof.name(), v);
      }
    }
  }
  None
}

#[derive(Default, Serialize, Deserialize)]
struct TestAggregate;

#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
enum Events {}

impl DomainEvent for Events {
  fn event_type(&self) -> String {
    todo!()
  }

  fn event_version(&self) -> String {
    todo!()
  }
}

#[derive(Debug, Error)]
enum Errors {}

#[async_trait]
impl Aggregate for TestAggregate {
  type Command = Command;

  type Event = Events;

  type Error = Errors;

  type Services = ();

  fn aggregate_type() -> String {
    String::new()
  }

  async fn handle(
    &self,
    command: Self::Command,
    _service: &Self::Services,
  ) -> Result<Vec<Self::Event>, Self::Error> {
    match command {
      Command::Create(_create) => todo!(),
      Command::Read(_read) => todo!(),
      Command::Update(Update {
        data: _, type_: _, ..
      }) => todo!(),
      Command::Delete(Delete { id: _, .. }) => todo!(),
    }
  }

  fn apply(&mut self, event: Self::Event) {
    match event {}
  }
}
