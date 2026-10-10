fn get_alpha(
    l1: f64,
    m1: f64,
    theta1: f64,
    omega1: f64,
    l2: f64,
    m2: f64,
    theta2: f64,
    omega2: f64,
) -> (f64, f64) {
    // Constants
    let g: f64 = 9.81;
    let delta = theta1 - theta2;

    // Precomputations
    let sin_delta = delta.sin();
    let cos_delta = delta.cos();

    let sin_theta1 = theta1.sin();
    let sin_theta2 = theta2.sin();

    let a1 = l1 * omega1 * omega1;
    let a2 = l2 * omega2 * omega2;

    let denominator = m1 + m2 * sin_delta * sin_delta;

    // alpha1
    let mut alpha1 = a1 * cos_delta + a2;
    alpha1 *= -(m2 * sin_delta);
    alpha1 += m2 * g * sin_theta2 * cos_delta;
    alpha1 -= (m1 + m2) * g * sin_theta1;
    alpha1 /= l1 * denominator;

    // alpha2
    let mut alpha2 = a1 * sin_delta;
    alpha2 -= g * sin_theta2;
    alpha2 += g * sin_theta1 * cos_delta;
    alpha2 *= m1 + m2;
    alpha2 += m2 * a2 * sin_delta * cos_delta;
    alpha2 /= l2 * denominator;

    (alpha1, alpha2)
}

pub fn step(
    l1: f64,
    m1: f64,
    theta1: &mut f64,
    omega1: &mut f64,
    l2: f64,
    m2: f64,
    theta2: &mut f64,
    omega2: &mut f64,
    dt: f64,
) {
    let (alpha1, alpha2) = get_alpha(l1, m1, *theta1, *omega1, l2, m2, *theta2, *omega2);

    *omega1 += alpha1 * dt;
    *omega2 += alpha2 * dt;

    *theta1 += *omega1 * dt;
    *theta2 += *omega2 * dt;
}
