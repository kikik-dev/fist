use axum::{routing::MethodRouter, Router};

pub struct FistRouter<S = ()> {
    inner: Router<S>,
}

impl<S> FistRouter<S> where S: Clone + Send + Sync + 'static {
    pub fn new() -> Self {
        Self { inner: Router::new() }
    }

    pub fn route(mut self, path: &str, method: MethodRouter<S>) -> Self {
        self.inner = self.inner.route(path, method);
        self
    }

    pub fn build(self) -> Router<S> {
        self.inner
    }
}
