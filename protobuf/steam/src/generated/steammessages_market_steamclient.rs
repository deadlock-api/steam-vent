// This file is generated by rust-protobuf 3.5.1. Do not edit
// .proto file is parsed by pure
// @generated

// https://github.com/rust-lang/rust-clippy/issues/702
#![allow(unknown_lints)]
#![allow(clippy::all)]

#![allow(unused_attributes)]
#![cfg_attr(rustfmt, rustfmt::skip)]

#![allow(dead_code)]
#![allow(missing_docs)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]
#![allow(trivial_casts)]
#![allow(unused_results)]
#![allow(unused_mut)]

//! Generated file from `steammessages_market.steamclient.proto`
// Generated for lite runtime

/// Generated files are compatible only with the same version
/// of protobuf runtime.
const _PROTOBUF_VERSION_CHECK: () = ::steam_vent_proto_common::protobuf::VERSION_3_7_2;

// @@protoc_insertion_point(message:CEconMarket_IsMarketplaceAllowed_Request)
#[derive(PartialEq,Clone,Default,Debug)]
pub struct CEconMarket_IsMarketplaceAllowed_Request {
    // message fields
    // @@protoc_insertion_point(field:CEconMarket_IsMarketplaceAllowed_Request.webcookie)
    pub webcookie: ::std::option::Option<::std::string::String>,
    // special fields
    // @@protoc_insertion_point(special_field:CEconMarket_IsMarketplaceAllowed_Request.special_fields)
    pub special_fields: ::steam_vent_proto_common::protobuf::SpecialFields,
}

impl<'a> ::std::default::Default for &'a CEconMarket_IsMarketplaceAllowed_Request {
    fn default() -> &'a CEconMarket_IsMarketplaceAllowed_Request {
        <CEconMarket_IsMarketplaceAllowed_Request as ::steam_vent_proto_common::protobuf::Message>::default_instance()
    }
}

impl CEconMarket_IsMarketplaceAllowed_Request {
    pub fn new() -> CEconMarket_IsMarketplaceAllowed_Request {
        ::std::default::Default::default()
    }

    // optional string webcookie = 1;

    pub fn webcookie(&self) -> &str {
        match self.webcookie.as_ref() {
            Some(v) => v,
            None => "",
        }
    }

    pub fn clear_webcookie(&mut self) {
        self.webcookie = ::std::option::Option::None;
    }

    pub fn has_webcookie(&self) -> bool {
        self.webcookie.is_some()
    }

