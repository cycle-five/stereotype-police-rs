use quote::quote;
use syn::{Data, DeriveInput, Fields, Meta, Path};

use super::ValidatorHandler;
use crate::{
    common::{attributes::base_xx_attribute::BaseXXAttribute, type_enum::TypeEnum},
    panic,
};

pub(crate) struct Base64UrlHandler;

#[derive(Debug)]
pub struct Struct(TypeEnum);

const ITEM: Struct = Struct(TypeEnum::String);

impl ValidatorHandler for Base64UrlHandler {
    fn meta_handler(ast: DeriveInput, meta: Meta) -> syn::Result<proc_macro2::TokenStream> {
        let type_attribute = BaseXXAttribute::build_from_meta(&meta)?;

        if let Data::Struct(data) = ast.data {
            if let Fields::Unnamed(_) = &data.fields {
                if data.fields.len() == 1 {
                    let mut token_stream = proc_macro2::TokenStream::new();

                    let name = ast.ident;

                    let error_path: Path =
                        syn::parse2(quote! { validators_prelude::Base64UrlError }).unwrap();

                    #[cfg(feature = "test")]
                    {
                        let v_padding = type_attribute.padding;

                        token_stream.extend(quote! {
                            impl #name {
                                pub(crate) const V_PADDING: validators_prelude::TriAllow = #v_padding;
                            }
                        });
                    }

                    let check_last_length = if type_attribute.padding.must() {
                        quote! {
                            if last_length != 4 {
                                return Err(#error_path::PaddingMust);
                            }
                        }
                    } else {
                        quote! {}
                    };

                    let handle_padding = if type_attribute.padding.disallow() {
                        quote! {
                            return Err(#error_path::PaddingDisallow);
                        }
                    } else {
                        quote! {
                            match p {
                                2 | 3 => {
                                    if last_length != 4 {
                                        // has padding
                                        return Err(#error_path::Invalid);
                                    }

                                    for e in last_bytes[p + 1..].iter().copied() {
                                        if e != b'=' {
                                            return Err(#error_path::Invalid);
                                        }
                                    }

                                    return Ok(());
                                }
                                _ => return Err(#error_path::Invalid),
                            }
                        }
                    };

                    token_stream.extend(quote! {
                        impl #name {
                            #[inline]
                            fn v_parse_str(s: &str) -> Result<(), #error_path> {
                                Self::v_parse_u8_slice(s.as_bytes())
                            }

                            fn v_parse_u8_slice(v: &[u8]) -> Result<(), #error_path> {
                                let length = v.len();

                                if length == 0 {
                                    return Err(#error_path::Invalid);
                                }

                                let last_length = {
                                    let l = length & 0b11;

                                    if l == 0 {
                                        4
                                    } else {
                                        l
                                    }
                                };

                                #check_last_length

                                let last_bytes = if length > 4 {
                                    for e in v.iter().copied().take(length - last_length) {
                                        match e {
                                            b'A'..=b'Z' | b'a'..=b'z' | b'0'..=b'9' | b'-' | b'_' => (),
                                            _ => return Err(#error_path::Invalid),
                                        }
                                    }

                                    &v[(length - last_length)..]
                                } else {
                                    v.as_ref()
                                };

                                let mut p = 0;

                                loop {
                                    if p == last_length {
                                        return Ok(());
                                    }

                                    let e = last_bytes[p];

                                    match e {
                                        b'A'..=b'Z' | b'a'..=b'z' | b'0'..=b'9' | b'-' | b'_' => (),
                                        b'=' => {
                                            #handle_padding
                                        }
                                        _ => return Err(#error_path::Invalid),
                                    }

                                    p += 1;
                                }
                            }
                        }
                    });

                    token_stream.extend(quote! {
                        impl ValidateString for #name {
                            type Error = #error_path;

                            #[inline]
                            fn parse_string<S: Into<validators_prelude::String>>(s: S) -> Result<Self, Self::Error> {
                                let s = s.into();

                                Self::v_parse_str(s.as_str())?;

                                Ok(Self(s))
                            }

                            #[inline]
                            fn parse_str<S: AsRef<str>>(s: S) -> Result<Self, Self::Error> {
                                let s = s.as_ref();

                                Self::v_parse_str(s)?;

                                Ok(Self(validators_prelude::String::from(s)))
                            }

                            #[inline]
                            fn validate_str<S: AsRef<str>>(s: S) -> Result<(), Self::Error> {
                                Self::v_parse_str(s.as_ref())?;

                                Ok(())
                            }
                        }

                        impl ValidateBytes for #name {
                            type Error = #error_path;

                            #[inline]
                            fn parse_vec_u8<V: Into<validators_prelude::Vec<u8>>>(v: V) -> Result<Self, Self::Error> {
                                let v = v.into();

                                Self::v_parse_u8_slice(v.as_slice())?;

                                Ok(Self(unsafe { validators_prelude::String::from_utf8_unchecked(v) }))
                            }

                            #[inline]
                            fn parse_u8_slice<V: AsRef<[u8]>>(v: V) -> Result<Self, Self::Error> {
                                let v = v.as_ref();

                                Self::v_parse_u8_slice(v)?;

                                Ok(Self(unsafe { validators_prelude::String::from_utf8_unchecked(v.to_vec()) }))
                            }

                            #[inline]
                            fn validate_u8_slice<V: AsRef<[u8]>>(v: V) -> Result<(), Self::Error> {
                                Self::v_parse_u8_slice(v.as_ref())?;

                                Ok(())
                            }
                        }
                    });

                    #[cfg(feature = "serde")]
                    {
                        if type_attribute.serde_options.serialize {
                            token_stream.extend(quote! {
                                impl validators_prelude::serde::Serialize for #name {
                                    #[inline]
                                    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
                                        where
                                            S: validators_prelude::serde::Serializer, {
                                        serializer.serialize_str(self.0.as_str())
                                    }
                                }
                            });
                        }

                        if type_attribute.serde_options.deserialize {
                            use crate::common::tri_allow::TriAllow;

                            let expect = match type_attribute.padding {
                                TriAllow::Allow => "a Base64-url string or data",
                                TriAllow::Must => "a Base64-url string or data with padding",
                                TriAllow::Disallow => "a Base64-url string or data without padding",
                            };

                            token_stream.extend(quote! {
                                impl<'de> validators_prelude::serde::Deserialize<'de> for #name {
                                    #[inline]
                                    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
                                    where
                                        D: validators_prelude::serde::Deserializer<'de>, {
                                        struct MyVisitor;

                                        impl<'de> validators_prelude::serde::de::Visitor<'de> for MyVisitor {
                                            type Value = #name;

                                            #[inline]
                                            fn expecting(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
                                                f.write_str(#expect)
                                            }

                                            #[inline]
                                            fn visit_str<E>(self, v: &str) -> Result<Self::Value, E>
                                            where
                                                E: validators_prelude::serde::de::Error, {
                                                <#name as ValidateString>::parse_str(v).map_err(validators_prelude::serde::de::Error::custom)
                                            }

                                            #[inline]
                                            fn visit_string<E>(self, v: validators_prelude::String) -> Result<Self::Value, E>
                                            where
                                                E: validators_prelude::serde::de::Error, {
                                                <#name as ValidateString>::parse_string(v).map_err(validators_prelude::serde::de::Error::custom)
                                            }

                                            #[inline]
                                            fn visit_bytes<E>(self, v: &[u8]) -> Result<Self::Value, E>
                                            where
                                                E: validators_prelude::serde::de::Error, {
                                                <#name as ValidateBytes>::parse_u8_slice(v).map_err(validators_prelude::serde::de::Error::custom)
                                            }

                                            #[inline]
                                            fn visit_byte_buf<E>(self, v: validators_prelude::Vec<u8>) -> Result<Self::Value, E>
                                            where
                                                E: validators_prelude::serde::de::Error, {
                                                <#name as ValidateBytes>::parse_vec_u8(v).map_err(validators_prelude::serde::de::Error::custom)
                                            }
                                        }

                                        deserializer.deserialize_any(MyVisitor)
                                    }
                                }
                            });
                        }
                    }

                    #[cfg(feature = "rocket")]
                    {
                        if type_attribute.rocket_options.from_form_field {
                            crate::common::rocket::impl_from_form_field(&mut token_stream, &name);
                        }

                        if type_attribute.rocket_options.from_param {
                            crate::common::rocket::impl_from_param(
                                &mut token_stream,
                                &name,
                                &error_path,
                            );
                        }
                    }

                    return Ok(token_stream);
                }
            }
        }

        Err(panic::validator_for_specific_item(meta.path().get_ident().unwrap(), ITEM))
    }
}
