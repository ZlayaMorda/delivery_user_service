use argon2::{Argon2, PasswordHash, PasswordHasher};





pub fn hashing_password<'a>(
    password_input: &'a str,
    salt: &'a str
) -> Result<PasswordHash<'a>, &'a str>{

    if password_input.is_empty() {
        return Err("Password must be not empty");
    }

    match Argon2::default()
        .hash_password(password_input.as_bytes(), salt) {
        Ok(result) => Ok(result),
        Err(_error) => Err("Error while hashing password")
    }
}