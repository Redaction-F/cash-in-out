extern crate proc_macro;

use proc_macro::TokenStream;
use quote::{format_ident, quote};
use syn::{parse_macro_input, DeriveInput};


#[proc_macro_derive(MyDeserialize)]
pub fn my_deserialize_derive(input: TokenStream) -> TokenStream {
    // 入力されたRustコードを解析
    let input = parse_macro_input!(input as DeriveInput);

    let data = match input.data {
        ::syn::Data::Struct(d) => d,
        _ => panic!("A Struct is required.")
    };

    let name = input.ident;
    let fields = match data.fields {
        ::syn::Fields::Named(f) => f.named,
        _ => panic!("A struct with named fields is required.")
    };
    let fields_array = {
        let fields_tmp = fields.iter().map(|field| {
            let ident = field.ident.as_ref().unwrap();

            quote! {
                stringify!(#ident)
            }
        });
        quote! {
            [#(#fields_tmp),*]
        }
    };
    let fields_var = {
        let fields_tmp = fields.iter().map(|field| {
            let ident = field.ident.as_ref().unwrap();
            let ty = &field.ty;

            quote! {
                let mut #ident: Option<#ty> = None;
            }
        });
        quote! {
            #(#fields_tmp)*
        }
    };
    let fields_match = {
        let fields_tmp = fields.iter().map(|field| {
            let field_name = field.ident.as_ref().unwrap();
            let camel_field_name = {
                let field_name_string = field_name.to_string();
                let mut field_name_splited = field_name_string.split("_");
                let first_field_name = field_name_splited.next().unwrap();
                let other_ident_field_names = field_name_splited.map(|v| {
                    let ident = v.to_string().chars().collect::<Vec<char>>();
                    format!("{}{}", ident.get(0).copied().unwrap_or_default().to_ascii_uppercase(), ident.into_iter().skip(1).collect::<String>())
                }).collect::<String>();
                format!("{}{}", first_field_name, other_ident_field_names)
            };
            let underbar_field_name = format!("_{}", camel_field_name);

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
    let fields_expand = {
        let fields_tmp = fields.iter().map(|field| {
            let field_name = field.ident.as_ref().unwrap();

            quote! {
                #field_name: #field_name.ok_or_else(|| {
                    let e = ::serde::de::Error::missing_field(stringify!($field_name));
                    ::log::error!("{:?}", e);
                    e
                })?,
            }
        });
        quote! {
            #(#fields_tmp)*
        }
    };

    let visitor_name = format_ident!("{}Visitor", name);

    let expanded = quote! {
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