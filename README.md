# rocket-versioning
api versioning for rocket web applications

# INSTALL

## Example

```rust
#[macro_use] extern crate rocket;

use rocket::local::blocking::Client;
use rocket::http::{Header, Status};

use super::Versioning;

#[get("/versioning", rank = 4)]
fn versioning(_v: Versioning<1, 0>) -> String {
    "v1.0".to_string()
}

#[get("/versioning", rank = 3)]
fn versioning_1_1(_v: Versioning<1, 1>) -> String {
    "v1.1".to_string()
}

#[get("/versioning", rank = 2)]
fn versioning_2_1(_v: Versioning<2, 1>) -> String {
    "v2.1".to_string()
}

#[launch]
fn rocket() -> _ {
    rocket::build().mount("/", routes![versioning, versioning_1_1, versioning_2_1])
}

#[test]
fn test_versioning() {
    let client = Client::tracked(rocket()).expect("invalid rocket instance");
    // NOTICE: version should be `Major.Minor.Patch`, and patch will be ignored.
    // see: https://docs.rs/semver/1.0.3/semver/struct.Version.html#errors
    let response = client.get("/versioning").header(Header::new("Api-Version", "1.0.0")).dispatch();
    assert_eq!(response.status(), Status::Ok);
    assert_eq!(response.into_string().unwrap(), "v1.0");

    let response = client.get("/versioning").header(Header::new("Api-Version", "1.1.0")).dispatch();
    assert_eq!(response.status(), Status::Ok);
    assert_eq!(response.into_string().unwrap(), "v1.1");

    let response = client.get("/versioning").header(Header::new("Api-Version", "2.1.0")).dispatch();
    assert_eq!(response.status(), Status::Ok);
    assert_eq!(response.into_string().unwrap(), "v2.1");

    let response = client.get("/versioning").header(Header::new("Api-Version", "2.0.0")).dispatch();
    assert_eq!(response.status(), Status::NotFound);
}
```

## NOTICE
version in header should be `Major.Minor.Patch`, and patch will be ignored.
see (unsopported)[https://docs.rs/semver/1.0.3/semver/struct.Version.html#errors] version formats.
