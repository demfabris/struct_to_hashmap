use proc_macro2::TokenStream;
use quote::{quote, quote_spanned};
use syn::spanned::Spanned;
use syn::{parse_macro_input, Data, DeriveInput, Fields};

#[proc_macro_derive(ToHashMap)]
pub fn test_macro(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    let input = parse_macro_input!(input as DeriveInput);

    let name = input.ident;

    let assemble = assemble_hashmap(&input.data);

    let expanded = quote! {
        impl ToHashMap for #name {
            fn to_hashmap(&self) -> HashMap<String, Wrapper> {
                let mut map: HashMap<String, Wrapper> = HashMap::new();
                #assemble
                map
            }
        }
    };

    proc_macro::TokenStream::from(expanded)
}

fn assemble_hashmap(data: &Data) -> TokenStream {
    match *data {
        Data::Struct(ref data) => match data.fields {
            Fields::Named(ref fields) => {
                let recurse = fields.named.iter().map(|f| {
                    let name = &f.ident;
                    quote_spanned! {f.span()=>
                        map.insert(std::stringify!(#name).to_string(),
                            to_hashmap::Wrapped::as_wrapper(&self.#name));
                    }
                });
                quote! {
                    #(#recurse)*
                }
            }
            _ => {
                quote! {
                    panic!("Only named structs supported")
                }
            }
        },
        _ => {
            quote! {
                panic!("Only structs supported")
            }
        }
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
