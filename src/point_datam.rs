use crate::tenpai_analysis::HoraType

#[derive(Debug)]
pub struct PointDatam {
    fu:i8,
    fan:i8,
    oya:bool,
    hora_type:HoraType,
    base_points:i32,
    points:i32,
    oya_payment:i32,
    ko_payent:i32,
}

impl PointDatam {
    
    
    fn ceil_points(points) -> i32 {
        1
    }
}