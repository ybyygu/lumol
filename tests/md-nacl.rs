// Lumol, an extensible molecular simulation engine
// Copyright (C) Lumol's contributors — BSD license

//! Testing physical properties of a sodium chloride crystal
extern crate env_logger;
extern crate lumol;
extern crate lumol_input as input;

use std::sync::{Once, ONCE_INIT};
pub static START: Once = ONCE_INIT;

mod utils;

mod wolf {
    use START;
    use input::Input;
    use lumol::units;
    use std::path::Path;

    #[test]
    fn constant_energy() {
        START.call_once(::env_logger::init);
        let path = Path::new(file!()).parent()
                                     .unwrap()
                                     .join("data")
                                     .join("md-nacl")
                                     .join("nve-wolf-small.toml");
        let mut config = Input::new(path).unwrap().read().unwrap();

        let e_initial = config.system.total_energy();
        config.simulation.run(&mut config.system, config.nsteps);

        let e_final = config.system.total_energy();
        assert!(f64::abs((e_initial - e_final) / e_final) < 1e-4);
    }

    #[test]
    fn anisotropic_berendsen() {
        START.call_once(::env_logger::init);
        let path = Path::new(file!()).parent()
                                     .unwrap()
                                     .join("data")
                                     .join("md-nacl")
                                     .join("npt-wolf-small.toml");
        let mut config = Input::new(path).unwrap().read().unwrap();

        let collecter = ::utils::Collecter::starting_at(9000);
        let temperatures = collecter.temperatures();
        let pressures = collecter.pressures();

        config.simulation.add_output(Box::new(collecter));
        config.simulation.run(&mut config.system, config.nsteps);

        let expected = units::from(50000.0, "bar").unwrap();
        let pressure = ::utils::mean(pressures.clone());
        assert!(f64::abs(pressure - expected) / expected < 2e-3);

        let expected = units::from(273.0, "K").unwrap();
        let temperature = ::utils::mean(temperatures.clone());
        assert!(f64::abs(temperature - expected) / expected < 1e-2);
    }
}

mod ewald {
    use START;
    use input::Input;
    use std::path::Path;

    #[test]
    fn constant_energy() {
        START.call_once(::env_logger::init);
        let path = Path::new(file!()).parent()
                                     .unwrap()
                                     .join("data")
                                     .join("md-nacl")
                                     .join("nve-ewald-small.toml");
        let mut config = Input::new(path).unwrap().read().unwrap();

        let e_initial = config.system.total_energy();
        config.simulation.run(&mut config.system, config.nsteps);
        let e_final = config.system.total_energy();
        assert!(f64::abs((e_initial - e_final) / e_final) < 5e-3);
    }

    #[test]
    fn constant_energy_kspace() {
        START.call_once(::env_logger::init);
        let path = Path::new(file!()).parent()
                                     .unwrap()
                                     .join("data")
                                     .join("md-nacl")
                                     .join("nve-ewald-kspace.toml");
        let mut config = Input::new(path).unwrap().read().unwrap();

        let e_initial = config.system.total_energy();
        config.simulation.run(&mut config.system, config.nsteps);
        let e_final = config.system.total_energy();
        assert!(f64::abs((e_initial - e_final) / e_final) < 5e-3);
    }
}
