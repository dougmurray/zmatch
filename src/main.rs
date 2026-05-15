use clap::Parser;
use std::f32::consts::PI;

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
/// Finds the components to match the circuit's impedances
struct Args {
    ///Source impedance
    #[arg(short = 's', long = "source-impedance")]
    rs: f32,

    /// Load impedance
    #[arg(short = 'l', long = "load-impedance")]
    rl: f32,

    /// Frequency
    #[arg(short = 'f', long = "frequency")]
    freq: f32,
}

fn main() {
    let args = Args::parse();

    println!("Impedance Matcher");

    let (x_s, x_p) = matched_q_impedances(args.rs, args.rl);
    let c_s = cap_from_impedance(args.freq, x_s);
    let l_p = ind_from_impedance(args.freq, x_p);
    let l_s = ind_from_impedance(args.freq, x_s);
    let c_p = cap_from_impedance(args.freq, x_p);
    let q_value = matched_q_from_load_source(args.rl, args.rs);

    println!("Q @ {q_value}:");
    println!("C_s: {c_s:E} F, \t L_p: {l_p:E} H");
    println!("Or");
    println!("L_s: {l_s:E} H, \t C_p: {c_p:E} F");
}

fn cap_from_impedance(f: f32, z: f32) -> f32 {
    // Capacitor which equals Z impedance at f frequency.
    1.0 / (2.0 * PI * f * z)
}

fn ind_from_impedance(f: f32, z: f32) -> f32 {
    // Inductor which equals Z impedance at f frequency.
    z / (2.0 * PI * f)
}

fn matched_q_from_load_source(r_p: f32, r_s: f32) -> f32 {
    // Outputs matched Q based on load and source impedances
    let pre_q  = (f32::max(r_p, r_s) / f32::min(r_p, r_s) ) - 1.0;
    pre_q.sqrt()
}

fn matched_q_impedances(r_s: f32, r_p: f32) -> (f32, f32) {
    // This matches the Q for impedance matching circuits
    let q = matched_q_from_load_source(r_p, r_s);
    let x_s = q * f32::min(r_p, r_s);
    let x_p = f32::max(r_p, r_s) / q;
    (x_s, x_p)
}
