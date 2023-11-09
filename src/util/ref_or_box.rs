pub enum RefOrBox<'a, T: ?Sized> {
    Ref(&'a T),
    Box(Box<T>),
}

impl<'a, G> RefOrBox<'a, G> {
    pub fn from_owned(inner: G) -> RefOrBox<'a, G> {
        Self::Box(Box::new(inner))
    }
}

impl<'a, T: ?Sized> RefOrBox<'a, T> {
    pub fn from_box(inner: Box<T>) -> RefOrBox<'a, T> {
        Self::Box(inner)
    }

    pub fn from_ref(inner: &'a T) -> RefOrBox<'a, T> {
        Self::Ref(inner)
    }
}

impl<'a, T: ?Sized> AsRef<T> for RefOrBox<'a, T> {
    fn as_ref(&self) -> &T {
        match self {
            RefOrBox::Ref(inner) => inner,
            RefOrBox::Box(inner) => inner.as_ref(),
        }
    }
}

impl<'a, T> From<T> for RefOrBox<'a, T> {
    fn from(value: T) -> Self {
        RefOrBox::from_owned(value)
    }
}

impl<'a, T> From<Box<T>> for RefOrBox<'a, T> {
    fn from(value: Box<T>) -> Self {
        RefOrBox::from_box(value)
    }
}

impl<'a, T> From<&'a T> for RefOrBox<'a, T> {
    fn from(value: &'a T) -> Self {
        RefOrBox::from_ref(value)
    }
}
