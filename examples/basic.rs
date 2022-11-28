use redorm::prelude::*;

#[derive(DeriveHashSet)]
#[redorm(prefix_name = "custom_prefix")]
pub struct Poet {
    #[redorm(key)]
    name: String,
    born: String,
    died: String,
    genre: String,
    nationality: String,
}

fn main() {
    let verlaine = Poet {
        name: "Verlaine".into(),
        born: "1884".into(),
        died: "1896".into(),
        genre: "Decadent".into(),
        nationality: "French".into(),
    };
    assert_eq!("HSET custom_prefix:Verlaine born 1884 died 1896 genre Decadent nationality French" ,verlaine.set_command());
    assert_eq!("HGETALL Verlaine", verlaine.getall_command());
}
