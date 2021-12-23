use std::fmt::{Debug, Display, Formatter, write};

#[derive(Debug)]
pub enum Error {
    ClippingFailure { message: &'static str },
    MissingVertex { message: &'static str },
    OutsideBoundingBox{message:&'static str}
}

impl Display for Error {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Error::ClippingFailure { message } => {
                write!(f, "Error! Failed to clip cell: {}", message)
            }
            Error::MissingVertex { message } => {
                write!(f, "Error! Missing Vertex: {}", message)
            }
            Error::OutsideBoundingBox {message}=>{
                write!(f,"Error! Outside Bounding Box: {}",message)
            }
        }
    }
}

impl std::error::Error for Error {}