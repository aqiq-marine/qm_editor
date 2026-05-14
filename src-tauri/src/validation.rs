use super::domain::{ChemicalSpec, Element, ValidationLevel, ValidationMessage};

pub fn validate_chemical_spec(spec: &ChemicalSpec) -> Vec<ValidationMessage> {
    let mut messages = Vec::new();
    let molecule = &spec.molecule;
    let calculation = &spec.calculation;

    if molecule.atoms.is_empty() {
        messages.push(error("Molecule must contain at least one atom."));
    }

    if calculation.multiplicity < 1 {
        messages.push(error("Multiplicity must be a positive integer."));
    }

    let charge_parity = calculation.charge.unsigned_abs() % 2;
    let electron_parity = molecule.atoms.iter().fold(charge_parity, |parity, atom| {
        (parity + valence_parity(atom.element)) % 2
    });
    let unpaired_parity = (calculation.multiplicity - 1) % 2;
    if !molecule.atoms.is_empty() && electron_parity != unpaired_parity {
        messages.push(warning(
            "Charge and multiplicity look inconsistent for common valence parity.",
        ));
    }

    messages
}

fn error(message: &str) -> ValidationMessage {
    ValidationMessage {
        level: ValidationLevel::Error,
        message: message.to_string(),
    }
}

fn warning(message: &str) -> ValidationMessage {
    ValidationMessage {
        level: ValidationLevel::Warning,
        message: message.to_string(),
    }
}

fn valence_parity(element: Element) -> u32 {
    match element {
        Element::H
        | Element::B
        | Element::N
        | Element::F
        | Element::P
        | Element::Cl
        | Element::Br
        | Element::I => 1,
        _ => 0,
    }
}
