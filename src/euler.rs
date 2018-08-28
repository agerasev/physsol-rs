use vecmat::vec::*;

pub type Wrap<T> = (T, T);
pub fn wrap<T>(t: T) -> Wrap<T> where T: Clone {
    (t.clone(), t)
}
pub fn wrap_ref<T>(t: &T) -> Wrap<T> where T: Clone {
    (t.clone(), t.clone())
}

pub trait Var {
    fn step(&mut self, dt: f64);
}

impl<'a> Var for Wrap<&'a mut f64> {
    fn step(&mut self, dt: f64) {
        *self.0 += *self.1*dt;
    }
}

impl Var for Wrap<f64> {
    fn step(&mut self, dt: f64) {
        self.0 += self.1*dt;
    }
}

macro_rules! var_vec {
    ($V:ident, $N:expr) => (
        impl<'a> Var for Wrap<&'a mut $V> {
            fn step(&mut self, dt: f64) {
                for i in 0..$N {
                    unsafe { (
                        self.0.d.get_unchecked_mut(i),
                        self.1.d.get_unchecked_mut(i)
                    ).step(dt); }
                }
            }
        }

        impl Var for Wrap<$V> {
            fn step(&mut self, dt: f64) {
                (&mut self.0, &mut self.1).step(dt);
            }
        }

    )
}
var_vec!(Vec2f64, 2);
var_vec!(Vec3f64, 3);
var_vec!(Vec4f64, 4);

pub fn solve<F, T>(mut fn_step: F, dt: f64) where F: FnMut(fn(&mut T, f64), f64), T: Var {
    fn_step(|v, dt| v.step(dt), dt);
}
