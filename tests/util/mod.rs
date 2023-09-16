tapioca::infer_api!(httpbin, "https://raw.githubusercontent.com/OJFord/tapioca/master/tests/schemata/httpbin.yml");

#[macro_export]
macro_rules! infer_test_api {
    (httpbin) => {
        use tapioca_testutil::Response;
        use tapioca_testutil::httpbin;
    }
}
