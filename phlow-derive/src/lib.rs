#[macro_use]
extern crate quote;
#[macro_use]
extern crate syn;

use proc_macro::TokenStream;

use proc_macro2::Literal;
use syn::{
    parse_str, AttributeArgs, ImplItem, ImplItemMethod, ItemImpl, Lit, NestedMeta, Path,
    PathArguments, Type,
};

#[proc_macro_attribute]
pub fn extensions(attr: TokenStream, item: TokenStream) -> TokenStream {
    let input = parse_macro_input!(item as ItemImpl);
    let mut attributes = parse_macro_input!(attr as AttributeArgs);

    let reflection_impl = match attributes.len() {
        2 => generate_phlow_implementation_for_external_type(
            input,
            attributes.remove(0),
            extract_target_type(attributes.remove(0)),
        ),
        _ => panic!("Must contain two arguments: extensions package and target type"),
    };

    TokenStream::from(reflection_impl)
}

fn extract_target_type(attribute: NestedMeta) -> proc_macro2::TokenStream {
    let tokens = match attribute {
        NestedMeta::Meta(meta) => {
            quote!( #meta )
        }
        NestedMeta::Lit(literal) => match literal {
            Lit::Str(string) => {
                let type_name = parse_str::<Path>(string.value().as_str()).unwrap();
                quote!( #type_name )
            }
            _ => panic!("Must be a string"),
        },
    };
    tokens
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
    extension_category: NestedMeta,
    extension_target_type: proc_macro2::TokenStream,
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
    let view_methods: Vec<&ImplItemMethod> = implementation
        .items
        .iter()
        .map(|each| match each {
            ImplItem::Method(method) => Some(method),
            _ => None,
        })
        .filter(|each| each.is_some())
        .map(|each| each.unwrap())
        .filter(is_view_method)
        .collect();

    let get_views = view_methods
        .iter()
        .map(|each_method| {
            let name_ident = &each_method.sig.ident;
            let method_name = quote! { #name_ident };

            let method_name_string = Literal::string(&method_name.to_string());

            let full_method_name_string = Literal::string(&format!(
                "{}::{}",
                extension_container_type.to_string(),
                method_name.to_string()
            ));

            quote! {
                phlow::PhlowViewMethod {
                    method: std::rc::Rc::new(| object: &phlow::PhlowObject | {
                        if let Some(typed_reference) = object.value_ref::<#target_type>() {
                            let view = <#extension_container_type> :: #method_name (typed_reference, phlow::PhlowProtoView::new(object.clone()));
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
                    method_name:  #method_name_string.to_string()
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

fn is_view_method(method: &&ImplItemMethod) -> bool {
    method.attrs.iter().any(|_each| {
        //println!("{:#?}", each);
        true
    })
}

#[proc_macro_attribute]
pub fn view(_attr: TokenStream, item: TokenStream) -> TokenStream {
    item
}
