//! # Validators
//!
//! This crate provides many validators for validating data from users and modeling them to structs without much extra effort.
//!
//! All validators are separated into different modules and unified for two main types: **XXX** and **XXXValidator** where **XXX** is a type that you want to validate.
//! The former is a struct or a enum, and the latter is a struct which can be considered as a generator of the former.
//! A **XXXValidator** struct usually contains some values of `ValidatorOption` in order to use different rules to check data.
//!
//! For example, the mod `domain` has `Domain` and `DomainValidator` structs. If we want to create a `Domain` instance, we need to create a `DomainValidator` instance first.
//! When initialing a `DomainValidator`, we can choose to make this `DomainValidator` **allow** or **not allow** the input to have or **must** have a port number.
//!
//! ```
//! extern crate validators;
//!
//! use validators::ValidatorOption;
//! use validators::domain::{Domain, DomainValidator};
//!
//! let domain = "tool.magiclen.org:8080".to_string();
//!
//! let dv = DomainValidator {
//!     port: ValidatorOption::Allow,
//!     localhost: ValidatorOption::NotAllow,
//! };
//!
//! let domain = dv.parse_string(domain).unwrap();
//!
//! assert_eq!("tool.magiclen.org:8080", domain.get_full_domain());
//! assert_eq!("tool.magiclen.org", domain.get_full_domain_without_port());
//! assert_eq!("org", domain.get_top_level_domain().unwrap());
//! assert_eq!("tool", domain.get_sub_domain().unwrap());
//! assert_eq!("magiclen", domain.get_domain());
//! assert_eq!(8080, domain.get_port().unwrap());
//! ```
//!
//! If you want the **XXX** model to be stricter, you can use its wrapper type which is something like **XXXWithPort** or **XXXWithoutPort**.
//! For instance, `Domain` has some wrappers, such as **DomainLocalhostableWithPort**, **DomainLocalhostableAllowPort** and **DomainLocalhostableWithoutPort**.
//!
//! ```
//! extern crate validators;
//!
//! use validators::domain::{DomainLocalhostableWithPort};
//!
//! let domain = "tool.magiclen.org:8080".to_string();
//!
//! let domain = DomainLocalhostableWithPort::from_string(domain).unwrap();
//!
//! assert_eq!("tool.magiclen.org:8080", domain.get_full_domain());
//! assert_eq!("tool.magiclen.org", domain.get_full_domain_without_port());
//! assert_eq!("org", domain.get_top_level_domain().unwrap());
//! assert_eq!("tool", domain.get_sub_domain().unwrap());
//! assert_eq!("magiclen", domain.get_domain());
//! assert_eq!(8080, domain.get_port()); // This function does not use `Option` as its return value, because the struct `DomainLocalhostableWithPort` has already made sure the input must have a port number!
//! ```
//!
//! This crate aims to use the simplest and slackest way (normally only use regular expressions) to validate data, in order to minimize the overhead.
//! Therefore, it may not be competent in some critical situations. Use it carefully. Check out the documentation to see more useful validators and wrapper types.
//!
//! ## Customization
//!
//! This crate also provides macros to create customized validated structs for any strings, numbers and Vecs.
//!
//! For example, to create a struct which only allows **"Hi"** or **"Hello"** restricted by a regular expression,
//!
//! ```
//! #[macro_use] extern crate validators;
//!
//! validated_customized_regex_string!(Greet, "^(Hi|Hello)$");
//!
//! let s = Greet::from_str("Hi").unwrap();
//! ```
//!
//! While a regex needs to be compiled before it operates, if you want to reuse a compiled regex, you can add a **ref** keyword, and pass a static Regex instance to the macro,
//!
//! ```
//! #[macro_use] extern crate validators;
//! #[macro_use] extern crate lazy_static;
//! extern crate regex;
//!
//! use regex::Regex;
//!
//! lazy_static! {
//!     static ref RE_GREET: Regex = {
//!         Regex::new("^(Hi|Hello)$").unwrap()
//!     };
//! }
//!
//! validated_customized_regex_string!(Greet, ref RE_GREET);
//!
//! let s = Greet::from_str("Hi").unwrap();
//! ```
//!
//! You can also make your struct public by adding a **pub** keyword,
//!
//! ```
//! #[macro_use] extern crate validators;
//!
//! validated_customized_regex_string!(pub Greet, "^(Hi|Hello)$");
//!
//! let s = Greet::from_str("Hi").unwrap();
//! ```
//!
//! For numbers limited in a range,
//!
//! ```
//! #[macro_use] extern crate validators;
//!
//! validated_customized_ranged_number!(Score, u8, 0, 100);
//!
//! let score = Score::from_str("80").unwrap();
//! ```
//!
//! For a Vec whose length is limited in a range,
//!
//! ```
//! #[macro_use] extern crate validators;
//!
//! validated_customized_regex_string!(Name, "^[A-Z][a-zA-Z]*( [A-Z][a-zA-Z]*)*$");
//! validated_customized_ranged_length_vec!(Names, 1, 5);
//!
//! let mut names = Vec::new();
//!
//! names.push(Name::from_str("Ron").unwrap());
//! names.push(Name::from_str("Magic Len").unwrap());
//!
//! let names = Names::from_vec(names).unwrap();
//! ```
//!
//! All validated wrapper types and validated customized structs implement the `ValidatedWrapper` trait.
//!
//! Read the documentation to know more helpful customized macros.
//!
//! ## Rocket Support
//!
//! This crate supports [Rocket](https://rocket.rs/) framework. All validated wrapper types and validated customized structs implement the `FromFormValue` and `FromParam` traits.
//! To use with Rocket support, you have to enable **rocketly** feature for this crate.
//!
//! ```toml
//! [dependencies.validators]
//! version = "*"
//! features = ["rocketly"]
//! ```
//!
//! For example,
//!
//! ```rust,ignore
//! #![feature(plugin)]
//! #![feature(custom_derive)]
//! #![plugin(rocket_codegen)]
//!
//! #[macro_use] extern crate validators;
//!
//! extern crate rocket;
//!
//! use rocket::request::Form;
//!
//! use validators::http_url::HttpUrlUnlocalableWithProtocol;
//! use validators::email::Email;
//!
//! validated_customized_ranged_number!(PersonID, u8, 0, 100);
//! validated_customized_regex_string!(Name, r"^[\S ]{1,80}$");
//! validated_customized_ranged_number!(PersonAge, u8, 0, 130);
//!
//! #[derive(Debug, FromForm)]
//! struct ContactModel {
//!     name: Name,
//!     age: Option<PersonAge>,
//!     email: Email,
//!     url: Option<HttpUrlUnlocalableWithProtocol>
//! }
//!
//! #[post("/contact/<id>", data = "<model>")]
//! fn contact(id: PersonID, model: Form<ContactModel>) -> &'static str {
//!     println!("{}", id);
//!     println!("{:?}", model.get());
//!     "do something..."
//! }
//! ```
//!
//! ## Serde Support
//!
//! Serde is a framework for serializing and deserializing Rust data structures efficiently and generically. And again, this crate supports [Serde](https://crates.io/crates/serde) framework.
//! All validated wrapper types and validated customized structs implement the `Serialize` and `Deserialize` traits.
//! To use with Serde support, you have to enable **serdely** feature for this crate.
//!
//! ```toml
//! [dependencies.validators]
//! version = "*"
//! features = ["serdely"]
//! ```
//!
//! For example,
//!
//! ```rust,ignore
//! #[macro_use] extern crate validators;
//! #[macro_use] extern crate serde_json;
//!
//! validated_customized_regex_string!(Name, "^[A-Z][a-zA-Z]*( [A-Z][a-zA-Z]*)*$");
//! validated_customized_ranged_length_vec!(Names, 1, 5);
//!
//! let mut names = Vec::new();
//!
//! names.push(Name::from_str("Ron").unwrap());
//! names.push(Name::from_str("Magic Len").unwrap());
//!
//! let names = Names::from_vec(names).unwrap();
//!
//! assert_eq!("[\"Ron\",\"Magic Len\"]", json!(names).to_string());
//! ```

