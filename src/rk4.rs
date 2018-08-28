use vecmat::vec::*;

pub type Wrap<T> = (T, T, T, T);

pub fn wrap<T>(t: T) -> Wrap<T> where T: Clone {
    (t.clone(), t.clone(), t.clone(), t)
}

pub fn wrap_ref<T>(t: &T) -> Wrap<T> where T: Clone {
    (t.clone(), t.clone(), t.clone(), t.clone())
}

pub trait Var {
    fn step(&mut self, dt: f64, f: fn(&mut Wrap<&mut f64>, f64));

    fn step_0(&mut self, dt: f64) {
        self.step(dt, |v, dt| {
            *v.3 = *v.0;
            *v.2 = *v.1;
            *v.0 = *v.3 + *v.1*dt*0.5;
        });
    }
    fn step_1(&mut self, dt: f64) {
        self.step(dt, |v, dt| {
            *v.2 += *v.1*2.0;
            *v.0 = *v.3 + *v.1*dt*0.5;
        });
    }
    fn step_2(&mut self, dt: f64) {
        self.step(dt, |v, dt| {
            *v.2 += *v.1*2.0;
            *v.0 = *v.3 + *v.1*dt;
        });
    }
    fn step_3(&mut self, dt: f64) {
        self.step(dt, |v, dt| {
            *v.2 += *v.1;
            *v.0 = *v.3 + *v.2*dt/6.0;
        });
    }
}

impl<'a> Var for Wrap<&'a mut f64> {
    fn step(&mut self, dt: f64, f: fn(&mut Wrap<&mut f64>, f64)) {
        f(self, dt);
    }
}

impl Var for Wrap<f64> {
    fn step(&mut self, dt: f64, f: fn(&mut Wrap<&mut f64>, f64)) {
        f(&mut (&mut self.0, &mut self.1, &mut self.2, &mut self.3), dt);
    }
}

macro_rules! var_vec {
    ($V:ident, $N:expr) => (
        impl<'a> Var for Wrap<&'a mut $V> {
            fn step(&mut self, dt: f64, f: fn(&mut Wrap<&mut f64>, f64)) {
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

        impl Var for Wrap<$V> {
            fn step(&mut self, dt: f64, f: fn(&mut Wrap<&mut f64>, f64)) {
                (&mut self.0, &mut self.1, &mut self.2, &mut self.3).step(dt, f);
            }
        }
    )
}
var_vec!(Vec2f64, 2);
var_vec!(Vec3f64, 3);
var_vec!(Vec4f64, 4);

pub fn solve<F, T>(mut fn_step: F, dt: f64) where F: FnMut(fn(&mut T, f64), f64), T: Var {
    fn_step(|v, dt| v.step_0(dt), dt);
    fn_step(|v, dt| v.step_1(dt), dt);
    fn_step(|v, dt| v.step_2(dt), dt);
    fn_step(|v, dt| v.step_3(dt), dt);
}
