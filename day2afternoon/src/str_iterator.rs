pub fn prefix_matches(prefix: &str, request_path: &str) -> bool {
    let prefix_setup_splitted: Vec<&str> = prefix.split("/").collect();
    let req_path_splitted: Vec<&str> = request_path.split("/").collect();

    let prefix_len = prefix_setup_splitted.len();
    let req_len = req_path_splitted.len();
    if prefix_len > req_len {
        return false;
    }

    for i in 0..prefix_len {
        let prefix_str = match prefix_setup_splitted.get(i) {
            Some(&str) => str,
            None => return false,
        };
        if prefix_str.eq("*") {
            continue;
        }
        let req_str = match req_path_splitted.get(i) {
            Some(&str) => str,
            None => return false,
        };
        if !req_str.eq(prefix_str) {
            return false;
        }
    }

    return true;
}

#[test]
fn test_matches_without_wildcard() {
    assert!(prefix_matches("/v1/publishers", "/v1/publishers"));
    assert!(prefix_matches("/v1/publishers", "/v1/publishers/abc-123"));
    assert!(prefix_matches("/v1/publishers", "/v1/publishers/abc/books"));

    assert!(!prefix_matches("/v1/publishers", "/v1"));
    assert!(!prefix_matches("/v1/publishers", "/v1/publishersBooks"));
    assert!(!prefix_matches("/v1/publishers", "/v1/parent/publishers"));
}

#[test]
fn test_matches_with_wildcard() {
    assert!(prefix_matches(
        "/v1/publishers/*/books",
        "/v1/publishers/foo/books"
    ));
    assert!(prefix_matches(
        "/v1/publishers/*/books",
        "/v1/publishers/bar/books"
    ));
    assert!(prefix_matches(
        "/v1/publishers/*/books",
        "/v1/publishers/foo/books/book1"
    ));

    assert!(!prefix_matches("/v1/publishers/*/books", "/v1/publishers"));
    assert!(!prefix_matches(
        "/v1/publishers/*/books",
        "/v1/publishers/foo/booksByAuthor"
    ));
}
