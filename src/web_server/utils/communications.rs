use serde::{Serialize, Deserialize};


pub trait WSBuffer {
     fn to_ws_buffers(&self, bufsize: usize) -> Vec<String>;
 }

impl WSBuffer for String {
    fn to_ws_buffers(&self, bufsize: usize) -> Vec<String> {
        let mut buffer = vec![char::default(); bufsize];
        let mut res = vec![];

        if self.len() < bufsize {
            for (i, item) in self.chars().enumerate() {
                buffer[i] = item;
            }
            res.push(buffer.iter().collect::<String>().clone());
        } else {
            for i in 0..self.len().div_ceil(bufsize) {
                let start = bufsize * i;
                let char_iter = self.chars().skip(start).take(bufsize);
                for (i, item) in char_iter.enumerate() {
                    buffer[i] = item;
                }
                res.push(buffer.iter().collect::<String>().clone());
                buffer.fill(char::default());
            }
        }

        return res;
    }
}

//  "status": Pass | Fail {ex: String, got: String} | URCodeErrorLOL | URCodeDontReturnAnything

#[derive(Serialize, Deserialize, Debug)]
pub enum Status {
    Pass,
    Fail(Fail),
    Cooked,
    URCodeErrorLOL,
    URCodeDontReturn,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Fail {
    ex: String,
    got: String,
}
