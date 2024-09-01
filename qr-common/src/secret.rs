pub fn get_secret_from_env() -> [u8; 28] {
    let var_name = "HMAC_KEY";
    match std::env::var(var_name) {
        Ok(mut val) => {
            val.retain(|c| !c.is_whitespace());
            hex::decode(val).unwrap().try_into().unwrap()
        }
        Err(_) => {
            eprintln!("{} not set", var_name);
            std::process::exit(1);
        }
    }
}
