fn main() {
    windows::build! {
        Windows::Win32::Media::Multimedia::{
            JOYERR_NOERROR,

            JOYINFO,
            joyGetPos,
        }
    }
}
