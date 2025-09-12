use std::fmt::{Display, Formatter};
use clap::ValueEnum;
use strum::EnumIter;

#[derive(Clone, Copy, Debug, ValueEnum, EnumIter, PartialEq, Eq)]
pub enum Target { Vectrex, Pitrex, Vecfever, Vextreme, All }

impl Display for Target { fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result { write!(f, "{}", match self { Target::Vectrex=>"vectrex", Target::Pitrex=>"pitrex", Target::Vecfever=>"vecfever", Target::Vextreme=>"vextreme", Target::All=>"all" }) } }

pub enum CpuArch { M6809, ARM, CortexM }

pub struct TargetInfo { pub name: &'static str, pub origin: &'static str, #[allow(dead_code)] pub init_label: &'static str, #[allow(dead_code)] pub line_routine: &'static str, pub arch: CpuArch }

pub fn info(t: Target) -> TargetInfo {
    match t {
    Target::Vectrex => TargetInfo { name: "Vectrex", origin: "$0000", init_label: "INIT_ENGINE", line_routine: "line", arch: CpuArch::M6809 },
    Target::Pitrex  => TargetInfo { name: "Pitrex", origin: "$0400", init_label: "PITREX_INIT", line_routine: "pitrex_line", arch: CpuArch::ARM },
    Target::Vecfever=> TargetInfo { name: "VecFever", origin: "$8000", init_label: "VF_INIT", line_routine: "vf_line", arch: CpuArch::CortexM },
    Target::Vextreme=> TargetInfo { name: "Vextreme", origin: "$6000", init_label: "VX_INIT", line_routine: "vx_line", arch: CpuArch::CortexM },
        Target::All => panic!("'All' is aggregate target and has no direct info")
    }
}

pub fn concrete_targets() -> &'static [Target] { &[Target::Vectrex, Target::Pitrex, Target::Vecfever, Target::Vextreme] }
