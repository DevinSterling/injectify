use proc_macro::TokenStream;
use quote::{format_ident, quote, ToTokens};
use syn::parse::Nothing;
use syn::spanned::Spanned;
use syn::{parse_macro_input, Data, DataStruct, DeriveInput, Error, Expr};

#[proc_macro_attribute]
#[allow(non_snake_case)]
pub fn Injectify(args: TokenStream, input: TokenStream) -> TokenStream {
    let _ = parse_macro_input!(args as Nothing);
    let input = parse_macro_input!(input as DeriveInput);

    match input.clone().data {
        Data::Struct(data) => injectify_struct_impl(data, input),
        Data::Enum(_) => todo!(),
        _ => Error::new(input.into_token_stream().span(), "Must be a `struct`")
            .into_compile_error()
            .to_token_stream()
            .into(),
    }
}

fn injectify_struct_impl(struct_data: DataStruct, derive_input: DeriveInput) -> TokenStream {
    // Original data
    let vis = derive_input.vis;
    let ident = derive_input.ident;
    let attrs = derive_input.attrs;
    let generics_params = derive_input.generics.params;
    let generics_where = derive_input.generics.where_clause;

    // New generics to insert into struct
    let mut generated_generics = Vec::new();

    let fields: Vec<_> = struct_data
        .fields
        .iter()
        .map(|field| {
            let vis = &field.vis;
            let attrs = &field.attrs;
            let ident = &field.ident;
            let field_type = field.ty.to_token_stream().to_string();

            // Field to modify
            if field_type.starts_with("impl ") {
                let trait_str = field_type
                    .strip_prefix("impl ")
                    .expect("Should have prefix");
                let impl_trait: Expr = syn::parse_str(trait_str).expect("Should be an expression");
                let generic = format_ident!("_IJ_{}", generated_generics.len());

                generated_generics.push(quote!(
                    #generic: #impl_trait,
                ));

                quote!(
                    #(#attrs)*
                    #vis #ident: #generic,
                )
            }
            // Keep field as is
            else {
                quote!(#field,)
            }
        })
        .collect();

    quote!(
        #(#attrs)*
        #vis struct #ident <#(#generated_generics)* #generics_params> #generics_where {
            #(#fields)*
        }
    )
    .into()
}
