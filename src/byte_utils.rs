use std::convert::TryInto;
use std::str::from_utf8;
use crate::common::R;

pub trait VecUtils {
    fn get_reversed_string(&self, from: usize, to: usize) -> R<String>;
    fn get_string(&self, from: usize, to: usize) -> R<String>;
    fn get_string_null_terminated(&self, offset: usize) -> R<String>;
    fn get_u16(&self, offset: usize) -> R<u16>;
    fn get_u32(&self, offset: usize) -> R<u32>;
    fn get_f32(&self, offset: usize) -> R<f32>;
    fn get_two_bytes(&self, offset: usize) -> R<[u8; 2]>;
    fn get_four_bytes(&self, offset: usize) -> R<[u8; 4]>;
    fn get_null_terminated_strings(&self) -> R<Vec<String>>;
}

impl VecUtils for Vec<u8> {
    fn get_reversed_string(&self, from: usize, to: usize) -> R<String> {
        let bytes = &self[from..from + to];
        let parsed: String = std::str::from_utf8(bytes)?
            .chars()
            .rev()
            .collect::<String>();
        Ok(parsed)
    }

    fn get_string(&self, from: usize, to: usize) -> R<String> {
        let bytes = &self[from..from + to];
        let parsed: String = std::str::from_utf8(bytes)?
            .chars()
            .collect::<String>();
        Ok(parsed)
    }

    fn get_string_null_terminated(&self, offset: usize) -> R<String> {
        let og: Vec<u8> = self.iter()
            .skip(offset)
            .take_while(|item| **item != 0)
            .map(|v| *v)
            .collect();
        Ok(String::from_utf8(og)?)
    }

    fn get_u16(&self, offset: usize) -> R<u16> {
        let slice: [u8; 2] = self.get_two_bytes(offset)?;
        Ok(u16::from_le_bytes(slice))
    }

    fn get_u32(&self, offset: usize) -> R<u32> {
        let slice: [u8; 4] = self.get_four_bytes(offset)?;
        Ok(u32::from_le_bytes(slice))
    }

    fn get_f32(&self, offset: usize) -> R<f32> {
        let slice: [u8; 4] = self.get_four_bytes(offset)?;
        Ok(f32::from_le_bytes(slice))
    }

    fn get_two_bytes(&self, offset: usize) -> R<[u8; 2]> {
        let v = &self[offset..offset + 2];
        Ok(v.try_into()?)
    }

    fn get_four_bytes(&self, offset: usize) -> R<[u8; 4]> {
        let v = &self[offset..offset + 4];
        Ok(v.try_into()?)
    }

    fn get_null_terminated_strings(&self) -> R<Vec<String>> {
        let mut acc = Vec::new();
        let mut buff = Vec::with_capacity(64);
        for byte in self {
            if *byte == 0 {
                acc.push(from_utf8(buff.as_slice())?.to_owned());
                buff.clear();
            } else {
                buff.push(*byte);
            }
        }
        Ok(acc)
    }
}