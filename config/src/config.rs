pub enum configDataType {
    STRING,
    U64,
    U32,
    U16,
    U8,
    BOOLEAN,
}

pub struct Config {
    config_type: configDataType,
    serialized_config: String,
}

impl Config {

    pub fn new(cfg_type:configDataType, serialized_cfg: String) -> Self {
        Config {config_type: cfg_type, serialized_config: serialized_cfg}
    }

    pub fn set_config(valid_parameter:bool) -> Result<(),()> {
        if valid_parameter == true {
            return Ok(());
        }
        else {
            return Err(());
        }
    }
}



#[cfg(test)]
mod unity_tests {
    use super::*;

    #[test]
    fn set_config_returns_ok() {
        assert_eq!(Config::set_config(true),Ok(()));        
    }

    #[test]
    fn set_config_returns_nok() {
        assert_eq!(Config::set_config(false),Err(()));        
    }
    
    fn valid_config_type(cfg_type: configDataType) -> bool {
        let valid_type = match cfg_type {
            configDataType::STRING => true,
            configDataType::U64 => true,
            configDataType::U32 => true,
            configDataType::U16 => true,
            configDataType::U8 => true,
            configDataType::BOOLEAN => true,
            _ => false,
        };
        return valid_type;
    }

    #[test]
    fn validating_possible_config_type() {
        let string_config = Config::new(configDataType::STRING, "".to_string());
        assert_eq!(valid_config_type(string_config.config_type), true);
        let u64_config = Config::new(configDataType::U64, "".to_string());
        assert_eq!(valid_config_type(u64_config.config_type), true);
        let u32_config = Config::new(configDataType::U32, "".to_string());
        assert_eq!(valid_config_type(u32_config.config_type), true);
        let u16_config = Config::new(configDataType::U16, "".to_string());
        assert_eq!(valid_config_type(u16_config.config_type), true);
        let u8_config = Config::new(configDataType::U8, "".to_string());
        assert_eq!(valid_config_type(u8_config.config_type), true);
        let boolean_config = Config::new(configDataType::BOOLEAN, "".to_string());
        assert_eq!(valid_config_type(boolean_config.config_type), true);
    }
    
}