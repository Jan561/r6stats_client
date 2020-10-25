macro_rules! api {
    ($e:expr) => {
        concat!("https://api2.r6stats.com/public-api", $e)
    };
}

#[cfg(not(feature = "threadsafe"))]
macro_rules! deref {
    ($e:expr) => {
        $e.borrow()
    };
}

#[cfg(not(feature = "threadsafe"))]
macro_rules! deref_mut {
    ($e:expr) => {
        $e.borrow_mut()
    };
}

#[cfg(feature = "threadsafe")]
macro_rules! deref {
    ($e:expr) => {
        $e.read().await
    };
}

#[cfg(feature = "threadsafe")]
macro_rules! deref_mut {
    ($e:expr) => {
        $e.write().await
    };
}
