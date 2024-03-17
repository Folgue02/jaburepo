use crate::repository::Artifact;

const SAMPLE_VALID_POM: &'static str = r#"
<?xml version="1.0" encoding="UTF-8"?>
<project xmlns="http://maven.apache.org/POM/4.0.0" xmlns:xsi="http://www.w3.org/2001/XMLSchema-instance" xsi:schemaLocation="http://maven.apache.org/POM/4.0.0 http://maven.apache.org/xsd/maven-4.0.0.xsd">
    <modelVersion>4.0.0</modelVersion>
    <groupId>me.folgue</groupId>
    <artifactId>adt_tar4</artifactId>
    <version>1.0-SNAPSHOT</version>
    <packaging>jar</packaging>
    <properties>
        <project.build.sourceEncoding>UTF-8</project.build.sourceEncoding>
        <maven.compiler.source>17</maven.compiler.source>
        <maven.compiler.target>17</maven.compiler.target>
        <exec.mainClass>me.folgue.adt_tar4.utils.CLI</exec.mainClass>
    </properties>

    <dependencies>
        <dependency>
            <groupId>org.mariadb.jdbc</groupId>
            <artifactId>mariadb-java-client</artifactId>
            <version>3.3.3</version>
        </dependency>
        <dependency>
            <groupId>org.hibernate.orm</groupId>
            <artifactId>hibernate-core</artifactId>
            <version>6.4.4.Final</version>
        </dependency>
        <dependency>
            <groupId>org.junit.jupiter</groupId>
            <artifactId>junit-jupiter</artifactId>
            <version>5.10.0</version>
        </dependency>
    </dependencies>
</project>
"#;

#[test]
fn testing_dependencies_from_pom() {
    let expected = vec![
        Artifact {
            artifact_id: "mariadb-java-client".to_string(),
            group_id: "org.mariadb.jdbc".to_string(),
            version: "3.3.3".to_string(),
        },
        Artifact {
            artifact_id: "hibernate-core".to_string(),
            group_id: "org.hibernate.orm".to_string(),
            version: "6.4.4.Final".to_string(),
        },
        Artifact {
            artifact_id: "junit-jupiter".to_string(),
            group_id: "org.junit.jupiter".to_string(),
            version: "5.10.0".to_string(),
        }
    ];

    assert_eq!(expected, crate::utils::dependencies_in_pom(SAMPLE_VALID_POM).unwrap())
}
