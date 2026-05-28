use leptos::*;
use crate::components::phase_space::PhaseSpace;
use crate::math::symplectic::*;

#[component]
pub fn SymplecticPage() -> impl IntoView {
    let (system_type, set_system_type) = create_signal(0u8); // 0=harmonic, 1=pendulum, 2=kepler
    let (integrator_type, set_integrator_type) = create_signal(0u8); // 0=euler, 1=verlet, 2=yoshida
    let (timestep, set_timestep) = create_signal(0.1);
    let (steps, set_steps) = create_signal(500);

    let trajectory = create_memo(move |_| {
        let system: Box<dyn DynamicalSystem> = match system_type.get() {
            0 => Box::new(HarmonicOscillator),
            1 => Box::new(Pendulum),
            _ => Box::new(KeplerOrbit),
        };

        let integrator = match integrator_type.get() {
            0 => Integrator::Euler,
            1 => Integrator::Verlet,
            _ => Integrator::Yoshida,
        };

        let (q0, p0) = match system_type.get() {
            0 => (1.0, 0.0),
            1 => (2.0, 0.0),
            _ => (1.0, 0.5),
        };

        run_trajectory(&*system, &integrator, q0, p0, timestep.get(), steps.get())
    });

    let energy_drift_data = create_memo(move |_| {
        let system: Box<dyn DynamicalSystem> = match system_type.get() {
            0 => Box::new(HarmonicOscillator),
            1 => Box::new(Pendulum),
            _ => Box::new(KeplerOrbit),
        };
        let traj = trajectory.get();
        let drift = energy_drift(&*system, &traj);
        drift.last().copied().unwrap_or(0.0)
    });

    let energy_color = move || {
        let drift = energy_drift_data.get();
        if drift < 0.001 { "#6bcb77" }
        else if drift < 0.01 { "#ffd93d" }
        else { "#ff6b6b" }
    };

    view! {
        <div class="page symplectic-page">
            <h1>"Symplectic Playground"</h1>
            <p class="page-desc">"Compare integrators on Hamiltonian systems. Watch phase space trajectories and energy conservation."</p>

            <div class="controls">
                <div class="control-group">
                    <label>"System:"</label>
                    <select on:change=move |e| set_system_type.set(event_target_value(&e).parse().unwrap_or(0))>
                        <option value="0">"Harmonic Oscillator"</option>
                        <option value="1">"Nonlinear Pendulum"</option>
                        <option value="2">"Kepler Orbit"</option>
                    </select>
                </div>
                <div class="control-group">
                    <label>"Integrator:"</label>
                    <select on:change=move |e| set_integrator_type.set(event_target_value(&e).parse().unwrap_or(0))>
                        <option value="0">"Euler (non-symplectic)"</option>
                        <option value="1">"Störmer-Verlet (symplectic)"</option>
                        <option value="2">"Yoshida 4th order (symplectic)"</option>
                    </select>
                </div>
                <div class="control-group">
                    <label>"Δt = " {move || format!("{:.3}", timestep.get())}</label>
                    <input
                        type="range" min="0.01" max="0.5" step="0.005"
                        prop:value=move || timestep.get()
                        on:input=move |e| set_timestep.set(event_target_value(&e).parse().unwrap_or(0.1))
                    />
                </div>
                <div class="control-group">
                    <label>"Steps: " {move || steps.get()}</label>
                    <input
                        type="range" min="100" max="2000" step="100"
                        prop:value=move || steps.get()
                        on:input=move |e| set_steps.set(event_target_value(&e).parse().unwrap_or(500))
                    />
                </div>
            </div>

            <div class="lab-grid">
                <div class="panel wide">
                    <h3>"Phase Space (q, p)"</h3>
                    {move || view! { <PhaseSpace trajectory=trajectory.get() width=700.0 height=500.0/> }}
                </div>

                <div class="panel">
                    <h3>"Energy Conservation"</h3>
                    <div class="energy-gauge">
                        <div class="gauge-bar" style=format!(
                            "background: {}; width: {}%",
                            energy_color(),
                            (1.0 - energy_drift_data.get().min(1.0)) * 100.0
                        )></div>
                    </div>
                    <div class="energy-value" style=format!("color: {}", energy_color())>
                        {move || format!("ΔE/E₀ = {:.6}", energy_drift_data.get())}
                    </div>
                    <div class="energy-hint">
                        {move || match integrator_type.get() {
                            0 => "⚠ Euler is not symplectic — energy drifts".to_string(),
                            1 => "✓ Verlet conserves energy (symplectic)".to_string(),
                            _ => "✓ Yoshida 4th order — excellent conservation".to_string(),
                        }}
                    </div>
                </div>
            </div>
        </div>
    }
}
