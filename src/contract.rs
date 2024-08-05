use soroban_sdk::{contract, contractimpl, Env, String};

use crate::string_concatenator::strings_concatenator;

pub trait ConcatTrait {
    fn concat_strings(e: Env, string1: String, string2: String) -> String;
}

#[contract]
pub struct Concatenator;

#[contractimpl]
impl ConcatTrait for Concatenator {
    fn concat_strings(e: Env, string1: String, string2: String) -> String {
        let string_combined = strings_concatenator(&e, string1, string2);
        string_combined
    }
}
