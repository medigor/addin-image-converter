use std::{error::Error, io::Cursor};

use addin1c::{cstr1c, AddinResult, CStr1C, MethodInfo, Methods, PropInfo, SimpleAddin, Variant};
use image::ImageFormat;

pub struct Addin {
    last_error: Option<Box<dyn Error>>,
}

impl Addin {
    pub fn new() -> Self {
        Self { last_error: None }
    }

    fn last_error(&mut self, value: &mut Variant) -> AddinResult {
        match &self.last_error {
            Some(err) => value
                .set_str1c(err.to_string().as_str())
                .map_err(|e| e.into()),
            None => value.set_str1c("").map_err(|e| e.into()),
        }
    }

    fn convert_file(
        &mut self,
        source: &mut Variant,
        destination: &mut Variant,
        _ret_value: &mut Variant,
    ) -> AddinResult {
        let source = source.get_string()?;
        let destination = destination.get_string()?;

        let image = image::open(&source)?;
        image.save(&destination)?;
        Ok(())
    }

    fn convert_buffer(
        &mut self,
        source: &mut Variant,
        source_format: &mut Variant,
        destination_format: &mut Variant,
        ret_value: &mut Variant,
    ) -> AddinResult {
        let source = source.get_blob()?;
        let source_format = ImageFormat::from_extension(source_format.get_string()?)
            .ok_or("Unknown source format")?;
        let destination_format = ImageFormat::from_extension(destination_format.get_string()?)
            .ok_or("Unknown destination format")?;

        let image = image::load_from_memory_with_format(source, source_format)?;

        let mut buf = Vec::<u8>::new();
        let mut cursor = Cursor::new(&mut buf);
        image.write_to(&mut cursor, destination_format)?;

        ret_value.set_blob(&buf)?;

        Ok(())
    }
}

impl SimpleAddin for Addin {
    fn name() -> &'static CStr1C {
        cstr1c!("ImageConverter")
    }

    fn save_error(&mut self, err: Option<Box<dyn Error>>) {
        self.last_error = err;
    }

    fn methods() -> &'static [addin1c::MethodInfo<Self>] {
        &[
            MethodInfo {
                name: cstr1c!("ConvertFile"),
                method: Methods::Method2(Self::convert_file),
            },
            MethodInfo {
                name: cstr1c!("ConvertBuffer"),
                method: Methods::Method3(Self::convert_buffer),
            },
        ]
    }

    fn properties() -> &'static [PropInfo<Self>] {
        &[PropInfo {
            name: cstr1c!("LastError"),
            getter: Some(Self::last_error),
            setter: None,
        }]
    }
}
