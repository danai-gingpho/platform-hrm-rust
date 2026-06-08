pub struct CodeGenerator;

impl CodeGenerator {
    pub fn generate(prefix: &str, last_code: Option<String>) -> String {
        let next_number = match last_code {
            Some(code) => {
                // Assuming code format is PREFIX001
                // We need to extract the numeric part. 
                // If prefix is "COMP", and code is "COMP001", we want "001"
                if code.starts_with(prefix) {
                    let number_str = &code[prefix.len()..];
                    match number_str.parse::<u32>() {
                        Ok(num) => num + 1,
                        Err(_) => 1,
                    }
                } else {
                    1
                }
            }
            None => 1,
        };

        format!("{}{:03}", prefix, next_number)
    }
}
