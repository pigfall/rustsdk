type ImageErr = image::ImageError;
pub struct Image{
    raw: image::DynamicImage
}

impl Image{
    pub fn open(filepath: &std::path::Path)->Result<Image,ImageErr>{
        let raw = image::open(filepath)?;
        return Ok(Image{
            raw:raw,
        });
    }
    pub fn width(&self)->u32{
        return  self.raw.width();
    }
    pub fn height(&self)->u32{
        return  self.raw.height();
    }

    pub fn as_bytes(&self)->&[u8]{
        return self.raw.as_bytes();
    }
}
