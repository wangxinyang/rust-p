use anyhow::Result;

trait Encoder {
    fn encode(&self) -> Result<Vec<u8>>;
}

struct _Event<Id, Data> {
    id: Id,
    data: Data,
}

impl<Id: Encoder, Data: Encoder> _Event<Id, Data> {
    fn _new(id: Id, data: Data) -> Self {
        Self { id, data }
    }

    fn _test(&self) -> Result<Vec<u8>> {
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
