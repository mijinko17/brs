use serde::Serialize;

#[derive(Debug, Serialize)]
pub struct RestResponse<T>
where
    T: Serialize,
{
    response: T,
}
