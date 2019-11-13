use super::{ParagraphProperty, Run};
use crate::documents::BuildXML;
use crate::types::*;
use crate::xml_builder::*;

#[derive(Debug, Clone)]
pub struct Paragraph {
    runs: Vec<Run>,
    property: ParagraphProperty,
    attrs: Vec<(String, String)>,
}

impl Default for Paragraph {
    fn default() -> Self {
        Self {
            runs: Vec::new(),
            property: ParagraphProperty::new(),
            attrs: Vec::new(),
        }
    }
}

impl Paragraph {
    pub fn new() -> Paragraph {
        Default::default()
    }

    pub fn add_run(mut self, run: Run) -> Paragraph {
        self.runs.push(run);
        self
    }

    pub fn add_attr(mut self, key: impl Into<String>, val: impl Into<String>) -> Paragraph {
        self.attrs.push((key.into(), val.into()));
        self
    }

    pub fn align(mut self, alignment_type: AlignmentType) -> Paragraph {
        self.property = self.property.align(alignment_type);
        self
    }

    pub fn size(mut self, size: usize) -> Paragraph {
        self.runs = self.runs.into_iter().map(|r| r.size(size)).collect();
        self
    }

    pub fn style(mut self, style_id: &str) -> Paragraph {
        self.property = self.property.style(style_id);
        self
    }

    pub fn indent(mut self, left: usize, special_indent: Option<SpecialIndentType>) -> Paragraph {
        self.property = self.property.indent(left, special_indent);
        self
    }
}

impl BuildXML for Paragraph {
    fn build(&self) -> Vec<u8> {
        XMLBuilder::new()
            .open_paragraph(&self.attrs)
            .add_child(&self.property)
            .add_children(&self.runs)
            .close()
            .build()
    }
}

#[cfg(test)]
mod tests {

    use super::*;
    #[cfg(test)]
    use pretty_assertions::assert_eq;
    use std::str;

    #[test]
    fn test_paragraph() {
        let b = Paragraph::new()
            .add_run(Run::new().add_text("Hello"))
            .build();
        assert_eq!(
            str::from_utf8(&b).unwrap(),
            r#"<w:p><w:pPr><w:pStyle w:val="Normal" /><w:rPr /></w:pPr><w:r><w:rPr /><w:t xml:space="preserve">Hello</w:t></w:r></w:p>"#
        );
    }

    #[test]
    fn test_paragraph_size() {
        let b = Paragraph::new()
            .add_run(Run::new().add_text("Hello"))
            .size(60)
            .build();
        assert_eq!(
            str::from_utf8(&b).unwrap(),
            r#"<w:p><w:pPr><w:pStyle w:val="Normal" /><w:rPr /></w:pPr><w:r><w:rPr><w:sz w:val="60" /><w:szCs w:val="60" /></w:rPr><w:t xml:space="preserve">Hello</w:t></w:r></w:p>"#
        );
    }

    #[test]
    fn test_custom_attr() {
        let b = Paragraph::new()
            .add_run(Run::new().add_text("Hello"))
            .add_attr("customId", "abcd-1234-567890")
            .build();
        assert_eq!(
            str::from_utf8(&b).unwrap(),
            r#"<w:p customId="abcd-1234-567890"><w:pPr><w:pStyle w:val="Normal" /><w:rPr /></w:pPr><w:r><w:rPr /><w:t xml:space="preserve">Hello</w:t></w:r></w:p>"#
        );
    }
}
