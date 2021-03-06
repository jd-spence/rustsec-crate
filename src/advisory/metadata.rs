//! Advisory information (i.e. the `[advisory]` section)

use super::{
    category::Category, date::Date, id::Id, informational::Informational, keyword::Keyword,
};
use crate::{collection::Collection, package, version::VersionReq};
use serde::{Deserialize, Serialize};

/// The `[advisory]` section of a RustSec security advisory
#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub struct Metadata {
    /// Security advisory ID (e.g. RUSTSEC-YYYY-NNNN)
    pub id: Id,

    /// Name of affected crate
    pub package: package::Name,

    /// One-liner description of a vulnerability
    #[serde(default)]
    pub title: String,

    /// Extended description of a vulnerability
    #[serde(default)]
    pub description: String,

    /// Date this advisory was officially issued
    pub date: Date,

    /// Advisory IDs in other databases which point to the same advisory
    #[serde(default)]
    pub aliases: Vec<Id>,

    /// Advisory IDs which are related to this advisory (use `aliases` if it
    /// is the same vulnerability syndicated to a different database)
    #[serde(default)]
    pub references: Vec<Id>,

    /// Collection this advisory belongs to. This isn't intended to be
    /// explicitly specified in the advisory, but rather is auto-populated
    /// based on the location
    pub collection: Option<Collection>,

    /// RustSec vulnerability categories: one of a fixed list of vulnerability
    /// categorizations accepted by the project.
    #[serde(default)]
    pub categories: Vec<Category>,

    /// Freeform keywords which succinctly describe this vulnerability (e.g. "ssl", "rce", "xss")
    #[serde(default)]
    pub keywords: Vec<Keyword>,

    /// CVSS v3.1 Base Metrics vector string containing severity information.
    ///
    /// Example:
    ///
    /// ```text
    /// CVSS:3.1/AV:N/AC:L/PR:N/UI:R/S:C/C:L/I:L/A:N
    /// ```
    pub cvss: Option<cvss::v3::Base>,

    /// Informational advisories can be used to warn users about issues
    /// affecting a particular crate without failing the build.
    pub informational: Option<Informational>,

    /// Is the advisory obsolete? Obsolete advisories will be ignored.
    #[serde(default)]
    pub obsolete: bool,

    /// URL with an announcement (e.g. blog post, PR, disclosure issue, CVE)
    pub url: Option<String>,

    /// Versions which are patched and not vulnerable (expressed as semantic version requirements)
    // TODO(tarcieri): phase this out
    #[serde(default)]
    pub(super) patched_versions: Vec<VersionReq>,

    /// Versions which were never affected in the first place
    #[serde(default)]
    pub(super) unaffected_versions: Vec<VersionReq>,
}
