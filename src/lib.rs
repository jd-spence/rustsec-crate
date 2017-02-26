//! rustsec: Client library for the `RustSec` security advisory database

#![crate_name = "rustsec"]
#![crate_type = "lib"]

#![deny(missing_docs, missing_debug_implementations, missing_copy_implementations)]
#![deny(trivial_casts, trivial_numeric_casts)]
#![deny(unsafe_code, unstable_features, unused_import_braces, unused_qualifications)]

extern crate reqwest;
extern crate semver;
extern crate toml;

mod advisory;
mod error;

use advisory::Advisory;
use error::{Error, Result};
use std::collections::HashMap;
use std::collections::hash_map::Entry::{Occupied, Vacant};
use std::io::Read;
use std::str;

/// URL where the TOML file containing the advisory database is located
pub const ADVISORY_DB_URL: &'static str = "https://raw.githubusercontent.\
                                           com/RustSec/advisory-db/master/Advisories.toml";

/// A collection of security advisories, indexed both by ID and crate
#[derive(Debug)]
pub struct AdvisoryDatabase {
    advisories: HashMap<String, Advisory>,
    crates: HashMap<String, Vec<String>>,
}

impl AdvisoryDatabase {
    /// Fetch the advisory database from the server where it is stored
    pub fn fetch() -> Result<Self> {
        let mut response = try!(reqwest::get(ADVISORY_DB_URL).map_err(|_| Error::Request));

        if !response.status().is_success() {
            return Err(Error::Response);
        }

        let mut body = Vec::new();
        try!(response.read_to_end(&mut body).map_err(|_| Error::Response));
        let response_str = try!(str::from_utf8(&body).map_err(|_| Error::Parse));

        AdvisoryDatabase::from_toml(response_str)
    }

    /// Parse the advisory database from a TOML serialization of it
    pub fn from_toml(data: &str) -> Result<Self> {
        let db_toml = try!(data.parse::<toml::Value>().map_err(|_| Error::Parse));

        let advisories_toml = match db_toml["advisory"] {
            toml::Value::Array(ref arr) => arr,
            _ => return Err(Error::MissingAttribute),
        };

        let mut advisories = HashMap::new();
        let mut crates = HashMap::<String, Vec<String>>::new();

        for advisory_toml in advisories_toml.iter() {
            let advisory = try!(Advisory::from_toml_value(advisory_toml));

            let mut crate_vec = match crates.entry(advisory.package.clone()) {
                Vacant(entry) => entry.insert(Vec::new()),
                Occupied(entry) => entry.into_mut(),
            };

            crate_vec.push(advisory.id.clone());
            advisories.insert(advisory.id.clone(), advisory);
        }

        Ok(AdvisoryDatabase {
            advisories: advisories,
            crates: crates,
        })
    }

    /// Look up an advisory by an advisory ID (e.g. "RUSTSEC-YYYY-XXXX")
    pub fn find(&self, id: &str) -> Option<&Advisory> {
        self.advisories.get(id)
    }

    /// Look up advisories relevant to a particular crate
    pub fn find_by_crate(&self, crate_name: &str) -> Vec<&Advisory> {
        let ids = self.crates.get(crate_name);
        let mut result = Vec::new();

        if ids.is_some() {
            for id in ids.unwrap() {
                result.push(self.find(id).unwrap())
            }
        }

        result
    }
}

#[cfg(test)]
mod tests {
    use AdvisoryDatabase;
    use semver::VersionReq;

    #[test]
    fn fetch() {
        let db = AdvisoryDatabase::fetch().unwrap();
        let ref example_advisory = db.find("RUSTSEC-2017-0001").unwrap();

        assert_eq!(example_advisory.id, "RUSTSEC-2017-0001");
        assert_eq!(example_advisory.package, "sodiumoxide");
        assert_eq!(example_advisory.patched_versions[0],
                   VersionReq::parse(">= 0.0.14").unwrap());
        assert_eq!(example_advisory.date, Some(String::from("2017-01-26")));
        assert_eq!(example_advisory.url,
                   Some(String::from("https://github.com/dnaq/sodiumoxide/issues/154")));
        assert_eq!(example_advisory.title,
                   "scalarmult() vulnerable to degenerate public keys");
        assert_eq!(&example_advisory.description[0..30],
                   "The `scalarmult()` function in");

        let ref crate_advisories = db.find_by_crate("sodiumoxide");
        assert_eq!(*example_advisory, crate_advisories[0])
    }
}