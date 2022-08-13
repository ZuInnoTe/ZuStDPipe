use std::any::Any;

pub enum ModuleKind {
    Cache,
    Index,
    Search,
    Signal,
    Step,
    Storage,
}

pub struct ModuleDefinition {
    name: String,
    version: String,
    zusearch_version_constraint: String,
    kind: ModuleKind,
}

pub trait Module: Any + Send + Sync {}
