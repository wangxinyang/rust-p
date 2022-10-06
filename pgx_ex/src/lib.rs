mod sorted_id;

use pgx::*;
pub use sorted_id::*;

pg_module_magic!();

#[pg_extern]
fn hello_pgx_ex() -> &'static str {
    "Hello, pgx_ex"
}

#[pg_extern]
fn my_generate_seris(start: i32, end: i32, step: i32) -> impl Iterator<Item = i32> {
    (start..end).step_by(step as usize)
}

#[pg_extern]
fn to_lowercase(str: &str) -> String {
    str.to_lowercase()
}

#[cfg(any(test, feature = "pg_test"))]
#[pg_schema]
mod tests {
    use pgx::*;

    #[pg_test]
    fn test_hello_pgx_ex() {
        assert_eq!("Hello, pgx_ex", crate::hello_pgx_ex());
    }
}

#[cfg(test)]
pub mod pg_test {
    pub fn setup(_options: Vec<&str>) {
        // perform one-off initialization when the pg_test framework starts
    }

    pub fn postgresql_conf_options() -> Vec<&'static str> {
        // return any postgresql.conf settings that are required for your tests
        vec![]
    }
}
