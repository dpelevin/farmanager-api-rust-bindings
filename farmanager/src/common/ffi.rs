pub trait AsInner<T> {
    fn as_inner(&self) -> &T;
}

pub trait AsMutInner<T> {
    fn as_mut_inner(&mut self) -> &mut T;
}

pub struct Array<T, R> where T: AsInner<R>, R: Copy {
    #[allow(dead_code)]
    buf: Box<[T]>,
    buf_of_inner: Box<[R]>,
    len: usize,
}

impl<T, R> Array<T, R> where T: AsInner<R>, R: Copy {
    pub fn new() -> Self {
        Array {
            buf: Vec::new().into_boxed_slice(),
            buf_of_inner: Vec::new().into_boxed_slice(),
            len: 0,
        }
    }

    pub fn as_ptr(&self) -> *const R {
        self.buf_of_inner.as_ptr()
    }

    pub fn len(&self) -> usize {
        self.len
    }
}

impl<T, R> From<Vec<T>> for Array<T, R> where T: AsInner<R>, R: Copy {
    fn from(v: Vec<T>) -> Self {
        let len = v.len();
        let buf: Box<[T]> = v.into_boxed_slice();
        let buf_of_inner: Box<[R]> = buf.iter().map(|s: &T| *s.as_inner())
            .collect::<Vec<R>>().into_boxed_slice();

        Array {
            buf,
            buf_of_inner,
            len,
        }
    }
}
