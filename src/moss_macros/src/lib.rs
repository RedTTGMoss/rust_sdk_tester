use proc_macro::TokenStream;
use quote::{format_ident, quote};
use syn::{parse_macro_input, DeriveInput, FnArg, ItemImpl, LitInt, Visibility};

#[proc_macro_attribute]
pub fn moss_screen(_attr: TokenStream, item: TokenStream) -> TokenStream {
    let input = parse_macro_input!(item as ItemImpl);
    let struct_name = match *input.self_ty {
        syn::Type::Path(ref type_path) => type_path.path.segments.last().unwrap().ident.clone(),
        _ => panic!("Unsupported type for impl"),
    };
    let struct_name_str = struct_name.to_string();
    let screen_manager = format_ident!("{}_SCREEN_MANAGER", struct_name.to_string().to_uppercase());

    let mut transformed_methods = Vec::new();
    let mut loop_function = None;
    let mut pre_loop_function = None;
    let mut post_loop_function = None;
    let mut event_hook_function = None;

    for item in input.items.iter() {
        if let syn::ImplItem::Fn(func) = item {
            if let Visibility::Public(_) = func.vis {
                let func_name = &func.sig.ident;
                let mut new_func_name = format_ident!("{}_{}", struct_name, func_name);
                let mut uses_self = false;

                let inputs = &func.sig.inputs;
                let mut inputs_transformed = Vec::new();

                for arg in inputs {
                    match arg {
                        FnArg::Receiver(receiver) => {
                            if receiver.reference.is_some() {
                                uses_self = true;
                            }
                        }
                        FnArg::Typed(pat) => {
                            let mut pat = pat.clone();
                            pat.attrs.clear(); // Remove attributes
                            inputs_transformed.push(FnArg::Typed(pat));
                        }
                    }
                }

                let block = &func.block;

                if uses_self {
                    new_func_name = format_ident!("{}_{}_self_", struct_name, func_name);
                }

                transformed_methods.push(if uses_self {
                    quote! {
                        pub unsafe fn #func_name(&mut self) #block
                    }
                } else {
                    quote! {
                        pub unsafe fn #func_name() #block
                    }
                });

                let inner_block = if uses_self {
                    quote! {
                        {
                            let instance_id = crate::moss_definitions::functions::moss_pe_get_screen_value::<i64>("id")?.value;
                            let instance = {
                                let mut screen_manager = #screen_manager.lock().unwrap();
                                screen_manager.get_mut(&instance_id).cloned()
                            };

                            if let Some(instance) = instance {
                                instance.lock().unwrap().#func_name();
                            } else {
                                extism_pdk::error!("The instance could not be found!");
                                panic!("The instance could not be found!");
                            }
                            Ok(())
                        }
                    }
                } else {
                    quote! {
                        {
                            #struct_name::#func_name();
                            Ok(())
                        }
                    }
                };

                transformed_methods.push(quote! {
                    #[extism_pdk::plugin_fn]
                    pub unsafe fn #new_func_name(#(#inputs_transformed),*) -> extism_pdk::FnResult<()> #inner_block
                });

                match func_name.to_string().as_str() {
                    "r#loop" => loop_function = Some(new_func_name.to_string()),
                    "pre_loop" => pre_loop_function = Some(new_func_name.to_string()),
                    "post_loop" => post_loop_function = Some(new_func_name.to_string()),
                    "event_hook" => event_hook_function = Some(new_func_name.to_string()),
                    _ => {}
                }
            } else {
                let func_name = &func.sig.ident;
                let inputs: Vec<_> = func.sig.inputs.iter().cloned().collect();
                let output = &func.sig.output;
                let block = &func.block;

                transformed_methods.push(quote! {
                    unsafe fn #func_name(#(#inputs), *) #output #block
                });
            }
        }
    }

    let loop_function = loop_function.expect("loop function is required.");
    let pre_loop_function = pre_loop_function
        .map(|f| quote!(Some(#f.to_string())))
        .unwrap_or(quote!(None));
    let post_loop_function = post_loop_function
        .map(|f| quote!(Some(#f.to_string())))
        .unwrap_or(quote!(None));
    let event_hook_function = event_hook_function
        .map(|f| quote!(Some(#f.to_string())))
        .unwrap_or(quote!(None));

    let open_methods = quote! {
        pub unsafe fn register() {
            if let Err(e) = crate::moss_definitions::functions::moss_pe_register_screen(moss_definitions::types::MossScreen {
                key: #struct_name_str.to_string(),
                screen_pre_loop: #pre_loop_function,
                screen_loop: #loop_function.to_string(),
                screen_post_loop: #post_loop_function,
                event_hook: #event_hook_function,
            }) {
                extism_pdk::error!("Failed to register screen: {:?}", e);
                panic!("{:?}", e);
            }
        }

        pub unsafe fn open_with_data(instance: Self) {
            match crate::moss_definitions::functions::moss_pe_open_screen(#struct_name_str, ()) {
                Ok(instance_id) => {
                    #screen_manager.lock().unwrap().insert(instance_id, std::sync::Arc::new(std::sync::Mutex::new(instance)));
                },
                Err(e) => {
                    extism_pdk::error!("Failed to open screen with data: {:?}", e);
                    panic!("{:?}", e);
                },
            }
        }
    };

    let struct_impl = &input.self_ty;

    let expanded = quote! {
        // Global screen registry
        static #screen_manager: once_cell::sync::Lazy<std::sync::Mutex<std::collections::HashMap<i64, std::sync::Arc<std::sync::Mutex<#struct_name>>>>> = once_cell::sync::Lazy::new(|| std::sync::Mutex::new(std::collections::HashMap::new()));

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

#[proc_macro_derive(Accessors, attributes(accessor_type, accessor_uuid, accessor))]
pub fn accessors_derive(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);
    let struct_name = &input.ident;

    if let syn::Data::Struct(data_struct) = input.data {
        let mut getters_setters = Vec::new();

        for field in data_struct.fields.iter() {
            if let Some(field_ident) = &field.ident {
                let field_ty = &field.ty;
                let field_name = field_ident.to_string();
                let setter_ident = format_ident!("set_{}", field_ident);
                let getter_ident = format_ident!("get_{}", field_ident);

                if field.attrs.iter().any(|attr| {
                    attr.path().is_ident("accessor")
                        && attr
                            .meta
                            .require_list()
                            .ok()
                            .and_then(|list| list.parse_args::<syn::Ident>().ok())
                            .map_or(false, |ident| ident == "exclude")
                }) {
                    continue;
                }

                // Skip generating accessors
                if field_name == "accessor" {
                    continue;
                }

                let setters_block = quote! {
                    crate::moss_definitions::functions::moss_api_set::<#field_ty>(&self.accessor, #field_name, value);
                };

                let getters_block = quote! {
                    return crate::moss_definitions::functions::moss_api_get::<#field_ty>(&self.accessor, #field_name);
                };

                getters_setters.push(quote! {
                    pub unsafe fn #getter_ident(&self) -> Result<crate::moss_definitions::types::ConfigGet<#field_ty>, extism_pdk::Error> {
                        #getters_block
                    }
                    pub unsafe fn #field_ident(&mut self) -> #field_ty {
                        match self.#getter_ident() {
                            Ok(value) => self.#field_ident = value.value,
                            Err(e) => extism_pdk::error!("Failed to get {}: {:?}", #field_name, e)
                        }
                        return self.#field_ident.clone()
                    }
                });
                getters_setters.push(quote! {
                    pub unsafe fn #setter_ident(&mut self, value: #field_ty) {
                        self.#field_ident = value.clone();
                        #setters_block
                    }
                });
            }
        }

        let expanded = quote! {
            impl #struct_name {
                #(#getters_setters)*
            }
        };

        TokenStream::from(expanded)
    } else {
        TokenStream::new()
    }
}

#[proc_macro_derive(DocumentAccessorBuilder)]
pub fn document_accessor_builder_derive(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);
    let struct_name = &input.ident;
    let builder_name = format_ident!("{}Builder", struct_name);

    let builder_impl = quote! {
        impl #builder_name {
            pub fn accessor_uuid(self, uuid: String) -> Self {
                let mut new = self;
                // if not new use Accessor::api_document(uuid)
                if let Some(ref mut accessor) = new.accessor {
                    accessor.uuid = Some(uuid);
                } else {
                    new.accessor = Some(Accessor::api_document(uuid));
                }
                new
            }
            pub fn accessor_type(self, item_type: AccessorType) -> Self {
                let mut new = self;

                if let Some(ref mut accessor) = new.accessor {
                    match item_type {
                        AccessorType::ApiItem => accessor.r#type = ACCESSOR_API_DOCUMENT.to_string(),
                        AccessorType::StandaloneItem => {
                            accessor.r#type = ACCESSOR_STANDALONE_DOCUMENT.to_string()
                        }
                    }
                } else {
                    match item_type {
                        AccessorType::ApiItem => new.accessor = Some(Accessor::unknown_api_document()),
                        AccessorType::StandaloneItem => {
                            new.accessor = Some(Accessor::unknown_standalone_document())
                        }
                    }
                }
                new
            }
        }
    };
    TokenStream::from(builder_impl)
}
