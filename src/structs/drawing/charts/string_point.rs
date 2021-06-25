// c:pt
use super::NumericValue;
use writer::driver::*;
use reader::driver::*;
use quick_xml::Reader;
use quick_xml::events::{Event, BytesStart};
use quick_xml::Writer;
use std::io::Cursor;

#[derive(Default, Debug)]
pub struct StringPoint {
    index: String,
    numeric_value: NumericValue,
}
impl StringPoint {
    pub fn get_index(&self)-> &str {
        &self.index
    }

    pub fn set_index<S: Into<String>>(&mut self, value:S)-> &mut StringPoint {
        self.index = value.into();
        self
    }

    pub fn get_numeric_value(&self)-> &NumericValue {
        &self.numeric_value
    }

    pub fn get_numeric_value_mut(&mut self)-> &mut NumericValue {
        &mut self.numeric_value
    }

    pub fn set_numeric_value(&mut self, value:NumericValue)-> &mut StringPoint {
        self.numeric_value = value;
        self
    }

    pub(crate) fn set_attributes(
        &mut self,
        reader:&mut Reader<std::io::BufReader<std::fs::File>>,
        e:&BytesStart
    ) {
        &mut self.set_index(get_attribute(e, b"idx").unwrap());

        let mut buf = Vec::new();
        loop {
            match reader.read_event(&mut buf) {
                Ok(Event::Start(ref e)) => {
                    match e.name() {
                        b"c:v" => {
                            &mut self.numeric_value.set_attributes(reader, e);
                        },
                        _ => (),
                    }
                },
                Ok(Event::End(ref e)) => {
                    match e.name() {
                        b"c:pt" => return,
                        _ => (),
                    }
                },
                Ok(Event::Eof) => panic!("Error not find {} end element", "c:pt"),
                Err(e) => panic!("Error at position {}: {:?}", reader.buffer_position(), e),
                _ => (),
            }
            buf.clear();
        }
    }

    pub(crate) fn write_to(&self, writer: &mut Writer<Cursor<Vec<u8>>>) {
        // c:pt
        write_start_tag(writer, "c:pt", vec![
            ("idx", &self.index),
        ], false);

        // c:v
        &self.numeric_value.write_to(writer);

        write_end_tag(writer, "c:pt");
    }
}