#![cfg_attr(feature = "nightly", feature(ip))]

#[doc(hidden)]
pub extern crate regex;

#[macro_use]
pub extern crate lazy_static;

#[cfg(feature = "rocketly")]
#[doc(hidden)]
pub extern crate rocket;

#[cfg(feature = "serdely")]
#[doc(hidden)]
#[macro_use]
pub extern crate serde;

pub extern crate number_as;

use number_as::Number;

#[cfg(feature = "serdely")]
use number_as::NumberAs;

use std::error::Error;
use std::fmt::{self, Display, Debug, Formatter};
use std::cmp::PartialEq;
use std::str::Utf8Error;

#[doc(hidden)]
pub const REGEX_SIZE_LIMIT: usize = 26214400;

#[derive(Debug, PartialEq)]
pub enum ValidatorOption {
    Must,
    Allow,
    NotAllow,
}

impl ValidatorOption {
    pub fn allow(&self) -> bool {
        match self {
            ValidatorOption::Must => true,
            ValidatorOption::Allow => true,
            ValidatorOption::NotAllow => false
        }
    }

    pub fn not_allow(&self) -> bool {
        match self {
            ValidatorOption::Must => false,
            ValidatorOption::Allow => false,
            ValidatorOption::NotAllow => true
        }
    }

    pub fn must(&self) -> bool {
        match self {
            ValidatorOption::Must => true,
            ValidatorOption::Allow => false,
            ValidatorOption::NotAllow => false
        }
    }
}

pub trait Validated: Display + PartialEq + Clone + Debug {}

pub trait ValidatedWrapper: Validated {
    type Error: Display + PartialEq + Clone + Debug;

    fn from_string(from_string_input: String) -> Result<Self, Self::Error>;

    fn from_str(from_str_input: &str) -> Result<Self, Self::Error>;
}

pub mod domain;
pub mod email;
pub mod ipv4;
pub mod ipv6;
pub mod host;
pub mod http_url;
pub mod base64;
pub mod base64_url;
pub mod base32;
pub mod short_crypt_url_component;
pub mod short_crypt_qr_code_alphanumeric;

// TODO -----ValidatedCustomizedString START-----

#[derive(Debug, PartialEq, Clone)]
pub enum ValidatedCustomizedStringError {
    RegexError(regex::Error),
    NotMatch,
    UTF8Error(Utf8Error),
}

impl Display for ValidatedCustomizedStringError {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        Debug::fmt(self, f)
    }
}

impl Error for ValidatedCustomizedStringError {}

#[cfg(feature = "serdely")]
pub struct StringVisitor<V>(pub Vec<V>);

#[cfg(feature = "serdely")]
impl<'de, V: ValidatedWrapper> serde::de::Visitor<'de> for StringVisitor<V> {
    type Value = V;

    fn expecting(&self, formatter: &mut Formatter) -> fmt::Result {
        formatter.write_fmt(format_args!("a string({})", stringify!($name)))
    }

    fn visit_str<E>(self, v: &str) -> Result<Self::Value, E> where E: serde::de::Error {
        V::from_str(v).map_err(|err| {
            E::custom(err.to_string())
        })
    }

    fn visit_string<E>(self, v: String) -> Result<Self::Value, E> where E: serde::de::Error {
        V::from_string(v).map_err(|err| {
            E::custom(err.to_string())
        })
    }
}

#[cfg(feature = "serdely")]
#[doc(hidden)]
#[macro_export]
macro_rules! validated_customized_string_struct_implement_se_de {
     ( $name:ident ) => {
        impl<'de> ::validators::serde::Deserialize<'de> for $name {
            fn deserialize<D>(deserializer: D) -> Result<Self, D::Error> where D: ::validators::serde::Deserializer<'de> {
                deserializer.deserialize_string(::validators::StringVisitor(Vec::<$name>::new()))
            }
        }

        impl ::validators::serde::Serialize for $name {
            fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error> where S: ::validators::serde::Serializer {
                serializer.serialize_str(self.as_str())
            }
        }
     }
}

#[cfg(not(feature = "serdely"))]
#[doc(hidden)]
#[macro_export]
macro_rules! validated_customized_string_struct_implement_se_de {
    ( $name:ident ) => {

    }
}

#[cfg(feature = "rocketly")]
#[doc(hidden)]
#[macro_export]
macro_rules! validated_customized_string_struct_implement_from_form_value {
    ( $name:ident ) => {
        impl<'a> ::validators::rocket::request::FromFormValue<'a> for $name {
            type Error = ::validators::ValidatedCustomizedStringError;

            fn from_form_value(form_value: &'a ::validators::rocket::http::RawStr) -> Result<Self, Self::Error>{
                $name::from_string(form_value.url_decode().map_err(|err| ::validators::ValidatedCustomizedStringError::UTF8Error(err))?)
            }
        }

        impl<'a> ::validators::rocket::request::FromParam<'a> for $name {
            type Error = ::validators::ValidatedCustomizedStringError;

            fn from_param(param: &'a ::validators::rocket::http::RawStr) -> Result<Self, Self::Error> {
                $name::from_string(param.url_decode().map_err(|err| ::validators::ValidatedCustomizedStringError::UTF8Error(err))?)
            }
        }
    }
}

#[cfg(not(feature = "rocketly"))]
#[doc(hidden)]
#[macro_export]
macro_rules! validated_customized_string_struct_implement_from_form_value {
    ( $name:ident ) => {

    }
}

#[macro_export]
macro_rules! validated_customized_string_struct {
    ( $name:ident, $field:ident, $from_string_input:ident $from_string:block, $from_str_input:ident $from_str:block ) => {
        impl Clone for $name {
            fn clone(&self) -> Self{
                let $field = self.$field.clone();

                $name{$field}
            }
        }

        impl ::std::fmt::Debug for $name {
            fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                f.write_fmt(format_args!("{}({})", stringify!($name), self.$field))?;
                Ok(())
            }
        }

        impl ::std::fmt::Display for $name {
            fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                f.write_str(&self.$field)?;
                Ok(())
            }
        }

        impl ::std::cmp::PartialEq for $name {
            fn eq(&self, other: &Self) -> bool {
                self.$field.eq(&other.$field)
            }

            fn ne(&self, other: &Self) -> bool {
                self.$field.ne(&other.$field)
            }
        }

        impl AsRef<[u8]> for $name {
            fn as_ref(&self) -> &[u8] {
                self.$field.as_bytes()
            }
        }

        impl AsRef<str> for $name {
            fn as_ref(&self) -> &str {
                self.$field.as_ref()
            }
        }

        impl ::validators::Validated for $name {}

        impl ::validators::ValidatedWrapper for $name {
            type Error = ::validators::ValidatedCustomizedStringError;

            fn from_string($from_string_input: String) -> Result<Self, Self::Error>{
                $name::from_string($from_string_input)
            }

            fn from_str($from_str_input: &str) -> Result<Self, Self::Error>{
                $name::from_str($from_str_input)
            }
        }

        impl<'a> $name {
            pub fn as_str(&'a self) -> &'a str {
                &self.$field
            }

            pub fn from_string($from_string_input: String) -> Result<Self, ::validators::ValidatedCustomizedStringError>{
                let $field = match $from_string {
                    Ok(s)=> s,
                    Err(e)=> return Err(e)
                };

                Ok($name{$field})
            }

            pub fn from_str($from_str_input: &str) -> Result<Self, ::validators::ValidatedCustomizedStringError>{
                let $field = match $from_str {
                    Ok(s)=> s,
                    Err(e)=> return Err(e)
                };

                Ok($name{$field})
            }
        }

        validated_customized_string_struct_implement_from_form_value!($name);

        validated_customized_string_struct_implement_se_de!($name);
    };
    ( $name:ident, $field:ident, from_string $from_string_input:ident $from_string:block, from_str $from_str_input:ident $from_str:block ) => {
        validated_customized_string_struct!($name, $field, $from_string_input $from_string, $from_str_input $from_str);
    };
    ( $name:ident, $field:ident, from_str $from_str_input:ident $from_str:block, from_string $from_string_input:ident $from_string:block ) => {
        validated_customized_string_struct!($name, $field, $from_string_input $from_string, $from_str_input $from_str);
    };
}

