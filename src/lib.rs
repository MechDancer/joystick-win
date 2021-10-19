use bindings_joystick_win::Windows::Win32::Media::Multimedia::*;
use std::time::Duration;

pub struct JoyStick(u32, Option<((u32, u32), (u32, u32))>);

impl Default for JoyStick {
    fn default() -> Self {
        Self::new(0)
    }
}

impl JoyStick {
    pub fn new(pos: u32) -> Self {
        Self(pos, get_caps(pos))
    }

    pub fn read(&mut self) -> (Duration, Option<(f32, f32)>) {
        if self.1.is_none() {
            self.1 = get_caps(self.0);
        }
        if let Some((x_range, y_range)) = self.1 {
            let mut info = JOYINFO::default();
            if unsafe { joyGetPos(self.0, &mut info as *mut _) } == JOYERR_NOERROR {
                return (
                    Duration::from_millis(1),
                    Some((
                        normalize(info.wXpos, x_range),
                        -normalize(info.wYpos, y_range),
                    )),
                );
            }
        }
        self.1 = None;
        (Duration::from_secs(1), None)
    }
}

fn get_caps(pos: u32) -> Option<((u32, u32), (u32, u32))> {
    let mut caps = JOYCAPSA::default();
    Some(unsafe {
        joyGetDevCapsA(
            pos as usize,
            &mut caps as *mut _,
            std::mem::size_of::<JOYCAPSA>() as u32,
        )
    })
    .filter(|e| *e == JOYERR_NOERROR)
    .and(Some(caps))
    .map(|c| ((c.wXmin, c.wXmax), (c.wYmin, c.wYmax)))
}

fn normalize(value: u32, range: (u32, u32)) -> f32 {
    println!("{} {} {}", value, range.0, range.1);
    let (min, max) = range;
    (value - min) as f32 / (max - min) as f32
}
