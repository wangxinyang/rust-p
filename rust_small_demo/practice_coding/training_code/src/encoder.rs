use anyhow::Result;

trait Encoder {
    fn encode(&self) -> Result<Vec<u8>>;
}

#[allow(dead_code)]
struct Event<Id, Data> {
    id: Id,
    data: Data,
}

#[allow(dead_code)]
impl<Id: Encoder, Data: Encoder> Event<Id, Data> {
    fn new(id: Id, data: Data) -> Self {
        Self { id, data }
    }

    fn test(&self) -> Result<Vec<u8>> {
        let mut result = self.id.encode()?;
        result.append(&mut self.data.encode()?);
        Ok(result)
    }
}

impl Encoder for u8 {
    fn encode(&self) -> Result<Vec<u8>> {
        Ok(vec![1, 2, 3, 4])
    }
}

impl Encoder for String {
    fn encode(&self) -> Result<Vec<u8>> {
        Ok(Vec::from(self.as_bytes()))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn encoder_test() {
        let event = Event::new(1, "hello world".to_string());
        let res = event.test();
        println!("{:?}", res.unwrap())
    }
}
