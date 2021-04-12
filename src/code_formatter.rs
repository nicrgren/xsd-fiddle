use crate::{ObjectImpl, TypeName};
use std::fmt;

pub trait CodeFormatter {
    fn write_impl_file<W>(&mut self, w: &mut W, object: &ObjectImpl) -> fmt::Result
    where
        W: fmt::Write;

    fn write_type<W>(&mut self, w: &mut W, p: &TypeName) -> fmt::Result
    where
        W: fmt::Write;
}
