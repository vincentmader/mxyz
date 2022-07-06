// use crate::config::EngineConfig;
// use crate::integrator::Integrator;
// use crate::state::State;
// use crate::system::System;

// pub fn get_interactions(sys_id: usize, sys_jd: usize, config: &EngineConfig) -> Vec<&Interaction> {
//     config
//         .interactions
//         .iter()
//         .filter(|interaction| {
//             match interaction
//                 .matrix
//                 .rows
//                 .get(sys_id)
//                 .expect(&format!(
//                     "System-ID \"{}\" not found in interaction matrix",
//                     sys_id
//                 ))
//                 .entries
//                 .get(sys_jd)
//                 .expect(&format!(
//                     "System-JD \"{}\" not found in interaction matrix",
//                     sys_jd
//                 ))
//                 .active
//             {
//                 Some(active) => active, // filter out all entries set to `false`
//                 None => false,          // also disregard entries set to `None`
//             }
//         })
//         .collect()
// }
