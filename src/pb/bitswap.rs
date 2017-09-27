// This file is generated. Do not edit
// @generated

// https://github.com/Manishearth/rust-clippy/issues/702
#![allow(unknown_lints)]
#![allow(clippy)]

#![cfg_attr(rustfmt, rustfmt_skip)]

#![allow(box_pointers)]
#![allow(dead_code)]
#![allow(missing_docs)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]
#![allow(trivial_casts)]
#![allow(unsafe_code)]
#![allow(unused_imports)]
#![allow(unused_results)]

use protobuf::Message as Message_imported_for_functions;
use protobuf::ProtobufEnum as ProtobufEnum_imported_for_functions;

#[derive(PartialEq,Clone,Default)]
pub struct Message {
    // message fields
    wantlist: ::protobuf::SingularPtrField<Message_Wantlist>,
    blocks: ::protobuf::RepeatedField<::std::vec::Vec<u8>>,
    payload: ::protobuf::RepeatedField<Message_Block>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for Message {}

impl Message {
    pub fn new() -> Message {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static Message {
        static mut instance: ::protobuf::lazy::Lazy<Message> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const Message,
        };
        unsafe {
            instance.get(Message::new)
        }
    }

    // optional .bitswap.message.pb.Message.Wantlist wantlist = 1;

    pub fn clear_wantlist(&mut self) {
        self.wantlist.clear();
    }

    pub fn has_wantlist(&self) -> bool {
        self.wantlist.is_some()
    }

    // Param is passed by value, moved
    pub fn set_wantlist(&mut self, v: Message_Wantlist) {
        self.wantlist = ::protobuf::SingularPtrField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_wantlist(&mut self) -> &mut Message_Wantlist {
        if self.wantlist.is_none() {
            self.wantlist.set_default();
        }
        self.wantlist.as_mut().unwrap()
    }

    // Take field
    pub fn take_wantlist(&mut self) -> Message_Wantlist {
        self.wantlist.take().unwrap_or_else(|| Message_Wantlist::new())
    }

    pub fn get_wantlist(&self) -> &Message_Wantlist {
        self.wantlist.as_ref().unwrap_or_else(|| Message_Wantlist::default_instance())
    }

    fn get_wantlist_for_reflect(&self) -> &::protobuf::SingularPtrField<Message_Wantlist> {
        &self.wantlist
    }

    fn mut_wantlist_for_reflect(&mut self) -> &mut ::protobuf::SingularPtrField<Message_Wantlist> {
        &mut self.wantlist
    }

    // repeated bytes blocks = 2;

    pub fn clear_blocks(&mut self) {
        self.blocks.clear();
    }

    // Param is passed by value, moved
    pub fn set_blocks(&mut self, v: ::protobuf::RepeatedField<::std::vec::Vec<u8>>) {
        self.blocks = v;
    }

    // Mutable pointer to the field.
    pub fn mut_blocks(&mut self) -> &mut ::protobuf::RepeatedField<::std::vec::Vec<u8>> {
        &mut self.blocks
    }

    // Take field
    pub fn take_blocks(&mut self) -> ::protobuf::RepeatedField<::std::vec::Vec<u8>> {
        ::std::mem::replace(&mut self.blocks, ::protobuf::RepeatedField::new())
    }

    pub fn get_blocks(&self) -> &[::std::vec::Vec<u8>] {
        &self.blocks
    }

    fn get_blocks_for_reflect(&self) -> &::protobuf::RepeatedField<::std::vec::Vec<u8>> {
        &self.blocks
    }

    fn mut_blocks_for_reflect(&mut self) -> &mut ::protobuf::RepeatedField<::std::vec::Vec<u8>> {
        &mut self.blocks
    }

    // repeated .bitswap.message.pb.Message.Block payload = 3;

    pub fn clear_payload(&mut self) {
        self.payload.clear();
    }

    // Param is passed by value, moved
    pub fn set_payload(&mut self, v: ::protobuf::RepeatedField<Message_Block>) {
        self.payload = v;
    }

    // Mutable pointer to the field.
    pub fn mut_payload(&mut self) -> &mut ::protobuf::RepeatedField<Message_Block> {
        &mut self.payload
    }

    // Take field
    pub fn take_payload(&mut self) -> ::protobuf::RepeatedField<Message_Block> {
        ::std::mem::replace(&mut self.payload, ::protobuf::RepeatedField::new())
    }

    pub fn get_payload(&self) -> &[Message_Block] {
        &self.payload
    }

    fn get_payload_for_reflect(&self) -> &::protobuf::RepeatedField<Message_Block> {
        &self.payload
    }

    fn mut_payload_for_reflect(&mut self) -> &mut ::protobuf::RepeatedField<Message_Block> {
        &mut self.payload
    }
}

impl ::protobuf::Message for Message {
    fn is_initialized(&self) -> bool {
        for v in &self.wantlist {
            if !v.is_initialized() {
                return false;
            }
        };
        for v in &self.payload {
            if !v.is_initialized() {
                return false;
            }
        };
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream) -> ::protobuf::ProtobufResult<()> {
        while !is.eof()? {
            let (field_number, wire_type) = is.read_tag_unpack()?;
            match field_number {
                1 => {
                    ::protobuf::rt::read_singular_message_into(wire_type, is, &mut self.wantlist)?;
                },
                2 => {
                    ::protobuf::rt::read_repeated_bytes_into(wire_type, is, &mut self.blocks)?;
                },
                3 => {
                    ::protobuf::rt::read_repeated_message_into(wire_type, is, &mut self.payload)?;
                },
                _ => {
                    ::protobuf::rt::read_unknown_or_skip_group(field_number, wire_type, is, self.mut_unknown_fields())?;
                },
            };
        }
        ::std::result::Result::Ok(())
    }

