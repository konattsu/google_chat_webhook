#[allow(unused)]
pub trait DetailResponseError {
    fn detail_resp_err(self, ok_prefix: &str, err_prefix: &str) -> String;
}

impl<T, E> DetailResponseError for Result<T, E>
where
    T: ToString,
    E: ToString,
{
    fn detail_resp_err(self, ok_prefix: &str, err_prefix: &str) -> String {
        match self {
            Ok(v) => format!("{}: {}", ok_prefix, v.to_string()),
            Err(e) => format!("{}: {}", err_prefix, e.to_string()),
        }
    }
}
