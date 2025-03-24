use inquire::{CustomType, error::InquireError};

pub struct NumberPrompt {

}

impl NumberPrompt {
    pub fn prompt(prompt_message: &str) -> Result<i32, InquireError> {
        CustomType::<i32>::new(prompt_message)
            .with_error_message("Please enter a valid number.")
            .prompt()
    }
}

#[cfg(test)]
mod tests {
    use crate::commands::start::number_prompt::NumberPrompt;


    #[test]
    fn test_valid_number() {
        // This is a placeholder test. Actual testing would require mocking user input.
        let result = NumberPrompt::prompt("Enter a number:");
        assert!(result.is_ok());
    }
}