mod build_xml;
mod content_types;
mod doc_props;
mod document;
mod elements;
mod rels;
mod styles;

pub(crate) use crate::xml_builder::*;
pub(crate) use build_xml::*;

pub use content_types::*;
pub use doc_props::*;
pub use document::*;
pub use elements::*;
pub use rels::*;
pub use styles::*;

pub struct Docx {
    content_type: ContentTypes,
    rels: Rels,
    doc_props: DocProps,
    styles: Styles,
    document: Document,
}

impl Default for Docx {
    fn default() -> Self {
        let content_type = ContentTypes::new();
        let rels = Rels::new();
        let doc_props = DocProps::new(None, None /* TODO: */);
        let styles = Styles::new();
        let document = Document::new();
        Docx {
            content_type,
            rels,
            doc_props,
            styles,
            document,
        }
    }
}

pub struct XMLDocx {
    pub content_type: Vec<u8>,
    pub rels: Vec<u8>,
    pub doc_props: XMLDocProps,
    pub styles: Vec<u8>,
    pub document: Vec<u8>,
}

impl Docx {
    pub fn new() -> Docx {
        Default::default()
    }

    pub fn add_paragraph(mut self, p: Paragraph) -> Docx {
        self.document = self.document.add_paragraph(p);
        self
    }

    pub(crate) fn build(&self) -> XMLDocx {
        XMLDocx {
            content_type: self.content_type.build(),
            rels: self.rels.build(),
            doc_props: self.doc_props.build(),
            styles: self.styles.build(),
            document: self.document.build(),
        }
    }
}