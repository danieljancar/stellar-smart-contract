mod test {
    use super::*;
    use soroban_sdk::{Env};

    fn test_save_and_get_value() {
        let env = Env::default();
        let key = String::new("favorite_language");
        let value = String::new("Rust");

        MyContract::save_value(env.clone(), key.clone(), value.clone());
        let result = MyContract::get_value(env, key);

        assert_eq!(result, Some(value));
    }
}