#[macro_export]
macro_rules! validated_customized_string {
    ( $name:ident, $from_string_input:ident $from_string:block, $from_str_input:ident $from_str:block ) => {
        struct $name{
            s: String
        }

        validated_customized_string_struct!($name, s, $from_string_input $from_string, $from_str_input $from_str);
    };
    ( $name:ident, from_string $from_string_input:ident $from_string:block, from_str $from_str_input:ident $from_str:block ) => {
        validated_customized_string!($name, $from_string_input $from_string, $from_str_input $from_str);
    };
    ( $name:ident, from_str $from_str_input:ident $from_str:block, from_string $from_string_input:ident $from_string:block ) => {
        validated_customized_string!($name, $from_string_input $from_string, $from_str_input $from_str);
    };
    ( pub $name:ident, $from_string_input:ident $from_string:block, $from_str_input:ident $from_str:block ) => {
        pub struct $name{
            s: String
        }

        validated_customized_string_struct!($name, s, $from_string_input $from_string, $from_str_input $from_str);
    };
    ( pub $name:ident, from_string $from_string_input:ident $from_string:block, from_str $from_str_input:ident $from_str:block ) => {
        validated_customized_string!(pub $name, $from_string_input $from_string, $from_str_input $from_str);
    };
    ( pub $name:ident, from_str $from_str_input:ident $from_str:block, from_string $from_string_input:ident $from_string:block ) => {
        validated_customized_string!(pub $name, $from_string_input $from_string, $from_str_input $from_str);
    };
}

#[macro_export]
macro_rules! validated_customized_regex_string_struct {
    ( $name:ident, $field:ident, $re:expr ) => {
        validated_customized_string_struct!($name, $field,
        input {
            let re = ::validators::regex::RegexBuilder::new($re).size_limit(::validators::REGEX_SIZE_LIMIT).build().map_err(|err| ::validators::ValidatedCustomizedStringError::RegexError(err))?;

            if re.is_match(&input) {
                Ok(input)
            } else{
                Err(::validators::ValidatedCustomizedStringError::NotMatch)
            }
        },
        input {
            let re = ::validators::regex::RegexBuilder::new($re).size_limit(::validators::REGEX_SIZE_LIMIT).build().map_err(|err| ::validators::ValidatedCustomizedStringError::RegexError(err))?;

            if re.is_match(&input) {
                Ok(input.to_string())
            } else{
                Err(::validators::ValidatedCustomizedStringError::NotMatch)
            }
        });
    };
    ( $name:ident, $field:ident, ref $re:expr ) => {
        validated_customized_string_struct!($name, $field,
        input {
            let re: &::validators::regex::Regex = &$re;

            if re.is_match(&input) {
                Ok(input)
            } else{
                Err(::validators::ValidatedCustomizedStringError::NotMatch)
            }
        },
        input {
            let re: &::validators::regex::Regex = &$re;

            if re.is_match(&input) {
                Ok(input.to_string())
            } else{
                Err(::validators::ValidatedCustomizedStringError::NotMatch)
            }
        });
    };
}

#[macro_export]
macro_rules! validated_customized_regex_string {
    ( $name:ident, $re:expr ) => {
        struct $name{
            s: String
        }

        validated_customized_regex_string_struct!($name, s, $re);
    };
    ( pub $name:ident, $re:expr ) => {
        pub struct $name{
            s: String
        }

        validated_customized_regex_string_struct!($name, s, $re);
    };
    ( $name:ident, ref $re:expr ) => {
        struct $name{
            s: String
        }

        validated_customized_regex_string_struct!($name, s, ref $re);
    };
    ( pub $name:ident, ref $re:expr ) => {
        pub struct $name{
            s: String
        }

        validated_customized_regex_string_struct!($name, s, ref $re);
    };
}

// TODO -----ValidatedCustomizedString END-----

// TODO -----ValidatedCustomizedNumber START-----

#[derive(Debug, PartialEq, Clone)]
pub enum ValidatedCustomizedNumberError {
    RegexError(regex::Error),
    ParseError(String),
    OutRange,
    NotMatch,
    UTF8Error(Utf8Error),
}

impl Display for ValidatedCustomizedNumberError {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        Debug::fmt(self, f)
    }
}

impl Error for ValidatedCustomizedNumberError {}

pub trait ValidatedNumberWrapper<T: Number>: ValidatedWrapper {
    fn from_number(n: T) -> Result<Self, ValidatedCustomizedNumberError>;
}

#[cfg(feature = "serdely")]
pub struct NumberVisitor<V, T>(pub Vec<V>, pub Vec<T>);

