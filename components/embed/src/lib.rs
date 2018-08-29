#![recursion_limit = "1024"]
extern crate proc_macro;
#[macro_use]
extern crate quote;
extern crate syn;

extern crate walkdir;

use proc_macro::TokenStream;
use quote::Tokens;
use std::path::Path;
use syn::*;

fn generate_file_list(ident: &syn::Ident, folder_path: impl AsRef<Path>) -> quote::Tokens {
    use walkdir::WalkDir;
    let mut values = Vec::<Tokens>::new();
    for entry in WalkDir::new(&folder_path) {
        let path = entry.unwrap().path().to_path_buf();
        if path.is_file() {
            let pathstr = path.strip_prefix(&folder_path).unwrap().to_str();
            values.push(quote!(#pathstr,));
        }
    }
    quote! {
        impl #ident {
            pub fn keys() -> ::std::vec::IntoIter<&'static str> {
                vec![
                    #(#values)*
                ].into_iter()
            }
        }
    }
}

#[cfg(debug_assertions)]
fn generate_assets(ident: &syn::Ident, folder_path: impl AsRef<Path>) -> quote::Tokens {
    let folder_path = folder_path.as_ref().to_str().unwrap();
    quote!{
        impl #ident {
            pub fn get(file_path: &str) -> Option<Vec<u8>> {
                use std::fs::File;
                use std::io::Read;
                use std::path::{PathBuf, Path};

                let mut path = PathBuf::from(#folder_path);
                let fpath = PathBuf::from(file_path);
                path.push(fpath);
                // println!("path = {:?}", path);
                let mut file = match File::open(path) {
                    Ok(mut file) => file,
                    Err(_e) => {
                        return None
                    }
                };
                let mut data: Vec<u8> = Vec::new();
                match file.read_to_end(&mut data) {
                    Ok(_) => Some(data),
                    Err(_e) =>  {
                        return None
                    }
                }
            }
        }
    }
}

#[cfg(not(debug_assertions))]
fn generate_assets(ident: &syn::Ident, folder_path: impl AsRef<Path>) -> quote::Tokens {
    use walkdir::WalkDir;
    let mut values = Vec::<Tokens>::new();
    for entry in WalkDir::new(&folder_path)
        .into_iter()
        .filter_map(|e| e.ok())
        .filter(|e| e.file_type().is_file())
    {
        let base = folder_path.as_ref();
        let key = String::from(
            entry
                .path()
                .strip_prefix(base)
                .unwrap()
                .to_str()
                .expect("Path does not have a string representation"),
        );
        println!("key: {}", key);
        let canonical_path =
            std::fs::canonicalize(entry.path()).expect("Could not get canonical path");
        let canonical_path_str = canonical_path.to_str();
        let value = quote!{
          #key => Some(include_bytes!(#canonical_path_str).to_vec()),
        };
        values.push(value);
    }
    quote!{
        impl #ident {
            pub fn get(file_path: &str) -> Option<Vec<u8>> {
                match file_path {
                    #(#values)*
                    _ => None,
                }
            }
        }
    }
}

fn help() {
    panic!("#[derive(RustEmbed)] should contain one attribute like this #[folder = \"examples/public/\"]");
}

fn impl_rust_embed(ast: &syn::DeriveInput) -> Tokens {
    match ast.body {
        Body::Enum(_) => help(),
        Body::Struct(ref data) => match data {
            &VariantData::Struct(_) => help(),
            _ => {}
        },
    };
    let ident = &ast.ident;
    if ast.attrs.len() == 0 || ast.attrs.len() > 1 {
        help();
    }
    let value = &ast.attrs[0].value;
    let literal_value = match value {
        &MetaItem::NameValue(ref attr_name, ref value) => {
            if attr_name == "folder" {
                value
            } else {
                panic!("#[derive(RustEmbed)] attribute name must be folder");
            }
        }
        _ => {
            panic!("#[derive(RustEmbed)] attribute name must be folder");
        }
    };
    let folder_path = match literal_value {
        &Lit::Str(ref val, _) => val.clone(),
        _ => {
            panic!("#[derive(RustEmbed)] attribute value must be a string literal");
        }
    };
    if !Path::new(&folder_path).exists() {
        panic!(
            "#[derive(RustEmbed)] folder '{}' does not exist. cwd: '{}'",
            folder_path,
            std::env::current_dir().unwrap().to_str().unwrap()
        );
    };

    let generate_file_list_fn = generate_file_list(ident, &folder_path);
    let generate_assets_fn = generate_assets(ident, &folder_path);
    quote! {
        #generate_file_list_fn
        #generate_assets_fn
    }
}

#[proc_macro_derive(RustEmbed, attributes(folder))]
pub fn derive_input_object(input: TokenStream) -> TokenStream {
    let s = input.to_string();
    let ast = syn::parse_derive_input(&s).unwrap();
    let gen = impl_rust_embed(&ast);
    gen.parse().unwrap()
}
