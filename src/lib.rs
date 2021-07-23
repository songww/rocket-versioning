#[cfg(test)]
#[macro_use] extern crate rocket;

use rocket::{http::Status, request::{self, Request, FromRequest}};

#[derive(Debug)]
pub struct Versioning<const MAJOR: u64, const MINOR: u64> {
    major: u64,
    minor: u64,
}

impl<const MAJOR: u64, const MINOR: u64> Versioning<MAJOR, MINOR> {
    pub const fn new() -> Versioning<MAJOR, MINOR> {
        Versioning {
            major: MAJOR,
            minor: MINOR,
        }
    }
}

#[derive(Debug)]
pub enum VersionError {
    SemverError(semver::Error),
    NotExists,
}

#[rocket::async_trait]
impl<'r, const MAJOR: u64, const MINOR: u64> FromRequest<'r> for Versioning<MAJOR, MINOR> {
    type Error = &'r VersionError;

    async fn from_request(req: &'r Request<'_>) -> request::Outcome<Self, Self::Error> {
        let version = req.local_cache(|| {
            let ver = req.headers().get_one("api-version").ok_or(VersionError::NotExists)?;
            semver::Version::parse(ver).map_err(VersionError::SemverError)
        });
        match version {
            Err(err) => request::Outcome::Failure((Status::NotFound, err)),
            Ok(version) => {
                if version.major == MAJOR && version.minor == MINOR {
                    request::Outcome::Success(Self::new())
                } else {
                    request::Outcome::Forward(())
                }
            }
        }
    }
}

#[cfg(test)]
mod tests {
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
}
