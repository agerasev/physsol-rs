use num::{Float};
use vec::*;
use euler::{Wrap as EulerWrap, Var as EulerVar};
use rk4::{Wrap as RK4Wrap, Var as RK4Var};

macro_rules! point_struct {
    ($P:ident, $V:ident) => (
        #[derive(Clone, Debug, PartialEq)]
        pub struct $P<T> where T: Copy + Float {
            pub pos: $V<T>,
            pub vel: $V<T>,
        }
    )
}

macro_rules! point_euler {
    ($P:ident, $V:ident) => (
        impl<'a, T> EulerVar<T> for EulerWrap<&'a mut $P<T>> where T: Copy + Float {
            fn step(&mut self, dt: T) {
                (&mut self.0.pos, &mut self.1.pos).step(dt);
                (&mut self.0.vel, &mut self.1.vel).step(dt);
            }
        }
        impl<'a, T> EulerVar<T> for EulerWrap<$P<T>> where T: Copy + Float {
            fn step(&mut self, dt: T) {
                (&mut self.0, &mut self.1).step(dt);
            }
        }
    )
}

macro_rules! point_rk4 {
    ($P:ident, $V:ident) => (
        impl<'a, T> RK4Var<T> for RK4Wrap<&'a mut $P<T>> where T: Copy + Float {
            fn step(&mut self, dt: T, f: fn(&mut RK4Wrap<&mut T>, T)) {
                (&mut self.0.pos, &mut self.1.pos, &mut self.2.pos, &mut self.3.pos).step(dt, f);
                (&mut self.0.vel, &mut self.1.vel, &mut self.2.vel, &mut self.3.vel).step(dt, f);
            }
        }
        impl<'a, T> RK4Var<T> for RK4Wrap<$P<T>> where T: Copy + Float {
            fn step(&mut self, dt: T, f: fn(&mut RK4Wrap<&mut T>, T)) {
                (&mut self.0, &mut self.1, &mut self.2, &mut self.3).step(dt, f);
            }
        }
    )
}

macro_rules! point_all {
    ($P:ident, $V:ident) => (
        point_struct!($P, $V);
        point_euler!($P, $V);
        point_rk4!($P, $V);
    )
}

point_all!(Point2, Vec2);
point_all!(Point3, Vec3);
point_all!(Point4, Vec4);
