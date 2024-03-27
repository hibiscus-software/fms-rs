// Copyright (C) 2024 Hibiscus Software. All rights reserved. This
// work is licensed under the terms of the MIT license which can be
// found in the root directory of this project.

#[derive(Debug)]
pub enum Alliance {
    Blue,
    Red,
}

#[derive(Debug)]
pub struct AllianceStation {
    pub alliance: Alliance,
    pub station: u8,
}

impl AllianceStation {
    pub fn new(alliance: Alliance, station: u8) -> Self {
        Self { alliance, station }
    }

    pub fn to_ds_number(&self) -> u8 {
        let station = ((self.station - 1) % 3).try_into().unwrap();

        match self.alliance {
            Alliance::Blue => station + 3,
            Alliance::Red => station,
        }
    }
}
