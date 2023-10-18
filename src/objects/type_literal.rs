use crate::objects::blob::BlobObject;
use crate::objects::tree::TreeObject;
use crate::objects::Object;
use clap::ValueEnum;
use std::fmt::{Display, Formatter};

#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, ValueEnum)]
pub enum ObjectTypeLiteral {
    Blob,
    Tree,
}

impl Display for ObjectTypeLiteral {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            ObjectTypeLiteral::Blob => write!(f, "blob"),
            ObjectTypeLiteral::Tree => write!(f, "tree"),
        }
    }
}

macro_rules! impl_to_object_type_literal {
    ($object_type: ty, $literal: expr) => {
        impl From<$object_type> for ObjectTypeLiteral {
            fn from(_: $object_type) -> Self {
                $literal
            }
        }

        impl<'a> From<&'a $object_type> for ObjectTypeLiteral {
            fn from(_: &'a $object_type) -> Self {
                $literal
            }
        }
    };
}

impl_to_object_type_literal!(BlobObject, ObjectTypeLiteral::Blob);
impl_to_object_type_literal!(TreeObject, ObjectTypeLiteral::Tree);

impl From<Object> for ObjectTypeLiteral {
    fn from(value: Object) -> Self {
        match value {
            Object::BlobObject(obj) => Self::from(obj),
            Object::TreeObject(obj) => Self::from(obj),
        }
    }
}

impl<'a> TryFrom<&'a str> for ObjectTypeLiteral {
    type Error = ();

    fn try_from(value: &'a str) -> Result<Self, Self::Error> {
        match value {
            "blob" => Ok(Self::Blob),
            "tree" => Ok(Self::Tree),
            _ => Err(()),
        }
    }
}
