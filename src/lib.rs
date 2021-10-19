use bindings::Windows::Win32::Media::Multimedia::*;
use std::time::Duration;

pub struct JoyStick {
    pos: u32,
    memory: (u32, u32),
}

impl Default for JoyStick {
    fn default() -> Self {
        Self::new(0)
    }
}

impl JoyStick {
    pub fn new(pos: u32) -> Self {
        Self {
            pos,
            memory: (0, 0),
        }
    }

    pub fn read(&mut self) -> (Duration, Option<(u32, u32)>) {
        let mut info = JOYINFO::default();
        let error = unsafe { joyGetPos(self.pos, &mut info as *mut _) };
        if error == JOYERR_NOERROR {
            (
                Duration::from_millis(1),
                Some((info.wXpos, info.wYpos))
                    .filter(|e| *e != self.memory)
                    .map(|e| {
                        self.memory = e;
                        e
                    }),
            )
        } else {
            (Duration::from_secs(1), None)
        }
    }
}
