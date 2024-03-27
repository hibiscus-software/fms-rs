// Copyright (C) 2024 Hibiscus Software. All rights reserved. This
// work is licensed under the terms of the MIT license which can be
// found in the root directory of this project.

use super::plc_inputs::PLCInputs;

pub struct PLC {
    inputs: PLCInputs,
}

impl PLC {
    #[must_use]
    pub const fn new(inputs: PLCInputs) -> Self {
        return Self { inputs };
    }

    /// Returns the state of the field e-stop button.
    pub async fn get_field_estop(&mut self) -> bool {
        return !self.inputs.field_estop;
    }
}
