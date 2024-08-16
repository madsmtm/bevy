use proc_macro::TokenStream;
use quote::quote;
use syn::{parse_macro_input, ItemFn};

pub fn bevy_main(_attr: TokenStream, item: TokenStream) -> TokenStream {
    let input = parse_macro_input!(item as ItemFn);
    assert_eq!(
        input.sig.ident, "main",
        "`bevy_main` can only be used on a function called 'main'."
    );

    TokenStream::from(quote! {
        #[no_mangle]
        #[cfg(target_os = "android")]
        fn android_main(android_app: bevy::winit::android_activity::AndroidApp) {
            let _ = bevy::winit::ANDROID_APP.set(android_app);
            main();
        }

        #[allow(unused)]
        #input
    })
}
