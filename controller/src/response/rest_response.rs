use serde::Serialize;
use util::new;

#[derive(Debug, Serialize, new)]
pub struct RestResponse<T>
where
    T: Serialize,
{
    response: T,
}
