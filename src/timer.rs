pub struct Timer {
    delay: u8,
    sound: u8
}

impl Timer {
    pub fn new() -> Self {
        Self {delay: 0, sound: 0}
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
        if self.sound > 0 { 
            self.sound -= 1;
            unimplemented!();
        } else {
            unimplemented!();
        }
    }

    pub fn get_delay_timer(&self) -> u8 {
        self.delay
    }
}

#[cfg(test)]
mod test {
    #[test]
    fn timer_test() {

    }
}