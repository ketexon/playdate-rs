extern crate syn;
extern crate quote;

use proc_macro::{self, TokenStream};

use syn::{parse_macro_input, ItemStruct};
use syn::parse::Parse;
use quote::{quote, ToTokens};

#[proc_macro_attribute]
pub fn game(attr: TokenStream, item: TokenStream) -> TokenStream {
    // println!("Item: \"{}\"", item);
    let game_struct: ItemStruct = parse_macro_input!(item as ItemStruct);

    // for field in game_struct.clone().fields {
    //     println!(
    //         "Field: \n\tIdent: {}", 
    //         field.ident.to_token_stream()
    //     );
    //     println!("\n\tAttrs:");
    //     for attr in field.attrs {
    //         println!("\n\t\t{}", attr.to_token_stream());
    //     }
    // }

    let game_struct_ident = game_struct.ident.clone();

    let tokens = quote! {
        #game_struct

        #[no_mangle]
        unsafe extern "C" fn eventHandler(
            __pdrs_internal_pd: &'static ::pdrs::Playdate,
            __pdrs_internal_ev: ::pdrs::event::SystemEvent,
            __pdrs_internal_arg: u32
        ) -> i32 {
            static mut __PDRS_INTERNAL_GAME: ::std::option::Option<::std::boxed::Box<#game_struct_ident>>
                = ::std::option::Option::None;
            if ::std::matches!(__pdrs_internal_ev, ::pdrs::event::SystemEvent::Init) {
                __pdrs_internal_pd.init_allocator();
                __PDRS_INTERNAL_GAME = ::std::option::Option::Some(
                    ::std::boxed::Box::from(
                        <#game_struct_ident as ::pdrs::game::Game>::new(__pdrs_internal_pd)
                    )
                );

                __pdrs_internal_pd.system.set_update_callback_mut(
                    __pdrs_internal_update, 
                    __PDRS_INTERNAL_GAME.as_mut().unwrap()
                );
            }
            else {
                __PDRS_INTERNAL_GAME.as_mut().unwrap().handle_event(
                    ::pdrs::event::Event::from_system_event(
                        __pdrs_internal_ev,
                        __pdrs_internal_arg
                    )
                );
            }
            0
        }

        extern "C" fn __pdrs_internal_update(mut __pdrs_internal_game: &mut #game_struct_ident) -> i32 {
            __pdrs_internal_game.update();
            1
        }
    };

    tokens.into()
}