    // Param is passed by value, moved
    pub fn set_webcookie(&mut self, v: ::std::string::String) {
        self.webcookie = ::std::option::Option::Some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_webcookie(&mut self) -> &mut ::std::string::String {
        if self.webcookie.is_none() {
            self.webcookie = ::std::option::Option::Some(::std::string::String::new());
        }
        self.webcookie.as_mut().unwrap()
    }

    // Take field
    pub fn take_webcookie(&mut self) -> ::std::string::String {
        self.webcookie.take().unwrap_or_else(|| ::std::string::String::new())
    }
}

impl ::steam_vent_proto_common::protobuf::Message for CEconMarket_IsMarketplaceAllowed_Request {
    const NAME: &'static str = "CEconMarket_IsMarketplaceAllowed_Request";

    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut ::steam_vent_proto_common::protobuf::CodedInputStream<'_>) -> ::steam_vent_proto_common::protobuf::Result<()> {
        while let Some(tag) = is.read_raw_tag_or_eof()? {
            match tag {
                10 => {
                    self.webcookie = ::std::option::Option::Some(is.read_string()?);
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
        if let Some(v) = self.webcookie.as_ref() {
            my_size += ::steam_vent_proto_common::protobuf::rt::string_size(1, &v);
        }
        my_size += ::steam_vent_proto_common::protobuf::rt::unknown_fields_size(self.special_fields.unknown_fields());
        self.special_fields.cached_size().set(my_size as u32);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::steam_vent_proto_common::protobuf::CodedOutputStream<'_>) -> ::steam_vent_proto_common::protobuf::Result<()> {
        if let Some(v) = self.webcookie.as_ref() {
            os.write_string(1, v)?;
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

    fn new() -> CEconMarket_IsMarketplaceAllowed_Request {
        CEconMarket_IsMarketplaceAllowed_Request::new()
    }

    fn clear(&mut self) {
        self.webcookie = ::std::option::Option::None;
        self.special_fields.clear();
    }

    fn default_instance() -> &'static CEconMarket_IsMarketplaceAllowed_Request {
        static instance: CEconMarket_IsMarketplaceAllowed_Request = CEconMarket_IsMarketplaceAllowed_Request {
            webcookie: ::std::option::Option::None,
            special_fields: ::steam_vent_proto_common::protobuf::SpecialFields::new(),
        };
        &instance
    }
}

// @@protoc_insertion_point(message:CEconMarket_IsMarketplaceAllowed_Response)
#[derive(PartialEq,Clone,Default,Debug)]
pub struct CEconMarket_IsMarketplaceAllowed_Response {
    // message fields
    // @@protoc_insertion_point(field:CEconMarket_IsMarketplaceAllowed_Response.allowed)
    pub allowed: ::std::option::Option<bool>,
    // @@protoc_insertion_point(field:CEconMarket_IsMarketplaceAllowed_Response.reason)
    pub reason: ::std::option::Option<u32>,
    // @@protoc_insertion_point(field:CEconMarket_IsMarketplaceAllowed_Response.allowed_at_time)
    pub allowed_at_time: ::std::option::Option<u32>,
    // @@protoc_insertion_point(field:CEconMarket_IsMarketplaceAllowed_Response.steamguard_required_days)
    pub steamguard_required_days: ::std::option::Option<u32>,
    // @@protoc_insertion_point(field:CEconMarket_IsMarketplaceAllowed_Response.forms_requested)
    pub forms_requested: ::std::option::Option<bool>,
    // @@protoc_insertion_point(field:CEconMarket_IsMarketplaceAllowed_Response.forms_require_verification)
    pub forms_require_verification: ::std::option::Option<bool>,
    // @@protoc_insertion_point(field:CEconMarket_IsMarketplaceAllowed_Response.new_device_cooldown_days)
    pub new_device_cooldown_days: ::std::option::Option<u32>,
    // special fields
    // @@protoc_insertion_point(special_field:CEconMarket_IsMarketplaceAllowed_Response.special_fields)
    pub special_fields: ::steam_vent_proto_common::protobuf::SpecialFields,
}

impl<'a> ::std::default::Default for &'a CEconMarket_IsMarketplaceAllowed_Response {
    fn default() -> &'a CEconMarket_IsMarketplaceAllowed_Response {
        <CEconMarket_IsMarketplaceAllowed_Response as ::steam_vent_proto_common::protobuf::Message>::default_instance()
    }
}

impl CEconMarket_IsMarketplaceAllowed_Response {
    pub fn new() -> CEconMarket_IsMarketplaceAllowed_Response {
        ::std::default::Default::default()
    }

    // optional bool allowed = 1;

    pub fn allowed(&self) -> bool {
        self.allowed.unwrap_or(false)
    }

    pub fn clear_allowed(&mut self) {
        self.allowed = ::std::option::Option::None;
    }

    pub fn has_allowed(&self) -> bool {
        self.allowed.is_some()
    }

    // Param is passed by value, moved
    pub fn set_allowed(&mut self, v: bool) {
        self.allowed = ::std::option::Option::Some(v);
    }

    // optional uint32 reason = 2;

    pub fn reason(&self) -> u32 {
        self.reason.unwrap_or(0)
    }

    pub fn clear_reason(&mut self) {
        self.reason = ::std::option::Option::None;
    }

    pub fn has_reason(&self) -> bool {
        self.reason.is_some()
    }

    // Param is passed by value, moved
    pub fn set_reason(&mut self, v: u32) {
        self.reason = ::std::option::Option::Some(v);
    }

    // optional uint32 allowed_at_time = 3;

    pub fn allowed_at_time(&self) -> u32 {
        self.allowed_at_time.unwrap_or(0)
    }

    pub fn clear_allowed_at_time(&mut self) {
        self.allowed_at_time = ::std::option::Option::None;
    }

    pub fn has_allowed_at_time(&self) -> bool {
        self.allowed_at_time.is_some()
    }

    // Param is passed by value, moved
    pub fn set_allowed_at_time(&mut self, v: u32) {
        self.allowed_at_time = ::std::option::Option::Some(v);
    }

    // optional uint32 steamguard_required_days = 4;

    pub fn steamguard_required_days(&self) -> u32 {
        self.steamguard_required_days.unwrap_or(0)
    }

    pub fn clear_steamguard_required_days(&mut self) {
        self.steamguard_required_days = ::std::option::Option::None;
    }

    pub fn has_steamguard_required_days(&self) -> bool {
        self.steamguard_required_days.is_some()
    }

    // Param is passed by value, moved
    pub fn set_steamguard_required_days(&mut self, v: u32) {
        self.steamguard_required_days = ::std::option::Option::Some(v);
    }

    // optional bool forms_requested = 7;

    pub fn forms_requested(&self) -> bool {
        self.forms_requested.unwrap_or(false)
    }

    pub fn clear_forms_requested(&mut self) {
        self.forms_requested = ::std::option::Option::None;
    }

    pub fn has_forms_requested(&self) -> bool {
        self.forms_requested.is_some()
    }

    // Param is passed by value, moved
    pub fn set_forms_requested(&mut self, v: bool) {
        self.forms_requested = ::std::option::Option::Some(v);
    }

    // optional bool forms_require_verification = 8;

    pub fn forms_require_verification(&self) -> bool {
        self.forms_require_verification.unwrap_or(false)
    }

    pub fn clear_forms_require_verification(&mut self) {
        self.forms_require_verification = ::std::option::Option::None;
    }

    pub fn has_forms_require_verification(&self) -> bool {
        self.forms_require_verification.is_some()
    }

    // Param is passed by value, moved
    pub fn set_forms_require_verification(&mut self, v: bool) {
        self.forms_require_verification = ::std::option::Option::Some(v);
    }

    // optional uint32 new_device_cooldown_days = 9;

    pub fn new_device_cooldown_days(&self) -> u32 {
        self.new_device_cooldown_days.unwrap_or(0)
    }

    pub fn clear_new_device_cooldown_days(&mut self) {
        self.new_device_cooldown_days = ::std::option::Option::None;
    }

    pub fn has_new_device_cooldown_days(&self) -> bool {
        self.new_device_cooldown_days.is_some()
    }

    // Param is passed by value, moved
    pub fn set_new_device_cooldown_days(&mut self, v: u32) {
        self.new_device_cooldown_days = ::std::option::Option::Some(v);
    }
}

impl ::steam_vent_proto_common::protobuf::Message for CEconMarket_IsMarketplaceAllowed_Response {
    const NAME: &'static str = "CEconMarket_IsMarketplaceAllowed_Response";

    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut ::steam_vent_proto_common::protobuf::CodedInputStream<'_>) -> ::steam_vent_proto_common::protobuf::Result<()> {
        while let Some(tag) = is.read_raw_tag_or_eof()? {
            match tag {
                8 => {
                    self.allowed = ::std::option::Option::Some(is.read_bool()?);
                },
                16 => {
                    self.reason = ::std::option::Option::Some(is.read_uint32()?);
                },
                24 => {
                    self.allowed_at_time = ::std::option::Option::Some(is.read_uint32()?);
                },
                32 => {
                    self.steamguard_required_days = ::std::option::Option::Some(is.read_uint32()?);
                },
                56 => {
                    self.forms_requested = ::std::option::Option::Some(is.read_bool()?);
                },
                64 => {
                    self.forms_require_verification = ::std::option::Option::Some(is.read_bool()?);
                },
                72 => {
                    self.new_device_cooldown_days = ::std::option::Option::Some(is.read_uint32()?);
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
        if let Some(v) = self.allowed {
            my_size += 1 + 1;
        }
        if let Some(v) = self.reason {
            my_size += ::steam_vent_proto_common::protobuf::rt::uint32_size(2, v);
        }
        if let Some(v) = self.allowed_at_time {
            my_size += ::steam_vent_proto_common::protobuf::rt::uint32_size(3, v);
        }
        if let Some(v) = self.steamguard_required_days {
            my_size += ::steam_vent_proto_common::protobuf::rt::uint32_size(4, v);
        }
        if let Some(v) = self.forms_requested {
            my_size += 1 + 1;
        }
        if let Some(v) = self.forms_require_verification {
            my_size += 1 + 1;
        }
        if let Some(v) = self.new_device_cooldown_days {
            my_size += ::steam_vent_proto_common::protobuf::rt::uint32_size(9, v);
        }
        my_size += ::steam_vent_proto_common::protobuf::rt::unknown_fields_size(self.special_fields.unknown_fields());
        self.special_fields.cached_size().set(my_size as u32);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::steam_vent_proto_common::protobuf::CodedOutputStream<'_>) -> ::steam_vent_proto_common::protobuf::Result<()> {
        if let Some(v) = self.allowed {
            os.write_bool(1, v)?;
        }
        if let Some(v) = self.reason {
            os.write_uint32(2, v)?;
        }
        if let Some(v) = self.allowed_at_time {
            os.write_uint32(3, v)?;
        }
        if let Some(v) = self.steamguard_required_days {
            os.write_uint32(4, v)?;
        }
        if let Some(v) = self.forms_requested {
            os.write_bool(7, v)?;
        }
        if let Some(v) = self.forms_require_verification {
            os.write_bool(8, v)?;
        }
        if let Some(v) = self.new_device_cooldown_days {
            os.write_uint32(9, v)?;
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

    fn new() -> CEconMarket_IsMarketplaceAllowed_Response {
        CEconMarket_IsMarketplaceAllowed_Response::new()
    }

    fn clear(&mut self) {
        self.allowed = ::std::option::Option::None;
        self.reason = ::std::option::Option::None;
        self.allowed_at_time = ::std::option::Option::None;
        self.steamguard_required_days = ::std::option::Option::None;
        self.forms_requested = ::std::option::Option::None;
        self.forms_require_verification = ::std::option::Option::None;
        self.new_device_cooldown_days = ::std::option::Option::None;
        self.special_fields.clear();
    }

    fn default_instance() -> &'static CEconMarket_IsMarketplaceAllowed_Response {
        static instance: CEconMarket_IsMarketplaceAllowed_Response = CEconMarket_IsMarketplaceAllowed_Response {
            allowed: ::std::option::Option::None,
            reason: ::std::option::Option::None,
            allowed_at_time: ::std::option::Option::None,
            steamguard_required_days: ::std::option::Option::None,
            forms_requested: ::std::option::Option::None,
            forms_require_verification: ::std::option::Option::None,
            new_device_cooldown_days: ::std::option::Option::None,
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
impl ::steam_vent_proto_common::RpcMessage for CEconMarket_IsMarketplaceAllowed_Request {
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
for CEconMarket_IsMarketplaceAllowed_Response {
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
struct EconMarket {}
impl ::steam_vent_proto_common::RpcService for EconMarket {
    const SERVICE_NAME: &'static str = "EconMarket";
}
impl ::steam_vent_proto_common::RpcMethod for CEconMarket_IsMarketplaceAllowed_Request {
    const METHOD_NAME: &'static str = "EconMarket.IsMarketplaceAllowed#1";
    type Response = CEconMarket_IsMarketplaceAllowed_Response;
}
