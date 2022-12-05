//! This crate is not intended for use outside of [factorio-lib-rs]!

extern crate proc_macro;

use heck::AsSnakeCase;
use core::fmt::Display;
use core::iter::Iterator;
use proc_macro::TokenStream;
use quote::{format_ident, quote};
use syn::spanned::Spanned;
use syn::{
    self, parse_macro_input, punctuated::Punctuated, Attribute, DeriveInput, Ident, ItemStruct,
    LitStr, Result, Token,
};

// Thanks to Yand!rs from Rust Community Discord server for this macro
#[proc_macro_derive(Base)]
pub fn base_macro_derive(input: TokenStream) -> TokenStream {
    let input: ItemStruct = parse_macro_input!(input);
    let struct_name_str = input.ident.to_string();
    let trait_name_str = struct_name_str.trim_end_matches("Base").trim_end_matches("Spec");
    let struct_name = &input.ident;
    let trait_name = format_ident!("{}", trait_name_str, span = input.ident.span());
    let trait_name__ = format_ident!("{}__", trait_name);
    let parent_field_name = format_ident!("{}", AsSnakeCase(trait_name_str).to_string(), span = input.ident.span());

    let each_field_name = input
        .fields
        .iter()
        .map(|f| f.ident.as_ref().unwrap())
        .collect::<Vec<_>>();
    let each_field_set_name = each_field_name
        .iter()
        .map(|f| format_ident!("set_{}", f))
        .collect::<Vec<_>>();
    let each_field_type = input.fields.iter().map(|f| &f.ty).collect::<Vec<_>>();

    quote!(
        impl Base for #struct_name {}

        pub trait #trait_name {
            #(
                fn #each_field_name(&self) -> &#each_field_type;
                fn #each_field_set_name(&mut self, value: #each_field_type);
            )*
        }

        impl #trait_name for #struct_name {
            #(
                fn #each_field_name(&self) -> &#each_field_type {
                    &self.#each_field_name
                }
                fn #each_field_set_name(&mut self, value: #each_field_type) {
                    self.#each_field_name = value
                }
            )*
        }

        // Generate a declarative derive macro
        macro_rules! #trait_name__ {(
            $( #[$attrs:meta] )*
            $pub:vis
            struct $name:ident {
                $(
                $( #[$other_field_attrs:meta] )*
                $other_field_pub:vis
                $other_field_name:ident : $other_field_type:ty, )* $(,)?
            }
        ) => (
            impl #trait_name for $name {
                #(
                    fn #each_field_name(&self) -> &#each_field_type {
                        &self.#parent_field_name.#each_field_name()
                    }
                    fn #each_field_set_name(&mut self, value: #each_field_type) {
                        self.#parent_field_name.#each_field_set_name(value)
                    }
                )*
            }
        )}
        pub(crate) use #trait_name__ as #trait_name;
    )
    .into()
}

#[proc_macro_derive(Prototype, attributes(ptype))]
pub fn prototype_macro_derive(input: TokenStream) -> TokenStream {
    let ast = parse_macro_input!(input as DeriveInput);
    impl_prototype_macro(&ast)
}

#[proc_macro_derive(ModSetting)]
pub fn mod_setting_macro_derive(input: TokenStream) -> TokenStream {
    let ast = parse_macro_input!(input as DeriveInput);
    impl_mod_setting_macro(&ast)
}

#[proc_macro_derive(DataTableAccessable, attributes(data_table))]
pub fn data_table_accessable_macro_derive(input: TokenStream) -> TokenStream {
    let ast = parse_macro_input!(input as DeriveInput);
    impl_data_table_accessable_macro(&ast)
}

fn impl_prototype_macro(ast: &syn::DeriveInput) -> TokenStream {
    let name = &ast.ident;
    let prot_type = get_prot_type(&ast.attrs).unwrap_or_else(|| name.clone());
    let gen = quote! {
        impl Prototype for #name {
            fn name(&self) -> &String { &self.name }
            fn prototype_type(&self) -> PrototypeType { PrototypeType::#prot_type }
        }
    };
    gen.into()
}

fn get_prot_type(attrs: &Vec<Attribute>) -> Option<Ident> {
    for attr in attrs {
        if attr.path.is_ident("ptype") {
            return attr.parse_args().ok();
        }
    }
    None
}

fn impl_mod_setting_macro(ast: &syn::DeriveInput) -> TokenStream {
    let name = &ast.ident;
    let gen = quote! {
        impl ModSetting for #name {
            fn localised_name(&self) -> &Option<LocalisedString> { &self.localised_name }
            fn localised_description(&self) -> &Option<LocalisedString> { &self.localised_description }
            fn order(&self) -> &Option<String> { &self.order }
            fn hidden(&self) -> bool { self.hidden }
            fn setting_type(&self) -> ModSettingType { self.setting_type }
        }
    };
    gen.into()
}

