
#[macro_export]
macro_rules! try_get_field{
    ($request:expr, $field:expr, $type:tt) => {
        match $request.data.get(stringify!($field)).and_then(|v| v.$type()) {
            Some(v) => v,
            None => return Err(::shared::ListenerError::FieldNotFound(String::from(stringify!($field)))),
        };

    }
}

