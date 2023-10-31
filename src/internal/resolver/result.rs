//! TODO
//!
//! TODO: Rename this file

use std::{
    pin::Pin,
    task::{Context, Poll},
};

use futures::{ready, Stream};
use serde::Serialize;

#[cfg(feature = "tracing")]
type Inner = tracing::Span;

use rspc_core::{error::ExecError, internal::IntoResolverError};

#[cfg(not(feature = "tracing"))]
type Inner = ();

pub(crate) use private::*;

pub(crate) mod private {
    use pin_project_lite::pin_project;
    use rspc_core::ValueOrBytes;

    use super::*;

    pin_project! {
        pub struct StreamToBody<S> {
            #[pin]
            pub(crate) stream: S,
            pub(crate) span: Option<Inner>
        }
    }

    impl<
            S: Stream<Item = Result<T, TErr>> + Send + 'static,
            T: Serialize + 'static,
            TErr: IntoResolverError,
        > Stream for StreamToBody<S>
    {
        type Item = Result<ValueOrBytes, ExecError>;

        fn poll_next(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Option<Self::Item>> {
            let this = self.project();

            #[cfg(feature = "tracing")]
            let _span = this.span.as_ref().map(|s| s.enter());

            match ready!(this.stream.poll_next(cx)) {
                Some(Ok(v)) => Poll::Ready(Some(
                    serde_json::to_value(v)
                        .map_err(ExecError::SerializingResultErr)
                        .map(ValueOrBytes::Value),
                )),
                Some(Err(e)) => {
                    Poll::Ready(Some(Err(ExecError::Resolver(e.into_resolver_error()))))
                }
                None => Poll::Ready(None),
            }
        }

        fn size_hint(&self) -> (usize, Option<usize>) {
            self.stream.size_hint()
        }
    }
}
