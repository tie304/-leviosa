use inflector::Inflector;
use proc_macro::TokenStream;
use quote::format_ident;
use quote::quote;
use syn::{
    parse_macro_input, Data, DeriveInput, Fields, GenericArgument, PathArguments, Type, TypePath,
};

fn is_option_type(ty: &Type) -> bool {
    if let Type::Path(TypePath { path, .. }) = ty {
        if path.segments.len() == 1 {
            let segment = &path.segments[0];
            if segment.ident == "Option" {
                if let PathArguments::AngleBracketed(args) = &segment.arguments {
                    return args.args.len() == 1
                        && matches!(args.args.first(), Some(GenericArgument::Type(_)));
                }
            }
        }
    }
    false
}

#[proc_macro_attribute]
pub fn leviosa(_attr: TokenStream, item: TokenStream) -> TokenStream {
    let input = parse_macro_input!(item as DeriveInput);
    let name = &input.ident;
    let struct_name_snake_case = name.to_string().to_snake_case();

    let methods = if let Data::Struct(data) = &input.data {
        match &data.fields {
            Fields::Named(fields) => fields.named.iter().map(|f| {
                let field_name = f.ident.as_ref().unwrap();
                let ty = &f.ty;
                let get_fn_name = format_ident!("get_by_{}", field_name);
                let update_fn_name = format_ident!("update_{}", field_name);
                
             
                // Generate get_by_ and update_ methods
                quote! {
                    pub async fn #get_fn_name(pool: &sqlx::PgPool, value: &#ty) -> sqlx::Result<Option<Self>> {
                        let query = format!("SELECT * FROM {} WHERE {} = $1", #struct_name_snake_case, stringify!(#field_name));
                        sqlx::query_as::<_, Self>(&query)
                            .bind(value)
                            .fetch_optional(pool).await
                    }
    
                    pub async fn #update_fn_name(&mut self, pool: &sqlx::PgPool, new_value: &#ty) -> sqlx::Result<()> {
                        let query = format!("UPDATE {} SET {} = $2 WHERE id = $1", #struct_name_snake_case, stringify!(#field_name));
                        sqlx::query(&query)
                            .bind(self.id)
                            .bind(new_value)
                            .execute(pool).await?;
                        self.#field_name = new_value.clone();
                        Ok(())
                    }
                }
            }).collect(),
            _ => quote! {},
        }
    } else {
        quote! {}
    };

    let create_method = if let Data::Struct(data) = &input.data {
        match &data.fields {
            Fields::Named(fields) => {
                let field_params =
                    fields
                        .named
                        .iter()
                        .filter(|f| !is_option_type(&f.ty))
                        .map(|f| {
                            let field_name = f.ident.as_ref().unwrap();
                            let ty = &f.ty;
                            quote! { #field_name: #ty }
                        });

                let field_tokens =
                    fields
                        .named
                        .iter()
                        .filter(|f| !is_option_type(&f.ty))
                        .map(|f| {
                            let field_name = f.ident.as_ref().unwrap();
                            quote! { #field_name }
                        });

                let joined_fields = fields
                    .named
                    .iter()
                    .filter(|f| !is_option_type(&f.ty))
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
                    pub async fn create(
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

    let query_builder_name = format_ident!(
        "{}FindAllQueryBuilder",
        input.ident.to_string().to_camel_case()
    );

    let find_all_query_builder = quote! {
        #[derive(Clone)]
        struct #query_builder_name {
            query: String,
            limit: Option<usize>,
            where_clause: Option<String>,
            order_by_clause: Option<String>
        }

        impl #query_builder_name {
            fn new() -> Self {
                Self {
                    query: format!("SELECT * FROM {}", #struct_name_snake_case),
                    limit: None,
                    where_clause: None,
                    order_by_clause: None
                }
            }

            fn limit(&mut self, limit: usize) -> &mut Self {
                self.limit = Some(limit);
                self
            }

            fn r#where(&mut self, _where: &str) -> &mut Self {
                self.where_clause = Some(String::from(_where));
                self
            }


            fn order_by(&mut self, order_by: &str) -> &mut Self {
                self.order_by_clause = Some(String::from(order_by));
                self
            }


            pub async fn execute(&self, pool: &PgPool) -> sqlx::Result<Vec<#name>> {
                let mut query = self.query.clone();
                if let Some(ref where_clause) = self.where_clause {
                    query.push_str(" WHERE ");
                    query.push_str(where_clause);
                }

                if let Some(ref order_by) = self.order_by_clause {
                    query.push_str(" ORDER BY ");
                    query.push_str(order_by);
                }

                // TODO impl group by, having

                if let Some(limit) = self.limit {
                    query.push_str(&format!(" LIMIT {}", limit));
                }

                sqlx::query_as::<_, #name>(&query)
                    .fetch_all(pool)
                    .await
            }
        }
    };

    // Define the find_all method for the struct
    let find_all_method = quote! {
        pub fn find() -> #query_builder_name {
            #query_builder_name::new()
        }
    };

    let constructor = if let Data::Struct(data) = &input.data {
        match &data.fields {
            Fields::Named(fields) => {
                let field_params = fields.named.iter().map(|f| {
                    let field_name = f.ident.as_ref().unwrap();
                    let ty = &f.ty;
                    quote! { #field_name: #ty }
                });

                // Extract field names for struct initialization
                let field_names = fields.named.iter().map(|f| {
                    let field_name = f.ident.as_ref().unwrap();
                    quote! { #field_name: #field_name }
                });
                quote! {
                    pub fn new(#(#field_params),*) -> Self {
                        Self {
                            #(#field_names),*
                        }
                    }
                }
            }
            _ => {
                quote! {}
            }
        }
    } else {
        quote! {}
    };

    let delete_method = quote! {
        pub async fn delete(&mut self, pool: &sqlx::PgPool) -> sqlx::Result<()> {
            let query = format!("DELETE FROM {} WHERE id = $1", #struct_name_snake_case);
            sqlx::query(&query)
                .bind(self.id)
                .execute(pool)
                .await?;
            Ok(())
        }
    };

    let delete_by_id_method = quote! {
        pub async fn delete_by_id(pool: &sqlx::PgPool, id: i32) -> sqlx::Result<()> {
            let query = format!("DELETE FROM {} WHERE id = $1", #struct_name_snake_case);
            sqlx::query(&query)
                .bind(id)
                .execute(pool)
                .await?;
            Ok(())
        }
    };

    let expanded = quote! {
        #input

        #find_all_query_builder

        impl #name {
            #methods
            #find_all_method
            #delete_method
            #delete_by_id_method
            #create_method
            #constructor

        }
    };

    TokenStream::from(expanded)
}
