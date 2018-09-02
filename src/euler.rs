use num::{Float};
use vec::*;

pub type Wrap<T> = (T, T);
pub fn wrap<T>(t: T) -> Wrap<T> where T: Clone {
    (t.clone(), t)
}
pub fn wrap_ref<T>(t: &T) -> Wrap<T> where T: Clone {
    (t.clone(), t.clone())
}

pub trait Var<T> where T: Copy + Float {
    fn step(&mut self, dt: T);
}

impl<'a, T> Var<T> for Wrap<&'a mut T> where T: Copy + Float {
    fn step(&mut self, dt: T) {
        *self.0 = *self.0 + *self.1*dt;
    }
}

impl<T> Var<T> for Wrap<T> where T: Copy + Float {
    fn step(&mut self, dt: T) {
        self.0 = self.0 + self.1*dt;
    }
}

macro_rules! var_vec {
    ($V:ident, $N:expr) => (
        impl<'a, T> Var<T> for Wrap<&'a mut $V<T>> where T: Copy + Float {
            fn step(&mut self, dt: T) {
                for i in 0..$N {
                    unsafe { (
                        self.0.d.get_unchecked_mut(i),
                        self.1.d.get_unchecked_mut(i)
                    ).step(dt); }
                }
            }
        }

        impl<T> Var<T> for Wrap<$V<T>> where T: Copy + Float {
            fn step(&mut self, dt: T) {
                (&mut self.0, &mut self.1).step(dt);
            }
        }

    )
}
var_vec!(Vec2, 2);
var_vec!(Vec3, 3);
var_vec!(Vec4, 4);

pub fn solve<F, V, T>(mut fn_step: F, dt: T) where F: FnMut(fn(&mut V, T), T), V: Var<T>, T: Copy + Float {
    fn_step(|v, dt| v.step(dt), dt);
}
