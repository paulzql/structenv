use proc_macro::TokenStream;
use quote::ToTokens;
use proc_macro2::{Ident};

#[proc_macro_derive(StructEnv, attributes(env, prefix))]
pub fn struct_env_fn(input: TokenStream) -> TokenStream {
    let st = syn::parse_macro_input!(input as syn::DeriveInput);
    let ident = &st.ident;
    let mut fields: Vec<Ident> = Vec::new();
    let mut types: Vec<Ident> = Vec::new();
    let mut keys: Vec<String> = Vec::new();
    let mut prefixes: Vec<String> = Vec::new();
    match st.data {
        syn::Data::Struct(ref data_struct) => match data_struct.fields {
            syn::Fields::Named(ref fields_named) => {
                for field in fields_named.named.iter() {
                    let field_ident = field.ident.clone().unwrap();
                    let mut prefix: String = String::new();
                    let mut env_key: String = field_ident.to_string().to_uppercase();
                    let field_type: Ident = match &field.ty {
                        syn::Type::Path(syn::TypePath {ref path, ..}) => {
                            path.get_ident().unwrap().clone()
                        },
                        fty => panic!("Type `{:?}` of field `{}` not supported", fty.to_token_stream(), field_ident),
                    };
                    for attr in field.attrs.iter() {
                        if attr.path.is_ident("env") {
                            env_key = attr.parse_args::<syn::LitStr>().unwrap().value();
                        } else if attr.path.is_ident("prefix") {
                            prefix = attr.parse_args::<syn::LitStr>().unwrap().value();
                        }
                    }
                    fields.push(field_ident);
                    types.push(field_type);
                    keys.push(env_key);
                    prefixes.push(prefix);              
                }
            },
            syn::Fields::Unnamed(_) => {
                panic!("unsupport derive StructEnv for unamed fields");
            },
            syn::Fields::Unit => {
                panic!("unsupport derive StructEnv for unit fields");
            },
        },
        _ => (),
    }
    let ast = quote::quote! (
        impl EnvParser for #ident {
            fn read_env(key: &str, prefix: &str) -> std::io::Result<Self> {
                Ok(#ident {
                    #(#fields: #types::read_env(#keys, &format!("{}{}", prefix, #prefixes))?,)*
                })
            }
            fn to_env(&self, key: &str, prefix: &str) -> std::collections::HashMap<String, String> {
                let mut envs = std::collections::HashMap::new();
                for item in vec![#(self.#fields.to_env(&#keys, &format!("{}{}", prefix, #prefixes)),)*] {
                    for (k, v) in item.into_iter() {
                        envs.insert(k, v);
                    }
                }
                envs
            }
        }

        impl #ident {
            pub fn get_env() -> std::io::Result<#ident> {
                #ident::read_env("", "")
            }
            pub fn load_env(path: &str) -> std::io::Result<#ident> {
                structenv::dotenv::from_path(path).ok();
                #ident::read_env("", "")
            }
            pub fn save_env(&self, path: &str) -> std::io::Result<()> {
                structenv::save_env(self, path)
            }
        }
    );
    TokenStream::from(ast)
}
