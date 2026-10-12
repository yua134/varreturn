#[proc_macro_attribute]
pub fn varreturn_callee(_attr: proc_macro::TokenStream, item: proc_macro::TokenStream) -> proc_macro::TokenStream {
    let mut input = syn::parse_macro_input!(item as syn::ItemFn);

    let return_ty = match &input.sig.output {
        syn::ReturnType::Default => {
            syn::parse_quote! { () }
        }
        syn::ReturnType::Type(_, ty) => {
            *ty.clone()
        }
    };

    let extra_arg = syn::parse_quote!{ mut __var_return_arr: <#return_ty as varreturn::AssociatedArray>::Array };

    input.sig.inputs.push(extra_arg);

    quote::quote! {
        #input
    }.into()
}

#[proc_macro]
pub fn var_return(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    let input = syn::parse_macro_input!(input as RetPat);

    let arr = syn::Ident::new("__var_return_arr", proc_macro2::Span::call_site());

    match input {
        RetPat::Empty => {
            var_return_init(arr)
        }
        RetPat::Exprs(exprs) => {
            var_return_extend(arr, exprs)
        }
    }.into()
}

#[proc_macro]
pub fn var_push(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    let expr = syn::parse_macro_input!(input as syn::Expr);

    let arr_ident = syn::Ident::new("__var_return_arr", proc_macro2::Span::call_site());

    quote::quote! {
        {
            let item = ::core::mem::ManuallyDrop::new(#expr);
            #arr_ident.__push(item);
        }
    }
    .into()
}

#[proc_macro_derive(VarReturnable, attributes(varreturn))]
pub fn derive_var_returnable(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    let input = syn::parse_macro_input!(input as syn::DeriveInput);

    let structure = match &input.data {
        syn::Data::Struct(s) => s,
        _ => {
            return syn::Error::new_spanned(input, "VarReturnable can only be derived for structs").to_compile_error().into();
        }
    };

    let target_field_ty = if structure.fields.len() == 1 {
        &structure.fields.iter().next().unwrap().ty
    } else {
        let mut target = None;
        for field in structure.fields.iter() {
            for attr in field.attrs.iter() {
                if attr.path().is_ident("varreturn") {
                    if target.is_some() {
                        return syn::Error::new_spanned(attr, "Multiple fields are marked with #[varreturn] attribute").to_compile_error().into();
                    }
                    target = Some(&field.ty);
                }
            }
        }
        match target {
            Some(ty) => ty,
            None => {
                return syn::Error::new_spanned(input, "No field is marked with #[varreturn] attribute").to_compile_error().into();
            }
        }
    };

    let name = &input.ident;
    let (impl_generics, type_generics, where_clause) = input.generics.split_for_impl();

    quote::quote! {
        impl #impl_generics ::varreturn::AssociatedArray for #name #type_generics #where_clause {
            type Array = <#target_field_ty as varreturn::AssociatedArray>::Array;
        }
    }.into()
}

fn var_return_extend(arr: syn::Ident, mut elems: Vec<syn::Expr>) -> proc_macro2::TokenStream {
    for elem in elems.iter_mut() {
        let expr = std::mem::replace(elem, syn::parse_quote!(()));
        *elem = syn::parse_quote! {
            ::core::mem::ManuallyDrop::new(#expr)
        };
    }

    quote::quote! {
        {
            let mut arr = [#(#elems),*];
            let ret = #arr.__copy(&mut arr);
            ::varreturn::VarReturn::__new(ret)
        }
    }
}

fn var_return_init(arr: syn::Ident) -> proc_macro2::TokenStream {
    quote::quote! {
        {
            let ret = #arr.__init();
            ::varreturn::VarReturn::__new(ret)
        }
    }
}

enum RetPat {
    Empty,
    Exprs(Vec<syn::Expr>),
}

impl syn::parse::Parse for RetPat {
    fn parse(input: syn::parse::ParseStream) -> syn::Result<Self> {
        if input.is_empty() {
            Ok(Self::Empty)
        } else if input.peek(syn::token::Bracket) {
            let content;
            syn::bracketed!(content in input);

            let exprs = content
                .parse_terminated(syn::Expr::parse, syn::Token![,])?
                .into_iter()
                .collect();

            Ok(Self::Exprs(exprs))
        } else {
            Err(input.error("expected `()` or `[expr, ...]`"))
        }
    }
}
