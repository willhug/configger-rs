use proc_macro::TokenStream;

#[proc_macro_derive(ConfiggerData, attributes(configger))]
pub fn configger(item: TokenStream) -> TokenStream {
    println!("item: \"{}\"", item);
    // TODO figure this out.
    "".parse().unwrap()
}
