#![allow(clippy::new_without_default)]

// Tutorial: Lead
// Destroy the enemy ship. Its position is given by the "target" function and velocity by the
// "target_velocity" function. Your ship is not able to accelerate in this scenario.
//
// This is where the game becomes challenging! You'll need to lead the target
// by firing towards where the target will be by the time the bullet gets there.
//
// Hint: target() + target_velocity() * t gives the position of the target after t seconds.
//
// You can scale a vector by a number: vec2(a, b) * c == vec2(a * c, b * c)
//
// p.s. You can change your username by clicking on it at the top of the page.
use oort_api::prelude::*;

const BULLET_SPEED: f64 = 1000.0; // m/s

pub struct Ship {}

impl Ship {
    pub fn new() -> Ship {
        Ship {}
    }

    // Need bullet and target to intersect position at the same time at the same time.
    // pos_bullet_at_t = pos_target_at_t
    // pos_ship + vel_bullet * t = pos_target + vel_target * t
    // pos_ship + BS * [cos(heading), sin(heading)] * t = pos_target + vel_target * t
    // [cos(heading), sin(heading)] = (pos_target - pos_ship + vel_target * t) / (BS * t)
    // [cos(heading), sin(heading)] = (pos_target - pos_ship + vel_target * t) / (BS * t)
    // cos(heading) = (pos_target[0] - pos_ship[0] + vel_target[0] * t) / (BS * t)
    // sin(heading) = (pos_target[1] - pos_ship[1] + vel_target[1] * t) / (BS * t)
    // 1 = ((pos_target[0] - pos_ship[0] + vel_target[0] * t) / (BS * t))^2 + ((pos_target[1] - pos_ship[1] + vel_target[1] * t) / (BS * t))^2
    // 1 = ((A0 + vel_target[0] * t) / (BS * t))^2 + ((A1 + vel_target[1] * t) / (BS * t))^2
    // 1 = (A0^2 + 2 * A0 * vel_target[0] * t + (vel_target[0] * t)^2) / (BS * t)^2 + (A1^2 + 2 * A1 * vel_target[1] * t + (vel_target[1] * t)^2) / (BS * t)^2
    // (BS * t)^2 = A0^2 + 2 * A0 * vel_target[0] * t + (vel_target[0] * t)^2 + A1^2 + 2 * A1 * vel_target[1] * t + (vel_target[1] * t)^2
    // (BS^2 - vel_target[0]^2 - vel_target[1]^2)t^2 - 2(A0*vel_target[0] + A1*vel_target[1])t = A0^2 + A1^2
    // (BS - vel_target[0] - vel_target[1])t^2 - 2(A0*vel_target[0] + A1*vel_target[1])t - (A0^2 + A1^2) = 0
    // a = BS - vel_target[0] - vel_target[1]
    // b = -2(A0*vel_target[0] + A1*vel_target[1])
    // c = -(A0^2 + A1^2)
    // (-b +- sqrt(4ac - b^2))/4ac = (2(A0*vel_target[0] - A1*vel_target[1]) +- sqrt(4*()))

    // acos((pos_target[0] - pos_ship[0] + vel_target[0] * t) / (BS * t)) = asin((pos_target[1] - pos_ship[1] + vel_target[1] * t) / (BS * t))
    // pi/2 - asin((pos_target[0] - pos_ship[0] + vel_target[0] * t) / (BS * t)) = asin((pos_target[1] - pos_ship[1] + vel_target[1] * t) / (BS * t))

    pub fn tick(&mut self) {
        // let orientation_ship = Vec2::new(heading().cos(), heading().sin());
        // let vel_bullet = BULLET_SPEED * orientation_ship;

        let pt = target();
        let vt = target_velocity();
        let ps = position();
        // let vs = velocity();

        let dx = pt[0] - ps[0];
        let dy = pt[1] - ps[1];
        let a = vt[0].powi(2) + vt[1].powi(2) - BULLET_SPEED.powi(2);
        let b = 2.0 * (dx * vt[0] + dy * vt[1]);
        let c = dx.powi(2) + dy.powi(2);

        // let tplus = (-b + (b.powi(2) - 4.0 * a * c).sqrt()) / (2.0 * a);
        // let tminus = (-b - (b.powi(2) - 4.0 * a * c).sqrt()) / (2.0 * a);
        let t = (-b - (b.powi(2) - 4.0 * a * c).sqrt()) / (2.0 * a);
        let cos_theta = (dx + vt[0] * t) / (BULLET_SPEED * t);
        let sin_theta = (dy + vt[1] * t) / (BULLET_SPEED * t);
        let mut theta = (sin_theta).atan2(cos_theta);

        theta = (2.0 * PI + theta) % (2.0 * PI);

        // debug!("a, b, c = ({a}, {b}, {c})");
        // debug!("rooted = {}", (4.0 * a * c - b.powi(2)));
        // debug!("tplus, tminus: {tplus}, {tminus}");

        debug!("heading: {}", heading());
        debug!("target heading: {theta}");
        debug!("time to impact: {t}");

        draw_line(position(), target(), 0x00ff00);
        // turn(theta - heading());
        let max = max_angular_acceleration() * 0.5;

        let mut diff = (theta + 0.20 * theta.signum()) - heading();
        if diff.abs() > PI {
            diff -= 2.0 * PI * diff.signum()
        }

        torque((diff.clamp(-max, max) - angular_velocity()).signum() * max_angular_acceleration());
        // torque(theta - heading());
        fire(0);
    }
}
