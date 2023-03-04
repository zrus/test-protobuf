use protobuf::descriptor::field_descriptor_proto::Type;
use protobuf::reflect::FieldDescriptor;
use protobuf::reflect::MessageDescriptor;
use protobuf::reflect::OneofDescriptor;
use protobuf_codegen::CustomizeCallback;
use protobuf_codegen::{Codegen, Customize};

struct SerdeGen;

impl CustomizeCallback for SerdeGen {
  fn message(&self, _message: &MessageDescriptor) -> Customize {
    Customize::default()
      .before("#[derive(::serde::Serialize, ::serde::Deserialize)]")
  }

  fn field(&self, field: &FieldDescriptor) -> Customize {
    let type_ = field.proto().type_();
    let default = Customize::default();
    match type_ {
      Type::TYPE_ENUM => default.before("#[serde(serialize_with = \"crate::serialize_enum_or_unknown\", deserialize_with = \"crate::deserialize_enum_or_unknown\")]"),
      Type::TYPE_MESSAGE => default.before("#[serde(with=\"crate::MessageFieldDef\")]"),
      _ => default,
    }
  }

  fn oneof(&self, _: &OneofDescriptor) -> Customize {
    Customize::default()
      .before("#[derive(::serde::Serialize, ::serde::Deserialize)]\n#[serde(untagged)]")
  }

  fn special_field(
    &self,
    _message: &MessageDescriptor,
    _field: &str,
  ) -> Customize {
    Customize::default().before("#[serde(skip)]")
  }
}

fn main() {
  Codegen::new()
    .pure()
    .include("src/")
    // .cargo_out_dir("protos")
    .out_dir("src/protos")
    .input("src/test.proto")
    .customize_callback(SerdeGen)
    .run_from_script();
}
