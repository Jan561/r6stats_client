#[cfg(not(feature = "threadsafe"))]
pub(crate) type Cell<T> = std::cell::RefCell<T>;

#[cfg(feature = "threadsafe")]
pub(crate) type Cell<T> = tokio::sync::RwLock<T>;
