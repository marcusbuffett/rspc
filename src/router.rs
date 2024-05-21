use std::{borrow::Cow, collections::HashMap, error, fmt};

use crate::procedure::Procedure;

pub struct Router<TCtx = (), TErr: error::Error = crate::Infallible>(
    HashMap<Cow<'static, str>, Procedure<TCtx, TErr>>,
);

impl<TCtx, TErr: error::Error> fmt::Debug for Router<TCtx, TErr> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_tuple("Router").field(&self.0).finish()
    }
}

impl<TCtx, TErr: error::Error> Default for Router<TCtx, TErr> {
    fn default() -> Self {
        Self(Default::default())
    }
}

impl<TCtx, TErr: error::Error> Router<TCtx, TErr> {
    pub fn procedure(
        mut self,
        name: impl Into<Cow<'static, str>>,
        procedure: Procedure<TCtx>,
    ) -> Self {
        let name = name.into();

        todo!();
    }

    pub fn merge(self, other: Self) -> Self {
        todo!();
    }

    pub fn iter(&self) -> impl Iterator<Item = (&Cow<'static, str>, &Procedure<TCtx, TErr>)> {
        self.0.iter()
    }
}

impl<TCtx, TErr: error::Error> FromIterator<(Cow<'static, str>, Procedure<TCtx, TErr>)>
    for Router<TCtx, TErr>
{
    fn from_iter<I: IntoIterator<Item = (Cow<'static, str>, Procedure<TCtx, TErr>)>>(
        iter: I,
    ) -> Self {
        let mut router = Router::<TCtx, TErr>::default();
        for (path, procedure) in iter {
            router.0.insert(path, procedure);
        }
        router
    }
}

impl<TCtx, TErr: error::Error> IntoIterator for Router<TCtx, TErr> {
    type Item = (Cow<'static, str>, Procedure<TCtx, TErr>);
    // TODO: This leaks the `HashMap` implementation detail into the public API. It would be nice if Rust let us `type IntoIter = impl Iterator<Item = ...>;`.
    type IntoIter = std::collections::hash_map::IntoIter<Cow<'static, str>, Procedure<TCtx, TErr>>;

    fn into_iter(self) -> Self::IntoIter {
        self.0.into_iter()
    }
}