#[proc_macro]
pub fn prot_from_lua_blanket(input: TokenStream) -> TokenStream {
    let target_type = parse_macro_input!(input as syn::Type);
    let gen = quote! {
        impl<'lua> PrototypeFromLua<'lua> for #target_type {
            fn prototype_from_lua(value: mlua::Value<'lua>, lua: &'lua mlua::Lua, data_table: &mut crate::prototypes::DataTable) -> mlua::prelude::LuaResult<Self> {
                lua.unpack(value)
            }
        }
    };
    gen.into()
}

#[proc_macro]
pub fn prot_from_str(input: TokenStream) -> TokenStream {
    let target_type = parse_macro_input!(input as syn::Type);
    let gen = quote! {
        impl<'lua> PrototypeFromLua<'lua> for #target_type {
            fn prototype_from_lua(v: mlua::Value<'lua>, l: &'lua mlua::Lua, dt: &mut crate::prototypes::DataTable) -> mlua::prelude::LuaResult<Self> {
                let s: String = l.unpack(v)?;
                s.parse().map_err(mlua::prelude::LuaError::external)
            }
        }
    };
    gen.into()
}

fn impl_data_table_accessable_macro(ast: &syn::DeriveInput) -> TokenStream {
    let name = &ast.ident;
    let attrs = &ast.attrs;

    let mut attrs = attrs
        .iter()
        .filter(|attr| attr.path.is_ident("data_table"))
        .map(|attr| {
            parse_data_table_attribute(attr).expect("failed to parse data_table attribute")
        });
    let attr = attrs.next().unwrap();
    let gen = quote! {
        impl crate::prototypes::DataTableAccessable for #name {
            fn find<'a>(data_table: &'a crate::prototypes::DataTable, name: &str) -> Result<&'a Self, crate::prototypes::PrototypesErr> {
                data_table.#attr.get(name).ok_or_else(|| crate::prototypes::PrototypesErr::PrototypeNotFound(name.into()))
            }

            fn extend(self, data_table: &mut crate::prototypes::DataTable) -> Result<(), crate::prototypes::PrototypesErr> {
                data_table.#attr.insert(self.name.clone(), self);
                Ok(())
            }
        }
    };
    gen.into()
}

fn parse_data_table_attribute(attr: &Attribute) -> Result<Ident> {
    let field: syn::Path = attr.parse_args()?;
    let ident = field.get_ident().expect("expected indentifier");
    Ok(ident.clone())
}

/// Attributes on fields
///
/// `#[default(expr)]` - `expr` is default value, which is used in case value of thsi field is None
/// after extraction
/// Incompatible with: `use_self`, `use_self_vec`, `use_self_forced`
///
/// `#[use_self]` - use the table which is used for constructing current prototype for property if
/// corresponding field does not exist in the table
/// Incompatible with: `default`, `from_str`, `use_self_vec`, `use_self_forced`, `resource`, `mandatory_if`
///
/// `#[use_self_vec]` - same as `use_self`, but puts result in a Vec
/// Incompatible with: `default`, `from_str`, `use_self`, `use_self_forced`, `resource`, `mandatory_if`
///
/// `#[use_self_forced]` - same as `use_self`, but forced instead of defaulting in case of failure.
/// Incompatible with: `default`, `from_str`, `use_self`, `use_self_vec`, `resource`, `mandatory_if`
///
/// `#[resource]` - this field is a resource record (sound only, textures should be done in post-extraction)
/// Incompatible with: `from_str`, `use_self`, `use_self_vec`, `use_self_forced`
///
/// `#[mandatory_if(expr)] - expr is a condition, if the condition results in `true`, field value
/// must be Some(_)
/// Incompatible with: `default`, `use_self`, `use_self_vec`, `use_self_forced`
///
/// `#[fallback(expr)]` - expr is used in case normal extraction retrieved None. Similar to
/// `#[default()]`, but applied before it and can return None. Can be used multiple times.
/// Use only on Option<>
/// Incompatible with: `use_self`, `use_self_vec`, `use_self_forced`
///
/// `#[required]` - use only with fallback. Yes, this is still a hack but a better one.
///
/// `#[rename(str)]` - str is a string supposed to be used for extracting field from table in case
/// name in table differs from name of this struct field
///
/// Attributes on container
///
/// `#[post_extr_fn(path)]` - path is a path to a function that needs to be executed after
/// field extraction and mandatory_if checks
#[proc_macro_derive(
    PrototypeFromLua,
    attributes(
        default,
        use_self,
        use_self_vec,
        use_self_forced,
        resource,
        mandatory_if,
        post_extr_fn,
        fallback,
        rename,
        required
    )
)]
pub fn prototype_from_lua_macro_derive(input: TokenStream) -> TokenStream {
    let ast = parse_macro_input!(input as DeriveInput);
    impl_prototype_from_lua_macro(&ast)
}

