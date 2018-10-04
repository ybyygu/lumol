initSidebarItems({"enum":[["TemperatureStrategy","Possible temperature computation strategies. Different propagators needs different ways to compute the temperature: Monte Carlo temperature is a constant of the simulation, whereas for molecular dynamics we use the instantaneous velocities."]],"mod":[["mc","Monte Carlo Metropolis algorithms"],["md","Molecular dynamics algorithms."],["min","Energy minimization algorithms"],["output","Saving properties of a system during a simulation"]],"struct":[["BoltzmannVelocities","Initialize the velocities from a Boltzmann distribution."],["Simulation","The Simulation struct holds all the needed algorithms for running the simulation. It should be use together with a `System` to perform the simulation."],["UniformVelocities","Initialize the velocities from an uniform distribution."]],"trait":[["InitVelocities","A method to initialize the velocities of a system."],["Propagator","The propagator trait is the main algorithm of a simulation, i.e. the one which update the system. The main function here is `propagate`, which should propagate the simulation for one step."]]});