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
        crate::Pointer::new(tokio::sync::Mutex::new($e))
    };
}

#[cfg(not(feature = "multithreaded"))]
macro_rules! deref {
    ($e:expr) => {
        $e.borrow_mut()
    };
}

#[cfg(feature = "multithreaded")]
macro_rules! deref {
    ($e:expr) => {
        $e.lock().await
    };
}
