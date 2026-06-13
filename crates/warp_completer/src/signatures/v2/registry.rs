//! `CommandRegistry` contains all registered `CommandSignature`s that are eligible for use in
//! completion suggestion generation.
//!
//! Completion engine callers must supply a `CommandRegistry` in their `CompletionContext`
//! implementation to generate suggestions for registered commands.
use std::sync::{Arc, OnceLock};

use memo_map::MemoMap;

use super::CommandSignature;

static GLOBAL_REGISTRY: OnceLock<Arc<CommandRegistry>> = OnceLock::new();

#[derive(Debug, Default, Clone)]
pub struct CommandRegistry {
    signatures: MemoMap<String, CommandSignature>,
}

impl CommandRegistry {
    pub fn new() -> Self {
        Self {
            signatures: MemoMap::default(),
        }
    }

    pub fn get_signature(&self, command: impl AsRef<str>) -> Option<&CommandSignature> {
        self.signatures.get(command.as_ref())
    }

    pub fn register_signature(&self, signature: CommandSignature) {
        self.signatures
            .insert(signature.command.name.clone(), signature);
    }

    pub fn registered_commands(&self) -> impl Iterator<Item = &str> {
        self.signatures.keys().map(|key| key.as_str())
    }

    pub fn global_instance() -> Arc<Self> {
        GLOBAL_REGISTRY
            .get_or_init(|| {
                // TODO(wasm): Determine how to asynchronously load command signatures on wasm.
                let registry = CommandRegistry::new();
                #[cfg(target_os = "macos")]
                registry.register_signature(macos_trash_signature());
                Arc::new(registry)
            })
            .clone()
    }

    pub fn empty() -> Self {
        Self::new()
    }
}

#[cfg(target_os = "macos")]
fn macos_trash_signature() -> CommandSignature {
    use super::{Argument, ArgumentValue, Arity, Command, Priority, TemplateType};

    CommandSignature {
        command: Command {
            name: "trash".to_owned(),
            alias: vec![],
            description: Some("Move files and folders to the trash".to_owned()),
            arguments: vec![Argument {
                name: "paths".to_owned(),
                values: vec![ArgumentValue::Template {
                    type_name: TemplateType::FilesAndFolders,
                    filter_name: None,
                }],
                optional: false,
                arity: Some(Arity {
                    limit: None,
                    delimiter: None,
                }),
                description: None,
            }],
            subcommands: vec![],
            options: vec![],
            priority: Priority::default(),
        },
    }
}

#[cfg(test)]
#[path = "registry_test.rs"]
mod test;
