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

//! Generated file from `steammessages_gamenetworking.steamclient.proto`
// Generated for lite runtime

/// Generated files are compatible only with the same version
/// of protobuf runtime.
const _PROTOBUF_VERSION_CHECK: () = ::steam_vent_proto_common::protobuf::VERSION_3_4_0;

// @@protoc_insertion_point(message:CGameNetworking_AllocateFakeIP_Request)
#[derive(PartialEq,Clone,Default,Debug)]
pub struct CGameNetworking_AllocateFakeIP_Request {
    // message fields
    // @@protoc_insertion_point(field:CGameNetworking_AllocateFakeIP_Request.app_id)
    pub app_id: ::std::option::Option<u32>,
    // @@protoc_insertion_point(field:CGameNetworking_AllocateFakeIP_Request.num_fake_ports)
    pub num_fake_ports: ::std::option::Option<u32>,
    // special fields
    // @@protoc_insertion_point(special_field:CGameNetworking_AllocateFakeIP_Request.special_fields)
    pub special_fields: ::steam_vent_proto_common::protobuf::SpecialFields,
}

impl<'a> ::std::default::Default for &'a CGameNetworking_AllocateFakeIP_Request {
    fn default() -> &'a CGameNetworking_AllocateFakeIP_Request {
        <CGameNetworking_AllocateFakeIP_Request as ::steam_vent_proto_common::protobuf::Message>::default_instance()
    }
}

impl CGameNetworking_AllocateFakeIP_Request {
    pub fn new() -> CGameNetworking_AllocateFakeIP_Request {
        ::std::default::Default::default()
    }

    // optional uint32 app_id = 1;

    pub fn app_id(&self) -> u32 {
        self.app_id.unwrap_or(0)
    }

    pub fn clear_app_id(&mut self) {
        self.app_id = ::std::option::Option::None;
    }

    pub fn has_app_id(&self) -> bool {
        self.app_id.is_some()
    }

    // Param is passed by value, moved
    pub fn set_app_id(&mut self, v: u32) {
        self.app_id = ::std::option::Option::Some(v);
    }

    // optional uint32 num_fake_ports = 2;

    pub fn num_fake_ports(&self) -> u32 {
        self.num_fake_ports.unwrap_or(0)
    }

    pub fn clear_num_fake_ports(&mut self) {
        self.num_fake_ports = ::std::option::Option::None;
    }

    pub fn has_num_fake_ports(&self) -> bool {
        self.num_fake_ports.is_some()
    }

    // Param is passed by value, moved
    pub fn set_num_fake_ports(&mut self, v: u32) {
        self.num_fake_ports = ::std::option::Option::Some(v);
    }
}