#[cfg(feature = "serdely")]
impl<'de, V, T> serde::de::Visitor<'de> for NumberVisitor<V, T> where V: ValidatedWrapper + ValidatedNumberWrapper<T>,
                                                                      T: Number,
                                                                      u8: NumberAs<T>,
                                                                      u16: NumberAs<T>,
                                                                      u32: NumberAs<T>,
                                                                      u64: NumberAs<T>,
                                                                      u128: NumberAs<T>,
                                                                      i8: NumberAs<T>,
                                                                      i16: NumberAs<T>,
                                                                      i32: NumberAs<T>,
                                                                      i64: NumberAs<T>,
                                                                      i128: NumberAs<T>,
                                                                      f32: NumberAs<T>,
                                                                      f64: NumberAs<T> {
    type Value = V;

    fn expecting(&self, formatter: &mut Formatter) -> fmt::Result {
        formatter.write_fmt(format_args!("a string({})", stringify!($name)))
    }

    fn visit_u8<E>(self, v: u8) -> Result<Self::Value, E> where E: serde::de::Error {
        V::from_number(v.number_as()).map_err(|err| {
            E::custom(err.to_string())
        })
    }

    fn visit_u16<E>(self, v: u16) -> Result<Self::Value, E> where E: serde::de::Error {
        V::from_number(v.number_as()).map_err(|err| {
            E::custom(err.to_string())
        })
    }

    fn visit_u32<E>(self, v: u32) -> Result<Self::Value, E> where E: serde::de::Error {
        V::from_number(v.number_as()).map_err(|err| {
            E::custom(err.to_string())
        })
    }

    fn visit_u64<E>(self, v: u64) -> Result<Self::Value, E> where E: serde::de::Error {
        V::from_number(v.number_as()).map_err(|err| {
            E::custom(err.to_string())
        })
    }

    serde_if_integer128! {
        fn visit_u128<E>(self, v: u128) -> Result<Self::Value, E> where E: serde::de::Error {
            V::from_number(v.number_as()).map_err(|err| {
                E::custom(err.to_string())
            })
        }
    }

    fn visit_i8<E>(self, v: i8) -> Result<Self::Value, E> where E: serde::de::Error {
        V::from_number(v.number_as()).map_err(|err| {
            E::custom(err.to_string())
        })
    }

    fn visit_i16<E>(self, v: i16) -> Result<Self::Value, E> where E: serde::de::Error {
        V::from_number(v.number_as()).map_err(|err| {
            E::custom(err.to_string())
        })
    }

    fn visit_i32<E>(self, v: i32) -> Result<Self::Value, E> where E: serde::de::Error {
        V::from_number(v.number_as()).map_err(|err| {
            E::custom(err.to_string())
        })
    }

    fn visit_i64<E>(self, v: i64) -> Result<Self::Value, E> where E: serde::de::Error {
        V::from_number(v.number_as()).map_err(|err| {
            E::custom(err.to_string())
        })
    }

    serde_if_integer128! {
        fn visit_i128<E>(self, v: i128) -> Result<Self::Value, E> where E: serde::de::Error {
            V::from_number(v.number_as()).map_err(|err| {
                E::custom(err.to_string())
            })
        }
    }

    fn visit_f32<E>(self, v: f32) -> Result<Self::Value, E> where E: serde::de::Error {
        V::from_number(v.number_as()).map_err(|err| {
            E::custom(err.to_string())
        })
    }

    fn visit_f64<E>(self, v: f64) -> Result<Self::Value, E> where E: serde::de::Error {
        V::from_number(v.number_as()).map_err(|err| {
            E::custom(err.to_string())
        })
    }
}

#[cfg(feature = "serdely")]
#[doc(hidden)]
#[macro_export]
macro_rules! validated_customized_number_struct_implement_se_de {
    ( $name:ident, $t:ident ) => {
        impl<'de> ::validators::serde::Deserialize<'de> for $name {
            fn deserialize<D>(deserializer: D) -> Result<Self, D::Error> where D: ::validators::serde::Deserializer<'de> {
                let v = ::validators::NumberVisitor(Vec::<$name>::new(), Vec::<$t>::new());

                match stringify!($t) {
                    "u8" => deserializer.deserialize_u8(v),
                    "u16" => deserializer.deserialize_u16(v),
                    "u32" => deserializer.deserialize_u32(v),
                    "u64" => deserializer.deserialize_u64(v),
                    "u128" => deserializer.deserialize_u128(v),
                    "i8" => deserializer.deserialize_i8(v),
                    "i16" => deserializer.deserialize_i16(v),
                    "i32" => deserializer.deserialize_i32(v),
                    "i64" => deserializer.deserialize_i64(v),
                    "i128" => deserializer.deserialize_i128(v),
                    "f32" => deserializer.deserialize_f32(v),
                    "f64" => deserializer.deserialize_f64(v),
                    _ => panic!("impossible")
                }
            }
        }

        impl ::validators::serde::Serialize for $name {
            fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error> where S: ::validators::serde::Serializer {
                match stringify!($t) {
                    "u8" => serializer.serialize_u8(self.get_number() as u8),
                    "u16" => serializer.serialize_u16(self.get_number() as u16),
                    "u32" => serializer.serialize_u32(self.get_number() as u32),
                    "u64" => serializer.serialize_u64(self.get_number() as u64),
                    "u128" => serializer.serialize_u128(self.get_number() as u128),
                    "i8" => serializer.serialize_i8(self.get_number() as i8),
                    "i16" => serializer.serialize_i16(self.get_number() as i16),
                    "i32" => serializer.serialize_i32(self.get_number() as i32),
                    "i64" => serializer.serialize_i64(self.get_number() as i64),
                    "i128" => serializer.serialize_i128(self.get_number() as i128),
                    "f32" => serializer.serialize_f32(self.get_number() as f32),
                    "f64" => serializer.serialize_f64(self.get_number() as f64),
                    _ => panic!("impossible")
                }
            }
        }
    }
}

#[cfg(not(feature = "serdely"))]
#[doc(hidden)]
#[macro_export]
macro_rules! validated_customized_number_struct_implement_se_de {
    ( $name:ident, $t:expr ) => {

    }
}

#[cfg(feature = "rocketly")]
#[doc(hidden)]
#[macro_export]
macro_rules! validated_customized_number_struct_implement_from_form_value {
    ( $name:ident ) => {
        impl<'a> ::validators::rocket::request::FromFormValue<'a> for $name {
            type Error = ::validators::ValidatedCustomizedNumberError;

            fn from_form_value(form_value: &'a ::validators::rocket::http::RawStr) -> Result<Self, Self::Error>{
                $name::from_string(form_value.url_decode().map_err(|err| ::validators::ValidatedCustomizedNumberError::UTF8Error(err))?)
            }
        }

        impl<'a> ::validators::rocket::request::FromParam<'a> for $name {
            type Error = ::validators::ValidatedCustomizedNumberError;

            fn from_param(param: &'a ::validators::rocket::http::RawStr) -> Result<Self, Self::Error> {
                $name::from_string(param.url_decode().map_err(|err| ::validators::ValidatedCustomizedNumberError::UTF8Error(err))?)
            }
        }
    }
}

#[cfg(not(feature = "rocketly"))]
#[doc(hidden)]
#[macro_export]
macro_rules! validated_customized_number_struct_implement_from_form_value {
    ( $name:ident ) => {

    }
}

