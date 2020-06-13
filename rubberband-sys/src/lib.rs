#![allow(non_upper_case_globals)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]

include!(concat!(env!("OUT_DIR"), "/bindings.rs"));

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn correctly_new_and_delete() {
        let channel_count = 2;
        let time_ratio = 1.0;
        let pitch_scale = 1.0;
        let rubberband = unsafe {
            rubberband_new(
                44100,
                channel_count,
                RubberBandOption_RubberBandOptionProcessRealTime as _,
                time_ratio,
                pitch_scale,
            )
        };
        assert!(!rubberband.is_null());
        unsafe {
            assert_eq!(rubberband_get_channel_count(rubberband), channel_count);
            assert!((rubberband_get_time_ratio(rubberband) - time_ratio).abs() < f64::EPSILON);
            assert!((rubberband_get_pitch_scale(rubberband) - pitch_scale).abs() < f64::EPSILON);
        }
        unsafe {
            rubberband_delete(rubberband);
        }
    }
}
