#[macro_use]
extern crate quote;
#[macro_use]
extern crate syn;

use proc_macro::TokenStream;

use proc_macro2::Literal;
use rust_format::Formatter;
use syn::parse::Parser;
use syn::punctuated::Punctuated;
use syn::{ImplItem, ImplItemFn, ItemImpl, Path, PathArguments, Type};

#[proc_macro_attribute]
pub fn extensions(args: TokenStream, item: TokenStream) -> TokenStream {
    let input = parse_macro_input!(item as ItemImpl);

    let tokens = args.clone();
    let parser = Punctuated::<Path, Token![,]>::parse_separated_nonempty;
    let mut parsed = parser
        .parse(tokens)
        .unwrap()
        .into_iter()
        .collect::<Vec<Path>>();
    if parsed.len() != 2 {
        panic!("Must contain two arguments: extensions package and a target type");
    }

    let category = parsed.remove(0);
    let target_type = parsed.remove(0);

    let reflection_impl =
        generate_phlow_implementation_for_external_type(input, category, target_type);

    TokenStream::from(reflection_impl)
}

fn extract_generics(t: &Type) -> Option<proc_macro2::TokenStream> {
    let tokens = match t {
        Type::Path(path) => {
            let segments = &path.path.segments;
            if segments.len() != 1 {
                panic!(
                    "Only supports single segments path, but was: {:?}",
                    segments
                );
            }
            match &segments[0].arguments {
                PathArguments::None => None,
                PathArguments::AngleBracketed(angle) => Some(quote! { #angle }),
                PathArguments::Parenthesized(_) => None,
            }
        }
        _ => {
            panic!("Unsupported type: {:?}", t)
        }
    };

    tokens
}

fn generate_phlow_implementation_for_external_type(
    implementation: ItemImpl,
    extension_category: Path,
    extension_target_type: Path,
) -> proc_macro2::TokenStream {
    let self_type = &implementation.self_ty;
    let extension_struct_name = quote! { #self_type };
    let extension_category_type_name = quote! { #extension_category };
    let target_type_name = quote! { #extension_target_type };

    let generics_with_bounds = &implementation.generics;
    let generics = extract_generics(&implementation.self_ty);

    let struct_impl = if generics.is_some() {
        quote! { pub struct #extension_struct_name(std::marker::PhantomData #generics ); }
    } else {
        quote! { pub struct #extension_struct_name; }
    };

    let phlow_methods = generate_phlow_methods(
        extension_struct_name.clone(),
        target_type_name.clone(),
        &implementation,
    );

    quote! {
        #struct_impl
        #implementation

        impl #generics_with_bounds phlow::Phlow<crate::#extension_category_type_name> for #target_type_name {
            #phlow_methods

            fn phlow_extension() -> Option<phlow::PhlowExtension> {
                Some(phlow::PhlowExtension::new::<crate::#extension_category_type_name, Self>())
            }
        }
    }
}

fn generate_phlow_methods(
    extension_container_type: proc_macro2::TokenStream,
    target_type: proc_macro2::TokenStream,
    implementation: &ItemImpl,
) -> proc_macro2::TokenStream {
    let view_methods: Vec<&ImplItemFn> = implementation
        .items
        .iter()
        .map(|each| match each {
            ImplItem::Fn(method) => Some(method),
            _ => None,
        })
        .filter(|each| each.is_some())
        .map(|each| each.unwrap())
        .filter(is_view_method)
        .collect();

    let get_views = view_methods
        .into_iter()
        .map(|each_method| {
            let name_ident = &each_method.sig.ident;
            let method_name = quote! { #name_ident };

            let method_name_string = Literal::string(&method_name.to_string());

            let full_method_name_string = Literal::string(&format!(
                "{}::{}",
                extension_container_type.to_string(),
                method_name.to_string()
            ));

            let formatted = get_source_code(each_method);
            let source_code = Literal::string(formatted.as_str());

            quote! {
                phlow::PhlowViewMethod {
                    method: std::sync::Arc::new(| object: &phlow::PhlowObject, method: &phlow::PhlowViewMethod | {
                        if let Some(typed_reference) = object.value_ref::<#target_type>() {
                            let view = <#extension_container_type> :: #method_name (
                                &typed_reference,
                                phlow::PhlowProtoView::new(object.clone(), method.clone()));
                            Some(Box::new(view))
                        } else {
                            phlow::log::warn!("Failed to cast object of type {} to {} when building a view {}",
                                object.value_type_name(),
                                std::any::type_name::<#target_type>(),
                                #full_method_name_string);
                            None
                        }
                    }),
                    extension: extension.clone(),
                    full_method_name:  #full_method_name_string.to_string(),
                    method_name:  #method_name_string.to_string(),
                    source_code: #source_code.to_string()
                }
            }
        })
        .collect::<Vec<proc_macro2::TokenStream>>();

    quote! {
        fn phlow_view_methods(extension: &phlow::PhlowExtension) -> Vec<phlow::PhlowViewMethod> {
            vec![#(#get_views),*]
        }
    }
}

fn is_view_method(method: &&ImplItemFn) -> bool {
    method.attrs.iter().any(|_each| true)
}

fn get_source_code(each_method: &ImplItemFn) -> String {
    let token_stream = quote! { #each_method };

    let config = rust_format::Config::new_str()
        .edition(rust_format::Edition::Rust2021)
        .option("reorder_imports", "false")
        .option("reorder_modules", "false")
        .option("max_width", "85");
    let rust_fmt = rust_format::RustFmt::from_config(config);
    rust_fmt.format_tokens(token_stream).unwrap()
}

#[proc_macro_attribute]
pub fn view(_attr: TokenStream, item: TokenStream) -> TokenStream {
    item
}
