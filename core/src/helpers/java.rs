/* 
use std::process::Command;

use thiserror::Error;

pub enum JavaError {

}

#[derive(Debug, Default)]
pub(crate) struct JavaHelper {
    common_args: Vec<String>
}

impl JavaHelper {
    #[allow(unused)]
	const fn new(args: Vec<String>) -> Self {
		Self {
			common_args: args,
		}
	}

    pub(crate) fn execute(&self, jvm_args: Vec<String>, main_class: &str, game_opts: Vec<String>) -> Result<(), JavaError> {
        if let Ok(Component::JavaComponent { path, .. }) = self.state().get_component("java") {
            let (exe, args) = match &self.state().wrapper {
                Some(wrapper) => (wrapper.as_str(), &["java"][..]),
                None => (path.as_str(), &[][..]),
            };

            let mut Command = Command::new(exe);
        }
    }
}
*/