impl ::steam_vent_proto_common::protobuf::Message for CGameNetworking_AllocateFakeIP_Request {
    const NAME: &'static str = "CGameNetworking_AllocateFakeIP_Request";

    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut ::steam_vent_proto_common::protobuf::CodedInputStream<'_>) -> ::steam_vent_proto_common::protobuf::Result<()> {
        while let Some(tag) = is.read_raw_tag_or_eof()? {
            match tag {
                8 => {
                    self.app_id = ::std::option::Option::Some(is.read_uint32()?);
                },
                16 => {
                    self.num_fake_ports = ::std::option::Option::Some(is.read_uint32()?);
                },
                tag => {
                    ::steam_vent_proto_common::protobuf::rt::read_unknown_or_skip_group(tag, is, self.special_fields.mut_unknown_fields())?;
                },
            };
        }
        ::std::result::Result::Ok(())
    }

    // Compute sizes of nested messages
    #[allow(unused_variables)]
    fn compute_size(&self) -> u64 {
        let mut my_size = 0;
        if let Some(v) = self.app_id {
            my_size += ::steam_vent_proto_common::protobuf::rt::uint32_size(1, v);
        }
        if let Some(v) = self.num_fake_ports {
            my_size += ::steam_vent_proto_common::protobuf::rt::uint32_size(2, v);
        }
        my_size += ::steam_vent_proto_common::protobuf::rt::unknown_fields_size(self.special_fields.unknown_fields());
        self.special_fields.cached_size().set(my_size as u32);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::steam_vent_proto_common::protobuf::CodedOutputStream<'_>) -> ::steam_vent_proto_common::protobuf::Result<()> {
        if let Some(v) = self.app_id {
            os.write_uint32(1, v)?;
        }
        if let Some(v) = self.num_fake_ports {
            os.write_uint32(2, v)?;
        }
        os.write_unknown_fields(self.special_fields.unknown_fields())?;
        ::std::result::Result::Ok(())
    }

    fn special_fields(&self) -> &::steam_vent_proto_common::protobuf::SpecialFields {
        &self.special_fields
    }

    fn mut_special_fields(&mut self) -> &mut ::steam_vent_proto_common::protobuf::SpecialFields {
        &mut self.special_fields
    }

    fn new() -> CGameNetworking_AllocateFakeIP_Request {
        CGameNetworking_AllocateFakeIP_Request::new()
    }

    fn clear(&mut self) {
        self.app_id = ::std::option::Option::None;
        self.num_fake_ports = ::std::option::Option::None;
        self.special_fields.clear();
    }

    fn default_instance() -> &'static CGameNetworking_AllocateFakeIP_Request {
        static instance: CGameNetworking_AllocateFakeIP_Request = CGameNetworking_AllocateFakeIP_Request {
            app_id: ::std::option::Option::None,
            num_fake_ports: ::std::option::Option::None,
            special_fields: ::steam_vent_proto_common::protobuf::SpecialFields::new(),
        };
        &instance
    }
}

// @@protoc_insertion_point(message:CGameNetworking_AllocateFakeIP_Response)
#[derive(PartialEq,Clone,Default,Debug)]
pub struct CGameNetworking_AllocateFakeIP_Response {
    // message fields
    // @@protoc_insertion_point(field:CGameNetworking_AllocateFakeIP_Response.fake_ip)
    pub fake_ip: ::std::option::Option<u32>,
    // @@protoc_insertion_point(field:CGameNetworking_AllocateFakeIP_Response.fake_ports)
    pub fake_ports: ::std::vec::Vec<u32>,
    // special fields
    // @@protoc_insertion_point(special_field:CGameNetworking_AllocateFakeIP_Response.special_fields)
    pub special_fields: ::steam_vent_proto_common::protobuf::SpecialFields,
}

impl<'a> ::std::default::Default for &'a CGameNetworking_AllocateFakeIP_Response {
    fn default() -> &'a CGameNetworking_AllocateFakeIP_Response {
        <CGameNetworking_AllocateFakeIP_Response as ::steam_vent_proto_common::protobuf::Message>::default_instance()
    }
}

impl CGameNetworking_AllocateFakeIP_Response {
    pub fn new() -> CGameNetworking_AllocateFakeIP_Response {
        ::std::default::Default::default()
    }

    // optional fixed32 fake_ip = 1;

    pub fn fake_ip(&self) -> u32 {
        self.fake_ip.unwrap_or(0)
    }

    pub fn clear_fake_ip(&mut self) {
        self.fake_ip = ::std::option::Option::None;
    }

    pub fn has_fake_ip(&self) -> bool {
        self.fake_ip.is_some()
    }

    // Param is passed by value, moved
    pub fn set_fake_ip(&mut self, v: u32) {
        self.fake_ip = ::std::option::Option::Some(v);
    }
}

