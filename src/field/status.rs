// Copyright (C) 2024 Hibiscus Software. All rights reserved. This
// work is licensed under the terms of the MIT license which can be
// found in the root directory of this project.

use super::alliance::AllianceStation;
use chrono::{DateTime, Local};

#[derive(Clone, Copy, Debug)]
pub enum RobotState {
    Auto,
    Test,
    Teleop,
}

#[derive(Clone, Copy, Debug)]
pub enum TournamentLevel {
    Test = 0,
    Practice = 1,
    Qualification = 2,
    Playoff = 3,
}

/// Statuses that are sent from field management system to driver station
#[derive(Clone, Copy, Debug)]
pub struct FMSToDS {
    pub estop: bool,
    pub enabled: bool,
    pub packet_count: u8,
    pub mode: RobotState,
    pub station: AllianceStation,
    pub tournament_level: TournamentLevel,
    pub match_number: u16,
    pub repeat_number: u8,
    pub current_time: DateTime<Local>,
    pub remaining_seconds: u16,
}
