// Lumol, an extensible molecular simulation engine
// Copyright (C) Lumol's contributors — BSD license

#[macro_use]
extern crate criterion;
extern crate rand;
extern crate lumol;

use criterion::Criterion;

use lumol::sys::EnergyCache;
use lumol::types::Vector3D;
use lumol::energy::{Ewald, SharedEwald, Wolf};
use lumol::energy::{GlobalPotential, CoulombicPotential, PairRestriction};

mod utils;

fn get_wolf() -> Wolf {
    let mut wolf = Wolf::new(9.0);
    wolf.set_restriction(PairRestriction::InterMolecular);
    return wolf;
}

fn get_ewald() -> SharedEwald {
    let mut ewald = SharedEwald::new(Ewald::new(8.0, 7, None));
    ewald.set_restriction(PairRestriction::InterMolecular);
    return ewald;
}

fn ewald_energy_computation(c: &mut Criterion) {
    let system = utils::get_system("water");
    let ewald = get_ewald();
    c.bench_function("water::ewald::energy", move |b| b.iter(|| {
        let _ = ewald.energy(&system);
    }));

    let system = utils::get_system("water");
    let ewald = get_ewald();
    c.bench_function("water::ewald::force", move |b| b.iter_with_setup(
        || vec![Vector3D::zero(); system.size()],
        |mut forces| ewald.forces(&system, &mut forces)
    ));

    let system = utils::get_system("water");
    let ewald = get_ewald();
    c.bench_function("water::ewald::atomic_virial", move |b| b.iter(|| {
        let _ = ewald.atomic_virial(&system);
    }));

    let system = utils::get_system("water");
    let ewald = get_ewald();
    c.bench_function("water::ewald::molecular_virial", move |b| b.iter(|| {
        let _ = ewald.molecular_virial(&system);
    }));
}

fn ewald_monte_carlo_cache(c: &mut Criterion) {
    let mut system = utils::get_system("water");
    system.set_coulomb_potential(Box::new(get_ewald()));
    let mut cache = EnergyCache::new();
    cache.init(&system);

    c.bench_function("water::ewald::move_molecule_cost", move |b| b.iter_with_setup(
        || utils::move_rigid_molecule(&system),
        |(molid, positions)| cache.move_molecule_cost(&system, molid, &positions)
    ));

    let mut system = utils::get_system("water");
    system.set_coulomb_potential(Box::new(get_ewald()));
    let mut cache = EnergyCache::new();
    cache.init(&system);

    c.bench_function("water::ewald::move_all_molecules_cost", move |b| b.iter_with_setup(
        || utils::move_all_rigid_molecule(&mut system),
        |system| cache.move_all_molecules_cost(&system)
    ));
}

fn wolf_energy_computation(c: &mut Criterion) {
    let system = utils::get_system("water");
    let wolf = get_wolf();
    c.bench_function("water::wolf::energy", move |b| b.iter(|| {
        let _ = wolf.energy(&system);
    }));

    let system = utils::get_system("water");
    let wolf = get_wolf();
    c.bench_function("water::wolf::force", move |b| b.iter_with_setup(
        || vec![Vector3D::zero(); system.size()],
        |mut forces| wolf.forces(&system, &mut forces)
    ));

    let system = utils::get_system("water");
    let wolf = get_wolf();
    c.bench_function("water::wolf::atomic_virial", move |b| b.iter(|| {
        let _ = wolf.atomic_virial(&system);
    }));

    let system = utils::get_system("water");
    let wolf = get_wolf();
    c.bench_function("water::wolf::molecular_virial", move |b| b.iter(|| {
        let _ = wolf.molecular_virial(&system);
    }));
}

fn wolf_monte_carlo_cache(c: &mut Criterion) {
    let mut system = utils::get_system("water");
    system.set_coulomb_potential(Box::new(get_wolf()));
    let mut cache = EnergyCache::new();
    cache.init(&system);

    c.bench_function("water::wolf::move_molecule_cost", move |b| b.iter_with_setup(
        || utils::move_rigid_molecule(&system),
        |(molid, positions)| cache.move_molecule_cost(&system, molid, &positions)
    ));

    let mut system = utils::get_system("water");
    system.set_coulomb_potential(Box::new(get_wolf()));
    let mut cache = EnergyCache::new();
    cache.init(&system);

    c.bench_function("water::wolf::move_all_molecules_cost", move |b| b.iter_with_setup(
        || utils::move_all_rigid_molecule(&mut system),
        |system| cache.move_all_molecules_cost(&system)
    ));
}

criterion_group!(ewald, ewald_energy_computation, ewald_monte_carlo_cache);
criterion_group!(wolf, wolf_energy_computation, wolf_monte_carlo_cache);

criterion_main!(ewald, wolf);
