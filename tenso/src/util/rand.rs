pub trait Rand
where
    Self: Sized,
{
    fn get(len: usize) -> Result<Self, getrandom::Error>;
}

impl Rand for Vec<u8> {
    fn get(len: usize) -> Result<Self, getrandom::Error> {
        let mut buf = vec![0u8; len];
        getrandom::getrandom(&mut buf)?;
        Ok(buf)
    }
}

impl Rand for String {
    fn get(len: usize) -> Result<Self, getrandom::Error> {
        let bytes: Vec<u8> = Rand::get(len)?;
        Ok(base64_url::encode(&bytes))
    }
}
