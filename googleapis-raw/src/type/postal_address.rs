// This file is generated by rust-protobuf 3.3.0. Do not edit
// .proto file is parsed by protoc --rust-out=...
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

//! Generated file from `google/type/postal_address.proto`

/// Generated files are compatible only with the same version
/// of protobuf runtime.
const _PROTOBUF_VERSION_CHECK: () = ::protobuf::VERSION_3_3_0;

// @@protoc_insertion_point(message:google.type.PostalAddress)
#[derive(PartialEq,Clone,Default,Debug)]
pub struct PostalAddress {
    // message fields
    ///  The schema revision of the `PostalAddress`. This must be set to 0, which is
    ///  the latest revision.
    ///
    ///  All new revisions **must** be backward compatible with old revisions.
    // @@protoc_insertion_point(field:google.type.PostalAddress.revision)
    pub revision: i32,
    ///  Required. CLDR region code of the country/region of the address. This
    ///  is never inferred and it is up to the user to ensure the value is
    ///  correct. See http://cldr.unicode.org/ and
    ///  http://www.unicode.org/cldr/charts/30/supplemental/territory_information.html
    ///  for details. Example: "CH" for Switzerland.
    // @@protoc_insertion_point(field:google.type.PostalAddress.region_code)
    pub region_code: ::std::string::String,
    ///  Optional. BCP-47 language code of the contents of this address (if
    ///  known). This is often the UI language of the input form or is expected
    ///  to match one of the languages used in the address' country/region, or their
    ///  transliterated equivalents.
    ///  This can affect formatting in certain countries, but is not critical
    ///  to the correctness of the data and will never affect any validation or
    ///  other non-formatting related operations.
    ///
    ///  If this value is not known, it should be omitted (rather than specifying a
    ///  possibly incorrect default).
    ///
    ///  Examples: "zh-Hant", "ja", "ja-Latn", "en".
    // @@protoc_insertion_point(field:google.type.PostalAddress.language_code)
    pub language_code: ::std::string::String,
    ///  Optional. Postal code of the address. Not all countries use or require
    ///  postal codes to be present, but where they are used, they may trigger
    ///  additional validation with other parts of the address (e.g. state/zip
    ///  validation in the U.S.A.).
    // @@protoc_insertion_point(field:google.type.PostalAddress.postal_code)
    pub postal_code: ::std::string::String,
    ///  Optional. Additional, country-specific, sorting code. This is not used
    ///  in most regions. Where it is used, the value is either a string like
    ///  "CEDEX", optionally followed by a number (e.g. "CEDEX 7"), or just a number
    ///  alone, representing the "sector code" (Jamaica), "delivery area indicator"
    ///  (Malawi) or "post office indicator" (e.g. Côte d'Ivoire).
    // @@protoc_insertion_point(field:google.type.PostalAddress.sorting_code)
    pub sorting_code: ::std::string::String,
    ///  Optional. Highest administrative subdivision which is used for postal
    ///  addresses of a country or region.
    ///  For example, this can be a state, a province, an oblast, or a prefecture.
    ///  Specifically, for Spain this is the province and not the autonomous
    ///  community (e.g. "Barcelona" and not "Catalonia").
    ///  Many countries don't use an administrative area in postal addresses. E.g.
    ///  in Switzerland this should be left unpopulated.
    // @@protoc_insertion_point(field:google.type.PostalAddress.administrative_area)
    pub administrative_area: ::std::string::String,
    ///  Optional. Generally refers to the city/town portion of the address.
    ///  Examples: US city, IT comune, UK post town.
    ///  In regions of the world where localities are not well defined or do not fit
    ///  into this structure well, leave locality empty and use address_lines.
    // @@protoc_insertion_point(field:google.type.PostalAddress.locality)
    pub locality: ::std::string::String,
    ///  Optional. Sublocality of the address.
    ///  For example, this can be neighborhoods, boroughs, districts.
    // @@protoc_insertion_point(field:google.type.PostalAddress.sublocality)
    pub sublocality: ::std::string::String,
    ///  Unstructured address lines describing the lower levels of an address.
    ///
    ///  Because values in address_lines do not have type information and may
    ///  sometimes contain multiple values in a single field (e.g.
    ///  "Austin, TX"), it is important that the line order is clear. The order of
    ///  address lines should be "envelope order" for the country/region of the
    ///  address. In places where this can vary (e.g. Japan), address_language is
    ///  used to make it explicit (e.g. "ja" for large-to-small ordering and
    ///  "ja-Latn" or "en" for small-to-large). This way, the most specific line of
    ///  an address can be selected based on the language.
    ///
    ///  The minimum permitted structural representation of an address consists
    ///  of a region_code with all remaining information placed in the
    ///  address_lines. It would be possible to format such an address very
    ///  approximately without geocoding, but no semantic reasoning could be
    ///  made about any of the address components until it was at least
    ///  partially resolved.
    ///
    ///  Creating an address only containing a region_code and address_lines, and
    ///  then geocoding is the recommended way to handle completely unstructured
    ///  addresses (as opposed to guessing which parts of the address should be
    ///  localities or administrative areas).
    // @@protoc_insertion_point(field:google.type.PostalAddress.address_lines)
    pub address_lines: ::std::vec::Vec<::std::string::String>,
    ///  Optional. The recipient at the address.
    ///  This field may, under certain circumstances, contain multiline information.
    ///  For example, it might contain "care of" information.
    // @@protoc_insertion_point(field:google.type.PostalAddress.recipients)
    pub recipients: ::std::vec::Vec<::std::string::String>,
    ///  Optional. The name of the organization at the address.
    // @@protoc_insertion_point(field:google.type.PostalAddress.organization)
    pub organization: ::std::string::String,
    // special fields
    // @@protoc_insertion_point(special_field:google.type.PostalAddress.special_fields)
    pub special_fields: ::protobuf::SpecialFields,
}

