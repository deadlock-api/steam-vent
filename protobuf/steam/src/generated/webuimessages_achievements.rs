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

//! Generated file from `webuimessages_achievements.proto`
// Generated for lite runtime

/// Generated files are compatible only with the same version
/// of protobuf runtime.
const _PROTOBUF_VERSION_CHECK: () = ::steam_vent_proto_common::protobuf::VERSION_3_4_0;

// @@protoc_insertion_point(message:CAchievements_GetInfo_Request)
#[derive(PartialEq,Clone,Default,Debug)]
pub struct CAchievements_GetInfo_Request {
    // message fields
    // @@protoc_insertion_point(field:CAchievements_GetInfo_Request.gameid)
    pub gameid: ::std::option::Option<u64>,
    // special fields
    // @@protoc_insertion_point(special_field:CAchievements_GetInfo_Request.special_fields)
    pub special_fields: ::steam_vent_proto_common::protobuf::SpecialFields,
}

impl<'a> ::std::default::Default for &'a CAchievements_GetInfo_Request {
    fn default() -> &'a CAchievements_GetInfo_Request {
        <CAchievements_GetInfo_Request as ::steam_vent_proto_common::protobuf::Message>::default_instance()
    }
}

impl CAchievements_GetInfo_Request {
    pub fn new() -> CAchievements_GetInfo_Request {
        ::std::default::Default::default()
    }

    // optional uint64 gameid = 1;

    pub fn gameid(&self) -> u64 {
        self.gameid.unwrap_or(0)
    }

    pub fn clear_gameid(&mut self) {
        self.gameid = ::std::option::Option::None;
    }

    pub fn has_gameid(&self) -> bool {
        self.gameid.is_some()
    }

    // Param is passed by value, moved
    pub fn set_gameid(&mut self, v: u64) {
        self.gameid = ::std::option::Option::Some(v);
    }
}

