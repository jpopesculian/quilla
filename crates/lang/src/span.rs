#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Span {
    pub start: usize,
    pub end: usize,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Spanned<T> {
    pub inner: T,
    pub span: Span,
}

impl<T> Spanned<T> {
    pub fn new(inner: T, span: Span) -> Self {
        Self { inner, span }
    }

    pub fn map<U, F: FnOnce(T) -> U>(self, f: F) -> Spanned<U> {
        Spanned {
            inner: f(self.inner),
            span: self.span,
        }
    }
}

impl<T> core::fmt::Display for Spanned<T>
where
    T: core::fmt::Display,
{
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        self.inner.fmt(f)
    }
}

impl<T> core::error::Error for Spanned<T>
where
    T: core::error::Error + core::fmt::Debug + core::fmt::Display + 'static,
{
    fn source(&self) -> Option<&(dyn core::error::Error + 'static)> {
        Some(&self.inner)
    }
}
