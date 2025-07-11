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

//! Generated file from `encrypted_app_ticket.proto`
// Generated for lite runtime

/// Generated files are compatible only with the same version
/// of protobuf runtime.
const _PROTOBUF_VERSION_CHECK: () = ::steam_vent_proto_common::protobuf::VERSION_3_7_2;

// @@protoc_insertion_point(message:EncryptedAppTicket)
#[derive(PartialEq,Clone,Default,Debug)]
pub struct EncryptedAppTicket {
    // message fields
    // @@protoc_insertion_point(field:EncryptedAppTicket.ticket_version_no)
    pub ticket_version_no: ::std::option::Option<u32>,
    // @@protoc_insertion_point(field:EncryptedAppTicket.crc_encryptedticket)
    pub crc_encryptedticket: ::std::option::Option<u32>,
    // @@protoc_insertion_point(field:EncryptedAppTicket.cb_encrypteduserdata)
    pub cb_encrypteduserdata: ::std::option::Option<u32>,
    // @@protoc_insertion_point(field:EncryptedAppTicket.cb_encrypted_appownershipticket)
    pub cb_encrypted_appownershipticket: ::std::option::Option<u32>,
    // @@protoc_insertion_point(field:EncryptedAppTicket.encrypted_ticket)
    pub encrypted_ticket: ::std::option::Option<::std::vec::Vec<u8>>,
    // special fields
    // @@protoc_insertion_point(special_field:EncryptedAppTicket.special_fields)
    pub special_fields: ::steam_vent_proto_common::protobuf::SpecialFields,
}

impl<'a> ::std::default::Default for &'a EncryptedAppTicket {
    fn default() -> &'a EncryptedAppTicket {
        <EncryptedAppTicket as ::steam_vent_proto_common::protobuf::Message>::default_instance()
    }
}

impl EncryptedAppTicket {
    pub fn new() -> EncryptedAppTicket {
        ::std::default::Default::default()
    }

    // optional uint32 ticket_version_no = 1;

    pub fn ticket_version_no(&self) -> u32 {
        self.ticket_version_no.unwrap_or(0)
    }

    pub fn clear_ticket_version_no(&mut self) {
        self.ticket_version_no = ::std::option::Option::None;
    }

    pub fn has_ticket_version_no(&self) -> bool {
        self.ticket_version_no.is_some()
    }

    // Param is passed by value, moved
    pub fn set_ticket_version_no(&mut self, v: u32) {
        self.ticket_version_no = ::std::option::Option::Some(v);
    }

    // optional uint32 crc_encryptedticket = 2;

    pub fn crc_encryptedticket(&self) -> u32 {
        self.crc_encryptedticket.unwrap_or(0)
    }

    pub fn clear_crc_encryptedticket(&mut self) {
        self.crc_encryptedticket = ::std::option::Option::None;
    }

    pub fn has_crc_encryptedticket(&self) -> bool {
        self.crc_encryptedticket.is_some()
    }

    // Param is passed by value, moved
    pub fn set_crc_encryptedticket(&mut self, v: u32) {
        self.crc_encryptedticket = ::std::option::Option::Some(v);
    }

    // optional uint32 cb_encrypteduserdata = 3;

    pub fn cb_encrypteduserdata(&self) -> u32 {
        self.cb_encrypteduserdata.unwrap_or(0)
    }

    pub fn clear_cb_encrypteduserdata(&mut self) {
        self.cb_encrypteduserdata = ::std::option::Option::None;
    }

    pub fn has_cb_encrypteduserdata(&self) -> bool {
        self.cb_encrypteduserdata.is_some()
    }

    // Param is passed by value, moved
    pub fn set_cb_encrypteduserdata(&mut self, v: u32) {
        self.cb_encrypteduserdata = ::std::option::Option::Some(v);
    }

    // optional uint32 cb_encrypted_appownershipticket = 4;

    pub fn cb_encrypted_appownershipticket(&self) -> u32 {
        self.cb_encrypted_appownershipticket.unwrap_or(0)
    }

    pub fn clear_cb_encrypted_appownershipticket(&mut self) {
        self.cb_encrypted_appownershipticket = ::std::option::Option::None;
    }

    pub fn has_cb_encrypted_appownershipticket(&self) -> bool {
        self.cb_encrypted_appownershipticket.is_some()
    }

    // Param is passed by value, moved
    pub fn set_cb_encrypted_appownershipticket(&mut self, v: u32) {
        self.cb_encrypted_appownershipticket = ::std::option::Option::Some(v);
    }

    // optional bytes encrypted_ticket = 5;

    pub fn encrypted_ticket(&self) -> &[u8] {
        match self.encrypted_ticket.as_ref() {
            Some(v) => v,
            None => &[],
        }
    }