impl<'a> ::std::default::Default for &'a PostalAddress {
    fn default() -> &'a PostalAddress {
        <PostalAddress as ::protobuf::Message>::default_instance()
    }
}

impl PostalAddress {
    pub fn new() -> PostalAddress {
        ::std::default::Default::default()
    }

    fn generated_message_descriptor_data() -> ::protobuf::reflect::GeneratedMessageDescriptorData {
        let mut fields = ::std::vec::Vec::with_capacity(11);
        let mut oneofs = ::std::vec::Vec::with_capacity(0);
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "revision",
            |m: &PostalAddress| { &m.revision },
            |m: &mut PostalAddress| { &mut m.revision },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "region_code",
            |m: &PostalAddress| { &m.region_code },
            |m: &mut PostalAddress| { &mut m.region_code },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "language_code",
            |m: &PostalAddress| { &m.language_code },
            |m: &mut PostalAddress| { &mut m.language_code },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "postal_code",
            |m: &PostalAddress| { &m.postal_code },
            |m: &mut PostalAddress| { &mut m.postal_code },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "sorting_code",
            |m: &PostalAddress| { &m.sorting_code },
            |m: &mut PostalAddress| { &mut m.sorting_code },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "administrative_area",
            |m: &PostalAddress| { &m.administrative_area },
            |m: &mut PostalAddress| { &mut m.administrative_area },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "locality",
            |m: &PostalAddress| { &m.locality },
            |m: &mut PostalAddress| { &mut m.locality },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "sublocality",
            |m: &PostalAddress| { &m.sublocality },
            |m: &mut PostalAddress| { &mut m.sublocality },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_vec_simpler_accessor::<_, _>(
            "address_lines",
            |m: &PostalAddress| { &m.address_lines },
            |m: &mut PostalAddress| { &mut m.address_lines },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_vec_simpler_accessor::<_, _>(
            "recipients",
            |m: &PostalAddress| { &m.recipients },
            |m: &mut PostalAddress| { &mut m.recipients },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "organization",
            |m: &PostalAddress| { &m.organization },
            |m: &mut PostalAddress| { &mut m.organization },
        ));
        ::protobuf::reflect::GeneratedMessageDescriptorData::new_2::<PostalAddress>(
            "PostalAddress",
            fields,
            oneofs,
        )
    }
}

