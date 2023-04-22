pub struct Timer {
    delay: u8,
    sound: u8,
    frequency: u32
}

impl Timer {
    pub fn new() -> Self {
        Self {delay: 0, sound: 0, frequency: 60}
    }

    pub fn set_delay_timer(&mut self, value: u8) {
        self.delay = value;
    }

    pub fn set_sound_timer(&mut self, value: u8) {
        self.sound = value;
    }

    pub fn update(&mut self) {
        self.update_delay_timer();
        self.update_sound_timer();
    }

    fn update_delay_timer(&mut self) {
        if self.delay > 0 { self.delay -=1 };
    }

    fn update_sound_timer(&mut self) {
        if self.sound > 0 { self.sound -= 1 };
    }

    pub fn get_delay_timer(&self) -> u8 {
        self.delay
    }

    pub fn get_sound_timer(&self) -> u8 {
        self.sound
    }

    pub fn get_frequency(&self) -> u32 {
        self.frequency
    }
}

impl Default for Timer {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod test {
    #[test]
    fn timer_test() {

    }
}