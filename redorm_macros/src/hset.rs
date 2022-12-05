use crate::attributes::derive_attr;
use proc_macro::TokenStream;
use quote::quote;
use syn::Fields::Named;
use syn::{
    punctuated::Punctuated, token::Comma, Field, Ident, Meta, PathArguments::AngleBracketed, Type,
};

#[derive(Clone)]
struct FieldContainer {
    ident: Ident,
    conversion_method: proc_macro2::TokenStream,
}

pub fn derive_proc_macro_impl(input: TokenStream) -> TokenStream {
    let syn::ItemStruct {
        attrs,
        ident,
        fields,
        ..
    } = syn::parse::<syn::ItemStruct>(input).expect("Failed to parse tokens");
    let red_attr =
        derive_attr::Redorm::from_attributes(&attrs).expect("Failed to unwrap attributes");

    let prefix = match red_attr.prefix_name {
        Some(lit) => match lit {
            syn::Lit::Str(lit_string) => lit_string.value(),
            _ => panic! {"prefix_name must be a string"},
        },
        None => ident.to_string(),
    };

    let mut key_field_opt = None;
    let fields = match fields {
        Named(fields) => {
            for field in fields.named.clone() {
                for attr in field.attrs.iter() {
                    if let Some(ident) = attr.path.get_ident() {
                        if ident == "redorm" {
                            if let Ok(list) =
                                attr.parse_args_with(Punctuated::<Meta, Comma>::parse_terminated)
                            {
                                for meta in list.iter() {
                                    match meta {
                                        Meta::Path(p) => {
                                            if let Some(name) = p.get_ident() {
                                                if name == "key" {
                                                    key_field_opt = Some(field.clone());
                                                }
                                            }
                                        }
                                        _ => {}
                                    }
                                }
                            }
                        }
                    }
                }
            }
            fields.named
        }
        _ => panic!("This only supports named fields"),
    };

    let key_ident = if let None = key_field_opt {
        panic!("No key specified");
    } else {
        key_field_opt
            .clone()
            .expect("Failed to unwrap key field")
            .ident
            .expect("Failed to unwrap key field")
    };

    let id_conversion_method =
        get_conversion_method(&key_field_opt.expect("Failed to get id conversion method"));

    let fields = fields
        .into_iter()
        .filter(|it| {
            if let Some(ident) = &it.ident {
                if ident != &key_ident {
                    true
                } else {
                    false
                }
            } else {
                true
            }
        })
        .map(|field| {
            let nf = field.clone();
            let nf2 = nf.clone();
            FieldContainer {
                ident: field.ident.expect("Failed to unwrap key field ident"),
                conversion_method: get_conversion_method(&nf2),
            }
        })
        .collect();

    let attr_setters = get_attribute_setters(&fields);
    let attr_getters = get_attribute_getters(&fields);

    quote! {
        impl HashSet for #ident {
            fn gen_hset_args(&self) -> Vec<String> {
                let mut id = &self.#key_ident.to_string();
                let mut return_vec = Vec::new();
                return_vec.push(format!("{}:{}", #prefix, self.#key_ident));
                #(
                    #attr_setters
                )*
                return_vec
            }

            fn get_hset_from_args(args: &HashMap<String, String>) -> Self {
                let return_struct = Self {
                    #key_ident: args.get(stringify!(#key_ident)).expect("Couldn't find key arg in object hash map").clone()#id_conversion_method,
                    #(
                        #attr_getters
                    )*
                };
                return_struct
            }

            fn getall_command(&self) -> String {
                let key = &self.#key_ident;
                format!("HGETALL {}", key.to_string())
            }

            fn set_command(&self) -> String {
                let args = &self.gen_hset_args();
                format!("HSET {}", args.join(" "))
            }

            fn get_prefix() -> String {
                #prefix.to_string()
            }

            fn key_name() -> String {
                stringify!(#key_ident).to_string()
            }

            fn get_key(&self) -> &String {
                &self.#key_ident
            }
        }
    }
    .into()
}

fn get_attribute_getters(fields: &Vec<FieldContainer>) -> Vec<proc_macro2::TokenStream> {
    let mut tokens: Vec<proc_macro2::TokenStream> = Vec::new();
    for field in fields {
        let FieldContainer {
            ident,
            conversion_method,
        } = field;
        let token = quote! {
            #ident: args.get(stringify!(#ident)).expect(&format!("Deserialization from redis failed, key {} not found in {:#?}", stringify!(#ident), &args)).clone()#conversion_method,
        };
        tokens.push(token);
    }
    tokens
}

fn get_attribute_setters(fields: &Vec<FieldContainer>) -> Vec<proc_macro2::TokenStream> {
    let mut tokens: Vec<proc_macro2::TokenStream> = Vec::new();
    for field in fields {
        let FieldContainer { ident, .. } = field;
        let token = quote! {
            return_vec.push(stringify!(#ident).to_string());
            return_vec.push(self.#ident.clone().to_string());
        };
        tokens.push(token);
    }
    tokens
}

fn get_conversion_method(field: &Field) -> proc_macro2::TokenStream {
    match &field.ty {
        Type::Path(path) => match path.path.segments[0].ident.to_string().as_str() {
            "String" => {
                quote! {}
            }
            "usize" | "u8" | "u16" | "u32" | "u64" | "u128" | "isize" | "i8" | "i16" | "i32"
            | "i64" | "i128" | "f32" | "f64" => {
                quote! {
                    .parse().expect("Failed to serialize number to string")
                }
            }
            "NaiveDate" | "NaiveTime" | "Time" => {
                let ident = &path.path.segments[0].ident;
                quote! {
                    .parse::<#ident>().expect("Failed to serialize timestamp to string")
                }
            }
            // Support for DateTimes with Timezones
            "DateTime" => {
                let ident = &path.path.segments[0].ident;
                if let AngleBracketed(argument) = &path.path.segments[0].arguments {
                    if let syn::GenericArgument::Type(ty) = &argument.args[0] {
                        if let syn::Type::Path(tp) = ty {
                            let tz_ident = &tp.path.segments[0].ident;
                            return quote! {
                                .parse::<#ident<#tz_ident>>().expect("Failed to serialize timestamp to string")
                            };
                        }
                    }
                }
                panic!("DateTime timezone ident not found")
            }
            _ => {
                quote! {
                    .try_into().expect("Failed to unwrap unknown type. Ensure that it implements From<String>")
                }
            }
        },
        _ => panic!("Unimplemented"),
    }
}
