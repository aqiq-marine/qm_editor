use serde::{Deserialize, Serialize};
use specta::Type;
use std::collections::{HashMap, HashSet};

use crate::domain::{atomic_number, Molecule};
use chematic_core::{
    Atom as SchematicAtom, AtomIdx, BondOrder, Element as SchematicElement, MoleculeBuilder,
};
use chematic_ff::{assign_uff_types, uff_total_energy};

#[derive(Clone, Debug, Serialize, Deserialize, Type)]
#[serde(rename_all = "camelCase")]
pub struct GeometryOptimizeRequest {
    pub molecule: Molecule,
    pub frozen_atom_ids: Vec<u32>,
}

#[derive(Clone, Debug, Serialize, Deserialize, Type)]
#[serde(rename_all = "camelCase")]
pub struct GeometryOptimizeResult {
    pub molecule: Molecule,
    pub frozen_atom_ids: Vec<u32>,
    pub iteration_count: u32,
    pub converged: bool,
    pub energy_kcal_mol: f64,
    pub warnings: Vec<String>,
}

/// Runs Schematic's UFF energy model with fixed-coordinate constraints.
pub fn optimize_molecule(
    request: GeometryOptimizeRequest,
) -> Result<GeometryOptimizeResult, String> {
    let frozen: HashSet<u32> = request.frozen_atom_ids.iter().copied().collect();
    if request.molecule.atoms.is_empty() {
        return Err("Cannot optimize an empty molecule.".to_string());
    }

    if let Some(atom_id) = frozen
        .iter()
        .find(|id| !request.molecule.atoms.iter().any(|atom| atom.id == **id))
    {
        return Err(format!("Frozen atom {atom_id} does not exist."));
    }

    let schematic = to_schematic_molecule(&request.molecule)?;
    let types = assign_uff_types(&schematic);
    let mut coords: Vec<[f64; 3]> = request
        .molecule
        .atoms
        .iter()
        .map(|atom| atom.position)
        .collect();
    let initial_coords = coords.clone();
    let free: Vec<bool> = request
        .molecule
        .atoms
        .iter()
        .map(|atom| !frozen.contains(&atom.id))
        .collect();

    let mut step = 0.05_f64;
    let mut previous_energy = f64::INFINITY;
    let mut converged = false;
    let mut iteration_count = 0;

    for iteration in 0..200 {
        let energy = uff_total_energy(&schematic, &types, &coords);
        let gradient = numerical_gradient(&schematic, &types, &coords, &free);
        let rms = rms_gradient(&gradient, &free);

        if rms < 0.01 {
            converged = true;
            iteration_count = iteration;
            break;
        }

        let mut candidate = coords.clone();
        for (index, is_free) in free.iter().copied().enumerate() {
            if is_free {
                candidate[index] = subtract_scaled(candidate[index], gradient[index], step);
            }
        }

        let candidate_energy = uff_total_energy(&schematic, &types, &candidate);
        if candidate_energy < energy {
            coords = candidate;
            if (energy - candidate_energy) < previous_energy * 1e-7 {
                step *= 1.2;
            }
            previous_energy = energy;
        } else {
            step *= 0.5;
            if step < 1e-8 {
                iteration_count = iteration;
                break;
            }
        }
        iteration_count = iteration + 1;
    }

    let final_energy = uff_total_energy(&schematic, &types, &coords);
    let mut optimized = request.molecule;
    for (index, atom) in optimized.atoms.iter_mut().enumerate() {
        if free[index] {
            atom.position = coords[index];
        } else {
            debug_assert_eq!(atom.position, initial_coords[index]);
        }
    }

    Ok(GeometryOptimizeResult {
        molecule: optimized,
        frozen_atom_ids: request.frozen_atom_ids,
        iteration_count,
        converged,
        energy_kcal_mol: final_energy,
        warnings: vec![
            "UFF minimization is provided by Schematic; fixed atoms are held by the QM Editor adapter."
                .to_string(),
        ],
    })
}

fn to_schematic_molecule(molecule: &Molecule) -> Result<chematic_core::Molecule, String> {
    let mut builder = MoleculeBuilder::new();
    let mut indices = HashMap::new();
    for atom in &molecule.atoms {
        let element = SchematicElement::from_atomic_number(atomic_number(atom.element) as u8)
            .ok_or_else(|| format!("Unsupported element for atom {}.", atom.id))?;
        let mut schematic_atom = SchematicAtom::new(element);
        schematic_atom.charge = atom.formal_charge.clamp(i8::MIN as i32, i8::MAX as i32) as i8;
        schematic_atom.isotope = atom.isotope.map(|isotope| isotope.0);
        let index = builder.add_atom(schematic_atom);
        indices.insert(atom.id, index);
    }

    for bond in &molecule.bonds {
        let order = match bond.order {
            1 => BondOrder::Single,
            2 => BondOrder::Double,
            3 => BondOrder::Triple,
            _ => return Err(format!("Unsupported bond order {}.", bond.order)),
        };
        let first = indices
            .get(&bond.atom_ids[0])
            .copied()
            .ok_or_else(|| format!("Bond references missing atom {}.", bond.atom_ids[0]))?;
        let second = indices
            .get(&bond.atom_ids[1])
            .copied()
            .ok_or_else(|| format!("Bond references missing atom {}.", bond.atom_ids[1]))?;
        builder
            .add_bond(first, second, order)
            .map_err(|error| format!("Failed to build Schematic bond: {error}"))?;
    }

    Ok(builder.build())
}

fn numerical_gradient(
    molecule: &chematic_core::Molecule,
    types: &[(AtomIdx, chematic_ff::UffType)],
    coords: &[[f64; 3]],
    free: &[bool],
) -> Vec<[f64; 3]> {
    const DELTA: f64 = 1e-4;
    let mut gradient = vec![[0.0; 3]; coords.len()];
    let mut perturbed = coords.to_vec();

    for (index, is_free) in free.iter().copied().enumerate() {
        if !is_free {
            continue;
        }
        for axis in 0..3 {
            perturbed[index][axis] += DELTA;
            let plus = uff_total_energy(molecule, types, &perturbed);
            perturbed[index][axis] -= 2.0 * DELTA;
            let minus = uff_total_energy(molecule, types, &perturbed);
            perturbed[index][axis] += DELTA;
            gradient[index][axis] = (plus - minus) / (2.0 * DELTA);
        }
    }

    gradient
}

fn rms_gradient(gradient: &[[f64; 3]], free: &[bool]) -> f64 {
    let values = gradient
        .iter()
        .zip(free)
        .filter(|(_, is_free)| **is_free)
        .flat_map(|(vector, _)| vector.iter())
        .map(|value| value * value)
        .sum::<f64>();
    let count = free.iter().filter(|is_free| **is_free).count() * 3;
    if count == 0 {
        0.0
    } else {
        (values / count as f64).sqrt()
    }
}

fn subtract_scaled(position: [f64; 3], gradient: [f64; 3], step: f64) -> [f64; 3] {
    [
        position[0] - step * gradient[0],
        position[1] - step * gradient[1],
        position[2] - step * gradient[2],
    ]
}
