use raylib::math::Vector2;

pub struct Polar {
    pub rho: f32,
    pub theta: f32,
}

impl From<Vector2> for Polar {
    fn from(value: Vector2) -> Self {
        let x = value.x;
        let y = value.y;
        Polar {
            rho: (x.powi(2) + y.powi(2)).sqrt(),
            theta: f32::atan2(y, x),
        }
    }
}
impl From<Polar> for Vector2 {
    fn from(value: Polar) -> Vector2 {
        let rho = value.rho;
        let theta = value.theta;
        Vector2 {
            x: rho * theta.cos(),
            y: rho * theta.sin(),
        }
    }
}
