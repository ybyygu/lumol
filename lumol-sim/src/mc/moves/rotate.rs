// Lumol, an extensible molecular simulation engine
// Copyright (C) 2015-2016 G. Fraux — BSD license
use rand::RngCore;
use rand::distributions::{Normal, Range, Distribution};

use std::collections::BTreeSet;
use std::f64;
use std::usize;

use super::{MCDegreeOfFreedom, MCMove};
use super::select_molecule;

use core::{EnergyCache, System, MoleculeHash, Matrix3, Vector3D};

/// Monte Carlo move for rotating a rigid molecule
pub struct Rotate {
    /// Hash of molecule to rotate. `None` means all molecules.
    hash: Option<MoleculeHash>,
    /// Index of the molecule to rotate
    molid: usize,
    /// New positions of the atom in the rotated molecule
    newpos: Vec<Vector3D>,
    /// Normal distribution, for generation of the axis
    axis_rng: Normal,
    /// Maximum values for the range of the range distribution of the angle
    theta: f64,
    /// Range distribution, for generation of the angle
    range: Range<f64>,
}

impl Rotate {
    /// Create a new `Rotate` move, with maximum angular displacement of
    /// `theta`. This move will apply to the molecules with the given `hash`,
    /// or all molecules if `hash` is `None`.
    pub fn new<H: Into<Option<MoleculeHash>>>(theta: f64, hash: H) -> Rotate {
        assert!(theta > 0.0, "theta must be positive in Rotate move");
        Rotate {
            hash: hash.into(),
            molid: usize::MAX,
            newpos: Vec::new(),
            axis_rng: Normal::new(0.0, 1.0),
            theta: theta,
            range: Range::new(-theta, theta),
        }
    }
}

impl MCMove for Rotate {
    fn describe(&self) -> &str {
        "molecular rotation"
    }

    fn degrees_of_freedom(&self) -> MCDegreeOfFreedom {
        match self.hash {
            Some(hash) => {
                let mut all = BTreeSet::new();
                let _ = all.insert(hash);
                MCDegreeOfFreedom::Molecules(all)
            }
            None => MCDegreeOfFreedom::AllMolecules,
        }
    }

    fn setup(&mut self, _: &System) {}

    fn prepare(&mut self, system: &mut System, rng: &mut RngCore) -> bool {
        if let Some(id) = select_molecule(system, self.hash, rng) {
            self.molid = id;
        } else {
            warn!("Can not rotate molecule: no molecule of this type in the system.");
            return false;
        }

        // Getting values from a 3D normal distribution gives an uniform
        // distribution on the unit sphere.
        let axis = Vector3D::new(
            self.axis_rng.sample(rng),
            self.axis_rng.sample(rng),
            self.axis_rng.sample(rng),
        ).normalized();
        let theta = self.range.sample(rng);

        // store positions of selected molecule
        self.newpos = system.molecule(self.molid).particles().position.to_vec();
        // get center-of-mass of molecule
        let com = system.molecule(self.molid).center_of_mass();
        rotate_around_axis(&mut self.newpos, com, axis, theta);
        true
    }

    fn cost(&self, system: &System, beta: f64, cache: &mut EnergyCache) -> f64 {
        return beta * cache.move_molecule_cost(system, self.molid, &self.newpos);
    }

    fn apply(&mut self, system: &mut System) {
        let mut molecule = system.molecule_mut(self.molid);
        for (position, newpos) in soa_zip!(molecule.particles_mut(), [mut position], &self.newpos) {
            *position = *newpos;
        }
    }

    fn restore(&mut self, _: &mut System) {
        // Nothing to do
    }

    fn update_amplitude(&mut self, scaling_factor: Option<f64>) {
        if let Some(s) = scaling_factor {
            if (s * self.theta).abs().to_degrees() <= 180.0 {
                self.theta *= s;
                self.range = Range::new(-self.theta, self.theta);
            } else {
                warn_once!(
                    "Tried to increase the maximum amplitude for rotations to more than 180°."
                );
            }
        }
    }
}

/// Rotate the particles at `positions` with the center-of-mass position
/// `com` around the `axis` axis by `angle`. The `positions` array is
/// overwritten with the new positions.
fn rotate_around_axis(positions: &mut [Vector3D], com: Vector3D, axis: Vector3D, angle: f64) {
    let rotation = Matrix3::rotation(&axis, angle);
    for position in positions {
        let oldpos = *position - com;
        *position = com + rotation * oldpos;
    }
}
