use darling::ast;
use darling::{FromDeriveInput, FromField};

use proc_macro::TokenStream;

use syn::parse_macro_input;
use syn::DeriveInput;

use quote::quote;
use quote::ToTokens;

#[derive(Debug, FromDeriveInput)]
#[darling(attributes(vertex_input_spec), supports(struct_named))]
struct VertexInputSpecArgs {
    #[allow(unused)]
    ident: syn::Ident,

    binding: syn::LitInt,

    data: ast::Data<(), FieldInputArgs>,
}

#[derive(Debug, FromField)]
#[darling(attributes(input))]
struct FieldInputArgs {
    ident: Option<syn::Ident>,

    #[allow(unused)]
    ty: syn::Type,

    location: syn::LitInt,
    format: syn::Expr,

    #[darling(skip)]
    parent_binding_expr: Option<syn::LitInt>,

    #[darling(skip)]
    parent_ty_ident: Option<syn::Ident>,
}

impl ToTokens for FieldInputArgs {
    fn to_tokens(&self, tokens: &mut proc_macro2::TokenStream) {
        let parent_ty_ident = self
            .parent_ty_ident
            .as_ref()
            .expect("Parent type identifier was not set");
        let member_ident = self.ident.as_ref().expect("Fields must have a name");
        let location = &self.location;
        let binding = self
            .parent_binding_expr
            .as_ref()
            .expect("Parent binding expression was not set");
        let format = &self.format;
        let attribute_description_tokens = quote! {
            VertexInputAttributeDescription {
                location: #location,
                binding: #binding,
                format: #format,
                offset: voxelar_vertex::offset_of!(#parent_ty_ident, #member_ident) as u32,
            }
        };

        tokens.extend(attribute_description_tokens);
    }
}

#[proc_macro_derive(VertexInput, attributes(vertex_input_spec, input))]
pub fn vertex_input_derive(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    let parsed_input = parse_macro_input!(input as DeriveInput);

    let name = &parsed_input.ident;

    let spec_args = match VertexInputSpecArgs::from_derive_input(&parsed_input) {
        Ok(v) => v,
        Err(e) => {
            return TokenStream::from(e.write_errors());
        }
    };

    let input_binding = spec_args.binding;
    let field_input_args = spec_args
        .data
        .take_struct()
        .unwrap()
        .map(|mut f| {
            f.parent_ty_ident = Some(name.clone());
            f.parent_binding_expr = Some(input_binding.clone());
            f
        })
        .fields;

    let expanded = quote! {
        impl voxelar_vertex::VertexInput for #name {
            fn input_state_info() -> VertexInputStateInfoConstructionData {
                let vertex_input_binding_descriptions = vec![VertexInputBindingDescription {
                    binding: #input_binding,
                    stride: std::mem::size_of::<#name>() as u32,
                    input_rate: VertexInputRate::VERTEX,
                }];
                let vertex_input_attribute_descriptions = vec![
                    #(
                        #field_input_args
                    ),*
                ];

                let construction_data = VertexInputStateInfoConstructionData {
                    vertex_input_binding_descriptions,
                    vertex_input_attribute_descriptions
                };

                construction_data
            }
        }
    };
    expanded.into()
}