#[macro_export]
macro_rules! validated_customized_number_struct {
    ( $name:ident, $field:ident, $t:ident, $from_string_input:ident $from_string:block, $from_str_input:ident $from_str:block, $from_number_input:ident $from_number:block ) => {
        impl Clone for $name {
            fn clone(&self) -> Self{
                let $field = self.$field;

                $name{$field}
            }
        }

        impl ::std::fmt::Debug for $name {
            fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                f.write_fmt(format_args!("{}({})", stringify!($name), self.$field))?;
                Ok(())
            }
        }

        impl ::std::fmt::Display for $name {
            fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                f.write_fmt(format_args!("{}", self.$field))?;
                Ok(())
            }
        }

        impl ::std::cmp::PartialEq for $name {
            fn eq(&self, other: &Self) -> bool {
                self.$field == other.$field
            }

            fn ne(&self, other: &Self) -> bool {
                self.$field != other.$field
            }
        }

        impl ::validators::Validated for $name {}

        impl ::validators::ValidatedWrapper for $name {
            type Error = ::validators::ValidatedCustomizedNumberError;

            fn from_string($from_string_input: String) -> Result<Self, Self::Error>{
                $name::from_string($from_string_input)
            }

            fn from_str($from_str_input: &str) -> Result<Self, Self::Error>{
                $name::from_str($from_str_input)
            }
        }

        impl<T: ::validators::number_as::Number> ::validators::ValidatedNumberWrapper<T> for $name {
            fn from_number($from_number_input: T) -> Result<Self, ::validators::ValidatedCustomizedNumberError>{
                $name::from_number($from_number_input.number_as())
            }
        }

        impl $name {
            pub fn get_number(&self) -> $t {
                self.$field
            }

            pub fn from_string($from_string_input: String) -> Result<Self, ::validators::ValidatedCustomizedNumberError>{
                let $field = match $from_string {
                    Ok(s)=> s,
                    Err(e)=> return Err(e)
                };

                Ok($name{$field})
            }

            pub fn from_str($from_str_input: &str) -> Result<Self, ::validators::ValidatedCustomizedNumberError>{
                let $field = match $from_str {
                    Ok(s)=> s,
                    Err(e)=> return Err(e)
                };

                Ok($name{$field})
            }

            pub fn from_number($from_number_input: $t) -> Result<Self, ::validators::ValidatedCustomizedNumberError>{
                let $field = match $from_number {
                    Ok(s)=> s,
                    Err(e)=> return Err(e)
                };

                Ok($name{$field})
            }
        }

        validated_customized_number_struct_implement_from_form_value!($name);

        validated_customized_number_struct_implement_se_de!($name, $t);
    };
    ( $name:ident, $field:ident, $t:ident, from_string $from_string_input:ident $from_string:block, from_str $from_str_input:ident $from_str:block, from_number $from_number_input:ident $from_number:block ) => {
        validated_customized_number_struct!($name, $field, $t, $from_string_input $from_string, $from_str_input $from_str, $from_number_input $from_number);
    };
    ( $name:ident, $field:ident, $t:ident, from_str $from_str_input:ident $from_str:block, from_string $from_string_input:ident $from_string:block, from_number $from_number_input:ident $from_number:block ) => {
        validated_customized_number_struct!($name, $field, $t, $from_string_input $from_string, $from_str_input $from_str, $from_number_input $from_number);
    };
    ( $name:ident, $field:ident, $t:ident, from_number $from_number_input:ident $from_number:block, from_string $from_string_input:ident $from_string:block, from_str $from_str_input:ident $from_str:block ) => {
        validated_customized_number_struct!($name, $field, $t, $from_string_input $from_string, $from_str_input $from_str, $from_number_input $from_number);
    };
    ( $name:ident, $field:ident, $t:ident, from_number $from_number_input:ident $from_number:block, from_str $from_str_input:ident $from_str:block, from_string $from_string_input:ident $from_string:block ) => {
        validated_customized_number_struct!($name, $field, $t, $from_string_input $from_string, $from_str_input $from_str, $from_number_input $from_number);
    };
    ( $name:ident, $field:ident, $t:ident, from_string $from_string_input:ident $from_string:block, from_number $from_number_input:ident $from_number:block, from_str $from_str_input:ident $from_str:block ) => {
        validated_customized_number_struct!($name, $field, $t, $from_string_input $from_string, $from_str_input $from_str, $from_number_input $from_number);
    };
    ( $name:ident, $field:ident, $t:ident, from_str $from_str_input:ident $from_str:block, from_number $from_number_input:ident $from_number:block, from_string $from_string_input:ident $from_string:block ) => {
        validated_customized_number_struct!($name, $field, $t, $from_string_input $from_string, $from_str_input $from_str, $from_number_input $from_number);
    };
}

#[macro_export]
macro_rules! validated_customized_number {
    ( $name:ident, $t:ident, $from_string_input:ident $from_string:block, $from_str_input:ident $from_str:block, $from_number_input:ident $from_number:block ) => {
        struct $name{
            n: $t
        }

        validated_customized_number_struct!($name, n, $t, $from_string_input $from_string, $from_str_input $from_str, $from_number_input $from_number);
    };
    ( $name:ident, $t:ident, from_string $from_string_input:ident $from_string:block, from_str $from_str_input:ident $from_str:block, from_number $from_number_input:ident $from_number:block ) => {
        validated_customized_number!($name, $t, $from_string_input $from_string, $from_str_input $from_str, $from_number_input $from_number);
    };
    ( $name:ident, $t:ident, from_str $from_str_input:ident $from_str:block, from_string $from_string_input:ident $from_string:block, from_number $from_number_input:ident $from_number:block ) => {
        validated_customized_number!($name, $t, $from_string_input $from_string, $from_str_input $from_str, $from_number_input $from_number);
    };
    ( $name:ident, $t:ident, from_number $from_number_input:ident $from_number:block, from_string $from_string_input:ident $from_string:block, from_str $from_str_input:ident $from_str:block ) => {
        validated_customized_number!($name, $t, $from_string_input $from_string, $from_str_input $from_str, $from_number_input $from_number);
    };
    ( $name:ident, $t:ident, from_number $from_number_input:ident $from_number:block, from_str $from_str_input:ident $from_str:block, from_string $from_string_input:ident $from_string:block ) => {
        validated_customized_number!($name, $t, $from_string_input $from_string, $from_str_input $from_str, $from_number_input $from_number);
    };
    ( $name:ident, $t:ident, from_string $from_string_input:ident $from_string:block, from_number $from_number_input:ident $from_number:block, from_str $from_str_input:ident $from_str:block ) => {
        validated_customized_number!($name, $t, $from_string_input $from_string, $from_str_input $from_str, $from_number_input $from_number);
    };
    ( $name:ident, $t:ident, from_str $from_str_input:ident $from_str:block, from_number $from_number_input:ident $from_number:block, from_string $from_string_input:ident $from_string:block ) => {
        validated_customized_number!($name, $t, $from_string_input $from_string, $from_str_input $from_str, $from_number_input $from_number);
    };
    ( pub $name:ident, $t:ident, $from_string_input:ident $from_string:block, $from_str_input:ident $from_str:block, $from_number_input:ident $from_number:block ) => {
        pub struct $name{
            n: $t
        }

        validated_customized_number_struct!($name, n, $t, $from_string_input $from_string, $from_str_input $from_str, $from_number_input $from_number);
    };
    ( pub $name:ident, $t:ident, from_string $from_string_input:ident $from_string:block, from_str $from_str_input:ident $from_str:block, from_number $from_number_input:ident $from_number:block ) => {
        validated_customized_number!(pub $name, $t, $from_string_input $from_string, $from_str_input $from_str, $from_number_input $from_number);
    };
    ( pub $name:ident, $t:ident, from_str $from_str_input:ident $from_str:block, from_string $from_string_input:ident $from_string:block, from_number $from_number_input:ident $from_number:block ) => {
        validated_customized_number!(pub $name, $t, $from_string_input $from_string, $from_str_input $from_str, $from_number_input $from_number);
    };
    ( pub $name:ident, $t:ident, from_number $from_number_input:ident $from_number:block, from_string $from_string_input:ident $from_string:block, from_str $from_str_input:ident $from_str:block ) => {
        validated_customized_number!(pub $name, $t, $from_string_input $from_string, $from_str_input $from_str, $from_number_input $from_number);
    };
    ( pub $name:ident, $t:ident, from_number $from_number_input:ident $from_number:block, from_str $from_str_input:ident $from_str:block, from_string $from_string_input:ident $from_string:block ) => {
        validated_customized_number!(pub $name, $t, $from_string_input $from_string, $from_str_input $from_str, $from_number_input $from_number);
    };
    ( pub $name:ident, $t:ident, from_string $from_string_input:ident $from_string:block, from_number $from_number_input:ident $from_number:block, from_str $from_str_input:ident $from_str:block ) => {
        validated_customized_number!(pub $name, $t, $from_string_input $from_string, $from_str_input $from_str, $from_number_input $from_number);
    };
    ( pub $name:ident, $t:ident, from_str $from_str_input:ident $from_str:block, from_number $from_number_input:ident $from_number:block, from_string $from_string_input:ident $from_string:block ) => {
        validated_customized_number!(pub $name, $t, $from_string_input $from_string, $from_str_input $from_str, $from_number_input $from_number);
    };
}

