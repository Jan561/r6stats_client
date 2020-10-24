macro_rules! api {
    ($e:expr) => {
        concat!("https://api2.r6stats.com/public-api", $e)
    };
}

#[cfg(not(feature = "multithreaded"))]
macro_rules! new_ptr {
    ($e:expr) => {
        crate::Pointer::new(std::cell::RefCell::new($e))
    };
}

#[cfg(feature = "multithreaded")]
macro_rules! new_ptr {
    ($e:expr) => {
        crate::Pointer::new(tokio::sync::RwLock::new($e))
    };
}

#[cfg(not(feature = "multithreaded"))]
macro_rules! deref {
    ($e:expr) => {
        $e.borrow()
    };
}

#[cfg(not(feature = "multithreaded"))]
macro_rules! deref_mut {
    ($e:expr) => {
        $e.borrow_mut()
    };
}

#[cfg(feature = "multithreaded")]
macro_rules! deref {
    ($e:expr) => {
        $e.read().await
    };
}

#[cfg(feature = "multithreaded")]
macro_rules! deref_mut {
    ($e:expr) => {
        $e.write().await
    };
}
