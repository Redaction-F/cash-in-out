extern crate proc_macro;

use proc_macro::{TokenStream as TokenStream1};
use proc_macro2::{Span, TokenStream};
use quote::{format_ident, quote};
use syn::{Data, DataStruct, DeriveInput, Expr, Fields, Ident, LitInt, Type, parse_macro_input, punctuated::Punctuated};

#[proc_macro_derive(MyDeserialize, attributes(MyDeserialize))]
/// Implment `Deserialize` trait with field aliases which are in camel case and have an underscore at the beginning.(e.g. user_name => userName, _userName)
/// 
/// If you want to set default value of deserialize, you can use `#[MyDeserialize(default=)]`.
pub fn my_deserialize_derive(input: TokenStream1) -> TokenStream1 {
    // read an input as a struct data
    let input: DeriveInput = parse_macro_input!(input as DeriveInput);
    let data: DataStruct = match input.data {
        Data::Struct(d) => d,
        _ => panic!("A Struct is required.")
    };

    // get a struct name
    let name: Ident = input.ident;
    // get struct fields
    let fields: Punctuated<syn::Field, syn::token::Comma> = match data.fields {
        Fields::Named(f) => f.named,
        _ => panic!("A struct with named fields is required.")
    };

    // create a token expressing an array which contains all field names
    let fields_array: TokenStream = {
        let fields_tmp = fields.iter().map(|field| {
            let ident: &Ident = field.ident.as_ref().unwrap();
            quote! {
                stringify!(#ident)
            }
        });
        quote! {
            [#(#fields_tmp),*]
        }
    };
    // create a token expressing mutable let bindings which correspond fields.
    let fields_var: TokenStream = {
        let fields_tmp = fields.iter().map(|field| {
            let ident: &Ident = field.ident.as_ref().unwrap();
            let ty: &Type = &field.ty;

            quote! {
                let mut #ident: Option<#ty> = None;
            }
        });
        quote! {
            #(#fields_tmp)*
        }
    };
    // create a token expressing match arms which correspond field names.
    let fields_match: TokenStream = {
        let fields_tmp = fields.iter().map(|field| {
            let field_name: &Ident = field.ident.as_ref().unwrap();
            let camel_field_name: String = snake_to_camel(field_name.to_string());
            let underbar_field_name: String = format!("_{}", camel_field_name);

            quote! {
                #camel_field_name | #underbar_field_name => {
                    if #field_name.is_some() {
                        let e = ::serde::de::Error::duplicate_field("id");
                        return Err(e);
                    }
                    #field_name = Some(map.next_value()?)
                },
            }
        });
        quote! {
            #(#fields_tmp)*
        }
    };
    // create a token expressing a struct constructor from vars which has the same name as fields.
    let fields_expand: TokenStream = {
        let fields_tmp = fields.iter().map(|field| {
            let field_name: &Ident = field.ident.as_ref().unwrap();
            let default: Option<Expr> = field
                .attrs
                .iter()
                .filter(|attr| attr.path().is_ident("MyDeserialize"))
                .next()
                .map(|attr| {
                    let mut default_value: Option<::syn::Expr> = None;
                    attr
                        .parse_nested_meta(|v| {
                            if v.path.is_ident("default") {
                                default_value = Some(v.value()?.parse()?);
                            };
                            Ok(())
                        })
                        .unwrap();
                    default_value
                })
                .flatten();
            
            match default {
                Some(v) => quote! {
                    #field_name: #field_name.unwrap_or(#v),
                },
                None => quote! {
                    #field_name: #field_name.ok_or_else(|| {
                        let e = ::serde::de::Error::missing_field(stringify!(#field_name));
                        ::log::error!("{:?}", e);
                        e
                    })?,
                }
            }
        });
        quote! {
            #(#fields_tmp)*
        }
    };
    // cerate visitor struct name for deserializing
    let visitor_name: Ident = format_ident!("{}Visitor", name);

    let expanded: TokenStream = quote! {
        impl<'de> ::serde::Deserialize<'de> for #name {
            fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
            where
                D: serde::Deserializer<'de>
            {
                deserializer.deserialize_struct(stringify!(#name), &#fields_array, #visitor_name)
            }
        }

        struct #visitor_name;

        impl<'de> ::serde::de::Visitor<'de> for #visitor_name {
            type Value = #name;

            fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
                write!(formatter, "fields: {}", &#fields_array.join(","))
            }

            fn visit_map<A>(self, map: A) -> Result<Self::Value, A::Error>
            where
                A: ::serde::de::MapAccess<'de>,
            {
                let mut map: A = map;
                #fields_var;

                while let Some(key) = map.next_key::<String>()? {
                    match key.as_str() {
                        #fields_match
                        v => {
                            let e = ::serde::de::Error::unknown_field(v, &#fields_array);
                            return Err(e);
                        }
                    }
                }

                Ok(
                    #name {
                        #fields_expand
                    }
                )
            }
        }
    };

    expanded.into()
}


#[proc_macro_derive(MySerialize)]
pub fn my_serialize_derive(input: TokenStream1) -> TokenStream1 {
    // read an input as a struct data
    let input: DeriveInput = parse_macro_input!(input as DeriveInput);
    let data: DataStruct = match input.data {
        Data::Struct(d) => d,
        _ => panic!("A Struct is required.")
    };

    // get a struct name
    let name: Ident = input.ident;
    // get struct fields
    let fields: Punctuated<syn::Field, syn::token::Comma> = match data.fields {
        Fields::Named(f) => f.named,
        _ => panic!("A struct with named fields is required.")
    };
    let fields_serialize: TokenStream = {
        let fields_tmp = fields
            .iter()
            .map(|field| {
                let field_name: &Ident = field.ident.as_ref().unwrap();
                let camel_field_name: String = snake_to_camel(field_name.to_string());

                quote! {
                    s.serialize_field(#camel_field_name, &self.#field_name).map_err(|e| {
                        ::log::error!("{}", e);
                        e
                    })?;
                }
            });
        quote! {
            #(#fields_tmp)*
        }
    };
    let fields_len: LitInt = LitInt::new(fields.len().to_string().as_str(), Span::call_site());

    let expanded: TokenStream = quote! {
        impl ::serde::Serialize for #name {
            fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
            where
                S: ::serde::Serializer,
            {
                use ::serde::ser::SerializeStruct;

                let mut s: <S as serde::Serializer>::SerializeStruct =
                    serializer.serialize_struct(stringify!(#name), #fields_len)?;
                #fields_serialize
                s.end().map_err(|e| {
                    ::log::error!("{}", e);
                    e
                })
            }
        }
    };

    expanded.into()
}

fn snake_to_camel(str: String) -> String {
    let mut str_splited = str.split("_");
    let first_block: &str = str_splited.next().unwrap();
    let other_blocks: String = str_splited.map(|b| {
        let b_vec: Vec<char> = b.to_string().chars().collect::<Vec<char>>();
        format!("{}{}", b_vec.get(0).copied().unwrap_or_default().to_ascii_uppercase(), b_vec.into_iter().skip(1).collect::<String>())
    })
        .collect::<String>();
    format!("{}{}", first_block, other_blocks)
}