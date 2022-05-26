#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error("Io: {0}")]
    Io(#[from] std::io::Error),
    #[error("NoResponse")]
    NoResponse,
}

use futures::Future;

#[cfg(not(target_arch = "wasm32"))]
use {once_cell::sync::OnceCell, tokio::runtime::Runtime};

#[cfg(not(target_arch = "wasm32"))]
fn get_runtime() -> Result<&'static Runtime, Error> {
    static RUNTIME: OnceCell<Runtime> = OnceCell::new();

    RUNTIME.get_or_try_init(|| Runtime::new().map_err(Error::from))
}

cfg_if::cfg_if! {
    if #[cfg(not(target_arch = "wasm32"))] {
        pub fn spawn(fut: impl Future<Output = ()> + Send + 'static) -> Result<(), Error> {
            let runtime = get_runtime()?;

            runtime.spawn(fut);

            Ok(())
        }
    } else {
        pub fn spawn(fut: impl Future<Output = ()> + 'static) -> Result<(), Error> {
            wasm_bindgen_futures::spawn_local(fut);

            Ok(())
        }
    }
}

pub fn block_on<T>(fut: impl Future<Output = T>) -> Result<T, Error> {
    cfg_if::cfg_if! {
        if #[cfg(not(target_arch = "wasm32"))] {
            let runtime = get_runtime()?;

            Ok(runtime.block_on(fut))
        } else {
            use futures::{future::FutureExt, sink::SinkExt, executor::LocalPool};

            let mut pool = LocalPool::new();

            Ok(pool.run_until(fut))
        }
    }
}