impl ::steam_vent_proto_common::protobuf::Message for CGameNetworking_AllocateFakeIP_Response {
    const NAME: &'static str = "CGameNetworking_AllocateFakeIP_Response";

    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut ::steam_vent_proto_common::protobuf::CodedInputStream<'_>) -> ::steam_vent_proto_common::protobuf::Result<()> {
        while let Some(tag) = is.read_raw_tag_or_eof()? {
            match tag {
                13 => {
                    self.fake_ip = ::std::option::Option::Some(is.read_fixed32()?);
                },
                18 => {
                    is.read_repeated_packed_uint32_into(&mut self.fake_ports)?;
                },
                16 => {
                    self.fake_ports.push(is.read_uint32()?);
                },
                tag => {
                    ::steam_vent_proto_common::protobuf::rt::read_unknown_or_skip_group(tag, is, self.special_fields.mut_unknown_fields())?;
                },
            };
        }
        ::std::result::Result::Ok(())
    }

    // Compute sizes of nested messages
    #[allow(unused_variables)]
    fn compute_size(&self) -> u64 {
        let mut my_size = 0;
        if let Some(v) = self.fake_ip {
            my_size += 1 + 4;
        }
        for value in &self.fake_ports {
            my_size += ::steam_vent_proto_common::protobuf::rt::uint32_size(2, *value);
        };
        my_size += ::steam_vent_proto_common::protobuf::rt::unknown_fields_size(self.special_fields.unknown_fields());
        self.special_fields.cached_size().set(my_size as u32);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::steam_vent_proto_common::protobuf::CodedOutputStream<'_>) -> ::steam_vent_proto_common::protobuf::Result<()> {
        if let Some(v) = self.fake_ip {
            os.write_fixed32(1, v)?;
        }
        for v in &self.fake_ports {
            os.write_uint32(2, *v)?;
        };
        os.write_unknown_fields(self.special_fields.unknown_fields())?;
        ::std::result::Result::Ok(())
    }

    fn special_fields(&self) -> &::steam_vent_proto_common::protobuf::SpecialFields {
        &self.special_fields
    }

    fn mut_special_fields(&mut self) -> &mut ::steam_vent_proto_common::protobuf::SpecialFields {
        &mut self.special_fields
    }

    fn new() -> CGameNetworking_AllocateFakeIP_Response {
        CGameNetworking_AllocateFakeIP_Response::new()
    }

    fn clear(&mut self) {
        self.fake_ip = ::std::option::Option::None;
        self.fake_ports.clear();
        self.special_fields.clear();
    }

    fn default_instance() -> &'static CGameNetworking_AllocateFakeIP_Response {
        static instance: CGameNetworking_AllocateFakeIP_Response = CGameNetworking_AllocateFakeIP_Response {
            fake_ip: ::std::option::Option::None,
            fake_ports: ::std::vec::Vec::new(),
            special_fields: ::steam_vent_proto_common::protobuf::SpecialFields::new(),
        };
        &instance
    }
}

// @@protoc_insertion_point(message:CGameNetworking_ReleaseFakeIP_Notification)
#[derive(PartialEq,Clone,Default,Debug)]
pub struct CGameNetworking_ReleaseFakeIP_Notification {
    // message fields
    // @@protoc_insertion_point(field:CGameNetworking_ReleaseFakeIP_Notification.app_id)
    pub app_id: ::std::option::Option<u32>,
    // @@protoc_insertion_point(field:CGameNetworking_ReleaseFakeIP_Notification.fake_ip)
    pub fake_ip: ::std::option::Option<u32>,
    // @@protoc_insertion_point(field:CGameNetworking_ReleaseFakeIP_Notification.fake_ports)
    pub fake_ports: ::std::vec::Vec<u32>,
    // special fields
    // @@protoc_insertion_point(special_field:CGameNetworking_ReleaseFakeIP_Notification.special_fields)
    pub special_fields: ::steam_vent_proto_common::protobuf::SpecialFields,
}

impl<'a> ::std::default::Default for &'a CGameNetworking_ReleaseFakeIP_Notification {
    fn default() -> &'a CGameNetworking_ReleaseFakeIP_Notification {
        <CGameNetworking_ReleaseFakeIP_Notification as ::steam_vent_proto_common::protobuf::Message>::default_instance()
    }
}

