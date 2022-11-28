pub mod derive_attr {
    use bae::FromAttributes;

    ///Attributes for Model
    #[derive(Default, FromAttributes)]
    pub struct Redorm {
        pub prefix_name: Option<syn::Lit>,
        pub key: Option<syn::Ident>,
    }
}
