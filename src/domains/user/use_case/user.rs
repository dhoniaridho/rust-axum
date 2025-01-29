
pub struct RegisterUserUseCase;

impl RegisterUserUseCase {
    pub fn execute() -> Result<String, String> {
        Ok("User registered successfully".to_string())
    }
}
