use crate::structs::{Point, Prediction};
use std::f64::consts::PI;

#[derive(Clone, Copy)]
pub struct Ring {
    pub inner_radius: f64,
    pub outer_radius: f64,
    pub count: i32,
    pub index: i32,
}

pub struct Triangulator {
    sigma: f64,
    measurements: Vec<Point>,
    rings: Vec<Ring>,
    beta_alpha: f64,
    beta_beta: f64,
    max_snap_distance: f64,
}

impl Triangulator {
    pub fn new(sigma: f64, measurements: Vec<Point>) -> Self {
        let rings = vec![
            Ring {
                inner_radius: 1408.0,
                outer_radius: 2688.0,
                count: 3,
                index: 0,
            },
            Ring {
                inner_radius: 4480.0,
                outer_radius: 5760.0,
                count: 6,
                index: 1,
            },
            Ring {
                inner_radius: 7552.0,
                outer_radius: 8832.0,
                count: 10,
                index: 2,
            },
            Ring {
                inner_radius: 10624.0,
                outer_radius: 11904.0,
                count: 15,
                index: 3,
            },
            Ring {
                inner_radius: 13696.0,
                outer_radius: 14976.0,
                count: 21,
                index: 4,
            },
            Ring {
                inner_radius: 16768.0,
                outer_radius: 18048.0,
                count: 28,
                index: 5,
            },
            Ring {
                inner_radius: 19840.0,
                outer_radius: 21120.0,
                count: 36,
                index: 6,
            },
            Ring {
                inner_radius: 22912.0,
                outer_radius: 24192.0,
                count: 9,
                index: 7,
            },
        ];

        let beta_alpha = 5.5;
        let beta_beta = 5.5;
        let max_snap_distance = 15.0 * 2_f64.sqrt();

        Self {
            sigma,
            measurements,
            rings,
            beta_alpha,
            beta_beta,
            max_snap_distance,
        }
    }

    fn cart_to_polar(&self, x: f64, z: f64) -> (f64, f64) {
        let r = (x.powi(2) + z.powi(2)).sqrt();
        let phi = x.atan2(z);
        (r, phi)
    }

    fn polar_to_cart(&self, r: f64, phi: f64) -> (f64, f64) {
        let x = r * phi.sin();
        let z = r * phi.cos();
        (x, z)
    }

    fn to_point_angle(&self, px: f64, pz: f64, tx: f64, tz: f64) -> f64 {
        let dx = tx - px;
        let dz = tz - pz;
        (-dx).atan2(dz).to_degrees()
    }

    fn get_ring(&self, distance: f64) -> Option<Ring> {
        for ring in &self.rings {
            if ring.inner_radius <= distance && distance <= ring.outer_radius {
                return Some(ring.clone());
            }
        }
        None
    }

    fn delta(&self, delta: f64, ring: Ring) -> f64 {
        let a_k = ring.inner_radius;
        let max_delta = self.max_snap_distance / a_k;

        if delta.abs() >= max_delta {
            return 0.0;
        }

        let x = a_k * delta / self.max_snap_distance;

        if x.abs() >= 1.0 {
            return 0.0;
        }

        let val = (1.0 + x).powf(4.5) * (1.0 - x).powf(4.5);

        let norm_const =
            self.beta_func(self.beta_alpha, self.beta_beta) * self.max_snap_distance / (2.0 * a_k);

        val / norm_const
    }

    fn beta_func(&self, a: f64, b: f64) -> f64 {
        libm::exp(libm::lgamma(a) + libm::lgamma(b) - libm::lgamma(a + b))
    }

