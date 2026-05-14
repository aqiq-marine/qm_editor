use crate::domain::{Basis, ChemicalSpec, JobType, Method, Solvent};
use crate::parser::{element_symbol, safe_name};

pub fn render_gaussian(spec: &ChemicalSpec) -> String {
    let calculation = &spec.calculation;
    let molecule = &spec.molecule;
    let mut route = vec![
        route_job(calculation.job_type).to_string(),
        format!(
            "{}/{}",
            method_name(calculation.method),
            basis_name(calculation.basis)
        ),
    ];
    if let Some(solvent) = calculation.solvent {
        route.push(format!("SCRF=(Solvent={})", solvent_name(solvent)));
    }

    let coordinates = molecule
        .atoms
        .iter()
        .map(|atom| {
            format!(
                "{:<2} {:>12.6} {:>12.6} {:>12.6}",
                element_symbol(atom.element),
                atom.position[0],
                atom.position[1],
                atom.position[2]
            )
        })
        .collect::<Vec<_>>()
        .join("\n");

    format!(
        "%chk={}.chk\n# {}\n\n{}\n\n{} {}\n{}\n\n",
        safe_name(&molecule.name),
        route.join(" "),
        if molecule.name.is_empty() {
            "Gaussian input"
        } else {
            &molecule.name
        },
        calculation.charge,
        calculation.multiplicity,
        coordinates
    )
}

fn route_job(job_type: JobType) -> &'static str {
    match job_type {
        JobType::Opt => "Opt",
        JobType::Freq => "Freq",
        JobType::OptFreq => "Opt Freq",
        JobType::Ts => "Opt=(TS,CalcFC,NoEigenTest)",
    }
}

pub fn method_name(method: Method) -> &'static str {
    match method {
        Method::B3LYP => "B3LYP",
        Method::WB97XD => "WB97XD",
    }
}

fn basis_name(basis: Basis) -> &'static str {
    match basis {
        Basis::Six31Gd => "6-31G(d)",
        Basis::Def2Svp => "def2-SVP",
        Basis::Def2Tzvp => "def2-TZVP",
    }
}

fn solvent_name(solvent: Solvent) -> &'static str {
    match solvent {
        Solvent::THF => "THF",
        Solvent::Water => "Water",
    }
}
