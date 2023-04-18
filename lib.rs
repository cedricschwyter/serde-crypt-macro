use proc_macro::TokenStream;
use quote::{format_ident, quote};
use syn::{parse_macro_input, DeriveInput, Meta, MetaList};

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
                let field_attrs = &field
                    .attrs
                    .iter()
                    .filter(|attr| {
                        if let Meta::List(MetaList { path, tokens, .. }) = &attr.meta {
                            if path.is_ident("serde") {
                                // TODO: verify `tokens` to contain `with = "serde_crypt"`
                                replace = true;
                                return false;
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
                    quote! {
                        #field_attrs
                        #field_vis #ident: String
                    }
                } else {
                    quote! { #field }
                }
            })
            .reduce(|a, e| {
                return quote! {
                    #a
                    #e
                };
            }),
        _ => panic!("#[serde_crypt] may only be used on structs"),
    };
    let decrypted_fields = match &ast.data {
        syn::Data::Struct(ref data_struct) => data_struct
            .fields
            .iter()
            .map(|e| quote! {#e})
            .reduce(|a, e| {
                quote! {
                    #a
                    #e
                }
            }),
        _ => panic!("#[serde_crypt] may only be used on structs"),
    };

    let sealed_type = quote! {
        #attrs
        #vis struct #encrypted_ident {
            #encrypted_fields
        }

        #attrs
        #vis struct #decrypted_ident {
            #decrypted_fields
        }
    };
    sealed_type.into()
}
