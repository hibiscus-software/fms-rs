// Copyright (C) 2024 Hibiscus Software. All rights reserved. This
// work is licensed under the terms of the MIT license which can be
// found in the root directory of this project.

use std::net::Ipv4Addr;

/// Statuses that are sent from driver station to field management system
pub struct DSStatus {
    pub linked: bool,
    pub missed_packet_count: u16,
    pub last_packet_time: u16,
    pub packet_count: u8,
    pub ip_address: Ipv4Addr,
    pub missed_packet_offset: u16,
    pub computer_battery_percent: u16,
    pub computer_cpu_percent: u16,
    pub last_log: &'static str,
}
