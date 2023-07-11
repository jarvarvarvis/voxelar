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
    ident: syn::Ident,

    binding: syn::LitInt,

    data: ast::Data<(), FieldInputArgs>,
}

#[derive(Debug, FromField)]
#[darling(attributes(input))]
struct FieldInputArgs {
    ident: Option<syn::Ident>,
    ty: syn::Type,

    #[darling(skip)]
    parent_ty_ident: Option<syn::Ident>,

    location: syn::LitInt,
    binding: syn::LitInt,

    format: syn::Expr,
}

impl ToTokens for FieldInputArgs {
    fn to_tokens(&self, tokens: &mut proc_macro2::TokenStream) {
        let parent_ty_ident = self
            .parent_ty_ident
            .as_ref()
            .expect("Parent type identifier was not set");
        let member_ident = self.ident.as_ref().expect("Fields must have a name");
        let location = &self.location;
        let binding = &self.binding;
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

    let field_input_args = spec_args
        .data
        .take_struct()
        .unwrap()
        .map(|mut f| {
            f.parent_ty_ident = Some(name.clone());
            f
        })
        .fields;
    let input_binding = spec_args.binding;

    let expanded = quote! {
        impl voxelar_vertex::VertexInput for #name {
            fn input_state_info() -> (VertexInputStateInfoConstructionData, PipelineVertexInputStateCreateInfo) {
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
                let vertex_input_state_info = PipelineVertexInputStateCreateInfo::builder()
                    .vertex_attribute_descriptions(&construction_data.vertex_input_attribute_descriptions)
                    .vertex_binding_descriptions(&construction_data.vertex_input_binding_descriptions)
                    .build();

                (construction_data, vertex_input_state_info)
            }
        }
    };
    expanded.into()
}