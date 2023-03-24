use std::io::Cursor;
use byteorder::{LittleEndian, ReadBytesExt};

pub fn read(input: &[u8]) -> anyhow::Result<()> {
    let mut input = Cursor::new(input);

    let _reserved=input.read_u16::<LittleEndian>()?;
    let typ=input.read_u16::<LittleEndian>()?;

    match typ {
        // case 0:
        //     float angle_axis[3];
        // break;
        // case 1:
        //     int32 pixel_exposure_time;
        // int32 rolling_shutter_skew_time;
        // break;
        // case 2:
        //     float gyro[3];
        // break;
        //
        // case 3:
        //     float acceleration[3];
        // break;
        //
        // case 4:
        //     float position[3];
        // break;
        //
        // case 5:
        //     double latitude;
        // double longitude;
        // double altitude;
        // break;
        //
        // case 6:
        //     double time_gps_epoch;
        // int gps_fix_type;
        // double latitude;
        // double longitude;
        // float altitude;
        // float horizontal_accuracy;
        // float vertical_accuracy;
        // float velocity_east;
        // float velocity_north;
        // float velocity_up;
        // float speed_accuracy;
        // break;
        //
        // case 7:
        //     float magnetic_field[3];
        // break;
        o => return Err(anyhow::Error::msg(format!("Type Not Supported {}",o)))
    }
    Ok(())

}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_read() {
        let input = [0,0,0,0];
        let res=read(&input);
        println!("Res {:?}",res);
        assert!(res.is_err());
    }
}
