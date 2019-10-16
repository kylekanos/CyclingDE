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

// returns the right hand side
fn rhs(v: f64) -> f64 {
    100.0 / v - 10.0 * v * v - 0.2 * 9.81
}

// returns right hand side with adding epsilon to division
fn rhs2(v: f64) -> f64 {
    100.0 / (v + 1e-3) - 10.0 * v * v - 0.2 * 9.81
}
// compute first step
fn first_step(v: f64, dt: f64) -> f64 {
    let mut a = 1.0e-10;
    let mut b = 100.0;
    let mut fa = a - dt * rhs(a) - v;
    let mut fb = b - dt * rhs(b) - v;
    let mut c=a;
    let mut fc=fa;

    while abs(a - b) > 1e-10 {
        c = 0.5 * (a + b);
        fc = c - dt * rhs(c) - v;
        let d = c + (c - a) * sign(fa) * fc / (fc * fc - fa * fb).sqrt();
        let fd = d - dt * rhs(d) - v;
        if fc < 1e-10 {
            return c;
        }
        if fc * fd < 0.0 {
            a = c; fa = fc;
            b = d; fb = fd;
        }
        else if fc * fa < 0.0 {
            b = c;
            fb = fc;
        }
        else {
            a = c;
            fa = fc;
        }
    }
    return c;
}

fn main() {
    let dt = 1e-5;
    let mut v1 = 0.0;
    let mut v2 = 0.0;
    let mut t = 0.0;

    println!("# t\tv1\tv2");
    println!("0.000000\t0.000000\t0.000000");
    // back out the first time-step
    v1 += first_step(0.0, dt);
    v2 += dt * rhs2(v2);
    t += dt;
    println!("{0:0.6}\t{1:0.6}\t{2:0.6}",t,v1,v2);

    let mut vold = 0.0;

    while t < 0.10 && abs(vold - v1) > 1e-10{
        vold = v1;
        v1 += dt * rhs(v1);
        v2 += dt * rhs2(v2);
        t += dt;
        println!("{0:0.6}\t{1:0.6}\t{2:0.6}",t,v1,v2);
        
    }

    println!("#v(0) -> {}", first_step(0.0, dt));
    
//    println!("Hello, world!");
}
