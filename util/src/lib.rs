pub use axum::async_trait;
pub use derive_new::new;

#[derive(new)]
struct Hoge {
    value: i32,
}