#[macro_export]
macro_rules! validated_customized_regex_number_struct {
    ( $name:ident, $field:ident, $t:ident, $re:expr ) => {
        validated_customized_number_struct!($name, $field, $t,
        input {
            let re = ::validators::regex::RegexBuilder::new($re).size_limit(::validators::REGEX_SIZE_LIMIT).build().map_err(|err| ::validators::ValidatedCustomizedNumberError::RegexError(err))?;

            if re.is_match(&input) {
                Ok(input.parse::<$t>().map_err(|err|::validators::ValidatedCustomizedNumberError::ParseError(err.to_string()))?)
            } else{
                Err(::validators::ValidatedCustomizedNumberError::NotMatch)
            }
        },
        input {
            let re = ::validators::regex::RegexBuilder::new($re).size_limit(::validators::REGEX_SIZE_LIMIT).build().map_err(|err| ::validators::ValidatedCustomizedNumberError::RegexError(err))?;

            if re.is_match(&input) {
                Ok(input.parse::<$t>().map_err(|err|::validators::ValidatedCustomizedNumberError::ParseError(err.to_string()))?)
            } else{
                Err(::validators::ValidatedCustomizedNumberError::NotMatch)
            }
        },
        input {
            let input = input.to_string();

            let re = ::validators::regex::RegexBuilder::new($re).size_limit(::validators::REGEX_SIZE_LIMIT).build().map_err(|err| ::validators::ValidatedCustomizedNumberError::RegexError(err))?;

            if re.is_match(&input) {
                Ok(input.parse::<$t>().map_err(|err|::validators::ValidatedCustomizedNumberError::ParseError(err.to_string()))?)
            } else{
                Err(::validators::ValidatedCustomizedNumberError::NotMatch)
            }
        });
    };
    ( $name:ident, $field:ident, $t:ident, ref $re:expr ) => {
        validated_customized_number_struct!($name, $field, $t,
        input {
            let re: &::validators::regex::Regex = &$re;

            if re.is_match(&input) {
                Ok(input.parse::<$t>().map_err(|err|::validators::ValidatedCustomizedNumberError::ParseError(err.to_string()))?)
            } else{
                Err(::validators::ValidatedCustomizedNumberError::NotMatch)
            }
        },
        input {
            let re: &::validators::regex::Regex = &$re;

            if re.is_match(&input) {
                Ok(input.parse::<$t>().map_err(|err|::validators::ValidatedCustomizedNumberError::ParseError(err.to_string()))?)
            } else{
                Err(::validators::ValidatedCustomizedNumberError::NotMatch)
            }
        },
        input {
            let input = input.to_string();

            let re: &::validators::regex::Regex = &$re;

            if re.is_match(&input) {
                Ok(input.parse::<$t>().map_err(|err|::validators::ValidatedCustomizedNumberError::ParseError(err.to_string()))?)
            } else{
                Err(::validators::ValidatedCustomizedNumberError::NotMatch)
            }
        });
    };
}

#[macro_export]
macro_rules! validated_customized_regex_number {
    ( $name:ident, $t:ident, $re:expr ) => {
        struct $name{
            n: $t
        }

        validated_customized_regex_number_struct!($name, n, $t, $re);
    };
    ( pub $name:ident, $t:ident, $re:expr ) => {
        pub struct $name{
            n: $t
        }

        validated_customized_regex_number_struct!($name, n, $t, $re);
    };
    ( $name:ident, $t:ident, ref $re:expr ) => {
        struct $name{
            n: $t
        }

        validated_customized_regex_number_struct!($name, n, $t, ref $re);
    };
    ( pub $name:ident, $t:ident, ref $re:expr ) => {
        pub struct $name{
            n: $t
        }

        validated_customized_regex_number_struct!($name, n, $t, ref $re);
    };
}

#[macro_export]
macro_rules! validated_customized_ranged_number_struct {
    ( $name:ident, $field:ident, $t:ident, $min:expr, $max:expr ) => {
        validated_customized_number_struct!($name, $field, $t,
        input {
            let input = input.parse::<$t>().map_err(|err|::validators::ValidatedCustomizedNumberError::ParseError(err.to_string()))?;

            if input >= $min && input <= $max {
                Ok(input)
            } else{
                Err(::validators::ValidatedCustomizedNumberError::OutRange)
            }
        },
        input {
            let input = input.parse::<$t>().map_err(|err|::validators::ValidatedCustomizedNumberError::ParseError(err.to_string()))?;

            if input >= $min && input <= $max {
                Ok(input)
            } else{
                Err(::validators::ValidatedCustomizedNumberError::OutRange)
            }
        },
        input {
            if input >= $min && input <= $max {
                Ok(input)
            } else{
                Err(::validators::ValidatedCustomizedNumberError::OutRange)
            }
        });
    };
}

#[macro_export]
macro_rules! validated_customized_ranged_number {
    ( $name:ident, $t:ident, $min:expr, $max:expr ) => {
        struct $name{
            n: $t
        }

        validated_customized_ranged_number_struct!($name, n, $t, $min, $max);
    };
    ( pub $name:ident, $t:ident, $min:expr, $max:expr ) => {
        pub struct $name{
            n: $t
        }

        validated_customized_ranged_number_struct!($name, n, $t, $min, $max);
    };
}

#[macro_export]
macro_rules! validated_customized_primitive_number_struct {
    ( $name:ident, $field:ident, $t:ident ) => {
        validated_customized_number_struct!($name, $field, $t,
        input {
            let input = input.parse::<$t>().map_err(|err|::validators::ValidatedCustomizedNumberError::ParseError(err.to_string()))?;

            Ok(input)
        },
        input {
            let input = input.parse::<$t>().map_err(|err|::validators::ValidatedCustomizedNumberError::ParseError(err.to_string()))?;

            Ok(input)
        },
        input {
            Ok(input)
        });
    };
}

#[macro_export]
macro_rules! validated_customized_primitive_number {
    ( $name:ident, $t:ident ) => {
        struct $name{
            n: $t
        }

        validated_customized_primitive_number_struct!($name, n, $t);
    };
    ( pub $name:ident, $t:ident ) => {
        pub struct $name{
            n: $t
        }

        validated_customized_primitive_number_struct!($name, n, $t);
    };
}

// TODO -----ValidatedCustomizedNumber END-----

// TODO -----ValidatedCustomizedRangedLengthVec START-----

#[derive(Debug, PartialEq, Clone)]
pub enum ValidatedCustomizedVecError {
    Overflow,
    Underflow,
    NotSupport,
    UTF8Error(Utf8Error),
}

impl Display for ValidatedCustomizedVecError {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        Debug::fmt(self, f)
    }
}

impl Error for ValidatedCustomizedVecError {}

pub trait ValidatedVecWrapper<T: ValidatedWrapper>: ValidatedWrapper {
    fn from_vec(v: Vec<T>) -> Result<Self, ValidatedCustomizedVecError>;
}

#[cfg(feature = "serdely")]
pub struct VecVisitor<V, T>(pub Vec<V>, pub Vec<T>);

#[cfg(feature = "serdely")]
impl<'de, V: ValidatedVecWrapper<T>, T: ValidatedWrapper + serde::Deserialize<'de>> serde::de::Visitor<'de> for VecVisitor<V, T> {
    type Value = V;

    fn expecting(&self, formatter: &mut Formatter) -> fmt::Result {
        formatter.write_fmt(format_args!("a string({})", stringify!($name)))
    }

    fn visit_seq<A>(self, mut seq: A) -> Result<Self::Value, A::Error> where A: serde::de::SeqAccess<'de> {
        let mut v = Vec::<T>::new();

        loop {
            match seq.next_element()? {
                Some(e) => {
                    v.push(e);
                }
                None => { break; }
            }
        }

        Ok(V::from_vec(v).map_err(|err| {
            serde::de::Error::custom(err.to_string())
        })?)
    }
}

