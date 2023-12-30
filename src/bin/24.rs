use nalgebra::{Matrix2, Matrix2x1, Matrix4, Matrix4x1, RowVector2, RowVector4};
use nom::{
    bytes::complete::tag,
    character::complete::{self, newline, space1},
    multi::separated_list1,
    sequence::{preceded, separated_pair, tuple},
    IResult, Parser,
};

advent_of_code::solution!(24);

#[derive(Debug, Clone)]
struct Position {
    px: f64,
    py: f64,
    pz: f64,
}

#[derive(Debug, Clone)]
struct Velocity {
    vx: f64,
    vy: f64,
    vz: f64,
}

#[derive(Debug, Clone)]
struct Hailstone {
    position: Position,
    velocity: Velocity,
}

// const PRECISION: u32 = 32;

fn parse_measurements(input: &str) -> IResult<&str, (f64, f64, f64)> {
    let (input, x) = complete::i64(input)?;
    let (input, y) = preceded(tuple((tag(","), space1)), complete::i64)(input)?;
    let (input, z) = preceded(tuple((tag(","), space1)), complete::i64)(input)?;
    // Ok((input, (Float::with_val(PRECISION, x), Float::with_val(PRECISION, y), Float::with_val(PRECISION, z))))
    Ok((input, (x as f64, y as f64, z as f64)))
}

fn parse_hailstone(input: &str) -> IResult<&str, (Position, Velocity)> {
    separated_pair(
        parse_measurements.map(|(px, py, pz)| Position { px, py, pz }),
        tuple((tag(" @"), space1)),
        parse_measurements.map(|(vx, vy, vz)| Velocity { vx, vy, vz }),
    )(input)
}

fn parse_input(input: &str) -> IResult<&str, Vec<Hailstone>> {
    separated_list1(
        newline,
        parse_hailstone.map(|(position, velocity)| Hailstone { position, velocity }),
    )(input)
}

fn check_collision(
    hs_a: &Hailstone,
    hs_b: &Hailstone,
    lower_bound: &f64,
    upper_bound: &f64,
) -> bool {
    let Hailstone {
        position: p_a,
        velocity: v_a,
    } = hs_a;
    let Hailstone {
        position: p_b,
        velocity: v_b,
    } = hs_b;

    let m_a = v_a.vy / v_a.vx;
    // let m_a = Float::with_val(PRECISION, &v_a.vy / &v_a.vx);
    let m_b = v_b.vy / v_b.vx;
    // let m_b = Float::with_val(PRECISION, &v_b.vy / &v_b.vx);

    if m_a == m_b {
        return false;
    }

    let b_a = p_a.py - m_a * p_a.px;
    // let b_a = Float::with_val(PRECISION, &p_a.py - Float::with_val(PRECISION, &m_a * &p_a.px));
    let b_b = p_b.py - m_b * p_b.px;
    // let b_b = Float::with_val(PRECISION, &p_b.py - Float::with_val(PRECISION, &m_b * &p_b.px));

    let x = (b_b - b_a) / (m_a - m_b);
    // let x = Float::with_val(PRECISION, Float::with_val(PRECISION, &b_b - &b_a) / Float::with_val(PRECISION, &m_a - &m_b));
    let y = m_a * x + b_a;
    // let y = Float::with_val(PRECISION, Float::with_val(PRECISION, &m_a * &x) + &b_a);

    let t_a = (x - p_a.px) / v_a.vx;
    // let t_a = Float::with_val(PRECISION, Float::with_val(PRECISION, &x - &p_a.px) / &v_a.vx);
    let t_b = (x - p_b.px) / v_b.vx;
    // let t_b = Float::with_val(PRECISION, Float::with_val(PRECISION, &x - &p_b.px) / &v_b.vx);

    (t_a >= 0.0 && t_b >= 0.0)
        && (lower_bound <= &x && &x <= upper_bound)
        && (lower_bound <= &y && &y <= upper_bound)
}

pub fn part_one(input: &str) -> Option<u32> {
    let (_, hailstones) = parse_input(input).unwrap();

    // let lower_bound = Float::with_val(PRECISION, 200000000000000.0);
    // let upper_bound = Float::with_val(PRECISION, 400000000000000.0);
    let lower_bound = 200000000000000.0;
    let upper_bound = 400000000000000.0;
    // let lower_bound = 7.0;
    // let upper_bound = 27.0;

    let mut collisions = 0;
    for i in 0..hailstones.len() {
        for j in (i + 1)..hailstones.len() {
            if check_collision(&hailstones[i], &hailstones[j], &lower_bound, &upper_bound) {
                collisions += 1;
            }
        }
    }
    Some(collisions)
}