impl ::protobuf::Message for PostalAddress {
    const NAME: &'static str = "PostalAddress";

    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream<'_>) -> ::protobuf::Result<()> {
        while let Some(tag) = is.read_raw_tag_or_eof()? {
            match tag {
                8 => {
                    self.revision = is.read_int32()?;
                },
                18 => {
                    self.region_code = is.read_string()?;
                },
                26 => {
                    self.language_code = is.read_string()?;
                },
                34 => {
                    self.postal_code = is.read_string()?;
                },
                42 => {
                    self.sorting_code = is.read_string()?;
                },
                50 => {
                    self.administrative_area = is.read_string()?;
                },
                58 => {
                    self.locality = is.read_string()?;
                },
                66 => {
                    self.sublocality = is.read_string()?;
                },
                74 => {
                    self.address_lines.push(is.read_string()?);
                },
                82 => {
                    self.recipients.push(is.read_string()?);
                },
                90 => {
                    self.organization = is.read_string()?;
                },
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
        if self.revision != 0 {
            my_size += ::protobuf::rt::int32_size(1, self.revision);
        }
        if !self.region_code.is_empty() {
            my_size += ::protobuf::rt::string_size(2, &self.region_code);
        }
        if !self.language_code.is_empty() {
            my_size += ::protobuf::rt::string_size(3, &self.language_code);
        }
        if !self.postal_code.is_empty() {
            my_size += ::protobuf::rt::string_size(4, &self.postal_code);
        }
        if !self.sorting_code.is_empty() {
            my_size += ::protobuf::rt::string_size(5, &self.sorting_code);
        }
        if !self.administrative_area.is_empty() {
            my_size += ::protobuf::rt::string_size(6, &self.administrative_area);
        }
        if !self.locality.is_empty() {
            my_size += ::protobuf::rt::string_size(7, &self.locality);
        }
        if !self.sublocality.is_empty() {
            my_size += ::protobuf::rt::string_size(8, &self.sublocality);
        }
        for value in &self.address_lines {
            my_size += ::protobuf::rt::string_size(9, &value);
        };
        for value in &self.recipients {
            my_size += ::protobuf::rt::string_size(10, &value);
        };
        if !self.organization.is_empty() {
            my_size += ::protobuf::rt::string_size(11, &self.organization);
        }
        my_size += ::protobuf::rt::unknown_fields_size(self.special_fields.unknown_fields());
        self.special_fields.cached_size().set(my_size as u32);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream<'_>) -> ::protobuf::Result<()> {
        if self.revision != 0 {
            os.write_int32(1, self.revision)?;
        }
        if !self.region_code.is_empty() {
            os.write_string(2, &self.region_code)?;
        }
        if !self.language_code.is_empty() {
            os.write_string(3, &self.language_code)?;
        }
        if !self.postal_code.is_empty() {
            os.write_string(4, &self.postal_code)?;
        }
        if !self.sorting_code.is_empty() {
            os.write_string(5, &self.sorting_code)?;
        }
        if !self.administrative_area.is_empty() {
            os.write_string(6, &self.administrative_area)?;
        }
        if !self.locality.is_empty() {
            os.write_string(7, &self.locality)?;
        }
        if !self.sublocality.is_empty() {
            os.write_string(8, &self.sublocality)?;
        }
        for v in &self.address_lines {
            os.write_string(9, &v)?;
        };
        for v in &self.recipients {
            os.write_string(10, &v)?;
        };
        if !self.organization.is_empty() {
            os.write_string(11, &self.organization)?;
        }
        os.write_unknown_fields(self.special_fields.unknown_fields())?;
        ::std::result::Result::Ok(())
    }

    fn special_fields(&self) -> &::protobuf::SpecialFields {
        &self.special_fields
    }

    fn mut_special_fields(&mut self) -> &mut ::protobuf::SpecialFields {
        &mut self.special_fields
    }

    fn new() -> PostalAddress {
        PostalAddress::new()
    }

    fn clear(&mut self) {
        self.revision = 0;
        self.region_code.clear();
        self.language_code.clear();
        self.postal_code.clear();
        self.sorting_code.clear();
        self.administrative_area.clear();
        self.locality.clear();
        self.sublocality.clear();
        self.address_lines.clear();
        self.recipients.clear();
        self.organization.clear();
        self.special_fields.clear();
    }

    fn default_instance() -> &'static PostalAddress {
        static instance: PostalAddress = PostalAddress {
            revision: 0,
            region_code: ::std::string::String::new(),
            language_code: ::std::string::String::new(),
            postal_code: ::std::string::String::new(),
            sorting_code: ::std::string::String::new(),
            administrative_area: ::std::string::String::new(),
            locality: ::std::string::String::new(),
            sublocality: ::std::string::String::new(),
            address_lines: ::std::vec::Vec::new(),
            recipients: ::std::vec::Vec::new(),
            organization: ::std::string::String::new(),
            special_fields: ::protobuf::SpecialFields::new(),
        };
        &instance
    }
}

