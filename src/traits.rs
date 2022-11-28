use std::collections::HashMap;

pub trait HashSet {
    fn gen_hset_args(&self) -> Vec<String>;
    fn get_hset_from_args(args: &HashMap<String, String>) -> Self;
    fn getall_command(&self) -> String;
    fn set_command(&self) -> String;
    fn get_prefix() -> String;
    fn key_name() -> String;
    fn get_key(&self) -> &String;
}