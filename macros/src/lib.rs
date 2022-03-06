use darling::FromDeriveInput;
use proc_macro::{self, TokenStream};
use quote::quote;
use syn::{parse_macro_input, DeriveInput, Ident};

#[derive(FromDeriveInput, Debug)]
#[darling(attributes(dereference), supports(enum_any))]
struct DereferenceOpts {
    pub client: Ident,
}

#[proc_macro_derive(Dereference, attributes(dereference))]
pub fn dereference(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as syn::DeriveInput);
    match FromDeriveInput::from_derive_input(&input).and_then(|opts| impl_dereference(input, opts))
    {
        Ok(x) => x,
        Err(e) => e.write_errors().into(),
    }
}

fn impl_dereference(
    input: DeriveInput,
    opts: DereferenceOpts,
) -> Result<TokenStream, darling::Error> {
    let ident = &input.ident;
    let client_mod = opts.client;
    let output = quote! {
        use crate::{async_trait, Result, #client_mod};
        #[async_trait]
        impl Dereference for #ident {
            type Output = Model;
            async fn dereference(&self, client: &crate::Client) ->  Result<Self::Output> {
                match self {
                    Self::Reference(r) => #client_mod::get(client, &r.id).await,
                    Self::Model(model) => Ok(model.to_owned())
                }
            }
        }
        #[async_trait]
        impl Dereference for Vec<#ident>
        {
            type Output = Vec<Model>;
            async fn dereference(&self, client: &crate::Client) -> Result<Self::Output> {
                let results =
                    futures::future::join_all(self.iter().map(|entry| entry.dereference(client))).await;
                results.into_iter().collect()
            }
        }

    };
    Ok(output.into())
}

#[proc_macro_derive(BaseModel)]
pub fn base_model(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as syn::DeriveInput);
    match impl_base_model(input) {
        Ok(x) => x,
        Err(e) => e.write_errors().into(),
    }
}

fn impl_base_model(input: DeriveInput) -> Result<TokenStream, darling::Error> {
    let ident = &input.ident;
    let impl_eq = impl_eq(ident);
    let impl_hash = impl_hash(ident);
    let base_model = match input.data {
        syn::Data::Enum(x) => {
            let variants: Vec<proc_macro2::Ident> = x
                .variants
                .into_iter()
                .map(|entry| entry.ident.clone())
                .collect();
            quote! {
                impl BaseModel for #ident {
                    fn id(&self) -> &str {
                        match self {
                            #( Self::#variants(v) => v.id(), )*
                        }
                    }
                    fn summary(&self) -> &str {
                        match self {
                            #( Self::#variants(v) => v.summary(), )*
                        }
                    }
                    fn html_url(&self) -> &http::Uri {
                        match self {
                            #( Self::#variants(v) => v.html_url(), )*
                        }
                    }
                }
            }
        }
        syn::Data::Struct(_) => quote! {
            impl BaseModel for #ident {
                fn id(&self) -> &str {
                    &self.id
                }
                fn summary(&self) -> &str {
                    &self.summary
                }
                fn html_url(&self) -> &http::Uri {
                    &self.html_url
                }
            }
        },
        _ => {
            return Err(syn::Error::new(
                input.ident.span(),
                "Only enums or structs can be used for model fields",
            )
            .into())
        }
    };
    let output = quote! {
        #base_model
        #impl_eq
        #impl_hash
    };
    Ok(output.into())
}

fn impl_eq(ident: &Ident) -> proc_macro2::TokenStream {
    let output = quote! {
        impl PartialEq for #ident {
            fn eq(&self, other: &#ident) -> bool {
                self.id() == other.id()
            }
        }
        impl Eq for #ident {}
    };
    output.into()
}

fn impl_hash(ident: &Ident) -> proc_macro2::TokenStream {
    let output = quote! {
        impl std::hash::Hash for #ident {
            fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
                self.id().hash(state);
            }
        }
    };
    output.into()
}
