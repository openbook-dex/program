#![deny(unaligned_references)]
#![allow(clippy::try_err)]

#[macro_use]
pub mod error;

#[cfg(test)]
mod tests;

pub mod critbit;
mod fees;
pub mod instruction;
pub mod matching;
pub mod state;

#[cfg(all(feature = "program", not(feature = "no-entrypoint")))]
use solana_program::entrypoint;
#[cfg(feature = "program")]
use solana_program::{account_info::AccountInfo, entrypoint::ProgramResult, pubkey::Pubkey};

#[cfg(feature = "program")]
#[cfg(not(feature = "no-entrypoint"))]
entrypoint!(process_instruction);
#[cfg(feature = "program")]
fn process_instruction(
    program_id: &Pubkey,
    accounts: &[AccountInfo],
    instruction_data: &[u8],
) -> ProgramResult {
    Ok(state::State::process(
        program_id,
        accounts,
        instruction_data,
    )?)
}

#[cfg(not(feature = "no-entrypoint"))]
use {default_env::default_env, solana_security_txt::security_txt};
#[cfg(not(feature = "no-entrypoint"))]
security_txt! {
    // Required fields
    name: "OpenBook DEX",
    project_url: "https://github.com/openbook-dex",
    contacts: "link:https://github.com/openbook-dex/program/security/advisories/new",
    policy: "https://raw.githubusercontent.com/openbook-dex/program/master/SECURITY.md",

    // Optional Fields
    preferred_languages: "en",
    source_code: "https://github.com/openbook-dex/program",
    source_revision: default_env!("GITHUB_SHA", "unknown source revision"),
    source_release: default_env!("GITHUB_REF_NAME", "unknown source release")
}