    fn get_distance_constraint(
        &self,
        ri: f64,
        phi_i: f64,
        rl: f64,
        phi_l: f64,
        px: f64,
        pz: f64,
    ) -> f64 {
        let (xi, zi) = self.polar_to_cart(ri, phi_i);
        let (xl, zl) = self.polar_to_cart(rl, phi_l);

        let di = ((xi - px).powi(2) + (zi - pz).powi(2)).sqrt();

        let (rp, phi_p) = self.cart_to_polar(px, pz);

        let angle_diff = phi_p - phi_l;

        if angle_diff.sin().abs() < 1e-10 {
            let dl = ((xl - px).powi(2) + (zl - pz).powi(2)).sqrt();
            if di <= dl {
                return 1.0;
            } else {
                return 0.0;
            }
        }

        let mut sin_term = rp * (phi_p - phi_l).sin() / di;
        if sin_term.abs() > 1.0 {
            sin_term = sin_term.clamp(-1.0, 1.0);
        }

        let beta_angle = match sin_term.asin() {
            angle if angle.is_nan() => return 0.5,
            angle => angle,
        };

        let alpha_0 = beta_angle - angle_diff;
        let alpha_1 = PI - angle_diff - beta_angle;

        let sin_angle_diff = angle_diff.sin();
        if sin_angle_diff.abs() < 1e-10 {
            return 0.5;
        }

        let r0 = di * alpha_0.sin() / sin_angle_diff;
        let r1 = di * alpha_1.sin() / sin_angle_diff;

        let ring_l = match self.get_ring(rl) {
            Some(r) => r,
            None => {
                return 0.5;
            }
        };

        let a_k = ring_l.inner_radius;
        let b_k = ring_l.outer_radius;

        let r0_clamped = r0.clamp(a_k, b_k);
        let r1_clamped = r1.clamp(a_k, b_k);

        if r1_clamped <= r0_clamped {
            return 1.0;
        }

        let fr0 = (r0_clamped - a_k) / (b_k - a_k);
        let fr1 = (r1_clamped - a_k) / (b_k - a_k);

        let prob_farther = (fr1 - fr0) / (2.0 * PI) * 2.0 * PI;

        1.0 - prob_farther
    }

    fn closest_stronghold_likelihood(&self, r: f64, phi: f64, px: f64, pz: f64, ring: Ring) -> f64 {
        let mut product = 1.0;

        for other_ring in &self.rings {
            if other_ring.index == ring.index {
                let angle_spacing = 2.0 * PI / ring.count as f64;

                for k in 1..ring.count {
                    let phi_other_nominal = phi + (k as f64) * angle_spacing;

                    let delta_samples = self.linspace(
                        -self.max_snap_distance / ring.inner_radius,
                        self.max_snap_distance / ring.inner_radius,
                        20,
                    );

                    let mut prob_closer_sum = 0.0;
                    let mut weight_sum = 0.0;

                    for delta in delta_samples {
                        let phi_other = phi_other_nominal + delta;

                        let r_samples =
                            self.linspace(other_ring.inner_radius, other_ring.outer_radius, 10);
                        for r_other in r_samples {
                            let p_closer =
                                self.get_distance_constraint(r, phi, r_other, phi_other, px, pz);
                            let weight = self.delta(delta, ring);
                            prob_closer_sum += p_closer * weight;
                            weight_sum += weight;
                        }
                    }

                    if weight_sum > 0.0 {
                        let avg_prob = prob_closer_sum / weight_sum;
                        product *= avg_prob;
                    }
                }
            } else {
                for _ in 0..5 {
                    let phi_other = rand::random::<f64>() * 2.0 * PI;
                    let r_other = rand::random::<f64>()
                        * (other_ring.outer_radius - other_ring.inner_radius)
                        + other_ring.inner_radius;

                    let p_closer = self.get_distance_constraint(r, phi, r_other, phi_other, px, pz);
                    product *= p_closer;
                }
            }
        }

        product
    }

    fn linspace(&self, start: f64, end: f64, num: usize) -> Vec<f64> {
        if num <= 1 {
            return vec![start];
        }
        let step = (end - start) / (num - 1) as f64;
        (0..num).map(|i| start + step * i as f64).collect()
    }

    fn calculate_prior(&self, chunk_x: i32, chunk_z: i32) -> f64 {
        let bx = chunk_x * 16 + 8;
        let bz = chunk_z * 16 + 8;
        let distance = ((bx as f64).powi(2) + (bz as f64).powi(2)).sqrt();

        let ring = match self.get_ring(distance) {
            Some(r) => r,
            None => {
                return 1e-20;
            }
        };

        let _ring_area = PI * (ring.outer_radius.powi(2) - ring.inner_radius.powi(2));

        let avg_radius = (ring.inner_radius + ring.outer_radius) / 2.0;
        let ring_circumference = 2.0 * PI * avg_radius;
        let ring_width = ring.outer_radius - ring.inner_radius;
        let approx_chunks_in_ring = (ring_circumference * ring_width) / (16.0 * 16.0);

        let prior = ring.count as f64 / approx_chunks_in_ring.max(1.0);

        prior
    }

