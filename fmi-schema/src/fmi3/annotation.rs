use yaserde_derive::{YaDeserialize, YaSerialize};

/// Container for vendor-specific annotations.
///
/// Per FMI 3.0 spec, the Annotations element can contain multiple Annotation
/// elements, each with vendor-specific nested XML content.
#[derive(Default, PartialEq, Debug, YaSerialize, YaDeserialize)]
#[yaserde(rename = "Annotations")]
pub struct Fmi3Annotations {
    /// List of vendor annotations. Each annotation is identified by its `type`
    /// attribute (e.g., "com.mathworks.Simulink").
    #[yaserde(rename = "Annotation")]
    pub annotations: Vec<Annotation>,
}

/// A single vendor-specific annotation.
///
/// Per FMI 3.0 spec (fmi3Annotation.xsd), each Annotation element:
/// - Has a required `type` attribute identifying the vendor
/// - Can contain arbitrary nested XML content (xs:any with processContents="lax")
///
/// Note: The nested vendor-specific XML content is currently not captured.
/// yaserde will skip unknown child elements during deserialization.
#[derive(Default, PartialEq, Debug, YaSerialize, YaDeserialize)]
pub struct Annotation {
    /// Vendor identifier (e.g., "com.mathworks.Simulink", "org.fmi-standard.fmi-ls-xcp")
    #[yaserde(attribute = true, rename = "type")]
    pub r#type: String,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_empty_annotations() {
        let xml = r#"<Annotations></Annotations>"#;
        let result: Fmi3Annotations = yaserde::de::from_str(xml).unwrap();
        assert!(result.annotations.is_empty());
    }

    #[test]
    fn test_single_annotation_no_content() {
        let xml = r#"<Annotations>
            <Annotation type="com.example.vendor"/>
        </Annotations>"#;
        let result: Fmi3Annotations = yaserde::de::from_str(xml).unwrap();
        assert_eq!(result.annotations.len(), 1);
        assert_eq!(result.annotations[0].r#type, "com.example.vendor");
    }

    #[test]
    fn test_multiple_annotations() {
        let xml = r#"<Annotations>
            <Annotation type="com.vendor1"/>
            <Annotation type="com.vendor2"/>
        </Annotations>"#;
        let result: Fmi3Annotations = yaserde::de::from_str(xml).unwrap();
        assert_eq!(result.annotations.len(), 2);
        assert_eq!(result.annotations[0].r#type, "com.vendor1");
        assert_eq!(result.annotations[1].r#type, "com.vendor2");
    }

    #[test]
    fn test_annotation_with_nested_content() {
        // Test that yaserde can handle (skip) nested vendor-specific XML
        let xml = r#"<Annotations>
            <Annotation type="com.mathworks.Simulink">
                <Simulink>
                    <ImportCompatibility FMUProduct="standalone FMU"/>
                </Simulink>
            </Annotation>
        </Annotations>"#;
        let result: Fmi3Annotations = yaserde::de::from_str(xml).unwrap();
        assert_eq!(result.annotations.len(), 1);
        assert_eq!(result.annotations[0].r#type, "com.mathworks.Simulink");
    }
}
