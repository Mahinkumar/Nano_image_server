use std::collections::HashMap;


use crate::plugin::error::ParseError;
use crate::plugin::inbuilt::resizer::resize;
use crate::plugin::{ImagePlugin, PluginOrdering, Plugins};


impl Plugins{
    pub fn get_plugin(&self, name: String) -> Result<(ImagePlugin, PluginOrdering), ParseError> {

        match self.registry.get(&name).cloned(){
            Some(result) => Ok(result),
            None => Err(ParseError::UnknownPlugin(name)),
        }
    }

    pub fn init_registry() -> Plugins {
        let mut registry: HashMap<String,(ImagePlugin, PluginOrdering)> = HashMap::new();

        // Register all plugins here
        registry.insert("resize".to_string(), (resize,PluginOrdering::Any));

        Plugins{
            registry
        }
        
    }
}