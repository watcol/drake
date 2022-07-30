#![no_std]
extern crate alloc;

mod files;
pub mod module;

use alloc::string::String;
use alloc::vec::Vec;
use codespan_reporting::files::{Error, Files};
use core::ops::Range;

#[doc(inline)]
pub use module::Module;

/// A struct contains all runtime informations
#[derive(Debug, Clone)]
#[allow(dead_code)]
pub struct Runtime {
    modules: Vec<Module>,
}

impl<'a> Files<'a> for Runtime {
    type FileId = usize;
    type Name = <Module as Files<'a>>::Name;
    type Source = <Module as Files<'a>>::Source;

    #[inline]
    fn name(&'a self, id: Self::FileId) -> Result<Self::Name, Error> {
        self.get_module(id).ok_or(Error::FileMissing)?.name(())
    }

    #[inline]
    fn source(&'a self, id: Self::FileId) -> Result<Self::Source, Error> {
        self.get_module(id).ok_or(Error::FileMissing)?.source(())
    }

    #[inline]
    fn line_index(&'a self, id: Self::FileId, byte_index: usize) -> Result<usize, Error> {
        self.get_module(id)
            .ok_or(Error::FileMissing)?
            .line_index((), byte_index)
    }

    #[inline]
    fn line_range(&'a self, id: Self::FileId, line_index: usize) -> Result<Range<usize>, Error> {
        self.get_module(id)
            .ok_or(Error::FileMissing)?
            .line_range((), line_index)
    }
}

impl Runtime {
    /// Creates a new instance.
    #[inline]
    pub fn new(name: String, source: String) -> Self {
        Self {
            modules: alloc::vec![Module::new(name, source)],
        }
    }

    /// Gets a slice of modules indexed by identifiers.
    #[inline]
    pub fn get_modules(&self) -> &[Module] {
        self.modules.as_slice()
    }

    /// Gets a reference of a module corresponding to given identifier.
    #[inline]
    pub fn get_module(&self, id: usize) -> Option<&Module> {
        self.modules.get(id)
    }

    /// Gets a reference of a module corresponding to given name.
    #[inline]
    pub fn get_module_by_name<S: AsRef<str>>(&self, name: S) -> Option<(usize, &Module)> {
        self.modules
            .iter()
            .enumerate()
            .find(|(_, m)| m.get_name() == name.as_ref())
    }
}
