use crate::repository::Artifact;
use serde::Deserialize;

#[derive(Deserialize)]
#[serde(rename = "project")]
struct Project {
    pub dependencies: Dependencies,
}

#[derive(Deserialize)]
#[serde(rename = "dependencies")]
struct Dependencies {
    #[serde(rename = "dependency")]
    pub artifacts: Vec<Artifact>,
}

/// Parses the given contents of the pom.xml file, and returns a `Vec<Artifact>` containing all of
/// the dependencies if there were no errors while parsing.
pub fn dependencies_in_pom<T: AsRef<str>>(
    pom_contents: T,
) -> Result<Vec<Artifact>, serde_xml_rs::Error> {
    Ok(
        serde_xml_rs::from_str::<Project>(trim_xml_file(pom_contents.as_ref()))?
            .dependencies
            .artifacts,
    )
}

/// Removes the first line of xml (*the XML declaration*), making it
/// parseable for `serde_xml_rs`. If the line doesn't start with '<?xml...',
/// this first line won't be trimmed, and the original contents passed will be returned.
fn trim_xml_file(pom_contents: &str) -> &str {
    let pom_contents_t = pom_contents.trim_start();
    if pom_contents_t.starts_with("<?xml") {
        pom_contents_t.split_once("\n").unwrap_or_default().1
    } else {
        pom_contents
    }
}
