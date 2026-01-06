use crate::models::UserTarget;
use color_eyre::eyre::{eyre, Result};
use validator::Validate;

pub fn validate_input(target: &UserTarget) -> Result<()> {
    target.validate().map_err(|e| {
        let errors: Vec<String> = e.field_errors()
            .iter()
            .flat_map(|(field, errors)| {
                errors.iter().map(move |e| format!("{}: {}", field, e.code))
            })
            .collect();
        eyre!("Invalid input: {}", errors.join(", "))
    })?;
    Ok(())
}
