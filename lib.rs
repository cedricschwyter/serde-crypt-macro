use proc_macro::TokenStream;
use quote::{format_ident, quote};
use syn::{parse::Parse, parse_macro_input, DeriveInput, Ident, Lit, Meta, MetaList, Token, Type};

struct SerdeCryptAttrStruct {
    ident: Ident,
    _punct: Token![=],
    literal: Lit,
}

impl Parse for SerdeCryptAttrStruct {
    fn parse(input: syn::parse::ParseStream) -> syn::Result<Self> {
        Ok(SerdeCryptAttrStruct {
            ident: input.parse()?,
            _punct: input.parse()?,
            literal: input.parse()?,
        })
    }
}

struct SerdeCryptTypes {
    e: Type,
    _punct: Token![,],
    d: Type,
}

impl Parse for SerdeCryptTypes {
    fn parse(input: syn::parse::ParseStream) -> syn::Result<Self> {
        Ok(SerdeCryptTypes {
            e: input.parse()?,
            _punct: input.parse()?,
            d: input.parse()?,
        })
    }
}

#[proc_macro_attribute]
pub fn serde_crypt_gen(_meta: TokenStream, input: TokenStream) -> TokenStream {
    let ast = parse_macro_input!(input as DeriveInput);
    let vis = ast.vis;
    let attrs = ast.attrs.iter().map(|e| quote! { #e }).reduce(|a, e| {
        quote! {
            #a
            #e
        }
    });
    let encrypted_ident = format_ident!("{}Encrypted", ast.ident);
    let decrypted_ident = format_ident!("{}Decrypted", ast.ident);
    let encrypted_fields = match &ast.data {
        syn::Data::Struct(ref data_struct) => data_struct
            .fields
            .iter()
            .map(|field| {
                let ident = &field.ident;
                let field_vis = &field.vis;
                let mut replace = false;
                let mut custom_types = false;
                let mut ty = field.ty.clone();
                let field_attrs = &field
                    .attrs
                    .iter()
                    .filter(|attr| {
                        if let Meta::List(MetaList { path, tokens, .. }) = &attr.meta {
                            let serde_tag: Result<SerdeCryptAttrStruct, _> =
                                syn::parse2(tokens.clone());
                            let crypt_types: Result<SerdeCryptTypes, _> =
                                syn::parse2(tokens.clone());
                            if serde_tag.is_ok() {
                                let tokens = serde_tag.unwrap();
                                let ident = tokens.ident.to_string();
                                let lit: String = match tokens.literal {
                                    Lit::Str(val) => val.value(),
                                    _ => return true,
                                };

                                if path.is_ident("serde") && ident == "with" && lit == "serde_crypt"
                                {
                                    replace = true;
                                    return false;
                                }
                            }
                            if crypt_types.is_ok() {
                                let tokens = crypt_types.unwrap();
                                let enc = tokens.e;
                                if path.is_ident("serde_crypt_types") {
                                    custom_types = true;
                                    replace = true;
                                    ty = enc;
                                    return false;
                                }
                            }
                        }
                        true
                    })
                    .map(|e| quote! {#e})
                    .reduce(|a, e| {
                        quote! {
                            #a
                            #e
                        }
                    });
                if replace {
                    if custom_types {
                        quote! {
                            #field_attrs
                            #field_vis #ident: #ty
                        }
                    } else {
                        quote! {
                            #field_attrs
                            #field_vis #ident: String
                        }
                    }
                } else {
                    quote! { #field }
                }
            })
            .reduce(|a, e| {
                return quote! {
                    #a,
                    #e
                };
            }),
        _ => panic!("#[serde_crypt] may only be used on structs"),
    };
    let decrypted_fields = match &ast.data {
        syn::Data::Struct(ref data_struct) => data_struct
            .fields
            .iter()
            .map(|field| {
                let ident = &field.ident;
                let field_vis = &field.vis;
                let mut custom_types = false;
                let mut ty = field.ty.clone();
                let field_attrs = &field
                    .attrs
                    .iter()
                    .filter(|attr| {
                        if let Meta::List(MetaList { path, tokens, .. }) = &attr.meta {
                            let crypt_types: Result<SerdeCryptTypes, _> =
                                syn::parse2(tokens.clone());
                            if crypt_types.is_ok() {
                                let tokens = crypt_types.unwrap();
                                let dec = tokens.d;
                                if path.is_ident("serde_crypt_types") {
                                    custom_types = true;
                                    ty = dec;
                                    return false;
                                }
                            }
                        }
                        true
                    })
                    .map(|e| quote! {#e})
                    .reduce(|a, e| {
                        quote! {
                            #a
                            #e
                        }
                    });
                if custom_types {
                    quote! {
                        #field_attrs
                        #field_vis #ident: #ty
                    }
                } else {
                    quote! { #field }
                }
            })
            .map(|e| quote! {#e})
            .reduce(|a, e| {
                quote! {
                    #a,
                    #e
                }
            }),
        _ => panic!("#[serde_crypt] may only be used on structs"),
    };

    let types = quote! {
        #attrs
        #vis struct #encrypted_ident {
            #encrypted_fields
        }

        #attrs
        #vis struct #decrypted_ident {
            #decrypted_fields
        }
    };
    types.into()
}
