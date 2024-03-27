// Copyright (C) 2024 Hibiscus Software. All rights reserved. This
// work is licensed under the terms of the MIT license which can be
// found in the root directory of this project.

use super::status::{FMSToDS, RobotState};
use async_std::net::{TcpStream, UdpSocket};
use bytes::BufMut;
use std::io::Error;

const DS_TCP_LISTEN_PORT: u16 = 1750;
const DS_UDP_SEND_PORT: u16 = 1121;
const DS_UDP_RECEIVE_PORT: u16 = 1160;
const DS_TCP_LINK_TIMEOUT_SECONDS: u8 = 5;
const DS_UDP_LINK_TIMEOUT_SECONDS: u8 = 1;
const MAX_TCP_PACKET_BYTES: u16 = 4096;

const DS_NAMES: [&'static str; 6] = ["Red 1", "Red 2", "Red 3", "Blue 1", "Blue 2", "Blue 3"];

pub struct DriverStation {
    team_number: u16,
    tcp_connection: TcpStream,
    udp_connection: UdpSocket,
    fms_to_ds: FMSToDS,
}

impl DriverStation {
    /// Creates a new driver station object
    pub fn new(
        team_number: u16,
        tcp_connection: TcpStream,
        udp_connection: UdpSocket,
        fms_to_ds: FMSToDS,
    ) -> Self {
        return Self {
            team_number,
            tcp_connection,
            udp_connection,
            fms_to_ds,
        };
    }

    /// Encodes the UDP control information into a packet
    pub fn encode_control_packet(&mut self, driver_station: u8) -> Result<(), Error> {
        let mut packet: Vec<u8> = vec![];

        // Packet number, stored big-endian in two bytes
        packet.put_u8((self.fms_to_ds.packet_count >> 8) & 0xff);
        packet.put_u8(self.fms_to_ds.packet_count & 0xff);

        // Protocol version
        packet.put_u8(0x00);

        // Robot status
        let mode: u8 = match self.fms_to_ds.mode {
            RobotState::Auto => 2,
            RobotState::Test => 1,
            RobotState::Teleop => 0,
        };
        let control: u8 = ((self.fms_to_ds.estop as u8) << 7)
            | ((self.fms_to_ds.enabled as u8) << 2)
            | (mode & 0b11);
        packet.put_u8(control);

        // Unknown or unused
        packet.put_u8(0x00);

        // Driver station number
        packet.put_u8(self.fms_to_ds.station.to_ds_number());

        self.fms_to_ds.packet_count += 1;

        Ok(())
    }
}
