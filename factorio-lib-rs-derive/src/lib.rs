extern crate proc_macro;

use proc_macro::TokenStream;
use quote::quote;
use syn;

#[proc_macro_derive(ModSetting)]
pub fn mod_setting_macro_derive(input: TokenStream) -> TokenStream {
    let ast = syn::parse(input).unwrap();
    impl_mod_setting_macro(&ast)
}

#[proc_macro_derive(PrototypeBase)]
pub fn prototype_base_macro_derive(input: TokenStream) -> TokenStream {
    let ast = syn::parse(input).unwrap();
    impl_prototype_base_macro(&ast)
}

fn impl_mod_setting_macro(ast: &syn::DeriveInput) -> TokenStream {
    let name = &ast.ident;
    let gen = quote! {
        impl ModSetting for #name<'_> {
            fn localised_name(&self) -> Option<LocalisedString> { self.localised_name.clone() }
            fn localised_description(&self) -> Option<LocalisedString> { self.localised_description.clone() }
            fn order(&self) -> Option<String> { self.order.clone() }
            fn hidden(&self) -> bool { self.hidden }
            fn setting_type(&self) -> ModSettingType { self.setting_type.clone() }
        }
    };
    gen.into()
}

fn impl_prototype_base_macro(ast: &syn::DeriveInput) -> TokenStream {
    let name = &ast.ident;
    let gen = quote! {
        impl PrototypeBase for #name<'_> {
            fn localised_name(&self) -> Option<LocalisedString> { self.localised_name.clone() }
            fn localised_description(&self) -> Option<LocalisedString> { self.localised_description.clone() }
            fn order(&self) -> String { self.order.clone() }
        }
    };
    gen.into()
}