fn impl_prototype_from_lua_macro(ast: &syn::DeriveInput) -> TokenStream {
    let name = &ast.ident;
    let str_name = name.to_string();
    let data = {
        match &ast.data {
            syn::Data::Struct(d) => d,
            _ => panic!("expected struct"),
        }
    };
    let fields = {
        match &data.fields {
            syn::Fields::Named(f) => f.named.iter(),
            _ => panic!("expected named fields"),
        }
    };
    let (parsed_fields, mut mandatory_exprs): (
        Vec<proc_macro2::TokenStream>,
        Vec<Option<proc_macro2::TokenStream>>,
    ) = fields
        .clone()
        .map(|f| prot_from_lua_field(f).unwrap())
        .unzip();
    mandatory_exprs.retain(|mex| mex.is_some());
    let mandatory_exprs: Vec<proc_macro2::TokenStream> = mandatory_exprs
        .into_iter()
        .map(|mex| mex.unwrap())
        .collect();
    let field_names = fields.map(|f| &f.ident);
    let post_extr_fn: Option<syn::Path> = {
        let mut result = None;
        for attr in &ast.attrs {
            if attr.path.is_ident("post_extr_fn") {
                result = Some(attr.parse_args::<syn::Path>().unwrap())
            }
        }
        result
    };
    let post_extr = if let Some(pef) = post_extr_fn {
        quote! {
            #pef(&mut result, lua, data_table)?;
        }
    } else {
        quote! {}
    };
    let gen = quote! {
        impl<'lua> crate::prototypes::PrototypeFromLua<'lua> for #name {
            fn prototype_from_lua(value: mlua::Value<'lua>, lua: &'lua mlua::Lua, data_table: &mut crate::prototypes::DataTable) -> mlua::prelude::LuaResult<Self> {
                let str_name = #str_name;
                if let mlua::Value::Table(ref prot_table) = value {
                    #(#parsed_fields)*
                    #(#mandatory_exprs)*
                    let mut result = Self{#(#field_names),*};
                    #[allow(unnecessary_mut_passed)]
                    #post_extr
                    Ok(result)
                } else {
                    Err(mlua::Error::FromLuaConversionError{from: value.type_name(), to: str_name,
                    message: Some("Expected Table".into())})
                }
            }
        }
    };
    gen.into()
}

#[derive(Default)]
struct PrototypeFromLuaFieldAttrArgs {
    default_value: Option<proc_macro2::TokenStream>, // Incompatible with: use_self, use_self_vec
    mandatory_if: Option<proc_macro2::TokenStream>, // Incompatible with: default, use_self, use_self_vec
    fallbacks: Vec<proc_macro2::TokenStream>,
    rename: Option<String>,
    required: bool,
    // use_self* is incompatible with default and mandatory_if
    // Only 1 can be used:
    use_self: bool,
    use_self_vec: bool,
    use_self_forced: bool,
    is_resource: bool,
}

impl PrototypeFromLuaFieldAttrArgs {
    fn from_attrs(attrs: &[Attribute]) -> Result<Self> {
        let mut result = Self::default();
        for attr in attrs {
            for v in result.compat_check_matrix() {
                if attr.path.is_ident(v.0) {
                    result.check_compat(v.0, &v.2, attr)?;
                    v.1(&mut result, attr)?
                }
            }
        }
        Ok(result)
    }

    fn attr_error<T: Display>(attr: &Attribute, message: T) -> Result<()> {
        Err(syn::Error::new(attr.span(), message))
    }

