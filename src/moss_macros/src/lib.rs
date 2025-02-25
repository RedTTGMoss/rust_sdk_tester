use proc_macro::TokenStream;
use quote::{format_ident, quote};
use syn::{parse_macro_input, DeriveInput, FnArg, ItemImpl, LitBool, LitInt, LitStr, Visibility};

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

    let accessor_type: String = input
        .attrs
        .iter()
        .find(|attr| attr.path().is_ident("accessor_type"))
        .map(|attr| {
            attr.parse_args::<LitStr>()
                .expect("expected a string literal for accessor_type")
                .value()
        })
        .unwrap_or_else(|| panic!("Missing required attribute: #[accessor_type(\"...\")]"));
    let accessor_uuid: bool = input
        .attrs
        .iter()
        .find(|attr| attr.path().is_ident("accessor_uuid"))
        .map(|attr| {
            attr.parse_args::<LitBool>()
                .expect("expected a bool for accessor_uuid")
                .value()
        })
        .unwrap_or(false);

    if let syn::Data::Struct(data_struct) = input.data {
        let mut getters_setters = Vec::new();
        let mut has_document_uuid = false;
        let mut has_collection_uuid = false;
        let mut has_accessor_id = false;
        let mut has_uuid = false;
        let accessor_id_field = format!("{}_id", accessor_type);

        for field in data_struct.fields.iter() {
            if let Some(field_ident) = &field.ident {
                let field_name = field_ident.to_string();
                if field_name == "document_uuid" {
                    has_document_uuid = true
                }
                if field_name == "collection_uuid" {
                    has_collection_uuid = true
                }
                if field_name == "uuid" {
                    has_uuid = true
                }
                if field_name == accessor_id_field {
                    has_accessor_id = true
                }
            }
        }

        if accessor_uuid && !has_uuid {
            return syn::Error::new_spanned(
                struct_name,
                "Struct must have a `uuid: String` field when using #[accessor_uuid(true)]",
            )
            .to_compile_error()
            .into();
        }

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
                if field_name == "document_uuid"
                    || field_name == "collection_uuid"
                    || field_name == accessor_id_field
                {
                    continue;
                }

                let uuid_set = format_ident!("moss_api_{}_set", accessor_type);
                let document_set = format_ident!("moss_api_document_{}_set", accessor_type);
                let collection_set = format_ident!("moss_api_collection_{}_set", accessor_type);
                let accessor_set = format_ident!("moss_api_{}_set", accessor_type);
                let uuid_get = format_ident!("moss_api_{}_get", accessor_type);
                let document_get = format_ident!("moss_api_document_{}_get", accessor_type);
                let collection_get = format_ident!("moss_api_collection_{}_get", accessor_type);
                let accessor_get = format_ident!("moss_api_{}_get", accessor_type);
                let accessor_id = format_ident!("{}_id", accessor_type);
                let panic_collection = format!(
                    "Neither document_uuid, collection_uuid or {}_id are set!",
                    accessor_type
                );
                let panic_document =
                    format!("Neither document_uuid nor {}_id is set!", accessor_type);

                let setters_block = if accessor_uuid {
                    quote! {
                        crate::moss_definitions::functions::#uuid_set::<#field_ty>(self.uuid.clone().as_str(), #field_name, value);
                    }
                } else {
                    if has_collection_uuid {
                        quote! {
                            if let Some(ref document_uuid) = self.document_uuid {
                                crate::moss_definitions::functions::#document_set::<#field_ty>(document_uuid, #field_name, value);
                            } else if let Some(ref collection_uuid) = self.collection_uuid {
                                crate::moss_definitions::functions::#collection_set::<#field_ty>(collection_uuid, #field_name, value);
                            } else if let Some(ref #accessor_id) = self.#accessor_id {
                                crate::moss_definitions::functions::#accessor_set::<#field_ty>(#accessor_id, #field_name, value);
                            } else {
                                panic!(#panic_collection);
                            }
                        }
                    } else {
                        quote! {
                            if let Some(ref document_uuid) = self.document_uuid {
                                    crate::moss_definitions::functions::#document_set::<#field_ty>(document_uuid, #field_name, value);
                                } else if let Some(ref #accessor_id) = self.#accessor_id {
                                    crate::moss_definitions::functions::#accessor_set::<#field_ty>(#accessor_id, #field_name, value);
                                } else {
                                    panic!(#panic_document);
                                }
                        }
                    }
                };

                let getters_block = if accessor_uuid {
                    quote! {
                        return crate::moss_definitions::functions::#uuid_get::<#field_ty>(self.uuid.clone().as_str(), #field_name);
                    }
                } else {
                    if has_collection_uuid {
                        quote! {
                            return if let Some(ref document_uuid) = self.document_uuid {
                                crate::moss_definitions::functions::#document_get::<#field_ty>(document_uuid, #field_name)
                            } else if let Some(ref collection_uuid) = self.collection_uuid {
                                crate::moss_definitions::functions::#collection_get::<#field_ty>(collection_uuid, #field_name)
                            } else if let Some(ref #accessor_id) = self.#accessor_id {
                                crate::moss_definitions::functions::#accessor_get::<#field_ty>(#accessor_id, #field_name)
                            } else {
                                Err(extism_pdk::Error::msg(#panic_collection))
                            }
                        }
                    } else {
                        quote! {
                            return if let Some(ref document_uuid) = self.document_uuid {
                                crate::moss_definitions::functions::#document_get::<#field_ty>(document_uuid, #field_name)
                            } else if let Some(ref #accessor_id) = self.#accessor_id {
                                crate::moss_definitions::functions::#accessor_get::<#field_ty>(#accessor_id, #field_name)
                            } else {
                                Err(extism_pdk::Error::msg(#panic_document))
                            }
                        }
                    }
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

        if !has_uuid && (!has_document_uuid || !has_accessor_id) {
            return syn::Error::new_spanned(
                struct_name,
                format!("Struct must have both `document_uuid: Option<String>` and `{}_id: Option<String>` fields.", accessor_type),
            )
                .to_compile_error()
                .into();
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