impl ::protobuf::MessageFull for PostalAddress {
    fn descriptor() -> ::protobuf::reflect::MessageDescriptor {
        static descriptor: ::protobuf::rt::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::rt::Lazy::new();
        descriptor.get(|| file_descriptor().message_by_package_relative_name("PostalAddress").unwrap()).clone()
    }
}

impl ::std::fmt::Display for PostalAddress {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for PostalAddress {
    type RuntimeType = ::protobuf::reflect::rt::RuntimeTypeMessage<Self>;
}

static file_descriptor_proto_data: &'static [u8] = b"\
    \n\x20google/type/postal_address.proto\x12\x0bgoogle.type\"\x8d\x03\n\rP\
    ostalAddress\x12\x1a\n\x08revision\x18\x01\x20\x01(\x05R\x08revision\x12\
    \x1f\n\x0bregion_code\x18\x02\x20\x01(\tR\nregionCode\x12#\n\rlanguage_c\
    ode\x18\x03\x20\x01(\tR\x0clanguageCode\x12\x1f\n\x0bpostal_code\x18\x04\
    \x20\x01(\tR\npostalCode\x12!\n\x0csorting_code\x18\x05\x20\x01(\tR\x0bs\
    ortingCode\x12/\n\x13administrative_area\x18\x06\x20\x01(\tR\x12administ\
    rativeArea\x12\x1a\n\x08locality\x18\x07\x20\x01(\tR\x08locality\x12\x20\
    \n\x0bsublocality\x18\x08\x20\x01(\tR\x0bsublocality\x12#\n\raddress_lin\
    es\x18\t\x20\x03(\tR\x0caddressLines\x12\x1e\n\nrecipients\x18\n\x20\x03\
    (\tR\nrecipients\x12\"\n\x0corganization\x18\x0b\x20\x01(\tR\x0corganiza\
    tionBx\n\x0fcom.google.typeB\x12PostalAddressProtoP\x01ZFgoogle.golang.o\
    rg/genproto/googleapis/type/postaladdress;postaladdress\xf8\x01\x01\xa2\
    \x02\x03GTPJ\xcb/\n\x07\x12\x05\x0f\0\x86\x01\x01\n\xbe\x04\n\x01\x0c\
    \x12\x03\x0f\0\x122\xb3\x04\x20Copyright\x202019\x20Google\x20LLC.\n\n\
    \x20Licensed\x20under\x20the\x20Apache\x20License,\x20Version\x202.0\x20\
    (the\x20\"License\");\n\x20you\x20may\x20not\x20use\x20this\x20file\x20e\
    xcept\x20in\x20compliance\x20with\x20the\x20License.\n\x20You\x20may\x20\
    obtain\x20a\x20copy\x20of\x20the\x20License\x20at\n\n\x20\x20\x20\x20\
    \x20http://www.apache.org/licenses/LICENSE-2.0\n\n\x20Unless\x20required\
    \x20by\x20applicable\x20law\x20or\x20agreed\x20to\x20in\x20writing,\x20s\
    oftware\n\x20distributed\x20under\x20the\x20License\x20is\x20distributed\
    \x20on\x20an\x20\"AS\x20IS\"\x20BASIS,\n\x20WITHOUT\x20WARRANTIES\x20OR\
    \x20CONDITIONS\x20OF\x20ANY\x20KIND,\x20either\x20express\x20or\x20impli\
    ed.\n\x20See\x20the\x20License\x20for\x20the\x20specific\x20language\x20\
    governing\x20permissions\x20and\n\x20limitations\x20under\x20the\x20Lice\
    nse.\n\n\n\x08\n\x01\x02\x12\x03\x11\0\x14\n\x08\n\x01\x08\x12\x03\x13\0\
    \x1f\n\t\n\x02\x08\x1f\x12\x03\x13\0\x1f\n\x08\n\x01\x08\x12\x03\x14\0]\
    \n\t\n\x02\x08\x0b\x12\x03\x14\0]\n\x08\n\x01\x08\x12\x03\x15\0\"\n\t\n\
    \x02\x08\n\x12\x03\x15\0\"\n\x08\n\x01\x08\x12\x03\x16\03\n\t\n\x02\x08\
    \x08\x12\x03\x16\03\n\x08\n\x01\x08\x12\x03\x17\0(\n\t\n\x02\x08\x01\x12\
    \x03\x17\0(\n\x08\n\x01\x08\x12\x03\x18\0!\n\t\n\x02\x08$\x12\x03\x18\0!\
    \n\xfe\x05\n\x02\x04\0\x12\x05+\0\x86\x01\x01\x1a\xf0\x05\x20Represents\
    \x20a\x20postal\x20address,\x20e.g.\x20for\x20postal\x20delivery\x20or\
    \x20payments\x20addresses.\n\x20Given\x20a\x20postal\x20address,\x20a\
    \x20postal\x20service\x20can\x20deliver\x20items\x20to\x20a\x20premise,\
    \x20P.O.\n\x20Box\x20or\x20similar.\n\x20It\x20is\x20not\x20intended\x20\
    to\x20model\x20geographical\x20locations\x20(roads,\x20towns,\n\x20mount\
    ains).\n\n\x20In\x20typical\x20usage\x20an\x20address\x20would\x20be\x20\
    created\x20via\x20user\x20input\x20or\x20from\x20importing\n\x20existing\
    \x20data,\x20depending\x20on\x20the\x20type\x20of\x20process.\n\n\x20Adv\
    ice\x20on\x20address\x20input\x20/\x20editing:\n\x20\x20-\x20Use\x20an\
    \x20i18n-ready\x20address\x20widget\x20such\x20as\n\x20\x20\x20\x20https\
    ://github.com/google/libaddressinput)\n\x20-\x20Users\x20should\x20not\
    \x20be\x20presented\x20with\x20UI\x20elements\x20for\x20input\x20or\x20e\
    diting\x20of\n\x20\x20\x20fields\x20outside\x20countries\x20where\x20tha\
    t\x20field\x20is\x20used.\n\n\x20For\x20more\x20guidance\x20on\x20how\
    \x20to\x20use\x20this\x20schema,\x20please\x20see:\n\x20https://support.\
    google.com/business/answer/6397478\n\n\n\n\x03\x04\0\x01\x12\x03+\x08\
    \x15\n\xb9\x01\n\x04\x04\0\x02\0\x12\x030\x02\x15\x1a\xab\x01\x20The\x20\
    schema\x20revision\x20of\x20the\x20`PostalAddress`.\x20This\x20must\x20b\
    e\x20set\x20to\x200,\x20which\x20is\n\x20the\x20latest\x20revision.\n\n\
    \x20All\x20new\x20revisions\x20**must**\x20be\x20backward\x20compatible\
    \x20with\x20old\x20revisions.\n\n\x0c\n\x05\x04\0\x02\0\x05\x12\x030\x02\
    \x07\n\x0c\n\x05\x04\0\x02\0\x01\x12\x030\x08\x10\n\x0c\n\x05\x04\0\x02\
    \0\x03\x12\x030\x13\x14\n\xbf\x02\n\x04\x04\0\x02\x01\x12\x037\x02\x19\
    \x1a\xb1\x02\x20Required.\x20CLDR\x20region\x20code\x20of\x20the\x20coun\
    try/region\x20of\x20the\x20address.\x20This\n\x20is\x20never\x20inferred\
    \x20and\x20it\x20is\x20up\x20to\x20the\x20user\x20to\x20ensure\x20the\
    \x20value\x20is\n\x20correct.\x20See\x20http://cldr.unicode.org/\x20and\
    \n\x20http://www.unicode.org/cldr/charts/30/supplemental/territory_infor\
    mation.html\n\x20for\x20details.\x20Example:\x20\"CH\"\x20for\x20Switzer\
    land.\n\n\x0c\n\x05\x04\0\x02\x01\x05\x12\x037\x02\x08\n\x0c\n\x05\x04\0\
    \x02\x01\x01\x12\x037\t\x14\n\x0c\n\x05\x04\0\x02\x01\x03\x12\x037\x17\
    \x18\n\xd5\x04\n\x04\x04\0\x02\x02\x12\x03E\x02\x1b\x1a\xc7\x04\x20Optio\
    nal.\x20BCP-47\x20language\x20code\x20of\x20the\x20contents\x20of\x20thi\
    s\x20address\x20(if\n\x20known).\x20This\x20is\x20often\x20the\x20UI\x20\
    language\x20of\x20the\x20input\x20form\x20or\x20is\x20expected\n\x20to\
    \x20match\x20one\x20of\x20the\x20languages\x20used\x20in\x20the\x20addre\
    ss'\x20country/region,\x20or\x20their\n\x20transliterated\x20equivalents\
    .\n\x20This\x20can\x20affect\x20formatting\x20in\x20certain\x20countries\
    ,\x20but\x20is\x20not\x20critical\n\x20to\x20the\x20correctness\x20of\
    \x20the\x20data\x20and\x20will\x20never\x20affect\x20any\x20validation\
    \x20or\n\x20other\x20non-formatting\x20related\x20operations.\n\n\x20If\
    \x20this\x20value\x20is\x20not\x20known,\x20it\x20should\x20be\x20omitte\
    d\x20(rather\x20than\x20specifying\x20a\n\x20possibly\x20incorrect\x20de\
    fault).\n\n\x20Examples:\x20\"zh-Hant\",\x20\"ja\",\x20\"ja-Latn\",\x20\
    \"en\".\n\n\x0c\n\x05\x04\0\x02\x02\x05\x12\x03E\x02\x08\n\x0c\n\x05\x04\
    \0\x02\x02\x01\x12\x03E\t\x16\n\x0c\n\x05\x04\0\x02\x02\x03\x12\x03E\x19\
    \x1a\n\x80\x02\n\x04\x04\0\x02\x03\x12\x03K\x02\x19\x1a\xf2\x01\x20Optio\
    nal.\x20Postal\x20code\x20of\x20the\x20address.\x20Not\x20all\x20countri\
    es\x20use\x20or\x20require\n\x20postal\x20codes\x20to\x20be\x20present,\
    \x20but\x20where\x20they\x20are\x20used,\x20they\x20may\x20trigger\n\x20\
    additional\x20validation\x20with\x20other\x20parts\x20of\x20the\x20addre\
    ss\x20(e.g.\x20state/zip\n\x20validation\x20in\x20the\x20U.S.A.).\n\n\
    \x0c\n\x05\x04\0\x02\x03\x05\x12\x03K\x02\x08\n\x0c\n\x05\x04\0\x02\x03\
    \x01\x12\x03K\t\x14\n\x0c\n\x05\x04\0\x02\x03\x03\x12\x03K\x17\x18\n\xf1\
    \x02\n\x04\x04\0\x02\x04\x12\x03R\x02\x1a\x1a\xe3\x02\x20Optional.\x20Ad\
    ditional,\x20country-specific,\x20sorting\x20code.\x20This\x20is\x20not\
    \x20used\n\x20in\x20most\x20regions.\x20Where\x20it\x20is\x20used,\x20th\
    e\x20value\x20is\x20either\x20a\x20string\x20like\n\x20\"CEDEX\",\x20opt\
    ionally\x20followed\x20by\x20a\x20number\x20(e.g.\x20\"CEDEX\x207\"),\
    \x20or\x20just\x20a\x20number\n\x20alone,\x20representing\x20the\x20\"se\
    ctor\x20code\"\x20(Jamaica),\x20\"delivery\x20area\x20indicator\"\n\x20(\
    Malawi)\x20or\x20\"post\x20office\x20indicator\"\x20(e.g.\x20C\xc3\xb4te\
    \x20d'Ivoire).\n\n\x0c\n\x05\x04\0\x02\x04\x05\x12\x03R\x02\x08\n\x0c\n\
    \x05\x04\0\x02\x04\x01\x12\x03R\t\x15\n\x0c\n\x05\x04\0\x02\x04\x03\x12\
    \x03R\x18\x19\n\xb7\x03\n\x04\x04\0\x02\x05\x12\x03[\x02!\x1a\xa9\x03\
    \x20Optional.\x20Highest\x20administrative\x20subdivision\x20which\x20is\
    \x20used\x20for\x20postal\n\x20addresses\x20of\x20a\x20country\x20or\x20\
    region.\n\x20For\x20example,\x20this\x20can\x20be\x20a\x20state,\x20a\
    \x20province,\x20an\x20oblast,\x20or\x20a\x20prefecture.\n\x20Specifical\
    ly,\x20for\x20Spain\x20this\x20is\x20the\x20province\x20and\x20not\x20th\
    e\x20autonomous\n\x20community\x20(e.g.\x20\"Barcelona\"\x20and\x20not\
    \x20\"Catalonia\").\n\x20Many\x20countries\x20don't\x20use\x20an\x20admi\
    nistrative\x20area\x20in\x20postal\x20addresses.\x20E.g.\n\x20in\x20Swit\
    zerland\x20this\x20should\x20be\x20left\x20unpopulated.\n\n\x0c\n\x05\
    \x04\0\x02\x05\x05\x12\x03[\x02\x08\n\x0c\n\x05\x04\0\x02\x05\x01\x12\
    \x03[\t\x1c\n\x0c\n\x05\x04\0\x02\x05\x03\x12\x03[\x1f\x20\n\x94\x02\n\
    \x04\x04\0\x02\x06\x12\x03a\x02\x16\x1a\x86\x02\x20Optional.\x20Generall\
    y\x20refers\x20to\x20the\x20city/town\x20portion\x20of\x20the\x20address\
    .\n\x20Examples:\x20US\x20city,\x20IT\x20comune,\x20UK\x20post\x20town.\
    \n\x20In\x20regions\x20of\x20the\x20world\x20where\x20localities\x20are\
    \x20not\x20well\x20defined\x20or\x20do\x20not\x20fit\n\x20into\x20this\
    \x20structure\x20well,\x20leave\x20locality\x20empty\x20and\x20use\x20ad\
    dress_lines.\n\n\x0c\n\x05\x04\0\x02\x06\x05\x12\x03a\x02\x08\n\x0c\n\
    \x05\x04\0\x02\x06\x01\x12\x03a\t\x11\n\x0c\n\x05\x04\0\x02\x06\x03\x12\
    \x03a\x14\x15\nr\n\x04\x04\0\x02\x07\x12\x03e\x02\x19\x1ae\x20Optional.\
    \x20Sublocality\x20of\x20the\x20address.\n\x20For\x20example,\x20this\
    \x20can\x20be\x20neighborhoods,\x20boroughs,\x20districts.\n\n\x0c\n\x05\
    \x04\0\x02\x07\x05\x12\x03e\x02\x08\n\x0c\n\x05\x04\0\x02\x07\x01\x12\
    \x03e\t\x14\n\x0c\n\x05\x04\0\x02\x07\x03\x12\x03e\x17\x18\n\xe0\t\n\x04\
    \x04\0\x02\x08\x12\x03}\x02$\x1a\xd2\t\x20Unstructured\x20address\x20lin\
    es\x20describing\x20the\x20lower\x20levels\x20of\x20an\x20address.\n\n\
    \x20Because\x20values\x20in\x20address_lines\x20do\x20not\x20have\x20typ\
    e\x20information\x20and\x20may\n\x20sometimes\x20contain\x20multiple\x20\
    values\x20in\x20a\x20single\x20field\x20(e.g.\n\x20\"Austin,\x20TX\"),\
    \x20it\x20is\x20important\x20that\x20the\x20line\x20order\x20is\x20clear\
    .\x20The\x20order\x20of\n\x20address\x20lines\x20should\x20be\x20\"envel\
    ope\x20order\"\x20for\x20the\x20country/region\x20of\x20the\n\x20address\
    .\x20In\x20places\x20where\x20this\x20can\x20vary\x20(e.g.\x20Japan),\
    \x20address_language\x20is\n\x20used\x20to\x20make\x20it\x20explicit\x20\
    (e.g.\x20\"ja\"\x20for\x20large-to-small\x20ordering\x20and\n\x20\"ja-La\
    tn\"\x20or\x20\"en\"\x20for\x20small-to-large).\x20This\x20way,\x20the\
    \x20most\x20specific\x20line\x20of\n\x20an\x20address\x20can\x20be\x20se\
    lected\x20based\x20on\x20the\x20language.\n\n\x20The\x20minimum\x20permi\
    tted\x20structural\x20representation\x20of\x20an\x20address\x20consists\
    \n\x20of\x20a\x20region_code\x20with\x20all\x20remaining\x20information\
    \x20placed\x20in\x20the\n\x20address_lines.\x20It\x20would\x20be\x20poss\
    ible\x20to\x20format\x20such\x20an\x20address\x20very\n\x20approximately\
    \x20without\x20geocoding,\x20but\x20no\x20semantic\x20reasoning\x20could\
    \x20be\n\x20made\x20about\x20any\x20of\x20the\x20address\x20components\
    \x20until\x20it\x20was\x20at\x20least\n\x20partially\x20resolved.\n\n\
    \x20Creating\x20an\x20address\x20only\x20containing\x20a\x20region_code\
    \x20and\x20address_lines,\x20and\n\x20then\x20geocoding\x20is\x20the\x20\
    recommended\x20way\x20to\x20handle\x20completely\x20unstructured\n\x20ad\
    dresses\x20(as\x20opposed\x20to\x20guessing\x20which\x20parts\x20of\x20t\
    he\x20address\x20should\x20be\n\x20localities\x20or\x20administrative\
    \x20areas).\n\n\x0c\n\x05\x04\0\x02\x08\x04\x12\x03}\x02\n\n\x0c\n\x05\
    \x04\0\x02\x08\x05\x12\x03}\x0b\x11\n\x0c\n\x05\x04\0\x02\x08\x01\x12\
    \x03}\x12\x1f\n\x0c\n\x05\x04\0\x02\x08\x03\x12\x03}\"#\n\xbb\x01\n\x04\
    \x04\0\x02\t\x12\x04\x82\x01\x02\"\x1a\xac\x01\x20Optional.\x20The\x20re\
    cipient\x20at\x20the\x20address.\n\x20This\x20field\x20may,\x20under\x20\
    certain\x20circumstances,\x20contain\x20multiline\x20information.\n\x20F\
    or\x20example,\x20it\x20might\x20contain\x20\"care\x20of\"\x20informatio\
    n.\n\n\r\n\x05\x04\0\x02\t\x04\x12\x04\x82\x01\x02\n\n\r\n\x05\x04\0\x02\
    \t\x05\x12\x04\x82\x01\x0b\x11\n\r\n\x05\x04\0\x02\t\x01\x12\x04\x82\x01\
    \x12\x1c\n\r\n\x05\x04\0\x02\t\x03\x12\x04\x82\x01\x1f!\nF\n\x04\x04\0\
    \x02\n\x12\x04\x85\x01\x02\x1b\x1a8\x20Optional.\x20The\x20name\x20of\
    \x20the\x20organization\x20at\x20the\x20address.\n\n\r\n\x05\x04\0\x02\n\
    \x05\x12\x04\x85\x01\x02\x08\n\r\n\x05\x04\0\x02\n\x01\x12\x04\x85\x01\t\
    \x15\n\r\n\x05\x04\0\x02\n\x03\x12\x04\x85\x01\x18\x1ab\x06proto3\
";

/// `FileDescriptorProto` object which was a source for this generated file
fn file_descriptor_proto() -> &'static ::protobuf::descriptor::FileDescriptorProto {
    static file_descriptor_proto_lazy: ::protobuf::rt::Lazy<::protobuf::descriptor::FileDescriptorProto> = ::protobuf::rt::Lazy::new();
    file_descriptor_proto_lazy.get(|| {
        ::protobuf::Message::parse_from_bytes(file_descriptor_proto_data).unwrap()
    })
}

/// `FileDescriptor` object which allows dynamic access to files
pub fn file_descriptor() -> &'static ::protobuf::reflect::FileDescriptor {
    static generated_file_descriptor_lazy: ::protobuf::rt::Lazy<::protobuf::reflect::GeneratedFileDescriptor> = ::protobuf::rt::Lazy::new();
    static file_descriptor: ::protobuf::rt::Lazy<::protobuf::reflect::FileDescriptor> = ::protobuf::rt::Lazy::new();
    file_descriptor.get(|| {
        let generated_file_descriptor = generated_file_descriptor_lazy.get(|| {
            let mut deps = ::std::vec::Vec::with_capacity(0);
            let mut messages = ::std::vec::Vec::with_capacity(1);
            messages.push(PostalAddress::generated_message_descriptor_data());
            let mut enums = ::std::vec::Vec::with_capacity(0);
            ::protobuf::reflect::GeneratedFileDescriptor::new_generated(
                file_descriptor_proto(),
                deps,
                messages,
                enums,
            )
        });
        ::protobuf::reflect::FileDescriptor::new_generated_2(generated_file_descriptor)
    })
}