    // This is horrifyingly inefficient, yet better than what was before
    fn compat_check_matrix<'a, 'b>(
        &'a self,
    ) -> Vec<(
        &'b str,
        fn(&mut Self, &Attribute) -> Result<()>,
        Vec<(&'b str, bool)>,
    )> {
        // These names don't make sense because they are not supposed to
        let sel = (
            ("use_self", self.use_self),
            ("use_self_vec", self.use_self_vec),
            ("use_self_forced", self.use_self_forced),
        );
        let oth = (
            ("default", self.default_value.is_some()),
            ("mandatory_if", self.mandatory_if.is_some()),
            ("resource", self.is_resource),
            ("fallback", !self.fallbacks.is_empty()),
        );
        vec![
            (
                "default",
                |s, a| {
                    s.default_value = Some(a.tokens.clone());
                    Ok(())
                },
                vec![
                    ("mandatory_if", self.mandatory_if.is_some()),
                    sel.0,
                    sel.1,
                    sel.2,
                ],
            ),
            (
                "mandatory_if",
                |s, a| {
                    s.mandatory_if = Some(a.tokens.clone());
                    Ok(())
                },
                vec![
                    ("default", self.default_value.is_some()),
                    sel.0,
                    sel.1,
                    sel.2,
                ],
            ),
            (
                "resource",
                |s, _| {
                    s.is_resource = true;
                    Ok(())
                },
                vec![sel.0, sel.1, sel.2],
            ),
            (
                "use_self",
                |s, _| {
                    s.use_self = true;
                    Ok(())
                },
                vec![oth.0, oth.1, oth.2, oth.3, sel.1, sel.2],
            ),
            (
                "use_self_vec",
                |s, _| {
                    s.use_self_vec = true;
                    Ok(())
                },
                vec![oth.0, oth.1, oth.2, oth.3, sel.0, sel.2],
            ),
            (
                "use_self_forced",
                |s, _| {
                    s.use_self_forced = true;
                    Ok(())
                },
                vec![oth.0, oth.1, oth.2, oth.3, sel.0, sel.1],
            ),
            (
                "fallback",
                |s, a| {
                    s.fallbacks.push(a.tokens.clone());
                    Ok(())
                },
                vec![sel.0, sel.1, sel.2],
            ),
            (
                "rename",
                |s, a| {
                    s.rename = Some(a.parse_args::<LitStr>()?.value());
                    Ok(())
                },
                vec![],
            ),
            (
                "required",
                |s, _| {
                    s.required = true;
                    Ok(())
                },
                vec![],
            ),
        ]
    }

    fn check_compat(
        &self,
        name: &str,
        incompats: &Vec<(&str, bool)>,
        attr: &Attribute,
    ) -> Result<()> {
        for incompat in incompats {
            if incompat.1 {
                return Self::attr_error(
                    attr,
                    format!("`{}` is incompatible with `{}`", name, incompat.0),
                );
            }
        }
        Ok(())
    }
}

fn prot_from_lua_field(
    field: &syn::Field,
) -> Result<(proc_macro2::TokenStream, Option<proc_macro2::TokenStream>)> {
    // First is get_expr, second is mandatory_if
    let ident = &field.ident;
    let field_type = &field.ty;
    let prototype_field_attrs = PrototypeFromLuaFieldAttrArgs::from_attrs(&field.attrs)?;
    let str_field = prototype_field_attrs
        .rename
        .unwrap_or_else(|| ident.as_ref().unwrap().to_string());
    let mut field_extr_type = if prototype_field_attrs.is_resource {
        quote! { String }
    } else {
        quote! { #field_type }
    };
    if prototype_field_attrs.default_value.is_some() || !prototype_field_attrs.fallbacks.is_empty()
    {
        field_extr_type = quote! { Option<#field_extr_type> };
    }
    let fallbacks = &prototype_field_attrs.fallbacks;
    let field_get_expr = {
        let mut get_expr = quote! { prot_table.get_prot::<_, #field_extr_type>(#str_field, lua, data_table)? #( .or_else(|| #fallbacks ) )* };
        if let Some(def_val) = &prototype_field_attrs.default_value {
            get_expr = quote! { #get_expr.or_else(|| Some(#def_val.into())).unwrap() };
        };
        get_expr
    };
    let get_self = quote! {
        crate::prototypes::PrototypeFromLua::prototype_from_lua(value.clone(), lua, data_table)
    };
    let get_expr = if prototype_field_attrs.use_self_forced {
        quote! {
            #get_self?;
        }
    } else if prototype_field_attrs.is_resource {
        quote! {
            {
                let name = #field_get_expr;
                data_table.register_resource(crate::prototypes::ResourceRecord{path: name.clone(), resource_type: crate::prototypes::ResourceType::Sound});
                name.into()
            };
        }
    } else if prototype_field_attrs.use_self_vec {
        quote! {
            prot_table.get_prot::<_, Option<#field_extr_type>>(#str_field, lua, data_table).transpose()
                .unwrap_or_else(|| Ok(Vec::from([#get_self?])))?;
        }
    } else if prototype_field_attrs.use_self {
        quote! {
            prot_table.get_prot::<_, Option<#field_extr_type>>(#str_field, lua, data_table).transpose()
                .unwrap_or_else(|| #get_self)?;
        }
    } else {
        quote! { #field_get_expr; }
    };
    let mand_expr = if let Some(mandatory_if) = prototype_field_attrs.mandatory_if {
        let err_str = format!("{} is required", ident.clone().unwrap());
        Some(quote! {
            if #mandatory_if && #ident.is_none() {
                return Err(mlua::Error::FromLuaConversionError{from: value.type_name(), to: str_name, message: Some(#err_str.into())})
            };
        })
    } else {
        None
    };
    let gen = quote! {
        let #ident: #field_type = #get_expr
    };
    Ok((gen, mand_expr))
}
