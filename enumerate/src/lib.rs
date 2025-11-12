use proc_macro::TokenStream;
use quote::quote;
use syn::{
    parse_macro_input, punctuated::Punctuated, Data, DeriveInput, Lit, Meta, Token,
};

#[proc_macro_derive(Enumerate, attributes(alt))]
pub fn derive_enumerate(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);
    let name = &input.ident;

    let Data::Enum(data_enum) = &input.data else {
        return syn::Error::new_spanned(
            name,
            "Enumerate can only be derived for enums",
        )
        .to_compile_error()
        .into();
    };

    let mut variant_idents = Vec::new();
    let mut variant_names = Vec::new();
    let mut alt_lists = Vec::new();

    for variant in &data_enum.variants {
        let variant_ident = &variant.ident;
        let variant_name = variant_ident.to_string();

        variant_idents.push(variant_ident.clone());
        variant_names.push(variant_name.clone());

        // canonical name + alts
        let mut alts: Vec<String> = vec![variant_name.clone()];

        for attr in &variant.attrs {
            if let Meta::List(list) = &attr.meta {
                if list.path.is_ident("alt") {
                    let parsed: Punctuated<Lit, Token![,]> =
                        list.parse_args_with(Punctuated::parse_terminated)
                            .unwrap_or_default();
                    for lit in parsed {
                        if let Lit::Str(s) = lit {
                            alts.push(s.value());
                        }
                    }
                }
            }
        }

        alt_lists.push(alts);
    }

    // Build the arrays
    let alt_arrays = alt_lists.iter().map(|alts| {
        let lits = alts.iter();
        quote!(&[#(#lits),*])
    });

    // Build pattern arms for parsing function
    let parse_arms = variant_idents.iter().zip(alt_lists.iter()).map(|(ident, alts)| {
        let patterns = alts.iter().map(|a| a.to_ascii_lowercase());
        quote! {
            #( #patterns )|* => Some(Self::#ident),
        }
    });

    quote! {
        impl Enumerate for #name {
            fn variants() -> &'static [&'static str] {
                &[#(#variant_names),*]
            }

            fn alternatives() -> &'static [&'static [&'static str]] {
                &[#(#alt_arrays),*]
            }

            /// Parses a string into a variant, matching canonical or `#[alt(...)]` names (case-insensitive).
            fn parse(s: &str) -> Option<Self> {
                match s.to_ascii_lowercase().as_str() {
                    #(#parse_arms)*
                    _ => None,
                }
            }
        }
    }.into()
}
