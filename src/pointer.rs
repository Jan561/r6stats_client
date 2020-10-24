#[cfg(not(feature = "multithreaded"))]
use std::cell::RefCell;
#[cfg(not(feature = "multithreaded"))]
use std::rc::Rc;

#[cfg(feature = "multithreaded")]
use std::sync::Arc;
#[cfg(feature = "multithreaded")]
use tokio::sync::RwLock;

#[cfg(not(feature = "multithreaded"))]
pub(crate) type Pointer<T> = Rc<RefCell<T>>;

#[cfg(feature = "multithreaded")]
pub(crate) type Pointer<T> = Arc<RwLock<T>>;