// Gives the row of A matrix for x, y, vx, vy, and b
fn xy_row(hs_0: &Hailstone, hs_1: &Hailstone) -> (RowVector4<f64>, f64) {
    let Hailstone {
        position: Position {
            px: px_0, py: py_0, ..
        },
        velocity: Velocity {
            vx: vx_0, vy: vy_0, ..
        },
    } = hs_0;
    let Hailstone {
        position: Position {
            px: px_1, py: py_1, ..
        },
        velocity: Velocity {
            vx: vx_1, vy: vy_1, ..
        },
    } = hs_1;
    (
        RowVector4::new(vy_1 - vy_0, vx_0 - vx_1, py_0 - py_1, px_1 - px_0),
        (*vx_0 as i128 * *py_0 as i128 - *vx_1 as i128 * *py_1 as i128
            + *px_1 as i128 * *vy_1 as i128
            - *px_0 as i128 * *vy_0 as i128) as f64,
    )
}

// Gives the row of A matrix for z and vz, and b
fn z_row(hs_0: &Hailstone, hs_1: &Hailstone, x_pos: f64, x_vel: f64) -> (RowVector2<f64>, f64) {
    let Hailstone {
        position: Position {
            px: px_0, pz: pz_0, ..
        },
        velocity: Velocity {
            vx: vx_0, vz: vz_0, ..
        },
    } = hs_0;
    let Hailstone {
        position: Position {
            px: px_1, pz: pz_1, ..
        },
        velocity: Velocity {
            vx: vx_1, vz: vz_1, ..
        },
    } = hs_1;
    (
        RowVector2::new(vx_0 - vx_1, px_1 - px_0),
        (*vx_0 as i128 * *pz_0 as i128 - *vx_1 as i128 * *pz_1 as i128
            + *px_1 as i128 * *vz_1 as i128
            - *px_0 as i128 * *vz_0 as i128
            - (pz_0 - pz_1) as i128 * x_vel as i128
            - (vz_1 - vz_0) as i128 * x_pos as i128) as f64,
    )
}

pub fn part_two(input: &str) -> Option<f64> {
    let (_, hailstones) = parse_input(input).unwrap();

    let hs_a = &hailstones[0];
    let hs_b = &hailstones[1];
    let hs_c = &hailstones[2];
    let hs_d = &hailstones[3];
    let hs_e = &hailstones[4];

    let (ab_xy_a, ab_xy_b) = xy_row(hs_a, hs_b);
    let (bc_xy_a, bc_xy_b) = xy_row(hs_b, hs_c);
    let (cd_xy_a, cd_xy_b) = xy_row(hs_c, hs_d);
    let (de_xy_a, de_xy_b) = xy_row(hs_d, hs_e);

    let a_xy_matrix = Matrix4::from_rows(&[ab_xy_a, bc_xy_a, cd_xy_a, de_xy_a]);
    let b_xy_vec = Matrix4x1::from([ab_xy_b, bc_xy_b, cd_xy_b, de_xy_b]);

    // let a_xy_inverse = a_xy_matrix.try_inverse().unwrap();
    // let x_xy_solution = a_xy_inverse * b_xy_vec;
    // let x_xy_solution = a_xy_matrix.lu().solve(&b_xy_vec).unwrap();
    // let x_xy_solution = a_xy_matrix.qr().solve(&b_xy_vec).unwrap();
    // let x_xy_solution = a_xy_matrix.full_piv_lu().solve(&b_xy_vec).unwrap();
    let x_xy_solution = a_xy_matrix.svd(true, true).solve(&b_xy_vec, 0.0).unwrap();
    let (xp, yp, xv, _) = (
        x_xy_solution.x,
        x_xy_solution.y,
        x_xy_solution.z,
        x_xy_solution.w,
    );

    let (ab_z_a, ab_z_b) = z_row(hs_a, hs_b, xp, xv);
    let (bc_z_a, bc_z_b) = z_row(hs_b, hs_c, xp, xv);

    let a_z_matrix = Matrix2::from_rows(&[ab_z_a, bc_z_a]);
    let b_z_vec = Matrix2x1::from([ab_z_b, bc_z_b]);

    // let a_z_inverse = a_z_matrix.try_inverse().unwrap();
    // let x_z_solution = a_z_inverse * b_z_vec;
    // let x_z_solution = a_z_matrix.lu().solve(&b_z_vec).unwrap();
    // let x_z_solution = a_z_matrix.qr().solve(&b_z_vec).unwrap();
    // let x_z_solution = a_z_matrix.full_piv_lu().solve(&b_z_vec).unwrap();
    let x_z_solution = a_z_matrix.svd(true, true).solve(&b_z_vec, 0.0).unwrap();
    let (zp, _) = (x_z_solution.x, x_z_solution.y);

    // 786_617_045_860_269.3 from A inverse : too high
    // 786_617_045_860_272.6 from LU decomp with partial pivoting : too high
    // 786_617_045_860_268.6 from QR decomp : forgot
    // 786_617_045_860_268   from LU with full pivoting : forgot
    // 786_579_841_992_401.1 from SVD : went over submission limit to give high/low
    // 786_546_539_760_670

    dbg!(&xp);
    dbg!(&yp);
    dbg!(&zp);
    Some(xp + yp + zp)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(2));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(47.0));
    }
}
