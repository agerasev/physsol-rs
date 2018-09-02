use num::{Float};
use vec::*;

pub type Wrap<V> = (V, V, V, V);

pub fn wrap<V>(t: V) -> Wrap<V> where V: Clone {
    (t.clone(), t.clone(), t.clone(), t)
}

pub fn wrap_ref<V>(t: &V) -> Wrap<V> where V: Clone {
    (t.clone(), t.clone(), t.clone(), t.clone())
}

pub trait Var<T> where T: Copy + Float {
    fn step(&mut self, dt: T, f: fn(&mut Wrap<&mut T>, T));

    fn step_0(&mut self, dt: T) {
        self.step(dt, |v, dt| {
            *v.3 = *v.0;
            *v.2 = *v.1;
            *v.0 = *v.3 + *v.1*dt*T::from(0.5).unwrap();
        });
    }
    fn step_1(&mut self, dt: T) {
        self.step(dt, |v, dt| {
            *v.2 = *v.2 + *v.1*T::from(2.0).unwrap();
            *v.0 = *v.3 + *v.1*dt*T::from(0.5).unwrap();
        });
    }
    fn step_2(&mut self, dt: T) {
        self.step(dt, |v, dt| {
            *v.2 = *v.2 + *v.1*T::from(2.0).unwrap();
            *v.0 = *v.3 + *v.1*dt;
        });
    }
    fn step_3(&mut self, dt: T) {
        self.step(dt, |v, dt| {
            *v.2 = *v.2 + *v.1;
            *v.0 = *v.3 + *v.2*dt/T::from(6.0).unwrap();
        });
    }
}

impl<'a, T> Var<T> for Wrap<&'a mut T> where T: Copy + Float {
    fn step(&mut self, dt: T, f: fn(&mut Wrap<&mut T>, T)) {
        f(self, dt);
    }
}

impl<T> Var<T> for Wrap<T> where T: Copy + Float {
    fn step(&mut self, dt: T, f: fn(&mut Wrap<&mut T>, T)) {
        f(&mut (&mut self.0, &mut self.1, &mut self.2, &mut self.3), dt);
    }
}

macro_rules! var_vec {
    ($V:ident, $N:expr) => (
        impl<'a, T> Var<T> for Wrap<&'a mut $V<T>> where T: Copy + Float {
            fn step(&mut self, dt: T, f: fn(&mut Wrap<&mut T>, T)) {
                for i in 0..$N {
                    unsafe { (
                        self.0.d.get_unchecked_mut(i),
                        self.1.d.get_unchecked_mut(i),
                        self.2.d.get_unchecked_mut(i),
                        self.3.d.get_unchecked_mut(i),
                    ).step(dt, f) }
                }
            }
        }

        impl<T> Var<T> for Wrap<$V<T>> where T: Copy + Float {
            fn step(&mut self, dt: T, f: fn(&mut Wrap<&mut T>, T)) {
                (&mut self.0, &mut self.1, &mut self.2, &mut self.3).step(dt, f);
            }
        }
    )
}
var_vec!(Vec2, 2);
var_vec!(Vec3, 3);
var_vec!(Vec4, 4);

pub fn solve<F, V, T>(mut fn_step: F, dt: T) 
    where F: FnMut(fn(&mut V, T), T), V: Var<T>, T: Copy + Float
{
    fn_step(|v, dt| v.step_0(dt), dt);
    fn_step(|v, dt| v.step_1(dt), dt);
    fn_step(|v, dt| v.step_2(dt), dt);
    fn_step(|v, dt| v.step_3(dt), dt);
}
