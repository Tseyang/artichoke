use alloc::borrow::Cow;
use alloc::string::String;
use alloc::vec::Vec;

use super::Utf8String;

impl Extend<u8> for Utf8String {
    #[inline]
    fn extend<I: IntoIterator<Item = u8>>(&mut self, iter: I) {
        self.inner.extend(iter.into_iter());
    }
}

impl<'a> Extend<&'a u8> for Utf8String {
    #[inline]
    fn extend<I: IntoIterator<Item = &'a u8>>(&mut self, iter: I) {
        self.inner.extend(iter.into_iter().copied());
    }
}

impl<'a> Extend<&'a mut u8> for Utf8String {
    #[inline]
    fn extend<I: IntoIterator<Item = &'a mut u8>>(&mut self, iter: I) {
        self.inner.extend(iter.into_iter().map(|byte| *byte));
    }
}

impl From<Vec<u8>> for Utf8String {
    #[inline]
    fn from(content: Vec<u8>) -> Self {
        Self::new(content)
    }
}

impl<'a> From<&'a [u8]> for Utf8String {
    #[inline]
    fn from(content: &'a [u8]) -> Self {
        Self::new(content.to_vec())
    }
}

impl<'a> From<&'a mut [u8]> for Utf8String {
    #[inline]
    fn from(content: &'a mut [u8]) -> Self {
        Self::new(content.to_vec())
    }
}

impl<'a> From<Cow<'a, [u8]>> for Utf8String {
    #[inline]
    fn from(content: Cow<'a, [u8]>) -> Self {
        Self::new(content.into_owned())
    }
}

impl From<String> for Utf8String {
    #[inline]
    fn from(s: String) -> Self {
        Self::new(s.into_bytes())
    }
}

impl From<&str> for Utf8String {
    #[inline]
    fn from(s: &str) -> Self {
        Self::new(s.as_bytes().to_vec())
    }
}

impl From<Utf8String> for Vec<u8> {
    #[inline]
    fn from(s: Utf8String) -> Self {
        s.into_vec()
    }
}

impl AsRef<[u8]> for Utf8String {
    #[inline]
    fn as_ref(&self) -> &[u8] {
        self.inner.as_slice()
    }
}

impl AsMut<[u8]> for Utf8String {
    #[inline]
    fn as_mut(&mut self) -> &mut [u8] {
        self.inner.as_mut_slice()
    }
}
