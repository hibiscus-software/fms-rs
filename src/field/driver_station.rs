// Copyright (C) 2024 Hibiscus Software. All rights reserved. This
// work is licensed under the terms of the MIT license which can be
// found in the root directory of this project.

use super::status::{FMSToDS, RobotState};
use async_std::net::{TcpStream, UdpSocket};
use bytes::BufMut;
use chrono::{Datelike, Timelike};
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
            | (mode & 0x03);
        packet.put_u8(control);

        // Unknown or unused
        packet.put_u8(0x00);

        // Driver station number
        packet.put_u8(self.fms_to_ds.station.to_ds_number());

        // Match type
        packet.put_u8(self.fms_to_ds.tournament_level as u8);

        // Match number
        packet.put_u16(self.fms_to_ds.match_number);

        // Match repeat number
        packet.put_u8(self.fms_to_ds.repeat_number);

        // Current time and date
        packet.put_u32(self.fms_to_ds.current_time.timestamp_subsec_micros());
        packet.put_u8(self.fms_to_ds.current_time.second() as u8);
        packet.put_u8(self.fms_to_ds.current_time.minute() as u8);
        packet.put_u8(self.fms_to_ds.current_time.hour() as u8);
        packet.put_u8(self.fms_to_ds.current_time.day() as u8);
        packet.put_u8(self.fms_to_ds.current_time.month() as u8);
        packet.put_u8((self.fms_to_ds.current_time.year() - 1900) as u8);

        // Remaining seconds
        packet.put_u16(self.fms_to_ds.remaining_seconds);

        // Increment packout count
        self.fms_to_ds.packet_count += 1;

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
