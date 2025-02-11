//! # House Finder
//!
//! Just my personal script to find myself a house to buy.
#![forbid(unsafe_code)]
#![forbid(bare_trait_objects)]
#![forbid(elided_lifetimes_in_paths)]
#![forbid(non_ascii_idents)]
#![forbid(unused_import_braces)]
#![deny(missing_docs)]
#![deny(warnings)]
#![deny(unused_doc_comments)]
#![deny(dead_code)]
#![deny(unreachable_code)]
#![deny(unused_must_use)]
#![deny(unused_mut)]
#![deny(unused_variables)]
#![deny(absolute_paths_not_starting_with_crate)]
#![deny(anonymous_parameters)]
#![deny(deprecated_in_future)]
#![deny(explicit_outlives_requirements)]
#![deny(keyword_idents)]
#![deny(macro_use_extern_crate)]
#![deny(meta_variable_misuse)]
#![deny(missing_copy_implementations)]
#![deny(missing_debug_implementations)]
#![deny(missing_doc_code_examples)]
#![deny(missing_fragment_specifier)]
#![deny(no_mangle_generic_items)]
#![deny(non_camel_case_types)]
#![deny(non_shorthand_field_patterns)]
#![deny(non_snake_case)]
#![deny(non_upper_case_globals)]
#![deny(single_use_lifetimes)]
#![deny(trivial_casts)]
#![deny(trivial_numeric_casts)]
#![deny(unsafe_op_in_unsafe_fn)]
#![deny(unused_crate_dependencies)]
#![deny(unused_extern_crates)]
#![deny(unused_lifetimes)]
#![deny(unused_macro_rules)]
#![deny(unused_results)]
#![deny(variant_size_differences)]
#![deny(unused_allocation)]
#![deny(unused_comparisons)]
#![deny(unused_parens)]

// Not used directly, but required to function.
use openssl as _;

mod app;
mod cache;
mod client;
mod elisa;
mod etuovi;
mod open_route_service;
mod telegram;

#[tokio::main]
async fn main() -> std::result::Result<(), self::app::Error> {
    let args: self::app::Args = <self::app::Args as clap::Parser>::parse();
    let telegram: std::option::Option<telegram::Telegram> = match args.telegram_bot_token.clone() {
        Some(telegram_bot_token) => match args.telegram_user_id.clone() {
            Some(telegram_user_id) => Some(telegram::Telegram::new(
                &telegram_bot_token,
                telegram_user_id,
            )),
            // Should not happen if clap is configured and working correctly.
            None => std::panic!("--telegram-bot-token was given but not --telegram-user-id."),
        },
        None => {
            if args.telegram_user_id.is_some() {
                // Should not happen if clap is configured and working correctly.
                std::panic!("--telegram-user-id was given but not --telegram-bot-token.");
            }
            None
        }
    };
    match self::app::run(args, telegram.clone()).await {
        Ok(count) => {
            let message: std::string::String = format!("Found {}!", count);
            println!("{}", message);
            if let Some(telegram) = &telegram {
                let _: teloxide::prelude::Message = telegram.send_message(&message).await?;
            }
            Ok(())
        }
        Err(error) => {
            let message: std::string::String = format!("Got error: {:?}", error);
            eprintln!("{}", message);
            eprintln!("Backtrace: {:?}", std::backtrace::Backtrace::capture());
            if let Some(telegram) = &telegram {
                let _: teloxide::prelude::Message = telegram.send_message(&message).await?;
            }
            Err(error)
        }
    }
}
