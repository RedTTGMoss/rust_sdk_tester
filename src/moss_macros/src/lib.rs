use proc_macro::TokenStream;
use quote::{format_ident, quote};
use syn::{parse_macro_input, ItemImpl, FnArg, LitInt};

#[proc_macro_attribute]
pub fn moss_screen(_attr: TokenStream, item: TokenStream) -> TokenStream {
    let input = parse_macro_input!(item as ItemImpl);
    let struct_name = match *input.self_ty {
        syn::Type::Path(ref type_path) => type_path.path.segments.last().unwrap().ident.clone(),
        _ => panic!("Unsupported type for impl"),
    };
    let struct_name_str = struct_name.to_string();

    let mut transformed_methods = Vec::new();
    let mut loop_function = None;
    let mut pre_loop_function = None;
    let mut post_loop_function = None;
    let mut event_hook_function = None;

    for item in input.items.iter() {
        if let syn::ImplItem::Fn(func) = item {
            let func_name = &func.sig.ident;
            let new_func_name = format_ident!("{}_{}", struct_name, func_name);

            let inputs = &func.sig.inputs;
            let inputs_transformed = inputs.iter().map(|arg| match arg {
                FnArg::Receiver(_) => arg.clone(),
                FnArg::Typed(pat) => {
                    let mut pat = pat.clone();
                    pat.attrs.clear(); // Remove attributes
                    FnArg::Typed(pat)
                }
            });

            let block = &func.block;
            let new_block = quote! {
                {
                    #block
                    Ok(())
                }
            };

            transformed_methods.push(quote! {
                #[extism_pdk::plugin_fn]
                pub unsafe fn #new_func_name(#(#inputs_transformed),*) -> extism_pdk::FnResult<()> #new_block
            });

            match func_name.to_string().as_str() {
                "r#loop" => loop_function = Some(new_func_name.to_string()),
                "pre_loop" => pre_loop_function = Some(new_func_name.to_string()),
                "post_loop" => post_loop_function = Some(new_func_name.to_string()),
                "event_hook" => event_hook_function = Some(new_func_name.to_string()),
                _ => {}
            }
        }
    }

    let loop_function = loop_function.expect("loop function is required.");
    let pre_loop_function = pre_loop_function.map(|f| quote!(Some(#f.to_string()))).unwrap_or(quote!(None));
    let post_loop_function = post_loop_function.map(|f| quote!(Some(#f.to_string()))).unwrap_or(quote!(None));
    let event_hook_function = event_hook_function.map(|f| quote!(Some(#f.to_string()))).unwrap_or(quote!(None));


    let open_methods = quote! {
        pub unsafe fn register() {
            moss_definitions::functions::moss_pe_register_screen(moss_definitions::types::MossScreen {
                key: #struct_name_str.to_string(),
                screen_pre_loop: #pre_loop_function,
                screen_loop: #loop_function.to_string(),
                screen_post_loop: #post_loop_function,
                event_hook: #event_hook_function,
            });
        }
        pub unsafe fn open() {
            moss_definitions::functions::moss_pe_open_screen(#struct_name_str, ()).unwrap()
        }

        pub unsafe fn open_with_data(initial_data: Self) {
            moss_definitions::functions::moss_pe_open_screen::<Self>(#struct_name_str, initial_data).unwrap()
        }
    };

    let struct_impl = &input.self_ty;

    let expanded = quote! {
        impl #struct_impl {
            #(#transformed_methods)*
            #open_methods
        }
    };

    TokenStream::from(expanded)
}

#[proc_macro]
pub fn moss_color(input: TokenStream) -> TokenStream {
    // Parse the input as a hexadecimal literal
    let input = parse_macro_input!(input as LitInt);
    let hex = input.base10_parse::<u32>().unwrap();

    let r;
    let g;
    let b;
    let a;

    if hex <= 0xFFFFFF {
        r = ((hex >> 16) & 0xFF) as i64;
        g = ((hex >> 8) & 0xFF) as i64;
        b = (hex & 0xFF) as i64;
        a = None;
    } else {
        r = ((hex >> 24) & 0xFF) as i64;
        g = ((hex >> 16) & 0xFF) as i64;
        b = ((hex >> 8) & 0xFF) as i64;
        a = Some((hex & 0xFF) as i64);
    }


    let expanded = match a {
        Some(a) => quote! {
            Color::new(#r, #g, #b, Some(#a))
        },
        None => quote! {
            Color::new(#r, #g, #b, None)
        },
    };

    TokenStream::from(expanded)
}