#[cfg(feature = "serdely")]
#[doc(hidden)]
#[macro_export]
macro_rules! validated_customized_vec_struct_implement_se_de {
     ( $name:ident ) => {
        impl<'de, T: ::validators::ValidatedWrapper + ::validators::serde::Deserialize<'de>> ::validators::serde::Deserialize<'de> for $name<T> {
            fn deserialize<D>(deserializer: D) -> Result<Self, D::Error> where D: ::validators::serde::Deserializer<'de> {
                deserializer.deserialize_seq(::validators::VecVisitor(Vec::<$name<T>>::new(), Vec::<T>::new()))
            }
        }

        impl<T: ::validators::ValidatedWrapper + ::validators::serde::Serialize> ::validators::serde::Serialize for $name<T> {
            fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error> where S: ::validators::serde::Serializer {
                serializer.collect_seq(self.as_vec().iter())
            }
        }
     }
}

#[cfg(not(feature = "serdely"))]
#[doc(hidden)]
#[macro_export]
macro_rules! validated_customized_vec_struct_implement_se_de {
    ( $name:ident ) => {

    }
}

#[cfg(feature = "rocketly")]
#[doc(hidden)]
#[macro_export]
macro_rules! validated_customized_vec_struct_implement_from_form_value {
    ( $name:ident ) => {
        impl<'a, T: ::validators::ValidatedWrapper> ::validators::rocket::request::FromFormValue<'a> for $name<T> {
            type Error = ::validators::ValidatedCustomizedVecError;

            fn from_form_value(form_value: &'a ::validators::rocket::http::RawStr) -> Result<Self, Self::Error>{
                $name::from_string(form_value.url_decode().map_err(|err| ::validators::ValidatedCustomizedVecError::UTF8Error(err))?)
            }
        }

        impl<'a, T: ::validators::ValidatedWrapper> ::validators::rocket::request::FromParam<'a> for $name<T> {
            type Error = ::validators::ValidatedCustomizedVecError;

            fn from_param(param: &'a ::validators::rocket::http::RawStr) -> Result<Self, Self::Error> {
                $name::from_string(param.url_decode().map_err(|err| ::validators::ValidatedCustomizedVecError::UTF8Error(err))?)
            }
        }

    }
}

#[cfg(not(feature = "rocketly"))]
#[doc(hidden)]
#[macro_export]
macro_rules! validated_customized_vec_struct_implement_from_form_value {
    ( $name:ident ) => {

    }
}

#[macro_export]
macro_rules! validated_customized_vec_struct {
    ( $name:ident, $field:ident, $from_string_input:ident $from_string:block, $from_str_input:ident $from_str:block, $from_vec_input:ident $from_vec:block ) => {
        impl<T: ::validators::ValidatedWrapper> Clone for $name<T> {
            fn clone(&self) -> Self{
                let $field = self.$field.clone();

                $name{$field}
            }
        }

        impl<T: ::validators::ValidatedWrapper> ::std::fmt::Debug for $name<T> {
            fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                f.write_fmt(format_args!("{}[", stringify!($name)))?;

                let len = self.$field.len();

                if len > 0 {
                    for n in self.$field.iter().skip(1) {
                        ::std::fmt::Debug::fmt(n, f)?;


                        f.write_str(", ")?;
                    }

                    ::std::fmt::Debug::fmt(&self.$field[len - 1], f)?;
                }

                f.write_str("]")?;

                Ok(())
            }
        }

        impl<T: ::validators::ValidatedWrapper> ::std::fmt::Display for $name<T> {
            fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                f.write_str("[")?;

                let len = self.$field.len();

                if len > 0 {
                    for n in self.$field.iter().skip(1) {
                        ::std::fmt::Display::fmt(n, f)?;


                        f.write_str(", ")?;
                    }

                    ::std::fmt::Display::fmt(&self.$field[len - 1], f)?;
                }

                f.write_str("]")?;

                Ok(())
            }
        }

        impl<T: ::validators::ValidatedWrapper> ::std::cmp::PartialEq for $name<T> {
            fn eq(&self, other: &Self) -> bool {
                self.$field == other.$field
            }

            fn ne(&self, other: &Self) -> bool {
                self.$field != other.$field
            }
        }

        impl<T: ::validators::ValidatedWrapper> ::validators::Validated for $name<T> {}

        impl<T: ::validators::ValidatedWrapper> ::validators::ValidatedWrapper for $name<T> {
            type Error = ::validators::ValidatedCustomizedVecError;

            fn from_string($from_string_input: String) -> Result<Self, Self::Error>{
                $name::from_string($from_string_input)
            }

            fn from_str($from_str_input: &str) -> Result<Self, Self::Error>{
                $name::from_str($from_str_input)
            }
        }

        impl<T: ::validators::ValidatedWrapper> ::validators::ValidatedVecWrapper<T> for $name<T> {
            fn from_vec($from_vec_input: Vec<T>) -> Result<Self, ::validators::ValidatedCustomizedVecError>{
                $name::from_vec($from_vec_input)
            }
        }

        impl<T: ::validators::ValidatedWrapper> $name<T> {
            pub fn as_vec(&self) -> &Vec<T> {
                &self.$field
            }

            pub fn into_vec(self) -> Vec<T> {
                self.$field
            }

            pub fn from_string($from_string_input: String) -> Result<Self, ::validators::ValidatedCustomizedVecError>{
                let $field = match $from_string {
                    Ok(s)=> s,
                    Err(e)=> return Err(e)
                };

                Ok($name{$field})
            }

            pub fn from_str($from_str_input: &str) -> Result<Self, ::validators::ValidatedCustomizedVecError>{
                let $field = match $from_str {
                    Ok(s)=> s,
                    Err(e)=> return Err(e)
                };

                Ok($name{$field})
            }

            pub fn from_vec($from_vec_input: Vec<T>) -> Result<Self, ::validators::ValidatedCustomizedVecError>{
                let $field = match $from_vec {
                    Ok(s)=> s,
                    Err(e)=> return Err(e)
                };

                Ok($name{$field})
            }
        }

         validated_customized_vec_struct_implement_from_form_value!($name);
         validated_customized_vec_struct_implement_se_de!($name);
    };
}

