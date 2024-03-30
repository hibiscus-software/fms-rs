// Copyright (C) 2024 Hibiscus Software. All rights reserved. This
// work is licensed under the terms of the MIT license which can be
// found in the root directory of this project.

#[derive(Clone, Copy, Debug)]
pub enum Mode {
    Teleop = 0,
    Test = 1,
    Auto = 2,
}

#[derive(Clone, Copy, Debug)]
pub struct Control {
    pub estop: bool,
    pub enabled: bool,
    pub mode: Mode,
}

#[derive(Clone, Copy, Debug)]
pub enum Alliance {
    Blue,
    Red,
}

#[derive(Clone, Copy, Debug)]
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

#[derive(Clone, Copy, Debug)]
pub enum TournamentLevel {
    MatchTest = 0,
    Practice = 1,
    Qualification = 2,
    Playoff = 3,
}

#[derive(Clone, Copy, Debug)]
pub struct Date {
    pub microseconds: u32,
    pub second: u8,
    pub minute: u8,
    pub hour: u8,
    pub day: u8,
    pub month: u8,
    pub year: u8,
}

#[derive(Clone, Copy, Debug)]
pub struct UdpStatus {
    pub estop: bool,
    pub robot_comms_active: bool,
    pub radio_ping: bool,
    pub rio_ping: bool,
    pub enabled: bool,
    pub mode: Mode,
}

#[derive(Clone, Copy, Debug)]
pub struct Battery {
    pub voltage: u16,
}

impl Battery {
    pub fn get_voltage(&self) -> f64 {
        let xx = self.voltage / 100;
        let yy = self.voltage % 100;

        return (xx + (yy / 256)) as f64;
    }
}

#[derive(Clone, Copy, Debug)]
pub struct UdpControlPacket {
    pub sequence_number: u8, // Should be u16, look at big-endian libray
    pub comm_version: u8,
    pub control_byte: Control,
    pub request_byte: u8,
    pub alliance_station: AllianceStation,
    pub tournament_level: TournamentLevel,
    pub match_number: u16,
    pub play_number: u8,
    pub date: Date,
    pub remaining_time: u16,
}

#[derive(Clone, Copy, Debug)]
pub struct UdpStatusPacket {
    pub sequence_number: u16,
    pub comm_version: u8,
    pub status_byte: UdpStatus,
    pub team_number: u16,
    pub battery: Battery,
}
