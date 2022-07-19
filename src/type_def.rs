use std::{
    any::TypeId,
    collections::{BTreeSet, HashMap, HashSet},
    path::PathBuf,
};

use ts_rs::{ExportError, TS};

// TODO
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub struct TSDependency {
    pub type_id: TypeId,
    pub ts_name: String,
}

impl From<ts_rs::Dependency> for TSDependency {
    fn from(dependency: ts_rs::Dependency) -> Self {
        Self {
            type_id: dependency.type_id,
            ts_name: dependency.ts_name,
        }
    }
}

/// TODO
pub trait TSType {
    fn name() -> String;

    fn dependencies() -> Vec<TSDependency>;

    fn export_to(path: PathBuf) -> Result<(), ExportError>;
}

impl<T: TS> TSType for T {
    fn name() -> String {
        // TODO: This is a very suboptiomal solution for https://github.com/Aleph-Alpha/ts-rs/issues/70
        match T::transparent() {
            true => T::inline(),
            false => T::name(),
        }
    }

    fn dependencies() -> Vec<TSDependency> {
        let mut dependencies = HashMap::new();
        T::dependencies(&mut dependencies);
        let mut dependencies: Vec<TSDependency> = dependencies
            .into_iter()
            .map(|v| v.1.into())
            .collect::<Vec<_>>();

        // TODO: Idk if this check is gonna be reliable enough.
        if T::EXPORT_TO != None && &T::name() != "Option" {
            dependencies.push(TSDependency {
                type_id: TypeId::of::<T>(),
                ts_name: T::name(),
            });
        }

        dependencies
    }

    fn export_to(path: PathBuf) -> Result<(), ExportError> {
        // TODO: Handle this error. Currently it throws when T is a primitive type but there is no good way to check this.
        T::export_to(path, &mut HashSet::new());
        Ok(())
    }
}

/// TODO
#[derive(Debug)]
pub struct TypeDef {
    pub(crate) arg_ty_name: String,
    pub(crate) arg_export: fn(PathBuf) -> Result<(), ExportError>,
    pub(crate) result_ty_name: String,
    pub(crate) result_export: fn(PathBuf) -> Result<(), ExportError>,
    pub(crate) dependencies: BTreeSet<TSDependency>,
}

impl TypeDef {
    pub(crate) fn new<TArg: TSType + 'static, TResolverResult: TSType + 'static>() -> Self {
        let mut dependencies = TArg::dependencies();
        dependencies.extend(TResolverResult::dependencies());

        Self {
            arg_ty_name: TArg::name(),
            arg_export: TArg::export_to,
            result_ty_name: TResolverResult::name(),
            result_export: TResolverResult::export_to,
            dependencies: BTreeSet::from_iter(dependencies),
        }
    }
}