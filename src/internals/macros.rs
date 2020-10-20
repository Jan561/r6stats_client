macro_rules! api {
    ($e:expr) => {
        concat!("https://api2.r6stats.com/public-api", $e)
    };
}

#[cfg(all(not(feature = "threadsafe"), feature = "ratelimiting"))]
macro_rules! borrow {
    ($e:expr) => {
        $e.borrow()
    };
}

#[cfg(all(feature = "threadsafe", feature = "ratelimiting"))]
macro_rules! borrow {
    ($e:expr) => {
        $e.read().await
    };
}

#[cfg(all(not(feature = "threadsafe"), feature = "ratelimiting"))]
macro_rules! borrow_mut {
    ($e:expr) => {
        $e.borrow_mut()
    };
}

#[cfg(all(feature = "threadsafe", feature = "ratelimiting"))]
macro_rules! borrow_mut {
    ($e:expr) => {
        $e.write().await
    };
}
