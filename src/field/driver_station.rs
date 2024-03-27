// Copyright (C) 2024 Hibiscus Software. All rights reserved. This
// work is licensed under the terms of the MIT license which can be
// found in the root directory of this project.

use std::io::Error;

use async_std::net::{TcpStream, UdpSocket};
use byteorder::BigEndian;

use super::status::DSStatus;

const DS_TCP_LISTEN_PORT: u16 = 1750;
const DS_UDP_SEND_PORT: u16 = 1121;
const DS_UDP_RECEIVE_PORT: u16 = 1160;
const DS_TCP_LINK_TIMEOUT_SECONDS: u8 = 5;
const DS_UDP_LINK_TIMEOUT_SECONDS: u8 = 1;
const MAX_TCP_PACKET_BYTES: u16 = 4096;

const DS_NAMES: [&'static str; 6] = ["Red 1", "Red 2", "Red 3", "Blue 1", "Blue 2", "Blue 3"];

pub struct DriverStation {
    team_number: u16,
    driver_station: u8,
    tcp_connection: TcpStream,
    udp_connection: UdpSocket,
    ds_status: DSStatus,
}

impl DriverStation {
    /// Creates a new driver station object
    pub const fn new(
        team_number: u16,
        driver_station: u8,
        tcp_connection: TcpStream,
        udp_connection: UdpSocket,
        ds_status: DSStatus,
    ) -> Self {
        return Self {
            team_number,
            driver_station,
            tcp_connection,
            udp_connection,
            ds_status,
        };
    }

    /// Encodes the UDP control information into a packet
    pub fn encode_control_packet(&mut self, driver_station: u8) -> Result<(), Error> {
        let mut packet: Vec<u8> = vec![];

        // Packet number, stored big-endian in two bytes
        packet.push((self.ds_status.packet_count >> 8) & 0xff);
        packet.push(self.ds_status.packet_count & 0xff);

        // Protocol versio
        packet.push(0x00);

        // Robot status

        Ok(())
    }
}
