// Copyright (C) 2024 Hibiscus Software. All rights reserved. This
// work is licensed under the terms of the MIT license which can be
// found in the root directory of this project.

use async_std::net::{TcpStream, UdpSocket};
use bytes::BufMut;
use std::io::Error;

use super::status::{UdpControlPacket, Mode};

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
    control_packet: UdpControlPacket,
}

impl DriverStation {
    /// Creates a new driver station object
    pub fn new(
        team_number: u16,
        tcp_connection: TcpStream,
        udp_connection: UdpSocket,
        control_packet: UdpControlPacket,
    ) -> Self {
        return Self {
            team_number,
            tcp_connection,
            udp_connection,
            control_packet,
        };
    }

    pub async fn listen_for_udp(&mut self) -> Result<(), Error> {
        let listener = UdpSocket::bind(format!("0.0.0.0:{}", DS_UDP_RECEIVE_PORT)).await?;
        listener
            .connect(format!("0.0.0.0:{}", DS_UDP_RECEIVE_PORT))
            .await?;

        loop {
            Ok(())
        }
    }

    /// Encodes the UDP control information into a packet
    pub fn encode_control_packet(&mut self) -> Result<Vec<u8>, Error> {
        let mut packet: Vec<u8> = vec![];

        // Packet number, stored big-endian in two bytes
        packet.put_u8((self.control_packet.sequence_number >> 8) & 0xff);
        packet.put_u8(self.control_packet.sequence_number & 0xff);

        // Protocol version
        packet.put_u8(self.control_packet.comm_version);

        // Robot status
        let mode: u8 = match self.control_packet.control_byte.mode {
            Mode::Auto => 2,
            Mode::Test => 1,
            Mode::Teleop => 0,
        };
        let control: u8 = ((self.control_packet.control_byte.estop as u8) << 7)
            | ((self.control_packet.control_byte.enabled as u8) << 2)
            | (mode & 0x03);
        packet.put_u8(control);

        // Unknown or unused
        packet.put_u8(self.control_packet.request_byte);

        // Driver station number
        packet.put_u8(self.control_packet.alliance_station.to_ds_number());

        // Match type
        packet.put_u8(self.control_packet.tournament_level as u8);

        // Match number
        packet.put_u16(self.control_packet.match_number);

        // Match repeat number
        packet.put_u8(self.control_packet.play_number);

        // Current time and date
        packet.put_u32(self.control_packet.date.microseconds);
        packet.put_u8(self.control_packet.date.second);
        packet.put_u8(self.control_packet.date.minute);
        packet.put_u8(self.control_packet.date.hour);
        packet.put_u8(self.control_packet.date.day);
        packet.put_u8(self.control_packet.date.month);
        packet.put_u8(self.control_packet.date.year);

        // Remaining seconds
        packet.put_u16(self.control_packet.remaining_time);

        // Increment packout count
        self.control_packet.sequence_number += 1;

        Ok(packet)
    }

    /// Sends the next control packet to the driver station
    pub async fn send_control_packet(&mut self) -> Result<(), Error> {
        let packet = self
            .encode_control_packet()
            .expect("[ERROR] Unable to construct control packet.");

        self.udp_connection
            .send(&packet)
            .await
            .expect("[ERROR] Unable to send control packet.");

        Ok(())
    }
}
