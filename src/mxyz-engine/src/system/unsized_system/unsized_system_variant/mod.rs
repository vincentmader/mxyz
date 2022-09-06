pub mod unsized_field_variant;
pub mod unsized_object_variant;
use crate::system::sized_system::sized_system_variant::SizedSystemVariant;
use unsized_field_variant::UnsizedFieldVariant;
use unsized_object_variant::UnsizedObjectsVariant;

/// System Variant Enumeration
#[derive(Debug, Clone)]
pub enum UnsizedSystemVariant {
    EntitiesV1,
    Field,
    Objects,
    // Field(UnsizedFieldVariant),
    // Objects(UnsizedObjectsVariant),
}
/// Convert from SizedSystemVariant to UnsizedSystemVariant. (other direction not possible)
impl From<&SizedSystemVariant> for UnsizedSystemVariant {
    fn from(system_variant: &SizedSystemVariant) -> UnsizedSystemVariant {
        match system_variant {
            SizedSystemVariant::EntitiesV1(_) => UnsizedSystemVariant::EntitiesV1,
            // TODO pass system-variant-id to unsized-system-variant
            SizedSystemVariant::Field(_) => UnsizedSystemVariant::EntitiesV1, // _ => todo!(),
            SizedSystemVariant::Objects(_) => UnsizedSystemVariant::EntitiesV1, // _ => todo!(),
        }
    }
}
/// Convert from UnsizedSystemVariant to usize.
impl Into<usize> for UnsizedSystemVariant {
    fn into(self) -> usize {
        match self {
            UnsizedSystemVariant::EntitiesV1 => 0,
            UnsizedSystemVariant::Field => 1,
            UnsizedSystemVariant::Objects => 2,
            // _ => todo!("Conversion: SystemVariant -> usize"),
        }
    }
}
/// Convert from usize to UnsizedSystemVariant.
impl From<usize> for UnsizedSystemVariant {
    fn from(system_variant: usize) -> UnsizedSystemVariant {
        match system_variant {
            0 => UnsizedSystemVariant::EntitiesV1,
            1 => UnsizedSystemVariant::Field,
            2 => UnsizedSystemVariant::Objects,
            _ => todo!("Conversion: usize -> SystemVariant"),
        }
    }
}
