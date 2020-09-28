use crate::tenpai_analysis::HoraType;

const KAZOE_YAKUMAN_FAN_MAX: u32 = 99;
const YAKUMAN_FAN: u32 = 100;

#[derive(Clone, Debug)]
pub struct PointDatam {
    pub fu: u32,
    pub fan: u32,
    pub oya: bool,
    pub hora_type: HoraType,
    pub base_points: i32,
    pub points: i32,
    pub oya_payment: i32,
    pub ko_payment: i32,
}

impl PointDatam {

    pub fn new(fu: u32, fan: u32, oya: bool, hora_type: HoraType) -> Self {
        let mut base_points: i32;
    
        match fan {
            YAKUMAN_FAN..=u32::MAX => {
                base_points = 8000 * (fan / YAKUMAN_FAN) as i32;
            }
            13..=KAZOE_YAKUMAN_FAN_MAX => {
                base_points = 8000;
            }
            11 | 12 => {
                base_points = 6000;
            }
            8 | 9 | 10 => {
                base_points = 4000;
            }
            6 | 7 => {
                base_points = 3000;
            }
            5 => {
                base_points = 2000;
            }
            _ => {
                base_points = (fu * u32::pow(2, fan + 2)) as i32;
                base_points = std::cmp::min(base_points, 2000);
            }
        };
        let multiple: i32;
        let points: i32;
        let oya_payment: i32;
        let ko_payment: i32;

        if hora_type == HoraType::Ron {
            if oya {
                multiple = 6;
            } else {
                multiple = 4;
            }
            points = ceil_points(base_points * multiple);
            oya_payment = points;
            ko_payment = points;
        } else {
            if oya {
                oya_payment = 0;
                ko_payment = ceil_points(base_points * 2);
                points = ko_payment * 3;
            } else {
                oya_payment = ceil_points(base_points * 2);
                ko_payment = ceil_points(base_points);
                points = oya_payment + ko_payment * 2;
            }
        }

        Self {
            fu,
            fan,
            oya,
            hora_type,
            base_points,
            points,
            oya_payment,
            ko_payment,
        }
    }
}

fn ceil_points(point: i32) -> i32 {
    (point / 100) * 100
}