impl CGameNetworking_ReleaseFakeIP_Notification {
    pub fn new() -> CGameNetworking_ReleaseFakeIP_Notification {
        ::std::default::Default::default()
    }

    // optional uint32 app_id = 1;

    pub fn app_id(&self) -> u32 {
        self.app_id.unwrap_or(0)
    }

    pub fn clear_app_id(&mut self) {
        self.app_id = ::std::option::Option::None;
    }

    pub fn has_app_id(&self) -> bool {
        self.app_id.is_some()
    }

    // Param is passed by value, moved
    pub fn set_app_id(&mut self, v: u32) {
        self.app_id = ::std::option::Option::Some(v);
    }

    // optional fixed32 fake_ip = 2;

    pub fn fake_ip(&self) -> u32 {
        self.fake_ip.unwrap_or(0)
    }

    pub fn clear_fake_ip(&mut self) {
        self.fake_ip = ::std::option::Option::None;
    }

    pub fn has_fake_ip(&self) -> bool {
        self.fake_ip.is_some()
    }

    // Param is passed by value, moved
    pub fn set_fake_ip(&mut self, v: u32) {
        self.fake_ip = ::std::option::Option::Some(v);
    }
}

impl ::steam_vent_proto_common::protobuf::Message for CGameNetworking_ReleaseFakeIP_Notification {
    const NAME: &'static str = "CGameNetworking_ReleaseFakeIP_Notification";

    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut ::steam_vent_proto_common::protobuf::CodedInputStream<'_>) -> ::steam_vent_proto_common::protobuf::Result<()> {
        while let Some(tag) = is.read_raw_tag_or_eof()? {
            match tag {
                8 => {
                    self.app_id = ::std::option::Option::Some(is.read_uint32()?);
                },
                21 => {
                    self.fake_ip = ::std::option::Option::Some(is.read_fixed32()?);
                },
                26 => {
                    is.read_repeated_packed_uint32_into(&mut self.fake_ports)?;
                },
                24 => {
                    self.fake_ports.push(is.read_uint32()?);
                },
                tag => {
                    ::steam_vent_proto_common::protobuf::rt::read_unknown_or_skip_group(tag, is, self.special_fields.mut_unknown_fields())?;
                },
            };
        }
        ::std::result::Result::Ok(())
    }

    // Compute sizes of nested messages
    #[allow(unused_variables)]
    fn compute_size(&self) -> u64 {
        let mut my_size = 0;
        if let Some(v) = self.app_id {
            my_size += ::steam_vent_proto_common::protobuf::rt::uint32_size(1, v);
        }
        if let Some(v) = self.fake_ip {
            my_size += 1 + 4;
        }
        for value in &self.fake_ports {
            my_size += ::steam_vent_proto_common::protobuf::rt::uint32_size(3, *value);
        };
        my_size += ::steam_vent_proto_common::protobuf::rt::unknown_fields_size(self.special_fields.unknown_fields());
        self.special_fields.cached_size().set(my_size as u32);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::steam_vent_proto_common::protobuf::CodedOutputStream<'_>) -> ::steam_vent_proto_common::protobuf::Result<()> {
        if let Some(v) = self.app_id {
            os.write_uint32(1, v)?;
        }
        if let Some(v) = self.fake_ip {
            os.write_fixed32(2, v)?;
        }
        for v in &self.fake_ports {
            os.write_uint32(3, *v)?;
        };
        os.write_unknown_fields(self.special_fields.unknown_fields())?;
        ::std::result::Result::Ok(())
    }

    fn special_fields(&self) -> &::steam_vent_proto_common::protobuf::SpecialFields {
        &self.special_fields
    }

    fn mut_special_fields(&mut self) -> &mut ::steam_vent_proto_common::protobuf::SpecialFields {
        &mut self.special_fields
    }

    fn new() -> CGameNetworking_ReleaseFakeIP_Notification {
        CGameNetworking_ReleaseFakeIP_Notification::new()
    }

    fn clear(&mut self) {
        self.app_id = ::std::option::Option::None;
        self.fake_ip = ::std::option::Option::None;
        self.fake_ports.clear();
        self.special_fields.clear();
    }

    fn default_instance() -> &'static CGameNetworking_ReleaseFakeIP_Notification {
        static instance: CGameNetworking_ReleaseFakeIP_Notification = CGameNetworking_ReleaseFakeIP_Notification {
            app_id: ::std::option::Option::None,
            fake_ip: ::std::option::Option::None,
            fake_ports: ::std::vec::Vec::new(),
            special_fields: ::steam_vent_proto_common::protobuf::SpecialFields::new(),
        };
        &instance
    }
}


