use std::fmt::Debug;
use std::slice::SliceIndex;

use chrono::{DateTime, NaiveDateTime, Utc};

pub struct Access {
    pub contents: Vec<u8>,
    pub commas: Vec<usize>,
}

#[derive(Debug)]
pub enum Location {
    Coords(u32, u32),
    Range(u32, u32, u32, u32),
}

impl Access {
    pub fn new(contents: Vec<u8>) -> Access {
        let commas: Vec<_> = contents
            .iter()
            .enumerate()
            .filter_map(|(i, &chr)| if chr == b',' { Some(i) } else { None })
            .collect();
        assert!([4, 6].contains(&commas.len()));
        Access { contents, commas }
    }

    pub fn get(&self, range: impl SliceIndex<[u8], Output = [u8]>) -> &str {
        let result = &self.contents[range];
        unsafe { std::str::from_utf8_unchecked(result) }
    }

    pub fn time_raw(&self) -> &str {
        self.get(..self.commas[0])
    }

    pub fn time(&self) -> DateTime<Utc> {
        let naive =
            NaiveDateTime::parse_from_str(self.time_raw(), "%Y-%m-%d %H:%M:%S%.3f UTC").unwrap();
        DateTime::<Utc>::from_utc(naive, Utc)
    }

    pub fn user_id(&self) -> &str {
        self.get(self.commas[0] + 1..self.commas[1])
    }

    pub fn colour(&self) -> &str {
        self.get(self.commas[1] + 1..self.commas[2])
    }

    pub fn coords_raw(&self) -> &str {
        self.get(self.commas[2] + 2..self.contents.len() - 1)
    }

    pub fn coords(&self) -> Location {
        let first_number = self
            .get(self.commas[2] + 2..self.commas[3])
            .parse()
            .unwrap();
        let last_number = self
            .get(self.commas.last().unwrap() + 1..self.contents.len() - 1)
            .parse()
            .unwrap();
        if self.commas.len() == 4 {
            Location::Coords(first_number, last_number)
        } else {
            Location::Range(
                first_number,
                self.get(self.commas[3] + 1..self.commas[4])
                    .parse()
                    .unwrap(),
                self.get(self.commas[4] + 1..self.commas[5])
                    .parse()
                    .unwrap(),
                last_number,
            )
        }
    }
}

impl Debug for Access {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Access")
            .field("time", &self.time_raw())
            .field("user_id", &self.user_id())
            .field("colour", &self.colour())
            .field("coords", &self.coords())
            .finish()
    }
}
