use vecmat::vec::*;
use euler::{Wrap as EulerWrap, Var as EulerVar};
use rk4::{Wrap as RK4Wrap, Var as RK4Var};

macro_rules! point_struct {
    ($P:ident, $V:ident, $N:expr) => (
        #[derive(Clone, Debug, PartialEq)]
        pub struct $P {
            pub pos: $V,
            pub vel: $V,
        }
    )
}

macro_rules! point_euler {
    ($P:ident, $V:ident, $N:expr) => (
        impl<'a> EulerVar for EulerWrap<&'a mut $P> {
            fn step(&mut self, dt: f64) {
                (&mut self.0.pos, &mut self.1.pos).step(dt);
                (&mut self.0.vel, &mut self.1.vel).step(dt);
            }
        }
        impl<'a> EulerVar for EulerWrap<$P> {
            fn step(&mut self, dt: f64) {
                (&mut self.0, &mut self.1).step(dt);
            }
        }
    )
}

macro_rules! point_rk4 {
    ($P:ident, $V:ident, $N:expr) => (
        impl<'a> RK4Var for RK4Wrap<&'a mut $P> {
            fn step(&mut self, dt: f64, f: fn(&mut RK4Wrap<&mut f64>, f64)) {
                (&mut self.0.pos, &mut self.1.pos, &mut self.2.pos, &mut self.3.pos).step(dt, f);
                (&mut self.0.vel, &mut self.1.vel, &mut self.2.vel, &mut self.3.vel).step(dt, f);
            }
        }
        impl<'a> RK4Var for RK4Wrap<$P> {
            fn step(&mut self, dt: f64, f: fn(&mut RK4Wrap<&mut f64>, f64)) {
                (&mut self.0, &mut self.1, &mut self.2, &mut self.3).step(dt, f);
            }
        }
    )
}

macro_rules! point_all {
    ($P:ident, $V:ident, $N:expr) => (
        point_struct!($P, $V, $N);
        point_euler!($P, $V, $N);
        point_rk4!($P, $V, $N);
    )
}

point_all!(Point2, Vec2f64, 2);
point_all!(Point3, Vec3f64, 3);
point_all!(Point4, Vec4f64, 4);