const _VENT_PROTO_VERSION_CHECK: () = ::steam_vent_proto_common::VERSION_0_5_0;

#[allow(unused_imports)]
use crate::steammessages_base::*;
#[allow(unused_imports)]
use crate::steammessages_unified_base_steamclient::*;
impl ::steam_vent_proto_common::RpcMessage for CGameNetworking_AllocateFakeIP_Request {
    fn parse(reader: &mut dyn std::io::Read) -> ::steam_vent_proto_common::protobuf::Result<Self> {
        <Self as ::steam_vent_proto_common::protobuf::Message>::parse_from_reader(reader)
    }
    fn write(&self, writer: &mut dyn std::io::Write) -> ::steam_vent_proto_common::protobuf::Result<()> {
        use ::steam_vent_proto_common::protobuf::Message;
        self.write_to_writer(writer)
    }
    fn encode_size(&self) -> usize {
        use ::steam_vent_proto_common::protobuf::Message;
        self.compute_size() as usize
    }
}
impl ::steam_vent_proto_common::RpcMessage for CGameNetworking_AllocateFakeIP_Response {
    fn parse(reader: &mut dyn std::io::Read) -> ::steam_vent_proto_common::protobuf::Result<Self> {
        <Self as ::steam_vent_proto_common::protobuf::Message>::parse_from_reader(reader)
    }
    fn write(&self, writer: &mut dyn std::io::Write) -> ::steam_vent_proto_common::protobuf::Result<()> {
        use ::steam_vent_proto_common::protobuf::Message;
        self.write_to_writer(writer)
    }
    fn encode_size(&self) -> usize {
        use ::steam_vent_proto_common::protobuf::Message;
        self.compute_size() as usize
    }
}
impl ::steam_vent_proto_common::RpcMessage
for CGameNetworking_ReleaseFakeIP_Notification {
    fn parse(reader: &mut dyn std::io::Read) -> ::steam_vent_proto_common::protobuf::Result<Self> {
        <Self as ::steam_vent_proto_common::protobuf::Message>::parse_from_reader(reader)
    }
    fn write(&self, writer: &mut dyn std::io::Write) -> ::steam_vent_proto_common::protobuf::Result<()> {
        use ::steam_vent_proto_common::protobuf::Message;
        self.write_to_writer(writer)
    }
    fn encode_size(&self) -> usize {
        use ::steam_vent_proto_common::protobuf::Message;
        self.compute_size() as usize
    }
}
///
struct GameNetworking {}
impl ::steam_vent_proto_common::RpcService for GameNetworking {
    const SERVICE_NAME: &'static str = "GameNetworking";
}
impl ::steam_vent_proto_common::RpcMethod for CGameNetworking_AllocateFakeIP_Request {
    const METHOD_NAME: &'static str = "GameNetworking.AllocateFakeIP#1";
    type Response = CGameNetworking_AllocateFakeIP_Response;
}
impl ::steam_vent_proto_common::RpcMethod
for CGameNetworking_ReleaseFakeIP_Notification {
    const METHOD_NAME: &'static str = "GameNetworking.NotifyReleaseFakeIP#1";
    type Response = ();
}
