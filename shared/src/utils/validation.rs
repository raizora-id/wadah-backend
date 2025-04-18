use serde::Deserialize;

pub fn validate_email(email: &str) -> bool {
    // Basic email validation
    let email_regex = regex::Regex::new(
        r"^([a-z0-9_+]([a-z0-9_+.]*[a-z0-9_+])?)@([a-z0-9]+(\.[a-z0-9]+)*\.[a-z]{2,6})$",
    )
    .unwrap();
    
    email_regex.is_match(email)
}

pub fn validate_password(password: &str) -> bool {
    // Password must be at least 8 characters
    password.len() >= 8
}

pub fn validate_tenant_slug(slug: &str) -> bool {
    // Slug must be lowercase alphanumeric with hyphens
    let slug_regex = regex::Regex::new(r"^[a-z0-9]+(-[a-z0-9]+)*$").unwrap();
    
    slug_regex.is_match(slug) && slug.len() >= 3 && slug.len() <= 63
}

// Generic function to validate a struct using validator crate
pub fn validate_struct<T: Deserialize>(data: &str) -> Result<T, String> {
    match serde_json::from_str::<T>(data) {
        Ok(validated) => Ok(validated),
        Err(e) => Err(format!("Validation error: {}", e)),
    }
}
