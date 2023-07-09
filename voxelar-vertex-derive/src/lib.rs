use darling::FromDeriveInput;

use proc_macro::TokenStream;

use syn::parse_macro_input;
use syn::DeriveInput;

use quote::quote;

#[derive(Debug, FromDeriveInput)]
struct VertexInputSpecArgs {
}

#[proc_macro_derive(VertexInput)]
pub fn vertex_input_derive(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    let parsed_input = parse_macro_input!(input as DeriveInput);

    let name = &parsed_input.ident;

    let spec_args = match VertexInputSpecArgs::from_derive_input(&parsed_input) {
        Ok(v) => v,
        Err(e) => {
            return TokenStream::from(e.write_errors());
        }
    };

    println!("{:#?}", spec_args);

    let expanded = quote! {
        impl voxelar_vertex::VertexInput for #name {
            fn input_state_info() -> PipelineVertexInputStateCreateInfo { 
                todo!() 
            }

            fn input_assembly_state_info() -> PipelineInputAssemblyStateCreateInfo { 
                todo!() 
            }
        }
    };
    expanded.into()
}
