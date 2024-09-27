extern crate proc_macro;
use proc_macro::TokenStream;
use quote::quote;
use syn::{parse_macro_input, ItemFn};

#[proc_macro_attribute]
pub fn interrupt(_attr: TokenStream, item: TokenStream) -> TokenStream {
    let input = parse_macro_input!(item as ItemFn);
    let fn_name = &input.sig.ident;
    let fn_block = &input.block;

    let expanded = quote! {
        #[naked]
        pub unsafe extern "C" fn #fn_name() -> ! {
            #[inline(always)]
            extern "C" fn internals() {
                #fn_block
            }

            unsafe {
                core::arch::asm!(
                    "call {}",
                    "iretq",
                    sym internals,
                    options(noreturn, att_syntax)
                );
            }
        }
    };

    TokenStream::from(expanded)
}