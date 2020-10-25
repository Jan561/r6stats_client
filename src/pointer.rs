#[cfg(not(feature = "threadsafe"))]
use std::cell::RefCell;
#[cfg(not(feature = "threadsafe"))]
use std::rc::Rc;

#[cfg(feature = "threadsafe")]
use std::sync::Arc;
#[cfg(feature = "threadsafe")]
use tokio::sync::RwLock;

#[cfg(not(feature = "threadsafe"))]
pub(crate) type Pointer<T> = Rc<T>;

#[cfg(feature = "threadsafe")]
pub(crate) type Pointer<T> = Arc<T>;

#[cfg(not(feature = "threadsafe"))]
pub(crate) type Cell<T> = RefCell<T>;

#[cfg(feature = "threadsafe")]
pub(crate) type Cell<T> = RwLock<T>;
