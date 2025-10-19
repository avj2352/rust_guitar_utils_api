use serde_json::Value;
use std::error::Error;

pub fn json_to_yaml(json_str: &str) -> Result<String, Box<dyn Error>> {
    // Parse the JSON string into a serde_json::Value
    let json_value: Value = serde_json::from_str(json_str)?;    
    // Convert the JSON Value to a YAML string
    let yaml_str = serde_yaml::to_string(&json_value)?;
    
    Ok(yaml_str)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_json_to_yaml() {
        let json_str = r#"
    {
        "name": "John Doe",
        "age": 30,
        "city": "New York"   
    }
    "#;

        let yaml_str = json_to_yaml(json_str).unwrap();
        println!("{}", yaml_str);
        assert_eq!(
            yaml_str,
            "age: 30\ncity: New York\nname: John Doe\n"
        );
    }
}

