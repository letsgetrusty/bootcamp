extern crate proc_macro;

use proc_macro::{TokenStream};
use quote::quote;
use syn::{self, parse_macro_input, AttributeArgs, ItemFn, parse_quote, Ident, FnArg, Pat, Stmt};
use chrono::prelude::*;
use darling::{FromMeta, ToTokens};

#[derive(FromMeta)]
struct MacroArgs {
    #[darling(default)]
    verbose: bool,
}

#[proc_macro_attribute]
pub fn log_call(args: TokenStream, input: TokenStream) -> TokenStream {
    let attr_args = parse_macro_input!(args as AttributeArgs);
    let mut input = parse_macro_input!(input as ItemFn);

    let attr_args = match MacroArgs::from_list(&attr_args) {
        Ok(v) => v,
        Err(e) => { return TokenStream::from(e.write_errors()); }
    };
    
    impl_log_call(&attr_args, &mut input)
}

fn impl_log_call(attr_args: &MacroArgs, input: &mut ItemFn) -> TokenStream {
    let fn_name = &input.sig.ident;

    if attr_args.verbose {
        let fn_args = extract_arg_names(input);
        let statements = generate_verbose_log(fn_name, fn_args);

        input.block.stmts.splice(0..0, statements);
    } else {
        input.block.stmts.insert(0, parse_quote! {
            println!("[Info] calling {}", stringify!(#fn_name));
        });
    }

    input.to_token_stream().into()
}

fn extract_arg_names(func: &ItemFn) -> Vec<&Ident> {
    func.sig.inputs.iter().filter_map(|arg| {
        if let FnArg::Typed(pat_type) = arg {
            if let Pat::Ident(pat) = &(*pat_type.pat) {
                return Some(&pat.ident);
            }
        }
        None
    }).collect()
}

fn generate_verbose_log(fn_name: &Ident, fn_args: Vec<&Ident>) -> Vec<Stmt> {
    let mut statements = vec![parse_quote! {
        print!("[Info] calling {} | ", stringify!(#fn_name));
    }];

    for arg in fn_args {
        statements.push(
            parse_quote! {
                print!("{} = {:?} ", stringify!(#arg), #arg);
            }
        );
    }

    statements.push(parse_quote! { println!(); });

    statements
}

#[proc_macro_derive(Log)]
pub fn log_derive(input: TokenStream) -> TokenStream {
    let ast: syn::DeriveInput = syn::parse(input).unwrap();
    let name = &ast.ident;

    let trait_impl = quote! {
        impl Log for #name {
            fn info(&self, msg: &str) {
                println!("[Info] {}: {}", stringify!(#name), msg);
            }
            fn warn(&self, msg: &str) {
                println!("[Warn] {}: {}", stringify!(#name), msg);
            }
            fn error(&self, msg: &str) {
                println!("[Err] {}: {}", stringify!(#name), msg);
            }
        }
    };

    trait_impl.into()
}

#[proc_macro]
pub fn log_info(input: TokenStream) -> TokenStream {
    let mut output = "[Info] ".to_owned();

    for token in input {
        let token_string = token.to_string();

        match token_string.as_str() {
            "[TIME]" => {
                let time = Utc::now().time().to_string();
                output.push_str(&format!("{} ", time));
            }
            _ => {
                output.push_str(&format!("{} ", token_string));
            }
        }
    }
    
    TokenStream::from(quote! {
        println!("{}", #output);
    })
}