#[macro_export]
macro_rules! validated_customized_vec {
    ( $name:ident, $from_string_input:ident $from_string:block, $from_str_input:ident $from_str:block, $from_vec_input:ident $from_vec:block ) => {
        struct $name<T: ::validators::ValidatedWrapper> {
            v: Vec<T>
        }

        validated_customized_vec_struct!($name, v, $from_string_input $from_string, $from_str_input $from_str, $from_vec_input $from_vec);
    };
    ( $name:ident, from_string $from_string_input:ident $from_string:block, from_str $from_str_input:ident $from_str:block, from_vec $from_vec_input:ident $from_vec:block ) => {
        validated_customized_vec!($name, $from_string_input $from_string, $from_str_input $from_str, $from_vec_input $from_vec);
    };
    ( $name:ident, from_str $from_str_input:ident $from_str:block, from_string $from_string_input:ident $from_string:block, from_vec $from_vec_input:ident $from_vec:block ) => {
        validated_customized_vec!($name, $from_string_input $from_string, $from_str_input $from_str, $from_vec_input $from_vec);
    };
    ( $name:ident, from_vec $from_vec_input:ident $from_vec:block, from_string $from_string_input:ident $from_string:block, from_str $from_str_input:ident $from_str:block ) => {
        validated_customized_vec!($name, $from_string_input $from_string, $from_str_input $from_str, $from_vec_input $from_vec);
    };
    ( $name:ident, from_vec $from_vec_input:ident $from_vec:block, from_str $from_str_input:ident $from_str:block, from_string $from_string_input:ident $from_string:block ) => {
        validated_customized_vec!($name, $from_string_input $from_string, $from_str_input $from_str, $from_vec_input $from_vec);
    };
    ( $name:ident, from_string $from_string_input:ident $from_string:block, from_vec $from_vec_input:ident $from_vec:block, from_str $from_str_input:ident $from_str:block ) => {
        validated_customized_vec!($name, $from_string_input $from_string, $from_str_input $from_str, $from_vec_input $from_vec);
    };
    ( $name:ident, from_str $from_str_input:ident $from_str:block, from_vec $from_vec_input:ident $from_vec:block, from_string $from_string_input:ident $from_string:block ) => {
        validated_customized_vec!($name, $from_string_input $from_string, $from_str_input $from_str, $from_vec_input $from_vec);
    };
    ( pub $name:ident, $from_string_input:ident $from_string:block, $from_str_input:ident $from_str:block, $from_vec_input:ident $from_vec:block ) => {
        pub struct $name<T: ::validators::ValidatedWrapper> {
            v: Vec<T>
        }

        validated_customized_vec_struct!($name, v, $from_string_input $from_string, $from_str_input $from_str, $from_vec_input $from_vec);
    };
    ( pub $name:ident, from_string $from_string_input:ident $from_string:block, from_str $from_str_input:ident $from_str:block, from_vec $from_vec_input:ident $from_vec:block ) => {
        validated_customized_vec!(pub $name, $from_string_input $from_string, $from_str_input $from_str, $from_vec_input $from_vec);
    };
    ( pub $name:ident, from_str $from_str_input:ident $from_str:block, from_string $from_string_input:ident $from_string:block, from_vec $from_vec_input:ident $from_vec:block ) => {
        validated_customized_vec!(pub $name, $from_string_input $from_string, $from_str_input $from_str, $from_vec_input $from_vec);
    };
    ( pub $name:ident, from_vec $from_vec_input:ident $from_vec:block, from_string $from_string_input:ident $from_string:block, from_str $from_str_input:ident $from_str:block ) => {
        validated_customized_vec!(pub $name, $from_string_input $from_string, $from_str_input $from_str, $from_vec_input $from_vec);
    };
    ( pub $name:ident, from_vec $from_vec_input:ident $from_vec:block, from_str $from_str_input:ident $from_str:block, from_string $from_string_input:ident $from_string:block ) => {
        validated_customized_vec!(pub $name, $from_string_input $from_string, $from_str_input $from_str, $from_vec_input $from_vec);
    };
    ( pub $name:ident, from_string $from_string_input:ident $from_string:block, from_vec $from_vec_input:ident $from_vec:block, from_str $from_str_input:ident $from_str:block ) => {
        validated_customized_vec!(pub $name, $from_string_input $from_string, $from_str_input $from_str, $from_vec_input $from_vec);
    };
    ( pub $name:ident, from_str $from_str_input:ident $from_str:block, from_vec $from_vec_input:ident $from_vec:block, from_string $from_string_input:ident $from_string:block ) => {
        validated_customized_vec!(pub $name, $from_string_input $from_string, $from_str_input $from_str, $from_vec_input $from_vec);
    };
}

#[macro_export]
macro_rules! validated_customized_ranged_length_vec_struct {
    ( $name:ident, $field:expr, $min:expr, $max:expr, $from_string_input:ident $from_string:block, $from_str_input:ident $from_str:block) => {
        validated_customized_vec_struct!($name, v,
        $from_string_input $from_string,
        $from_str_input $from_str,
        input {
            let len = input.len();

            if len > $max {
                Err(::validators::ValidatedCustomizedVecError::Overflow)
            } else if len < $min {
                Err(::validators::ValidatedCustomizedVecError::Underflow)
            } else {
                Ok(input)
            }
        });
    };
}

#[macro_export]
macro_rules! validated_customized_ranged_length_vec {
    ( $name:ident, $min:expr, $max:expr, $from_string_input:ident $from_string:block, $from_str_input:ident $from_str:block) => {
        struct $name<T: ::validators::ValidatedWrapper> {
            v: Vec<T>
        }

        validated_customized_ranged_length_vec_struct!($name, v, $min, $max, $from_string_input $from_string, $from_str_input $from_str);
    };
    ( $name:ident, $min:expr, $max:expr, from_string $from_string_input:ident $from_string:block, from_str $from_str_input:ident $from_str:block) => {
        validated_customized_ranged_length_vec!($name, $min, $max, $from_string_input $from_string, $from_str_input $from_str);
    };
    ( $name:ident, $min:expr, $max:expr, from_str $from_str_input:ident $from_str:block, from_string $from_string_input:ident $from_string:block) => {
        validated_customized_ranged_length_vec!($name, $min, $max, $from_string_input $from_string, $from_str_input $from_str);
    };
    ( $name:ident, $min:expr, $max:expr) => {
        validated_customized_ranged_length_vec!($name, $min, $max,
        _input {Err(::validators::ValidatedCustomizedVecError::NotSupport)},
        _input {Err(::validators::ValidatedCustomizedVecError::NotSupport)});
    };
    ( $name:ident, $equal:expr, $from_string_input:ident $from_string:block, $from_str_input:ident $from_str:block) => {
        validated_customized_ranged_length_vec!($name, $equal, $equal, $from_string_input $from_string, $from_str_input $from_str);
    };
    ( $name:ident, $equal:expr) => {
        validated_customized_ranged_length_vec!($name, $equal, $equal);
    };
    ( pub $name:ident, $min:expr, $max:expr, $from_string_input:ident $from_string:block, $from_str_input:ident $from_str:block) => {
        pub struct $name<T: ::validators::ValidatedWrapper> {
            v: Vec<T>
        }

        validated_customized_ranged_length_vec_struct!($name, v, $min, $max, $from_string_input $from_string, $from_str_input $from_str);
    };
    ( pub $name:ident, $min:expr, $max:expr, from_string $from_string_input:ident $from_string:block, from_str $from_str_input:ident $from_str:block) => {
        validated_customized_ranged_length_vec!(pub $name, $min, $max, $from_string_input $from_string, $from_str_input $from_str);
    };
    ( pub $name:ident, $min:expr, $max:expr, from_str $from_str_input:ident $from_str:block, from_string $from_string_input:ident $from_string:block) => {
        validated_customized_ranged_length_vec!(pub $name, $min, $max, $from_string_input $from_string, $from_str_input $from_str);
    };
    ( pub $name:ident, $min:expr, $max:expr) => {
        validated_customized_ranged_length_vec!(pub $name, $min, $max,
        _input {Err(::validators::ValidatedCustomizedVecError::NotSupport)},
        _input {Err(::validators::ValidatedCustomizedVecError::NotSupport)});
    };
    ( pub $name:ident, $equal:expr, $from_string_input:ident $from_string:block, $from_str_input:ident $from_str:block) => {
        validated_customized_ranged_length_vec!(pub $name, $equal, $equal, $from_string_input $from_string, $from_str_input $from_str);
    };
    ( pub $name:ident, $equal:expr) => {
        validated_customized_ranged_length_vec!(pub $name, $equal, $equal);
    };
}

// TODO -----ValidatedCustomizedRangedLengthVec End-----
