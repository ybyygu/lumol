// Lumol, an extensible molecular simulation engine
// Copyright (C) Lumol's contributors — BSD license
use lumol::sim::{Minimization, MolecularDynamics, MonteCarlo, Propagator};

use super::Input;
use {FromToml, FromTomlWithData};
use error::{Error, Result};
use extract;

impl Input {
    /// Get the the simulation propagator.
    pub(crate) fn read_propagator(&self) -> Result<Box<Propagator>> {
        let config = self.simulation_table()?;
        let propagator = extract::table("propagator", config, "simulation")?;
        match extract::typ(propagator, "propagator")? {
            "MolecularDynamics" => Ok(Box::new(MolecularDynamics::from_toml(propagator)?)),
            "MonteCarlo" => Ok(Box::new(MonteCarlo::from_toml(propagator, self.path.clone())?)),
            "Minimization" => Ok(Box::new(Minimization::from_toml(propagator)?)),
            other => Err(Error::from(format!("Unknown propagator type '{}'", other))),
        }
    }
}
