// use crate::config::EngineConfig;
// use crate::entity::object::PhysicalObject;
// use crate::entity::Entity as PhysicalObject;
// use crate::interaction::Interaction;
// use crate::interaction::InteractionVariant;
// use crate::system::discrete_field::DiscreteField;

//#[derive(Clone)]
///// System: Physical Objects
//pub struct PhysicalObjects {
//    pub entities: Vec<Box<dyn PhysicalObject>>,
//}
//impl PhysicalObjects {
//    pub fn new() -> Self {
//        let entities = vec![];
//        PhysicalObjects { entities }
//    }
//    pub fn step(&mut self, interactions: Vec<&Interaction>) {
//        for _interaction in interactions.iter() {}
//    }
//    ///// Object-Field Interaction
//    /////
//    ///// Examples:
//    ///// - charges accelerated by voltage
//    ///// - object falling to the ground
//    ///// - wall collisions
//    //pub fn interact_with_field(
//    //    &mut self,
//    //    _other: &DiscreteField,
//    //    interactions: &Vec<&Interaction>,
//    //    _config: &EngineConfig,
//    //    _self_interaction: bool,
//    //) {
//    //    for interaction in interactions.iter() {
//    //        match interaction.variant {
//    //            InteractionVariant::Force(_) => todo!(),
//    //            InteractionVariant::Collision(_) => todo!(),
//    //        }
//    //    }
//    //}
//    ///// Object-Object Interaction
//    /////
//    ///// Examples:
//    ///// - mutual gravitational attraction
//    ///// - Coulomb & Lennard-Jones
//    ///// - boid forces (avoidance, cohesion, alignment)
//    //pub fn interact_with_objects(
//    //    &mut self,
//    //    // others: &Vec<Box<dyn PhysicalObject>>,
//    //    other: &PhysicalObjects,
//    //    interactions: &Vec<&Interaction>,
//    //    _config: &EngineConfig,
//    //    _self_interaction: bool,
//    //) {
//    //    for _interaction in interactions.iter() {
//    //        let _entities = &mut self.entities; // TODO filter
//    //        let _others = &other.entities;

//    //        // let integrator = &interaction.integrator;
//    //        // match &interaction.variant {
//    //        //     InteractionVariant::Collision(_) => todo!(),
//    //        //     InteractionVariant::Force(f) => {
//    //        //         f.apply_to_objects_from_objects(entities, others, integrator, self_interaction)
//    //        //     }
//    //        // }
//    //    }
//    //}
//}
