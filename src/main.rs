use std::f64;

// abs function
fn abs(a: f64) -> f64 {
    if a < 0.0 { return -a;}
    return a;
}

// sign function
fn sign(a: f64) -> f64 {
    if a > 0.0 {
        return 1.0;
    }
    else if a < 0.0 {
        return -1.0;
    }
    return 0.0;
}

/*
The ode we're solving is v' = P/(m*v) - k*v^2/m - ug
Using:
  * P = 1e3
  * m = 10
  * k = 100
  * u = 0.2
  * g = 9.81
The issue is, of course, that 1/v with v=0 returns an error.
*/

// returns the right hand side without modifications
fn rhs(v: f64) -> f64 {
    100.0 / v - 10.0 * v * v - 0.2 * 9.81
}

// returns right hand side but adds an epsilon to division portion
fn rhs2(v: f64) -> f64 {
    100.0 / (v + 1e-3) - 10.0 * v * v - 0.2 * 9.81
}

// Determine the v that satisfies dt * rhs(v) - v = 0
// with a simple root-finding algorithm
fn first_step(v: f64, dt: f64) -> f64 {
    let mut a = 1.0e-10;
    let mut b = 100.0;
    let mut fa = a - dt * rhs(a) - v;
    let mut fb = b - dt * rhs(b) - v;
    let mut c=a;
    let mut fc=fa;

    while abs(a - b) > 1e-10 {
        // get midpoint & function value at midpoint
        c = 0.5 * (a + b);
        fc = c - dt * rhs(c) - v;
        // this is Ridder's method
        let d = c + (c - a) * sign(fa) * fc / (fc * fc - fa * fb).sqrt();
        let fd = d - dt * rhs(d) - v;
        // if fd is small, accept this, otherwise return
        if fd < 1e-10 {
            return c;
        }
        // update the brackets
        if fc * fd < 0.0 {
            // (c,fc), (d,fd) bracket the root
            a = c; fa = fc;
            b = d; fb = fd;
        }
        else if fc * fa < 0.0 {
            // (a,fa), (c,fc) bracket the root
            b = c;
            fb = fc;
        }
        else {
            // (c,fc), (b,fb) bracket the root
            a = c;
            fa = fc;
        }
    }
    return c;
}

// main function
fn main() {
    let dt = 1e-5;          // the const time-step
    let mut v1 = 0.0;       // the velocity using an interpolated first point
    let mut v2 = 0.0;       // the velocity using the v+epsilon method
    let mut t = 0.0;        // the current time

    println!("# t\tv1\tv2");
    println!("0.000000\t0.000000\t0.000000");
    // back out the first time-step
    v1 += first_step(0.0, dt);
    // use epsilon for first step
    v2 += dt * rhs2(v2);
    t += dt;
    println!("{0:0.6}\t{1:0.6}\t{2:0.6}",t,v1,v2);

    let mut vold = 0.0;
    // integrate while t < 0.1 and v hasn't yet converged to a stable value
    while t < 0.10 && abs(vold - v1) > 1e-10{
        vold = v1;
        v1 += dt * rhs(v1);
        v2 += dt * rhs2(v2);
        t += dt;
        println!("{0:0.6}\t{1:0.6}\t{2:0.6}",t,v1,v2);
    }

    // for reference, display the first step again, but don't
    // want it displayed in output, so prepend the text with #
    // so that gnuplot won't see it
    println!("#v(0) -> {}", first_step(0.0, dt));
}
