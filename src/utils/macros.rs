#[macro_export(local_inner_macros)]
macro_rules! make_response {
    ($status:expr, $message:expr, $data:expr) => {
        crate::utils::tools::make_res($status, $message, $data)
    };

    ($status:expr, $message:expr) => {
        crate::utils::tools::make_res($status, $message, ())
    };

    ($data:expr) => {
        crate::utils::tools::make_res(crate::utils::tools::Status::SUCCESS, "", $data)
    };

    ($message:expr) => {
        crate::utils::tools::make_res(crate::utils::tools::Status::SUCCESS, $message, ())
    };
}
