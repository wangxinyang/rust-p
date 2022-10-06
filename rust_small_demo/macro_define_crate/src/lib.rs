use proc_macro::TokenStream;
use syn::ItemFn;

// #[proc_macro_attribute]
// pub fn test_proc_macro(attr: TokenStream, item: TokenStream) -> TokenStream {
//     eprintln!("---------attr部分-----------");
//     eprintln!("{:?}", attr);
//     eprintln!("---------item部分-----------");
//     eprintln!("{:#?}", item);
//     item
// }
// #[proc_macro_attribute]
// pub fn test_proc_macro(attr: TokenStream, item: TokenStream) -> TokenStream {
//     let input_fn = syn::parse_macro_input!(item as ItemFn);
//     eprintln!("{:?}", input_fn);
//     attr
// }
