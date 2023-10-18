use std::mem;

use clap::ValueEnum;

use crate::crypto;

#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Debug, ValueEnum)]
pub enum GitObjectType {
    Blob,
}

impl GitObjectType {
    pub fn kind_string(&self) -> String {
        match self {
            GitObjectType::Blob => "blob".into(),
        }
    }
}

impl TryFrom<String> for GitObjectType {
    type Error = String;

    fn try_from(value: String) -> Result<Self, Self::Error> {
        match value.as_str() {
            "blob" => Ok(GitObjectType::Blob),
            other => Err(format!("unknown git object type `{}`", other)),
        }
    }
}

pub struct GitObject<'a> {
    contents: &'a [u8],
    obj_type: GitObjectType,
}

impl<'a> GitObject<'a> {
    pub fn new(contents: &'a [u8], obj_type: GitObjectType) -> Self {
        Self { contents, obj_type }
    }

    pub fn obj_type(&self) -> GitObjectType {
        self.obj_type
    }

    pub fn string_of_content(&self) -> String {
        String::from_utf8(self.contents.to_vec()).unwrap()
    }

    /// iter "{type}\x00{contents}" to produce `u8`
    pub fn bytes_iter(&self) -> GitObjBytesIterator {
        GitObjBytesIterator::new(self.obj_type.kind_string().as_bytes().into(), self.contents)
    }

    /// combine type str bytes and content bytes of obj.
    /// the type is just a string that's going to be prepended
    /// to the start of the file, followed by a null byte.
    pub fn collect_bytes(&self) -> Vec<u8> {
        self.bytes_iter().collect()
    }

    /// hexadecimal representation of the result of the SHA-1 hash.
    ///
    /// "OID" - object ID
    pub fn get_oid_by_sha1(&self) -> String {
        let hash_val = crypto::sha1(&self.collect_bytes());
        hex::encode(&hash_val)
    }
}

#[derive(Debug)]
pub enum ParseGitObjectError {
    UnrecognizedObjectFormat,
    UnknownObjectType { msg: String },
}

impl<'a> TryFrom<&'a [u8]> for GitObject<'a> {
    type Error = ParseGitObjectError;

    fn try_from(value: &'a [u8]) -> Result<Self, Self::Error> {
        if let Some(idx) = value.iter().position(|&x| x == 0x00) {
            let (type_str, obj_contents) = value.split_at(idx);

            match GitObjectType::try_from(String::from_utf8(type_str.to_vec()).unwrap()) {
                Ok(obj_type) => Ok(Self::new(obj_contents, obj_type)),
                Err(e) => Err(ParseGitObjectError::UnknownObjectType { msg: e }),
            }
        } else {
            Err(ParseGitObjectError::UnrecognizedObjectFormat)
        }
    }
}

pub struct GitObjBytesIterator<'a> {
    type_vec: Vec<u8>,
    type_iter: Option<std::slice::Iter<'a, u8>>,
    content_iter: std::slice::Iter<'a, u8>,
    visiting_type_iter: bool,
}

impl<'a> GitObjBytesIterator<'a> {
    fn new(type_vec: Vec<u8>, content_vec: &'a [u8]) -> Self {
        let mut iter = GitObjBytesIterator {
            type_vec,
            type_iter: None,
            content_iter: content_vec.iter(),
            visiting_type_iter: true,
        };

        iter.type_iter = Some(unsafe { mem::transmute(iter.type_vec.iter()) });

        iter
    }
}

impl<'a> Iterator for GitObjBytesIterator<'a> {
    type Item = u8;

    fn next(&mut self) -> Option<Self::Item> {
        if self.visiting_type_iter {
            if let Some(ref mut type_iter) = self.type_iter {
                match type_iter.next() {
                    Some(value) => Some(*value),
                    None => {
                        self.visiting_type_iter = false;
                        Some(0x00)
                    }
                }
            } else {
                unreachable!()
            }
        } else {
            self.content_iter.next().copied()
        }
    }
}
