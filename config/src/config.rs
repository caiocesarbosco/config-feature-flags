pub fn set_config(valid_parameter:bool) -> Result<(),()> {
    if valid_parameter == true {
        return Ok(());
    }
    else {
        return Err(());
    }
}


#[cfg(test)]
mod unity_tests {
    use super::*;

    #[test]
    fn set_config_returns_ok() {
        assert_eq!(set_config(true),Ok(()));        
    }
    
    #[test]
    fn set_config_returns_nok() {
        assert_eq!(set_config(false),Err(()));        
    }     
    
}