    pub fn clear_encrypted_ticket(&mut self) {
        self.encrypted_ticket = ::std::option::Option::None;
    }

    pub fn has_encrypted_ticket(&self) -> bool {
        self.encrypted_ticket.is_some()
    }

    // Param is passed by value, moved
    pub fn set_encrypted_ticket(&mut self, v: ::std::vec::Vec<u8>) {
        self.encrypted_ticket = ::std::option::Option::Some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_encrypted_ticket(&mut self) -> &mut ::std::vec::Vec<u8> {
        if self.encrypted_ticket.is_none() {
            self.encrypted_ticket = ::std::option::Option::Some(::std::vec::Vec::new());
        }
        self.encrypted_ticket.as_mut().unwrap()
    }

    // Take field
    pub fn take_encrypted_ticket(&mut self) -> ::std::vec::Vec<u8> {
        self.encrypted_ticket.take().unwrap_or_else(|| ::std::vec::Vec::new())
    }
}

impl ::steam_vent_proto_common::protobuf::Message for EncryptedAppTicket {
    const NAME: &'static str = "EncryptedAppTicket";

    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut ::steam_vent_proto_common::protobuf::CodedInputStream<'_>) -> ::steam_vent_proto_common::protobuf::Result<()> {
        while let Some(tag) = is.read_raw_tag_or_eof()? {
            match tag {
                8 => {
                    self.ticket_version_no = ::std::option::Option::Some(is.read_uint32()?);
                },
                16 => {
                    self.crc_encryptedticket = ::std::option::Option::Some(is.read_uint32()?);
                },
                24 => {
                    self.cb_encrypteduserdata = ::std::option::Option::Some(is.read_uint32()?);
                },
                32 => {
                    self.cb_encrypted_appownershipticket = ::std::option::Option::Some(is.read_uint32()?);
                },
                42 => {
                    self.encrypted_ticket = ::std::option::Option::Some(is.read_bytes()?);
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
        if let Some(v) = self.ticket_version_no {
            my_size += ::steam_vent_proto_common::protobuf::rt::uint32_size(1, v);
        }
        if let Some(v) = self.crc_encryptedticket {
            my_size += ::steam_vent_proto_common::protobuf::rt::uint32_size(2, v);
        }
        if let Some(v) = self.cb_encrypteduserdata {
            my_size += ::steam_vent_proto_common::protobuf::rt::uint32_size(3, v);
        }
        if let Some(v) = self.cb_encrypted_appownershipticket {
            my_size += ::steam_vent_proto_common::protobuf::rt::uint32_size(4, v);
        }
        if let Some(v) = self.encrypted_ticket.as_ref() {
            my_size += ::steam_vent_proto_common::protobuf::rt::bytes_size(5, &v);
        }
        my_size += ::steam_vent_proto_common::protobuf::rt::unknown_fields_size(self.special_fields.unknown_fields());
        self.special_fields.cached_size().set(my_size as u32);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::steam_vent_proto_common::protobuf::CodedOutputStream<'_>) -> ::steam_vent_proto_common::protobuf::Result<()> {
        if let Some(v) = self.ticket_version_no {
            os.write_uint32(1, v)?;
        }
        if let Some(v) = self.crc_encryptedticket {
            os.write_uint32(2, v)?;
        }
        if let Some(v) = self.cb_encrypteduserdata {
            os.write_uint32(3, v)?;
        }
        if let Some(v) = self.cb_encrypted_appownershipticket {
            os.write_uint32(4, v)?;
        }
        if let Some(v) = self.encrypted_ticket.as_ref() {
            os.write_bytes(5, v)?;
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

    fn new() -> EncryptedAppTicket {
        EncryptedAppTicket::new()
    }

    fn clear(&mut self) {
        self.ticket_version_no = ::std::option::Option::None;
        self.crc_encryptedticket = ::std::option::Option::None;
        self.cb_encrypteduserdata = ::std::option::Option::None;
        self.cb_encrypted_appownershipticket = ::std::option::Option::None;
        self.encrypted_ticket = ::std::option::Option::None;
        self.special_fields.clear();
    }

    fn default_instance() -> &'static EncryptedAppTicket {
        static instance: EncryptedAppTicket = EncryptedAppTicket {
            ticket_version_no: ::std::option::Option::None,
            crc_encryptedticket: ::std::option::Option::None,
            cb_encrypteduserdata: ::std::option::Option::None,
            cb_encrypted_appownershipticket: ::std::option::Option::None,
            encrypted_ticket: ::std::option::Option::None,
            special_fields: ::steam_vent_proto_common::protobuf::SpecialFields::new(),
        };
        &instance
    }
}


const _VENT_PROTO_VERSION_CHECK: () = ::steam_vent_proto_common::VERSION_0_5_0;

impl ::steam_vent_proto_common::RpcMessage for EncryptedAppTicket {
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
