/// Symplectic integrators

/// A dynamical system: dq/dt = f(q,p), dp/dt = g(q,p)
pub trait DynamicalSystem {
    fn dqdt(&self, q: f64, p: f64) -> f64;
    fn dpdt(&self, q: f64, p: f64) -> f64;
    fn hamiltonian(&self, q: f64, p: f64) -> f64;
    fn name(&self) -> &'static str;
}

/// Harmonic oscillator: H = p²/2 + q²/2
pub struct HarmonicOscillator;

impl DynamicalSystem for HarmonicOscillator {
    fn dqdt(&self, _q: f64, p: f64) -> f64 { p }
    fn dpdt(&self, q: f64, _p: f64) -> f64 { -q }
    fn hamiltonian(&self, q: f64, p: f64) -> f64 { 0.5 * (p * p + q * q) }
    fn name(&self) -> &'static str { "Harmonic Oscillator" }
}

/// Nonlinear pendulum: H = p²/2 - cos(q)
pub struct Pendulum;

impl DynamicalSystem for Pendulum {
    fn dqdt(&self, _q: f64, p: f64) -> f64 { p }
    fn dpdt(&self, q: f64, _p: f64) -> f64 { -q.sin() }
    fn hamiltonian(&self, q: f64, p: f64) -> f64 { 0.5 * p * p - q.cos() }
    fn name(&self) -> &'static str { "Pendulum" }
}

/// Kepler problem: H = p²/2 - 1/r
pub struct KeplerOrbit;

impl DynamicalSystem for KeplerOrbit {
    fn dqdt(&self, _q: f64, p: f64) -> f64 { p }
    fn dpdt(&self, q: f64, _p: f64) -> f64 {
        if q.abs() < 0.01 { return -100.0 * q.signum(); }
        -1.0 / (q * q)
    }
    fn hamiltonian(&self, q: f64, p: f64) -> f64 {
        let r = q.abs().max(0.01);
        0.5 * p * p - 1.0 / r
    }
    fn name(&self) -> &'static str { "Kepler Orbit" }
}

/// Integration methods
pub enum Integrator {
    Euler,
    Verlet,
    Yoshida,
}

impl Integrator {
    pub fn name(&self) -> &'static str {
        match self {
            Integrator::Euler => "Euler",
            Integrator::Verlet => "Störmer-Verlet",
            Integrator::Yoshida => "Yoshida (4th)",
        }
    }

    pub fn step(&self, system: &dyn DynamicalSystem, q: f64, p: f64, dt: f64) -> (f64, f64) {
        match self {
            Integrator::Euler => {
                let q_new = q + dt * system.dqdt(q, p);
                let p_new = p + dt * system.dpdt(q, p);
                (q_new, p_new)
            }
            Integrator::Verlet => {
                // Symplectic Euler (Verlet leapfrog)
                let p_half = p + 0.5 * dt * system.dpdt(q, p);
                let q_new = q + dt * system.dqdt(q, p_half);
                let p_new = p_half + 0.5 * dt * system.dpdt(q_new, p_half);
                (q_new, p_new)
            }
            Integrator::Yoshida => {
                // Yoshida 4th order
                let c1 = 1.0 / (2.0 - 2.0_f64.powf(1.0/3.0));
                let c2 = -2.0_f64.powf(1.0/3.0) / (2.0 - 2.0_f64.powf(1.0/3.0));
                let c3 = c1;
                let d1 = c1;
                let d2 = c2;
                let d3 = c1;

                let (q1, p1) = verlet_step(system, q, p, dt * c1, d1);
                let (q2, p2) = verlet_step(system, q1, p1, dt * c2, d2);
                let (q3, p3) = verlet_step(system, q2, p2, dt * c3, d3);
                (q3, p3)
            }
        }
    }
}

fn verlet_step(system: &dyn DynamicalSystem, q: f64, p: f64, dt: f64, _d: f64) -> (f64, f64) {
    let p_half = p + 0.5 * dt * system.dpdt(q, p);
    let q_new = q + dt * system.dqdt(q, p_half);
    let p_new = p_half + 0.5 * dt * system.dpdt(q_new, p_half);
    (q_new, p_new)
}

/// Run a trajectory
pub fn run_trajectory(
    system: &dyn DynamicalSystem,
    integrator: &Integrator,
    q0: f64,
    p0: f64,
    dt: f64,
    steps: usize,
) -> Vec<(f64, f64)> {
    let mut traj = vec![(q0, p0)];
    let (mut q, mut p) = (q0, p0);
    for _ in 0..steps {
        let (q_new, p_new) = integrator.step(system, q, p, dt);
        // Clamp to prevent divergence
        q = q_new.clamp(-100.0, 100.0);
        p = p_new.clamp(-100.0, 100.0);
        traj.push((q, p));
    }
    traj
}

/// Compute energy drift over trajectory
pub fn energy_drift(
    system: &dyn DynamicalSystem,
    trajectory: &[(f64, f64)],
) -> Vec<f64> {
    let e0 = system.hamiltonian(trajectory[0].0, trajectory[0].1);
    trajectory.iter()
        .map(|(q, p)| ((system.hamiltonian(*q, *p) - e0) / e0.abs().max(1e-10)).abs())
        .collect()
}