    // Compute sizes of nested messages
    #[allow(unused_variables)]
    fn compute_size(&self) -> u32 {
        let mut my_size = 0;
        if let Some(ref v) = self.wantlist.as_ref() {
            let len = v.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        }
        for value in &self.blocks {
            my_size += ::protobuf::rt::bytes_size(2, &value);
        };
        for value in &self.payload {
            let len = value.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        };
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if let Some(ref v) = self.wantlist.as_ref() {
            os.write_tag(1, ::protobuf::wire_format::WireTypeLengthDelimited)?;
            os.write_raw_varint32(v.get_cached_size())?;
            v.write_to_with_cached_sizes(os)?;
        }
        for v in &self.blocks {
            os.write_bytes(2, &v)?;
        };
        for v in &self.payload {
            os.write_tag(3, ::protobuf::wire_format::WireTypeLengthDelimited)?;
            os.write_raw_varint32(v.get_cached_size())?;
            v.write_to_with_cached_sizes(os)?;
        };
        os.write_unknown_fields(self.get_unknown_fields())?;
        ::std::result::Result::Ok(())
    }

    fn get_cached_size(&self) -> u32 {
        self.cached_size.get()
    }

    fn get_unknown_fields(&self) -> &::protobuf::UnknownFields {
        &self.unknown_fields
    }

    fn mut_unknown_fields(&mut self) -> &mut ::protobuf::UnknownFields {
        &mut self.unknown_fields
    }

    fn as_any(&self) -> &::std::any::Any {
        self as &::std::any::Any
    }
    fn as_any_mut(&mut self) -> &mut ::std::any::Any {
        self as &mut ::std::any::Any
    }
    fn into_any(self: Box<Self>) -> ::std::boxed::Box<::std::any::Any> {
        self
    }

    fn descriptor(&self) -> &'static ::protobuf::reflect::MessageDescriptor {
        ::protobuf::MessageStatic::descriptor_static(None::<Self>)
    }
}

impl ::protobuf::MessageStatic for Message {
    fn new() -> Message {
        Message::new()
    }

