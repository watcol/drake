#![no_std]
extern crate alloc;

mod module;
mod files;

use alloc::string::String;
use alloc::vec::Vec;
use codespan_reporting::files::SimpleFiles;

pub use module::Module;
pub use files::Source;

/// A struct contains all runtime informations
#[derive(Debug, Clone)]
#[allow(dead_code)]
pub struct Runtime {
    modules: Vec<Module>,
    files: SimpleFiles<String, Source>,
}

impl Default for Runtime {
    #[inline]
    fn default() -> Self {
        Self {
            modules: Vec::new(),
            files: SimpleFiles::new(),
        }
    }
}

impl Runtime {
    /// Creates a new instance.
    #[inline]
    pub fn new() -> Self {
        Self::default()
    }

    /// Registers a new module by the name and the source code.
    pub fn add(&mut self, name: String, source: String) -> usize {
        if let Some((id, _)) = self.get_module_by_name(&name) {
            return id;
        }

        let source = Source::from(source);
        let mod_id = self.files.add(name.clone(), source.clone());

        let module = Module::new(name, source);
        self.modules.push(module);
        mod_id
    }

    /// Gets a slice of modules indexed by identifiers.
    #[inline]
    pub fn get_modules(&self) -> &[Module] {
        self.modules.as_slice()
    }

    /// Gets a reference of a corresponding module by the given identifier.
    #[inline]
    pub fn get_module(&self, id: usize) -> Option<&Module> {
        self.modules.get(id)
    }

    /// Gets a reference of a corresponding module by the given name.
    #[inline]
    pub fn get_module_by_name<S: AsRef<str>>(&self, name: S) -> Option<(usize, &Module)> {
        self.modules
            .iter()
            .enumerate()
            .find(|(_, m)| m.get_name() == name.as_ref())
    }

    /// Gets a mutable reference of a corresponding module by the module identifier.
    #[inline]
    pub fn get_mut_module(&mut self, id: usize) -> Option<&mut Module> {
        self.modules.get_mut(id)
    }

    /// Gets a mutable reference of a corresponding module by the given name.
    #[inline]
    pub fn get_mut_module_by_name<S: AsRef<str>>(
        &mut self,
        name: S,
    ) -> Option<(usize, &mut Module)> {
        self.modules
            .iter_mut()
            .enumerate()
            .find(|(_, m)| m.get_name() == name.as_ref())
    }
}
