pub struct MyImageData {
    pub width: usize,
    pub height: usize,
    pub bytes: Vec<u8>,
}

pub enum ClipboardContent {
    Text(String),
    Image(MyImageData),
    UnkownOrEmpty,
}