impl ::steam_vent_proto_common::protobuf::Message for CAchievements_GetInfo_Request {
    const NAME: &'static str = "CAchievements_GetInfo_Request";

    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut ::steam_vent_proto_common::protobuf::CodedInputStream<'_>) -> ::steam_vent_proto_common::protobuf::Result<()> {
        while let Some(tag) = is.read_raw_tag_or_eof()? {
            match tag {
                8 => {
                    self.gameid = ::std::option::Option::Some(is.read_uint64()?);
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
        if let Some(v) = self.gameid {
            my_size += ::steam_vent_proto_common::protobuf::rt::uint64_size(1, v);
        }
        my_size += ::steam_vent_proto_common::protobuf::rt::unknown_fields_size(self.special_fields.unknown_fields());
        self.special_fields.cached_size().set(my_size as u32);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::steam_vent_proto_common::protobuf::CodedOutputStream<'_>) -> ::steam_vent_proto_common::protobuf::Result<()> {
        if let Some(v) = self.gameid {
            os.write_uint64(1, v)?;
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

    fn new() -> CAchievements_GetInfo_Request {
        CAchievements_GetInfo_Request::new()
    }

    fn clear(&mut self) {
        self.gameid = ::std::option::Option::None;
        self.special_fields.clear();
    }

    fn default_instance() -> &'static CAchievements_GetInfo_Request {
        static instance: CAchievements_GetInfo_Request = CAchievements_GetInfo_Request {
            gameid: ::std::option::Option::None,
            special_fields: ::steam_vent_proto_common::protobuf::SpecialFields::new(),
        };
        &instance
    }
}

// @@protoc_insertion_point(message:CAchievements_GetInfo_Response)
#[derive(PartialEq,Clone,Default,Debug)]
pub struct CAchievements_GetInfo_Response {
    // message fields
    // @@protoc_insertion_point(field:CAchievements_GetInfo_Response.achievements)
    pub achievements: ::std::vec::Vec<cachievements_get_info_response::Info>,
    // special fields
    // @@protoc_insertion_point(special_field:CAchievements_GetInfo_Response.special_fields)
    pub special_fields: ::steam_vent_proto_common::protobuf::SpecialFields,
}

impl<'a> ::std::default::Default for &'a CAchievements_GetInfo_Response {
    fn default() -> &'a CAchievements_GetInfo_Response {
        <CAchievements_GetInfo_Response as ::steam_vent_proto_common::protobuf::Message>::default_instance()
    }
}

impl CAchievements_GetInfo_Response {
    pub fn new() -> CAchievements_GetInfo_Response {
        ::std::default::Default::default()
    }
}

impl ::steam_vent_proto_common::protobuf::Message for CAchievements_GetInfo_Response {
    const NAME: &'static str = "CAchievements_GetInfo_Response";

    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut ::steam_vent_proto_common::protobuf::CodedInputStream<'_>) -> ::steam_vent_proto_common::protobuf::Result<()> {
        while let Some(tag) = is.read_raw_tag_or_eof()? {
            match tag {
                10 => {
                    self.achievements.push(is.read_message()?);
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
        for value in &self.achievements {
            let len = value.compute_size();
            my_size += 1 + ::steam_vent_proto_common::protobuf::rt::compute_raw_varint64_size(len) + len;
        };
        my_size += ::steam_vent_proto_common::protobuf::rt::unknown_fields_size(self.special_fields.unknown_fields());
        self.special_fields.cached_size().set(my_size as u32);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::steam_vent_proto_common::protobuf::CodedOutputStream<'_>) -> ::steam_vent_proto_common::protobuf::Result<()> {
        for v in &self.achievements {
            ::steam_vent_proto_common::protobuf::rt::write_message_field_with_cached_size(1, v, os)?;
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

    fn new() -> CAchievements_GetInfo_Response {
        CAchievements_GetInfo_Response::new()
    }

    fn clear(&mut self) {
        self.achievements.clear();
        self.special_fields.clear();
    }

    fn default_instance() -> &'static CAchievements_GetInfo_Response {
        static instance: CAchievements_GetInfo_Response = CAchievements_GetInfo_Response {
            achievements: ::std::vec::Vec::new(),
            special_fields: ::steam_vent_proto_common::protobuf::SpecialFields::new(),
        };
        &instance
    }
}

/// Nested message and enums of message `CAchievements_GetInfo_Response`
pub mod cachievements_get_info_response {
    // @@protoc_insertion_point(message:CAchievements_GetInfo_Response.Info)
    #[derive(PartialEq,Clone,Default,Debug)]
    pub struct Info {
        // message fields
        // @@protoc_insertion_point(field:CAchievements_GetInfo_Response.Info.id)
        pub id: ::std::option::Option<::std::string::String>,
        // @@protoc_insertion_point(field:CAchievements_GetInfo_Response.Info.name)
        pub name: ::std::option::Option<::std::string::String>,
        // @@protoc_insertion_point(field:CAchievements_GetInfo_Response.Info.desc)
        pub desc: ::std::option::Option<::std::string::String>,
        // @@protoc_insertion_point(field:CAchievements_GetInfo_Response.Info.image_url_achieved)
        pub image_url_achieved: ::std::option::Option<::std::string::String>,
        // @@protoc_insertion_point(field:CAchievements_GetInfo_Response.Info.image_url_not_achieved)
        pub image_url_not_achieved: ::std::option::Option<::std::string::String>,
        // @@protoc_insertion_point(field:CAchievements_GetInfo_Response.Info.achieved)
        pub achieved: ::std::option::Option<bool>,
        // @@protoc_insertion_point(field:CAchievements_GetInfo_Response.Info.unlock_time)
        pub unlock_time: ::std::option::Option<u32>,
        // special fields
        // @@protoc_insertion_point(special_field:CAchievements_GetInfo_Response.Info.special_fields)
        pub special_fields: ::steam_vent_proto_common::protobuf::SpecialFields,
    }

    impl<'a> ::std::default::Default for &'a Info {
        fn default() -> &'a Info {
            <Info as ::steam_vent_proto_common::protobuf::Message>::default_instance()
        }
    }

    impl Info {
        pub fn new() -> Info {
            ::std::default::Default::default()
        }

        // optional string id = 1;

        pub fn id(&self) -> &str {
            match self.id.as_ref() {
                Some(v) => v,
                None => "",
            }
        }

        pub fn clear_id(&mut self) {
            self.id = ::std::option::Option::None;
        }

        pub fn has_id(&self) -> bool {
            self.id.is_some()
        }

        // Param is passed by value, moved
        pub fn set_id(&mut self, v: ::std::string::String) {
            self.id = ::std::option::Option::Some(v);
        }

        // Mutable pointer to the field.
        // If field is not initialized, it is initialized with default value first.
        pub fn mut_id(&mut self) -> &mut ::std::string::String {
            if self.id.is_none() {
                self.id = ::std::option::Option::Some(::std::string::String::new());
            }
            self.id.as_mut().unwrap()
        }

        // Take field
        pub fn take_id(&mut self) -> ::std::string::String {
            self.id.take().unwrap_or_else(|| ::std::string::String::new())
        }

        // optional string name = 2;

        pub fn name(&self) -> &str {
            match self.name.as_ref() {
                Some(v) => v,
                None => "",
            }
        }

        pub fn clear_name(&mut self) {
            self.name = ::std::option::Option::None;
        }

        pub fn has_name(&self) -> bool {
            self.name.is_some()
        }

        // Param is passed by value, moved
        pub fn set_name(&mut self, v: ::std::string::String) {
            self.name = ::std::option::Option::Some(v);
        }

        // Mutable pointer to the field.
        // If field is not initialized, it is initialized with default value first.
        pub fn mut_name(&mut self) -> &mut ::std::string::String {
            if self.name.is_none() {
                self.name = ::std::option::Option::Some(::std::string::String::new());
            }
            self.name.as_mut().unwrap()
        }

        // Take field
        pub fn take_name(&mut self) -> ::std::string::String {
            self.name.take().unwrap_or_else(|| ::std::string::String::new())
        }

        // optional string desc = 3;

        pub fn desc(&self) -> &str {
            match self.desc.as_ref() {
                Some(v) => v,
                None => "",
            }
        }

        pub fn clear_desc(&mut self) {
            self.desc = ::std::option::Option::None;
        }

        pub fn has_desc(&self) -> bool {
            self.desc.is_some()
        }

        // Param is passed by value, moved
        pub fn set_desc(&mut self, v: ::std::string::String) {
            self.desc = ::std::option::Option::Some(v);
        }

        // Mutable pointer to the field.
        // If field is not initialized, it is initialized with default value first.
        pub fn mut_desc(&mut self) -> &mut ::std::string::String {
            if self.desc.is_none() {
                self.desc = ::std::option::Option::Some(::std::string::String::new());
            }
            self.desc.as_mut().unwrap()
        }

        // Take field
        pub fn take_desc(&mut self) -> ::std::string::String {
            self.desc.take().unwrap_or_else(|| ::std::string::String::new())
        }

        // optional string image_url_achieved = 4;

        pub fn image_url_achieved(&self) -> &str {
            match self.image_url_achieved.as_ref() {
                Some(v) => v,
                None => "",
            }
        }

        pub fn clear_image_url_achieved(&mut self) {
            self.image_url_achieved = ::std::option::Option::None;
        }

        pub fn has_image_url_achieved(&self) -> bool {
            self.image_url_achieved.is_some()
        }

        // Param is passed by value, moved
        pub fn set_image_url_achieved(&mut self, v: ::std::string::String) {
            self.image_url_achieved = ::std::option::Option::Some(v);
        }

        // Mutable pointer to the field.
        // If field is not initialized, it is initialized with default value first.
        pub fn mut_image_url_achieved(&mut self) -> &mut ::std::string::String {
            if self.image_url_achieved.is_none() {
                self.image_url_achieved = ::std::option::Option::Some(::std::string::String::new());
            }
            self.image_url_achieved.as_mut().unwrap()
        }

        // Take field
        pub fn take_image_url_achieved(&mut self) -> ::std::string::String {
            self.image_url_achieved.take().unwrap_or_else(|| ::std::string::String::new())
        }

        // optional string image_url_not_achieved = 5;

        pub fn image_url_not_achieved(&self) -> &str {
            match self.image_url_not_achieved.as_ref() {
                Some(v) => v,
                None => "",
            }
        }

        pub fn clear_image_url_not_achieved(&mut self) {
            self.image_url_not_achieved = ::std::option::Option::None;
        }

        pub fn has_image_url_not_achieved(&self) -> bool {
            self.image_url_not_achieved.is_some()
        }

        // Param is passed by value, moved
        pub fn set_image_url_not_achieved(&mut self, v: ::std::string::String) {
            self.image_url_not_achieved = ::std::option::Option::Some(v);
        }

        // Mutable pointer to the field.
        // If field is not initialized, it is initialized with default value first.
        pub fn mut_image_url_not_achieved(&mut self) -> &mut ::std::string::String {
            if self.image_url_not_achieved.is_none() {
                self.image_url_not_achieved = ::std::option::Option::Some(::std::string::String::new());
            }
            self.image_url_not_achieved.as_mut().unwrap()
        }

        // Take field
        pub fn take_image_url_not_achieved(&mut self) -> ::std::string::String {
            self.image_url_not_achieved.take().unwrap_or_else(|| ::std::string::String::new())
        }

        // optional bool achieved = 6;

        pub fn achieved(&self) -> bool {
            self.achieved.unwrap_or(false)
        }

        pub fn clear_achieved(&mut self) {
            self.achieved = ::std::option::Option::None;
        }

        pub fn has_achieved(&self) -> bool {
            self.achieved.is_some()
        }

        // Param is passed by value, moved
        pub fn set_achieved(&mut self, v: bool) {
            self.achieved = ::std::option::Option::Some(v);
        }

        // optional uint32 unlock_time = 7;

        pub fn unlock_time(&self) -> u32 {
            self.unlock_time.unwrap_or(0)
        }

        pub fn clear_unlock_time(&mut self) {
            self.unlock_time = ::std::option::Option::None;
        }

        pub fn has_unlock_time(&self) -> bool {
            self.unlock_time.is_some()
        }

        // Param is passed by value, moved
        pub fn set_unlock_time(&mut self, v: u32) {
            self.unlock_time = ::std::option::Option::Some(v);
        }
    }

    impl ::steam_vent_proto_common::protobuf::Message for Info {
        const NAME: &'static str = "Info";

        fn is_initialized(&self) -> bool {
            true
        }

        fn merge_from(&mut self, is: &mut ::steam_vent_proto_common::protobuf::CodedInputStream<'_>) -> ::steam_vent_proto_common::protobuf::Result<()> {
            while let Some(tag) = is.read_raw_tag_or_eof()? {
                match tag {
                    10 => {
                        self.id = ::std::option::Option::Some(is.read_string()?);
                    },
                    18 => {
                        self.name = ::std::option::Option::Some(is.read_string()?);
                    },
                    26 => {
                        self.desc = ::std::option::Option::Some(is.read_string()?);
                    },
                    34 => {
                        self.image_url_achieved = ::std::option::Option::Some(is.read_string()?);
                    },
                    42 => {
                        self.image_url_not_achieved = ::std::option::Option::Some(is.read_string()?);
                    },
                    48 => {
                        self.achieved = ::std::option::Option::Some(is.read_bool()?);
                    },
                    56 => {
                        self.unlock_time = ::std::option::Option::Some(is.read_uint32()?);
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
            if let Some(v) = self.id.as_ref() {
                my_size += ::steam_vent_proto_common::protobuf::rt::string_size(1, &v);
            }
            if let Some(v) = self.name.as_ref() {
                my_size += ::steam_vent_proto_common::protobuf::rt::string_size(2, &v);
            }
            if let Some(v) = self.desc.as_ref() {
                my_size += ::steam_vent_proto_common::protobuf::rt::string_size(3, &v);
            }
            if let Some(v) = self.image_url_achieved.as_ref() {
                my_size += ::steam_vent_proto_common::protobuf::rt::string_size(4, &v);
            }
            if let Some(v) = self.image_url_not_achieved.as_ref() {
                my_size += ::steam_vent_proto_common::protobuf::rt::string_size(5, &v);
            }
            if let Some(v) = self.achieved {
                my_size += 1 + 1;
            }
            if let Some(v) = self.unlock_time {
                my_size += ::steam_vent_proto_common::protobuf::rt::uint32_size(7, v);
            }
            my_size += ::steam_vent_proto_common::protobuf::rt::unknown_fields_size(self.special_fields.unknown_fields());
            self.special_fields.cached_size().set(my_size as u32);
            my_size
        }

        fn write_to_with_cached_sizes(&self, os: &mut ::steam_vent_proto_common::protobuf::CodedOutputStream<'_>) -> ::steam_vent_proto_common::protobuf::Result<()> {
            if let Some(v) = self.id.as_ref() {
                os.write_string(1, v)?;
            }
            if let Some(v) = self.name.as_ref() {
                os.write_string(2, v)?;
            }
            if let Some(v) = self.desc.as_ref() {
                os.write_string(3, v)?;
            }
            if let Some(v) = self.image_url_achieved.as_ref() {
                os.write_string(4, v)?;
            }
            if let Some(v) = self.image_url_not_achieved.as_ref() {
                os.write_string(5, v)?;
            }
            if let Some(v) = self.achieved {
                os.write_bool(6, v)?;
            }
            if let Some(v) = self.unlock_time {
                os.write_uint32(7, v)?;
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

        fn new() -> Info {
            Info::new()
        }

        fn clear(&mut self) {
            self.id = ::std::option::Option::None;
            self.name = ::std::option::Option::None;
            self.desc = ::std::option::Option::None;
            self.image_url_achieved = ::std::option::Option::None;
            self.image_url_not_achieved = ::std::option::Option::None;
            self.achieved = ::std::option::Option::None;
            self.unlock_time = ::std::option::Option::None;
            self.special_fields.clear();
        }

        fn default_instance() -> &'static Info {
            static instance: Info = Info {
                id: ::std::option::Option::None,
                name: ::std::option::Option::None,
                desc: ::std::option::Option::None,
                image_url_achieved: ::std::option::Option::None,
                image_url_not_achieved: ::std::option::Option::None,
                achieved: ::std::option::Option::None,
                unlock_time: ::std::option::Option::None,
                special_fields: ::steam_vent_proto_common::protobuf::SpecialFields::new(),
            };
            &instance
        }
    }
}


const _VENT_PROTO_VERSION_CHECK: () = ::steam_vent_proto_common::VERSION_0_5_0;

#[allow(unused_imports)]
use crate::steammessages_base::*;
#[allow(unused_imports)]
use crate::webuimessages_base::*;
impl ::steam_vent_proto_common::RpcMessage for CAchievements_GetInfo_Request {
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
impl ::steam_vent_proto_common::RpcMessage for CAchievements_GetInfo_Response {
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
struct Achievements {}
impl ::steam_vent_proto_common::RpcService for Achievements {
    const SERVICE_NAME: &'static str = "Achievements";
}
impl ::steam_vent_proto_common::RpcMethod for CAchievements_GetInfo_Request {
    const METHOD_NAME: &'static str = "Achievements.GetInfo#1";
    type Response = CAchievements_GetInfo_Response;
}
