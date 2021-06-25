// c:showLegendKey
use writer::driver::*;
use reader::driver::*;
use quick_xml::Reader;
use quick_xml::events::{BytesStart};
use quick_xml::Writer;
use std::io::Cursor;

#[derive(Default, Debug)]
pub struct ShowLegendKey {
    val: String,
}
impl ShowLegendKey {
    pub fn get_val(&self)-> &str {
        &self.val
    }

    pub fn set_val<S: Into<String>>(&mut self, value:S)-> &mut ShowLegendKey {
        self.val = value.into();
        self
    }

    pub(crate) fn set_attributes(
        &mut self,
        _reader:&mut Reader<std::io::BufReader<std::fs::File>>,
        e:&BytesStart
    ) {
        self.val = get_attribute(e, b"val").unwrap();
    }

    pub(crate) fn write_to(&self, writer: &mut Writer<Cursor<Vec<u8>>>) {
        // c:showLegendKey
        write_start_tag(writer, "c:showLegendKey", vec![
            ("val", &self.val),
        ], true);
    }
}
