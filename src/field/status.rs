// Copyright (C) 2024 Hibiscus Software. All rights reserved. This
// work is licensed under the terms of the MIT license which can be
// found in the root directory of this project.

use super::r#match::AllianceStation;

/// Statuses that are sent from field management system to driver station
#[derive(Debug)]
pub struct FMSToDS {
    pub estop: bool,
    pub enabled: bool,
    pub packet_count: u8,
    pub mode: RobotState,
    pub station: AllianceStation,
    pub tournament_level: TournamentLevel,
}

#[derive(Clone, Copy, Debug)]
pub enum RobotState {
    Auto,
    Test,
    Teleop,
}

#[derive(Debug)]
pub enum TournamentLevel {
    Test = 0,
    Practice = 1,
    Qualification = 2,
    Playoff = 3,
}
