use proc_macro::TokenStream;

use inflector::Inflector;
use quote::format_ident;
use quote::quote;
use syn::Ident;
use syn::{Data, DeriveInput, Fields};

use crate::utils::{extract_relation_generic_type, is_field_type, type_to_string_identifier};

pub fn many_to_many_methods(name: &Ident, input: &DeriveInput) -> TokenStream {
    let struct_name_snake_case = name.to_string().to_snake_case();

    let create_method = if let Data::Struct(data) = &input.data {
        match &data.fields {
            Fields::Named(fields) => {
                let field_params = fields
                    .named
                    .iter()
                    .map(|f| {
                        let field_name = f.ident.as_ref().unwrap();
                        let ty = &f.ty;
                        quote! { #field_name: #ty }
                    });

                let field_tokens = fields
                    .named
                    .iter()
                    .map(|f| {
                        let field_name = f.ident.as_ref().unwrap();
                        quote! { #field_name }
                    });

                let joined_fields = fields
                    .named
                    .iter()
                    .map(|f| f.ident.as_ref().unwrap().to_string())
                    .collect::<Vec<_>>()
                    .join(", ");

                let values_str: Vec<String> = joined_fields
                    .split(", ")
                    .enumerate()
                    .map(|(i, _)| format!("${}", i + 1))
                    .collect();
                let values_str = values_str.join(", ");

                let struct_name_snake_case =
                    format_ident!("{}", input.ident.to_string().to_snake_case());
                let query_str = format!(
                    "INSERT INTO {} ({}) VALUES ({}) RETURNING *",
                    struct_name_snake_case, joined_fields, values_str
                );

                quote! {
                    pub async fn associate(
                        pool: &sqlx::PgPool,
                        #(#field_params),*
                    ) -> Result<Self, sqlx::Error> {
                        let new_entity = sqlx::query_as::<_, Self>(&#query_str)
                            #( .bind(#field_tokens) )*
                            .fetch_one(pool) // Execute query within the transaction
                            .await?;
                        Ok(new_entity)
                    }
                }
            }
            _ => quote! {},
        }
    } else {
        quote! {}
    };

    let many_to_many = quote! {
        #input


        impl #name {
           #create_method
        }
    };

    TokenStream::from(many_to_many)
}