    fn calculate_measurement_likelihood(&self, chunk_x: i32, chunk_z: i32) -> f64 {
        if self.measurements.is_empty() {
            return 1.0;
        }

        let bx = chunk_x * 16 + 8;
        let bz = chunk_z * 16 + 8;

        let mut log_likelihood: f64 = 0.0;

        for m in &self.measurements {
            let true_angle = self.to_point_angle(m.x, m.z, bx as f64, bz as f64);

            let mut diff = m.yaw as f64 - true_angle;
            while diff > 180.0 {
                diff -= 360.0;
            }
            while diff < -180.0 {
                diff += 360.0;
            }

            log_likelihood +=
                -0.5 * (diff / self.sigma).powi(2) - (self.sigma * (2.0 * PI).sqrt()).ln();
        }

        log_likelihood.exp()
    }

    fn calculate_posterior(&self, chunk_x: i32, chunk_z: i32, use_closest_constraint: bool) -> f64 {
        let prior = self.calculate_prior(chunk_x, chunk_z);
        if prior < 1e-20 {
            return 0.0;
        }

        let meas_likelihood = self.calculate_measurement_likelihood(chunk_x, chunk_z);
        if meas_likelihood < 1e-300 {
            return 0.0;
        }

        let mut posterior = prior * meas_likelihood;

        if use_closest_constraint && !self.measurements.is_empty() {
            let bx = (chunk_x * 16 + 8) as f64;
            let bz = (chunk_z * 16 + 8) as f64;
            let distance = (bx.powi(2) + bz.powi(2)).sqrt();
            let ring_res = self.get_ring(distance);

            if let Some(ring) = ring_res {
                let (r, phi) = self.cart_to_polar(bx, bz);
                let closest_factor = self.closest_stronghold_likelihood(
                    r,
                    phi,
                    self.measurements[0].x,
                    self.measurements[0].z,
                    ring,
                );

                posterior *= closest_factor;
            }
        }

        posterior
    }

    pub fn find_stronghold(
        &self,
        search_radius: i32,
        grid_resolution: i32,
        use_closest_constraint: bool,
    ) -> Option<Prediction> {
        if self.measurements.is_empty() {
            return None;
        }

        let mut best_chunk: Option<(i32, i32)> = None;
        let mut best_posterior = 0.0;
        let mut posteriors: std::collections::HashMap<(i32, i32), f64> =
            std::collections::HashMap::new();

        // search grid
        for cx in ((-search_radius)..search_radius).step_by(grid_resolution as usize) {
            for cz in ((-search_radius)..search_radius).step_by(grid_resolution as usize) {
                // quick distance check
                let dist = (((cx * 16 + 8) as f64).powi(2) + ((cz * 16 + 8) as f64).powi(2)).sqrt();
                if dist < 1280.0 || dist > 24320.0 {
                    continue;
                }

                // calc posterior
                let posterior = self.calculate_posterior(cx, cz, use_closest_constraint);

                if posterior > 0.0 {
                    posteriors.insert((cx, cz), posterior);

                    if posterior > best_posterior {
                        best_posterior = posterior;
                        best_chunk = Some((cx, cz));
                    }
                }
            }
        }

        if best_chunk.is_none() || posteriors.is_empty() {
            return None;
        }

        // normalize
        let total: f64 = posteriors.values().sum();
        let (bx, bz) = best_chunk.unwrap();
        let normalized_prob = if total > 0.0 {
            posteriors[&(bx, bz)] / total
        } else {
            0.0
        };

        Some(Prediction {
            x: (bx * 16 + 8) as f64,
            z: (bz * 16 + 8) as f64,
            confidence: normalized_prob,
        })
    }
}
