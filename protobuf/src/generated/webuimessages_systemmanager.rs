// This file is generated by rust-protobuf 3.4.0. Do not edit
// .proto file is parsed by pure
// @generated

// https://github.com/rust-lang/rust-clippy/issues/702
#![allow(unknown_lints)]
#![allow(clippy::all)]

#![allow(unused_attributes)]
#![cfg_attr(rustfmt, rustfmt::skip)]

#![allow(box_pointers)]
#![allow(dead_code)]
#![allow(missing_docs)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]
#![allow(trivial_casts)]
#![allow(unused_results)]
#![allow(unused_mut)]

//! Generated file from `webuimessages_systemmanager.proto`
// Generated for lite runtime

/// Generated files are compatible only with the same version
/// of protobuf runtime.
const _PROTOBUF_VERSION_CHECK: () = ::protobuf::VERSION_3_4_0;

// @@protoc_insertion_point(message:CSystemManager_Hibernate_Request)
#[derive(PartialEq,Clone,Default,Debug)]
pub struct CSystemManager_Hibernate_Request {
    // special fields
    // @@protoc_insertion_point(special_field:CSystemManager_Hibernate_Request.special_fields)
    pub special_fields: ::protobuf::SpecialFields,
}

impl<'a> ::std::default::Default for &'a CSystemManager_Hibernate_Request {
    fn default() -> &'a CSystemManager_Hibernate_Request {
        <CSystemManager_Hibernate_Request as ::protobuf::Message>::default_instance()
    }
}

impl CSystemManager_Hibernate_Request {
    pub fn new() -> CSystemManager_Hibernate_Request {
        ::std::default::Default::default()
    }
}

impl ::protobuf::Message for CSystemManager_Hibernate_Request {
    const NAME: &'static str = "CSystemManager_Hibernate_Request";

    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream<'_>) -> ::protobuf::Result<()> {
        while let Some(tag) = is.read_raw_tag_or_eof()? {
            match tag {
                tag => {
                    ::protobuf::rt::read_unknown_or_skip_group(tag, is, self.special_fields.mut_unknown_fields())?;
                },
            };
        }
        ::std::result::Result::Ok(())
    }

    // Compute sizes of nested messages
    #[allow(unused_variables)]
    fn compute_size(&self) -> u64 {
        let mut my_size = 0;
        my_size += ::protobuf::rt::unknown_fields_size(self.special_fields.unknown_fields());
        self.special_fields.cached_size().set(my_size as u32);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream<'_>) -> ::protobuf::Result<()> {
        os.write_unknown_fields(self.special_fields.unknown_fields())?;
        ::std::result::Result::Ok(())
    }

    fn special_fields(&self) -> &::protobuf::SpecialFields {
        &self.special_fields
    }

    fn mut_special_fields(&mut self) -> &mut ::protobuf::SpecialFields {
        &mut self.special_fields
    }

    fn new() -> CSystemManager_Hibernate_Request {
        CSystemManager_Hibernate_Request::new()
    }

    fn clear(&mut self) {
        self.special_fields.clear();
    }

    fn default_instance() -> &'static CSystemManager_Hibernate_Request {
        static instance: CSystemManager_Hibernate_Request = CSystemManager_Hibernate_Request {
            special_fields: ::protobuf::SpecialFields::new(),
        };
        &instance
    }
}

// @@protoc_insertion_point(message:CSystemManager_Hibernate_Response)
#[derive(PartialEq,Clone,Default,Debug)]
pub struct CSystemManager_Hibernate_Response {
    // special fields
    // @@protoc_insertion_point(special_field:CSystemManager_Hibernate_Response.special_fields)
    pub special_fields: ::protobuf::SpecialFields,
}

impl<'a> ::std::default::Default for &'a CSystemManager_Hibernate_Response {
    fn default() -> &'a CSystemManager_Hibernate_Response {
        <CSystemManager_Hibernate_Response as ::protobuf::Message>::default_instance()
    }
}

impl CSystemManager_Hibernate_Response {
    pub fn new() -> CSystemManager_Hibernate_Response {
        ::std::default::Default::default()
    }
}

impl ::protobuf::Message for CSystemManager_Hibernate_Response {
    const NAME: &'static str = "CSystemManager_Hibernate_Response";

    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream<'_>) -> ::protobuf::Result<()> {
        while let Some(tag) = is.read_raw_tag_or_eof()? {
            match tag {
                tag => {
                    ::protobuf::rt::read_unknown_or_skip_group(tag, is, self.special_fields.mut_unknown_fields())?;
                },
            };
        }
        ::std::result::Result::Ok(())
    }

    // Compute sizes of nested messages
    #[allow(unused_variables)]
    fn compute_size(&self) -> u64 {
        let mut my_size = 0;
        my_size += ::protobuf::rt::unknown_fields_size(self.special_fields.unknown_fields());
        self.special_fields.cached_size().set(my_size as u32);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream<'_>) -> ::protobuf::Result<()> {
        os.write_unknown_fields(self.special_fields.unknown_fields())?;
        ::std::result::Result::Ok(())
    }

    fn special_fields(&self) -> &::protobuf::SpecialFields {
        &self.special_fields
    }

    fn mut_special_fields(&mut self) -> &mut ::protobuf::SpecialFields {
        &mut self.special_fields
    }

    fn new() -> CSystemManager_Hibernate_Response {
        CSystemManager_Hibernate_Response::new()
    }

    fn clear(&mut self) {
        self.special_fields.clear();
    }

    fn default_instance() -> &'static CSystemManager_Hibernate_Response {
        static instance: CSystemManager_Hibernate_Response = CSystemManager_Hibernate_Response {
            special_fields: ::protobuf::SpecialFields::new(),
        };
        &instance
    }
}
#[allow(unused_imports)]
use crate::steammessages_base::*;
#[allow(unused_imports)]
use crate::webuimessages_base::*;
impl crate::RpcMessage for CSystemManager_Hibernate_Request {
    fn parse(reader: &mut dyn std::io::Read) -> protobuf::Result<Self> {
        <Self as protobuf::Message>::parse_from_reader(reader)
    }
    fn write(&self, writer: &mut dyn std::io::Write) -> protobuf::Result<()> {
        use protobuf::Message;
        self.write_to_writer(writer)
    }
    fn encode_size(&self) -> usize {
        use protobuf::Message;
        self.compute_size() as usize
    }
}
impl crate::RpcMessage for CSystemManager_Hibernate_Response {
    fn parse(reader: &mut dyn std::io::Read) -> protobuf::Result<Self> {
        <Self as protobuf::Message>::parse_from_reader(reader)
    }
    fn write(&self, writer: &mut dyn std::io::Write) -> protobuf::Result<()> {
        use protobuf::Message;
        self.write_to_writer(writer)
    }
    fn encode_size(&self) -> usize {
        use protobuf::Message;
        self.compute_size() as usize
    }
}
///
struct SystemManager {}
impl crate::RpcService for SystemManager {
    const SERVICE_NAME: &'static str = "SystemManager";
}
impl crate::RpcMethod for CSystemManager_Hibernate_Request {
    const METHOD_NAME: &'static str = "SystemManager.Hibernate#1";
    type Response = CSystemManager_Hibernate_Response;
}
