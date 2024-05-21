use std::{borrow::Cow, error, fmt, marker::PhantomData};

use specta::{DataType, TypeDefs};

use super::{
    builder::GG,
    exec_input::{AnyInput, InputValueInner},
    stream::ProcedureStream,
    InternalError, ProcedureBuilder, ProcedureInput,
};

/// Represents a single operations on the server that can be executed.
///
/// A [`Procedure`] is built from a [`ProcedureBuilder`] and holds the type information along with the logic to execute the operation.
///
pub struct Procedure<TCtx = (), TErr: error::Error = crate::Infallible> {
    pub(super) ty: ProcedureType,
    pub(super) input: fn(&mut TypeDefs) -> DataType,
    pub(super) result: fn(&mut TypeDefs) -> DataType,
    pub(super) handler:
        Box<dyn Fn(TCtx, &mut dyn InputValueInner) -> Result<ProcedureStream<TErr>, InternalError>>,
}

impl<TCtx, TErr: error::Error> fmt::Debug for Procedure<TCtx, TErr> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("Procedure").finish()
    }
}

impl<TCtx, TErr: error::Error> Procedure<TCtx, TErr> {
    /// Construct a new procedure using [`ProcedureBuilder`].
    pub fn builder<R, I>() -> ProcedureBuilder<TCtx, TErr, GG<TCtx, R, I>> {
        ProcedureBuilder::<_, TErr, _> {
            input: None,
            phantom: PhantomData,
        }
    }

    /// Export the [Specta](https://docs.rs/specta) types for this procedure.
    ///
    /// TODO - Use this with `rspc::typescript`
    ///
    /// # Usage
    /// ```rust
    /// todo!(); # TODO: Example
    /// ```
    pub fn types(
        &self,
        key: Cow<'static, str>,
        type_map: &mut TypeDefs,
    ) -> ProcedureTypeDefinition {
        ProcedureTypeDefinition {
            key,
            ty: self.ty,
            input: (self.input)(type_map),
            result: (self.result)(type_map),
        }
    }

    /// Execute a procedure with the given context and input.
    ///
    /// This will return a [`ProcedureStream`] which can be used to stream the result of the procedure.
    ///
    /// # Usage
    /// ```rust
    /// use serde_json::Value;
    ///
    /// fn run_procedure(procedure: Procedure) -> Vec<Value> {
    ///     procedure
    ///         .exec((), Value::Null)
    ///         .collect::<Vec<_>>()
    ///         .await
    ///         .into_iter()
    ///         .map(|result| result.serialize(serde_json::value::Serializer).unwrap())
    ///         .collect::<Vec<_>>()
    /// }
    /// ```
    pub fn exec<'de, T: ProcedureInput<'de>>(
        &self,
        ctx: TCtx,
        input: T,
    ) -> Result<ProcedureStream<TErr>, InternalError> {
        match input.into_deserializer() {
            Ok(deserializer) => {
                let mut input = <dyn erased_serde::Deserializer>::erase(deserializer);
                (self.handler)(ctx, &mut input)
            }
            Err(input) => (self.handler)(ctx, &mut AnyInput(Some(input.into_value()))),
        }
    }
}

pub struct ProcedureTypeDefinition {
    pub key: Cow<'static, str>,
    pub ty: ProcedureType,
    pub input: DataType,
    pub result: DataType,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub enum ProcedureType {
    Query,
    Mutation,
    Subscription,
}