    fn descriptor_static(_: ::std::option::Option<Message>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_singular_ptr_field_accessor::<_, ::protobuf::types::ProtobufTypeMessage<Message_Wantlist>>(
                    "wantlist",
                    Message::get_wantlist_for_reflect,
                    Message::mut_wantlist_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_repeated_field_accessor::<_, ::protobuf::types::ProtobufTypeBytes>(
                    "blocks",
                    Message::get_blocks_for_reflect,
                    Message::mut_blocks_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_repeated_field_accessor::<_, ::protobuf::types::ProtobufTypeMessage<Message_Block>>(
                    "payload",
                    Message::get_payload_for_reflect,
                    Message::mut_payload_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<Message>(
                    "Message",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for Message {
    fn clear(&mut self) {
        self.clear_wantlist();
        self.clear_blocks();
        self.clear_payload();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for Message {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for Message {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct Message_Wantlist {
    // message fields
    entries: ::protobuf::RepeatedField<Message_Wantlist_Entry>,
    full: ::std::option::Option<bool>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for Message_Wantlist {}

impl Message_Wantlist {
    pub fn new() -> Message_Wantlist {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static Message_Wantlist {
        static mut instance: ::protobuf::lazy::Lazy<Message_Wantlist> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const Message_Wantlist,
        };
        unsafe {
            instance.get(Message_Wantlist::new)
        }
    }

    // repeated .bitswap.message.pb.Message.Wantlist.Entry entries = 1;

    pub fn clear_entries(&mut self) {
        self.entries.clear();
    }

    // Param is passed by value, moved
    pub fn set_entries(&mut self, v: ::protobuf::RepeatedField<Message_Wantlist_Entry>) {
        self.entries = v;
    }

    // Mutable pointer to the field.
    pub fn mut_entries(&mut self) -> &mut ::protobuf::RepeatedField<Message_Wantlist_Entry> {
        &mut self.entries
    }

    // Take field
    pub fn take_entries(&mut self) -> ::protobuf::RepeatedField<Message_Wantlist_Entry> {
        ::std::mem::replace(&mut self.entries, ::protobuf::RepeatedField::new())
    }

    pub fn get_entries(&self) -> &[Message_Wantlist_Entry] {
        &self.entries
    }

    fn get_entries_for_reflect(&self) -> &::protobuf::RepeatedField<Message_Wantlist_Entry> {
        &self.entries
    }

    fn mut_entries_for_reflect(&mut self) -> &mut ::protobuf::RepeatedField<Message_Wantlist_Entry> {
        &mut self.entries
    }

    // optional bool full = 2;

    pub fn clear_full(&mut self) {
        self.full = ::std::option::Option::None;
    }

    pub fn has_full(&self) -> bool {
        self.full.is_some()
    }

    // Param is passed by value, moved
    pub fn set_full(&mut self, v: bool) {
        self.full = ::std::option::Option::Some(v);
    }

    pub fn get_full(&self) -> bool {
        self.full.unwrap_or(false)
    }

    fn get_full_for_reflect(&self) -> &::std::option::Option<bool> {
        &self.full
    }

    fn mut_full_for_reflect(&mut self) -> &mut ::std::option::Option<bool> {
        &mut self.full
    }
}

impl ::protobuf::Message for Message_Wantlist {
    fn is_initialized(&self) -> bool {
        for v in &self.entries {
            if !v.is_initialized() {
                return false;
            }
        };
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream) -> ::protobuf::ProtobufResult<()> {
        while !is.eof()? {
            let (field_number, wire_type) = is.read_tag_unpack()?;
            match field_number {
                1 => {
                    ::protobuf::rt::read_repeated_message_into(wire_type, is, &mut self.entries)?;
                },
                2 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_bool()?;
                    self.full = ::std::option::Option::Some(tmp);
                },
                _ => {
                    ::protobuf::rt::read_unknown_or_skip_group(field_number, wire_type, is, self.mut_unknown_fields())?;
                },
            };
        }
        ::std::result::Result::Ok(())
    }

    // Compute sizes of nested messages
    #[allow(unused_variables)]
    fn compute_size(&self) -> u32 {
        let mut my_size = 0;
        for value in &self.entries {
            let len = value.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        };
        if let Some(v) = self.full {
            my_size += 2;
        }
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        for v in &self.entries {
            os.write_tag(1, ::protobuf::wire_format::WireTypeLengthDelimited)?;
            os.write_raw_varint32(v.get_cached_size())?;
            v.write_to_with_cached_sizes(os)?;
        };
        if let Some(v) = self.full {
            os.write_bool(2, v)?;
        }
        os.write_unknown_fields(self.get_unknown_fields())?;
        ::std::result::Result::Ok(())
    }

    fn get_cached_size(&self) -> u32 {
        self.cached_size.get()
    }

    fn get_unknown_fields(&self) -> &::protobuf::UnknownFields {
        &self.unknown_fields
    }

    fn mut_unknown_fields(&mut self) -> &mut ::protobuf::UnknownFields {
        &mut self.unknown_fields
    }

    fn as_any(&self) -> &::std::any::Any {
        self as &::std::any::Any
    }
    fn as_any_mut(&mut self) -> &mut ::std::any::Any {
        self as &mut ::std::any::Any
    }
    fn into_any(self: Box<Self>) -> ::std::boxed::Box<::std::any::Any> {
        self
    }

    fn descriptor(&self) -> &'static ::protobuf::reflect::MessageDescriptor {
        ::protobuf::MessageStatic::descriptor_static(None::<Self>)
    }
}

impl ::protobuf::MessageStatic for Message_Wantlist {
    fn new() -> Message_Wantlist {
        Message_Wantlist::new()
    }

    fn descriptor_static(_: ::std::option::Option<Message_Wantlist>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_repeated_field_accessor::<_, ::protobuf::types::ProtobufTypeMessage<Message_Wantlist_Entry>>(
                    "entries",
                    Message_Wantlist::get_entries_for_reflect,
                    Message_Wantlist::mut_entries_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeBool>(
                    "full",
                    Message_Wantlist::get_full_for_reflect,
                    Message_Wantlist::mut_full_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<Message_Wantlist>(
                    "Message_Wantlist",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for Message_Wantlist {
    fn clear(&mut self) {
        self.clear_entries();
        self.clear_full();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for Message_Wantlist {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for Message_Wantlist {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct Message_Wantlist_Entry {
    // message fields
    block: ::protobuf::SingularField<::std::string::String>,
    priority: ::std::option::Option<i32>,
    cancel: ::std::option::Option<bool>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for Message_Wantlist_Entry {}

impl Message_Wantlist_Entry {
    pub fn new() -> Message_Wantlist_Entry {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static Message_Wantlist_Entry {
        static mut instance: ::protobuf::lazy::Lazy<Message_Wantlist_Entry> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const Message_Wantlist_Entry,
        };
        unsafe {
            instance.get(Message_Wantlist_Entry::new)
        }
    }

    // optional string block = 1;

    pub fn clear_block(&mut self) {
        self.block.clear();
    }

    pub fn has_block(&self) -> bool {
        self.block.is_some()
    }

    // Param is passed by value, moved
    pub fn set_block(&mut self, v: ::std::string::String) {
        self.block = ::protobuf::SingularField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_block(&mut self) -> &mut ::std::string::String {
        if self.block.is_none() {
            self.block.set_default();
        }
        self.block.as_mut().unwrap()
    }

    // Take field
    pub fn take_block(&mut self) -> ::std::string::String {
        self.block.take().unwrap_or_else(|| ::std::string::String::new())
    }

    pub fn get_block(&self) -> &str {
        match self.block.as_ref() {
            Some(v) => &v,
            None => "",
        }
    }

    fn get_block_for_reflect(&self) -> &::protobuf::SingularField<::std::string::String> {
        &self.block
    }

    fn mut_block_for_reflect(&mut self) -> &mut ::protobuf::SingularField<::std::string::String> {
        &mut self.block
    }

    // optional int32 priority = 2;

    pub fn clear_priority(&mut self) {
        self.priority = ::std::option::Option::None;
    }

    pub fn has_priority(&self) -> bool {
        self.priority.is_some()
    }

    // Param is passed by value, moved
    pub fn set_priority(&mut self, v: i32) {
        self.priority = ::std::option::Option::Some(v);
    }

    pub fn get_priority(&self) -> i32 {
        self.priority.unwrap_or(0)
    }

    fn get_priority_for_reflect(&self) -> &::std::option::Option<i32> {
        &self.priority
    }

    fn mut_priority_for_reflect(&mut self) -> &mut ::std::option::Option<i32> {
        &mut self.priority
    }

    // optional bool cancel = 3;

    pub fn clear_cancel(&mut self) {
        self.cancel = ::std::option::Option::None;
    }

    pub fn has_cancel(&self) -> bool {
        self.cancel.is_some()
    }

    // Param is passed by value, moved
    pub fn set_cancel(&mut self, v: bool) {
        self.cancel = ::std::option::Option::Some(v);
    }

    pub fn get_cancel(&self) -> bool {
        self.cancel.unwrap_or(false)
    }

    fn get_cancel_for_reflect(&self) -> &::std::option::Option<bool> {
        &self.cancel
    }

    fn mut_cancel_for_reflect(&mut self) -> &mut ::std::option::Option<bool> {
        &mut self.cancel
    }
}

impl ::protobuf::Message for Message_Wantlist_Entry {
    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream) -> ::protobuf::ProtobufResult<()> {
        while !is.eof()? {
            let (field_number, wire_type) = is.read_tag_unpack()?;
            match field_number {
                1 => {
                    ::protobuf::rt::read_singular_string_into(wire_type, is, &mut self.block)?;
                },
                2 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_int32()?;
                    self.priority = ::std::option::Option::Some(tmp);
                },
                3 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_bool()?;
                    self.cancel = ::std::option::Option::Some(tmp);
                },
                _ => {
                    ::protobuf::rt::read_unknown_or_skip_group(field_number, wire_type, is, self.mut_unknown_fields())?;
                },
            };
        }
        ::std::result::Result::Ok(())
    }

    // Compute sizes of nested messages
    #[allow(unused_variables)]
    fn compute_size(&self) -> u32 {
        let mut my_size = 0;
        if let Some(ref v) = self.block.as_ref() {
            my_size += ::protobuf::rt::string_size(1, &v);
        }
        if let Some(v) = self.priority {
            my_size += ::protobuf::rt::value_size(2, v, ::protobuf::wire_format::WireTypeVarint);
        }
        if let Some(v) = self.cancel {
            my_size += 2;
        }
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if let Some(ref v) = self.block.as_ref() {
            os.write_string(1, &v)?;
        }
        if let Some(v) = self.priority {
            os.write_int32(2, v)?;
        }
        if let Some(v) = self.cancel {
            os.write_bool(3, v)?;
        }
        os.write_unknown_fields(self.get_unknown_fields())?;
        ::std::result::Result::Ok(())
    }

    fn get_cached_size(&self) -> u32 {
        self.cached_size.get()
    }

    fn get_unknown_fields(&self) -> &::protobuf::UnknownFields {
        &self.unknown_fields
    }

    fn mut_unknown_fields(&mut self) -> &mut ::protobuf::UnknownFields {
        &mut self.unknown_fields
    }

    fn as_any(&self) -> &::std::any::Any {
        self as &::std::any::Any
    }
    fn as_any_mut(&mut self) -> &mut ::std::any::Any {
        self as &mut ::std::any::Any
    }
    fn into_any(self: Box<Self>) -> ::std::boxed::Box<::std::any::Any> {
        self
    }

    fn descriptor(&self) -> &'static ::protobuf::reflect::MessageDescriptor {
        ::protobuf::MessageStatic::descriptor_static(None::<Self>)
    }
}

impl ::protobuf::MessageStatic for Message_Wantlist_Entry {
    fn new() -> Message_Wantlist_Entry {
        Message_Wantlist_Entry::new()
    }

    fn descriptor_static(_: ::std::option::Option<Message_Wantlist_Entry>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_singular_field_accessor::<_, ::protobuf::types::ProtobufTypeString>(
                    "block",
                    Message_Wantlist_Entry::get_block_for_reflect,
                    Message_Wantlist_Entry::mut_block_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeInt32>(
                    "priority",
                    Message_Wantlist_Entry::get_priority_for_reflect,
                    Message_Wantlist_Entry::mut_priority_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeBool>(
                    "cancel",
                    Message_Wantlist_Entry::get_cancel_for_reflect,
                    Message_Wantlist_Entry::mut_cancel_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<Message_Wantlist_Entry>(
                    "Message_Wantlist_Entry",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for Message_Wantlist_Entry {
    fn clear(&mut self) {
        self.clear_block();
        self.clear_priority();
        self.clear_cancel();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for Message_Wantlist_Entry {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for Message_Wantlist_Entry {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct Message_Block {
    // message fields
    prefix: ::protobuf::SingularField<::std::vec::Vec<u8>>,
    data: ::protobuf::SingularField<::std::vec::Vec<u8>>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for Message_Block {}

impl Message_Block {
    pub fn new() -> Message_Block {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static Message_Block {
        static mut instance: ::protobuf::lazy::Lazy<Message_Block> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const Message_Block,
        };
        unsafe {
            instance.get(Message_Block::new)
        }
    }

    // optional bytes prefix = 1;

    pub fn clear_prefix(&mut self) {
        self.prefix.clear();
    }

    pub fn has_prefix(&self) -> bool {
        self.prefix.is_some()
    }

    // Param is passed by value, moved
    pub fn set_prefix(&mut self, v: ::std::vec::Vec<u8>) {
        self.prefix = ::protobuf::SingularField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_prefix(&mut self) -> &mut ::std::vec::Vec<u8> {
        if self.prefix.is_none() {
            self.prefix.set_default();
        }
        self.prefix.as_mut().unwrap()
    }

    // Take field
    pub fn take_prefix(&mut self) -> ::std::vec::Vec<u8> {
        self.prefix.take().unwrap_or_else(|| ::std::vec::Vec::new())
    }

    pub fn get_prefix(&self) -> &[u8] {
        match self.prefix.as_ref() {
            Some(v) => &v,
            None => &[],
        }
    }

    fn get_prefix_for_reflect(&self) -> &::protobuf::SingularField<::std::vec::Vec<u8>> {
        &self.prefix
    }

    fn mut_prefix_for_reflect(&mut self) -> &mut ::protobuf::SingularField<::std::vec::Vec<u8>> {
        &mut self.prefix
    }

    // optional bytes data = 2;

    pub fn clear_data(&mut self) {
        self.data.clear();
    }

    pub fn has_data(&self) -> bool {
        self.data.is_some()
    }

    // Param is passed by value, moved
    pub fn set_data(&mut self, v: ::std::vec::Vec<u8>) {
        self.data = ::protobuf::SingularField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_data(&mut self) -> &mut ::std::vec::Vec<u8> {
        if self.data.is_none() {
            self.data.set_default();
        }
        self.data.as_mut().unwrap()
    }

    // Take field
    pub fn take_data(&mut self) -> ::std::vec::Vec<u8> {
        self.data.take().unwrap_or_else(|| ::std::vec::Vec::new())
    }

    pub fn get_data(&self) -> &[u8] {
        match self.data.as_ref() {
            Some(v) => &v,
            None => &[],
        }
    }

    fn get_data_for_reflect(&self) -> &::protobuf::SingularField<::std::vec::Vec<u8>> {
        &self.data
    }

    fn mut_data_for_reflect(&mut self) -> &mut ::protobuf::SingularField<::std::vec::Vec<u8>> {
        &mut self.data
    }
}

impl ::protobuf::Message for Message_Block {
    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream) -> ::protobuf::ProtobufResult<()> {
        while !is.eof()? {
            let (field_number, wire_type) = is.read_tag_unpack()?;
            match field_number {
                1 => {
                    ::protobuf::rt::read_singular_bytes_into(wire_type, is, &mut self.prefix)?;
                },
                2 => {
                    ::protobuf::rt::read_singular_bytes_into(wire_type, is, &mut self.data)?;
                },
                _ => {
                    ::protobuf::rt::read_unknown_or_skip_group(field_number, wire_type, is, self.mut_unknown_fields())?;
                },
            };
        }
        ::std::result::Result::Ok(())
    }

    // Compute sizes of nested messages
    #[allow(unused_variables)]
    fn compute_size(&self) -> u32 {
        let mut my_size = 0;
        if let Some(ref v) = self.prefix.as_ref() {
            my_size += ::protobuf::rt::bytes_size(1, &v);
        }
        if let Some(ref v) = self.data.as_ref() {
            my_size += ::protobuf::rt::bytes_size(2, &v);
        }
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if let Some(ref v) = self.prefix.as_ref() {
            os.write_bytes(1, &v)?;
        }
        if let Some(ref v) = self.data.as_ref() {
            os.write_bytes(2, &v)?;
        }
        os.write_unknown_fields(self.get_unknown_fields())?;
        ::std::result::Result::Ok(())
    }

    fn get_cached_size(&self) -> u32 {
        self.cached_size.get()
    }

    fn get_unknown_fields(&self) -> &::protobuf::UnknownFields {
        &self.unknown_fields
    }

    fn mut_unknown_fields(&mut self) -> &mut ::protobuf::UnknownFields {
        &mut self.unknown_fields
    }

    fn as_any(&self) -> &::std::any::Any {
        self as &::std::any::Any
    }
    fn as_any_mut(&mut self) -> &mut ::std::any::Any {
        self as &mut ::std::any::Any
    }
    fn into_any(self: Box<Self>) -> ::std::boxed::Box<::std::any::Any> {
        self
    }

    fn descriptor(&self) -> &'static ::protobuf::reflect::MessageDescriptor {
        ::protobuf::MessageStatic::descriptor_static(None::<Self>)
    }
}

impl ::protobuf::MessageStatic for Message_Block {
    fn new() -> Message_Block {
        Message_Block::new()
    }

    fn descriptor_static(_: ::std::option::Option<Message_Block>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_singular_field_accessor::<_, ::protobuf::types::ProtobufTypeBytes>(
                    "prefix",
                    Message_Block::get_prefix_for_reflect,
                    Message_Block::mut_prefix_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_field_accessor::<_, ::protobuf::types::ProtobufTypeBytes>(
                    "data",
                    Message_Block::get_data_for_reflect,
                    Message_Block::mut_data_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<Message_Block>(
                    "Message_Block",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for Message_Block {
    fn clear(&mut self) {
        self.clear_prefix();
        self.clear_data();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for Message_Block {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for Message_Block {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

static file_descriptor_proto_data: &'static [u8] = b"\
    \n\x15bitswap_message.proto\x12\x12bitswap.message.pb\"\x8f\x03\n\x07Mes\
    sage\x12@\n\x08wantlist\x18\x01\x20\x01(\x0b2$.bitswap.message.pb.Messag\
    e.WantlistR\x08wantlist\x12\x16\n\x06blocks\x18\x02\x20\x03(\x0cR\x06blo\
    cks\x12;\n\x07payload\x18\x03\x20\x03(\x0b2!.bitswap.message.pb.Message.\
    BlockR\x07payload\x1a\xb7\x01\n\x08Wantlist\x12D\n\x07entries\x18\x01\
    \x20\x03(\x0b2*.bitswap.message.pb.Message.Wantlist.EntryR\x07entries\
    \x12\x12\n\x04full\x18\x02\x20\x01(\x08R\x04full\x1aQ\n\x05Entry\x12\x14\
    \n\x05block\x18\x01\x20\x01(\tR\x05block\x12\x1a\n\x08priority\x18\x02\
    \x20\x01(\x05R\x08priority\x12\x16\n\x06cancel\x18\x03\x20\x01(\x08R\x06\
    cancel\x1a3\n\x05Block\x12\x16\n\x06prefix\x18\x01\x20\x01(\x0cR\x06pref\
    ix\x12\x12\n\x04data\x18\x02\x20\x01(\x0cR\x04dataJ\x97\n\n\x06\x12\x04\
    \0\0\x18\x01\n\x08\n\x01\x02\x12\x03\0\x08\x1a\n\n\n\x02\x04\0\x12\x04\
    \x02\0\x18\x01\n\n\n\x03\x04\0\x01\x12\x03\x02\x08\x0f\n\x0c\n\x04\x04\0\
    \x03\0\x12\x04\x04\x02\x0e\x03\n\x0c\n\x05\x04\0\x03\0\x01\x12\x03\x04\n\
    \x12\n\x0e\n\x06\x04\0\x03\0\x03\0\x12\x04\x06\x04\n\x05\n\x0e\n\x07\x04\
    \0\x03\0\x03\0\x01\x12\x03\x06\x0c\x11\nQ\n\x08\x04\0\x03\0\x03\0\x02\0\
    \x12\x03\x07\x06\x20\"@\x20the\x20block\x20cid\x20(cidV0\x20in\x20bitswa\
    p\x201.0.0,\x20cidV1\x20in\x20bitswap\x201.1.0)\n\n\x10\n\t\x04\0\x03\0\
    \x03\0\x02\0\x04\x12\x03\x07\x06\x0e\n\x10\n\t\x04\0\x03\0\x03\0\x02\0\
    \x05\x12\x03\x07\x0f\x15\n\x10\n\t\x04\0\x03\0\x03\0\x02\0\x01\x12\x03\
    \x07\x16\x1b\n\x10\n\t\x04\0\x03\0\x03\0\x02\0\x03\x12\x03\x07\x1e\x1f\n\
    :\n\x08\x04\0\x03\0\x03\0\x02\x01\x12\x03\x08\x06\"\")\x20the\x20priorit\
    y\x20(normalized).\x20default\x20to\x201\n\n\x10\n\t\x04\0\x03\0\x03\0\
    \x02\x01\x04\x12\x03\x08\x06\x0e\n\x10\n\t\x04\0\x03\0\x03\0\x02\x01\x05\
    \x12\x03\x08\x0f\x14\n\x10\n\t\x04\0\x03\0\x03\0\x02\x01\x01\x12\x03\x08\
    \x15\x1d\n\x10\n\t\x04\0\x03\0\x03\0\x02\x01\x03\x12\x03\x08\x20!\n0\n\
    \x08\x04\0\x03\0\x03\0\x02\x02\x12\x03\t\x06\x1f\"\x1f\x20whether\x20thi\
    s\x20revokes\x20an\x20entry\n\n\x10\n\t\x04\0\x03\0\x03\0\x02\x02\x04\
    \x12\x03\t\x06\x0e\n\x10\n\t\x04\0\x03\0\x03\0\x02\x02\x05\x12\x03\t\x0f\
    \x13\n\x10\n\t\x04\0\x03\0\x03\0\x02\x02\x01\x12\x03\t\x14\x1a\n\x10\n\t\
    \x04\0\x03\0\x03\0\x02\x02\x03\x12\x03\t\x1d\x1e\n+\n\x06\x04\0\x03\0\
    \x02\0\x12\x03\x0c\x04\x1f\"\x1c\x20a\x20list\x20of\x20wantlist\x20entri\
    es\n\n\x0e\n\x07\x04\0\x03\0\x02\0\x04\x12\x03\x0c\x04\x0c\n\x0e\n\x07\
    \x04\0\x03\0\x02\0\x06\x12\x03\x0c\r\x12\n\x0e\n\x07\x04\0\x03\0\x02\0\
    \x01\x12\x03\x0c\x13\x1a\n\x0e\n\x07\x04\0\x03\0\x02\0\x03\x12\x03\x0c\
    \x1d\x1e\nD\n\x06\x04\0\x03\0\x02\x01\x12\x03\r\x04\x1b\"5\x20whether\
    \x20this\x20is\x20the\x20full\x20wantlist.\x20default\x20to\x20false\n\n\
    \x0e\n\x07\x04\0\x03\0\x02\x01\x04\x12\x03\r\x04\x0c\n\x0e\n\x07\x04\0\
    \x03\0\x02\x01\x05\x12\x03\r\r\x11\n\x0e\n\x07\x04\0\x03\0\x02\x01\x01\
    \x12\x03\r\x12\x16\n\x0e\n\x07\x04\0\x03\0\x02\x01\x03\x12\x03\r\x19\x1a\
    \n\x0c\n\x04\x04\0\x03\x01\x12\x04\x10\x02\x13\x03\n\x0c\n\x05\x04\0\x03\
    \x01\x01\x12\x03\x10\n\x0f\nY\n\x06\x04\0\x03\x01\x02\0\x12\x03\x11\x04\
    \x1e\"J\x20CID\x20prefix\x20(cid\x20version,\x20multicodec\x20and\x20mul\
    tihash\x20prefix\x20(type\x20+\x20length)\n\n\x0e\n\x07\x04\0\x03\x01\
    \x02\0\x04\x12\x03\x11\x04\x0c\n\x0e\n\x07\x04\0\x03\x01\x02\0\x05\x12\
    \x03\x11\r\x12\n\x0e\n\x07\x04\0\x03\x01\x02\0\x01\x12\x03\x11\x13\x19\n\
    \x0e\n\x07\x04\0\x03\x01\x02\0\x03\x12\x03\x11\x1c\x1d\n\r\n\x06\x04\0\
    \x03\x01\x02\x01\x12\x03\x12\x04\x1c\n\x0e\n\x07\x04\0\x03\x01\x02\x01\
    \x04\x12\x03\x12\x04\x0c\n\x0e\n\x07\x04\0\x03\x01\x02\x01\x05\x12\x03\
    \x12\r\x12\n\x0e\n\x07\x04\0\x03\x01\x02\x01\x01\x12\x03\x12\x13\x17\n\
    \x0e\n\x07\x04\0\x03\x01\x02\x01\x03\x12\x03\x12\x1a\x1b\n\x0b\n\x04\x04\
    \0\x02\0\x12\x03\x15\x02!\n\x0c\n\x05\x04\0\x02\0\x04\x12\x03\x15\x02\n\
    \n\x0c\n\x05\x04\0\x02\0\x06\x12\x03\x15\x0b\x13\n\x0c\n\x05\x04\0\x02\0\
    \x01\x12\x03\x15\x14\x1c\n\x0c\n\x05\x04\0\x02\0\x03\x12\x03\x15\x1f\x20\
    \n3\n\x04\x04\0\x02\x01\x12\x03\x16\x02\x1c\"&\x20used\x20to\x20send\x20\
    Blocks\x20in\x20bitswap\x201.0.0\n\n\x0c\n\x05\x04\0\x02\x01\x04\x12\x03\
    \x16\x02\n\n\x0c\n\x05\x04\0\x02\x01\x05\x12\x03\x16\x0b\x10\n\x0c\n\x05\
    \x04\0\x02\x01\x01\x12\x03\x16\x11\x17\n\x0c\n\x05\x04\0\x02\x01\x03\x12\
    \x03\x16\x1a\x1b\n3\n\x04\x04\0\x02\x02\x12\x03\x17\x02\x1d\"&\x20used\
    \x20to\x20send\x20Blocks\x20in\x20bitswap\x201.1.0\n\n\x0c\n\x05\x04\0\
    \x02\x02\x04\x12\x03\x17\x02\n\n\x0c\n\x05\x04\0\x02\x02\x06\x12\x03\x17\
    \x0b\x10\n\x0c\n\x05\x04\0\x02\x02\x01\x12\x03\x17\x11\x18\n\x0c\n\x05\
    \x04\0\x02\x02\x03\x12\x03\x17\x1b\x1c\
";

static mut file_descriptor_proto_lazy: ::protobuf::lazy::Lazy<::protobuf::descriptor::FileDescriptorProto> = ::protobuf::lazy::Lazy {
    lock: ::protobuf::lazy::ONCE_INIT,
    ptr: 0 as *const ::protobuf::descriptor::FileDescriptorProto,
};

fn parse_descriptor_proto() -> ::protobuf::descriptor::FileDescriptorProto {
    ::protobuf::parse_from_bytes(file_descriptor_proto_data).unwrap()
}

pub fn file_descriptor_proto() -> &'static ::protobuf::descriptor::FileDescriptorProto {
    unsafe {
        file_descriptor_proto_lazy.get(|| {
            parse_descriptor_proto()
        })
    }
}
