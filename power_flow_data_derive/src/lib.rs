use proc_macro::TokenStream;
use quote::quote;
use syn::{parse_macro_input, DeriveInput};

#[proc_macro_derive(RawRecord, attributes(rev30, rev33, line2, line3, line4, line5))]
pub fn derive_raw_record(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);

    let name = input.ident;

    let expanded = match input.data {
        syn::Data::Struct(_data) => {
            quote! {
                impl RawRecord for #name {
                    fn parse_raw(input: &str) -> nom::IResult<&str, Self> {
                        Ok((input, Self::default()))
                    }
                }
            }
        }
        syn::Data::Enum(_) | syn::Data::Union(_) => {
            panic!("#[derive(RawRecord)] must be applied to a struct")
        }
    };

    TokenStream::from(expanded)
}
