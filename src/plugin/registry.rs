use std::collections::HashMap;

use image::DynamicImage;

use crate::plugin::error::ParseError;
use crate::plugin::inbuilt::resize;
use crate::plugin::{ImagePlugin, PluginOrdering, Plugins};


trait PluginRegistry{
    async fn get_plugin(&self, name: String) ->  Result<(ImagePlugin, PluginOrdering), ParseError>;
    async fn init_registry() -> Self;
}


impl PluginRegistry for Plugins{
    async fn get_plugin(&self, name: String) -> Result<(ImagePlugin, PluginOrdering), ParseError> {

        match self.registry.get(&name).cloned(){
            Some(result) => Ok(result),
            None => Err(ParseError::UnknownPlugin(name)),
        }
    }

    async fn init_registry() -> Self {
        let mut registry: HashMap<String,(ImagePlugin, PluginOrdering)> = HashMap::new();

        // Insert all plugins in here
        registry.insert("resize".to_string(), (resize,PluginOrdering::Any));



        Plugins{
            registry
        }
        
    }
}