pub use anyhow::bail;
pub use anyhow::Error;
pub use anyhow::Result;
pub use axum::async_trait;
pub use derive_new::new;
pub mod error_handling;

#[derive(new)]
struct Hoge {
    value: i32,
}
