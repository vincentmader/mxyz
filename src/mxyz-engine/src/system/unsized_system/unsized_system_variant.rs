use crate::system::sized_system::sized_system_variant::SizedSystemVariant;

/// System Variant Enumeration
#[derive(Debug, Clone)]
pub enum UnsizedSystemVariant {
    EntitiesV1,
    Field(FieldVariant),
    Objects(ObjectsVariant),
    Network(NetworkVariant),
}
impl Into<usize> for UnsizedSystemVariant {
    fn into(self) -> usize {
        match self {
            UnsizedSystemVariant::EntitiesV1 => 0,
            _ => todo!("Conversion: SystemVariant -> usize"),
        }
    }
}
impl From<usize> for UnsizedSystemVariant {
    fn from(system_variant: usize) -> UnsizedSystemVariant {
        match system_variant {
            0 => UnsizedSystemVariant::EntitiesV1,
            _ => todo!("Conversion: usize -> SystemVariant"),
        }
    }
}
impl From<&SizedSystemVariant> for UnsizedSystemVariant {
    fn from(system_variant: &SizedSystemVariant) -> UnsizedSystemVariant {
        match system_variant {
            SizedSystemVariant::EntitiesV1(_) => UnsizedSystemVariant::EntitiesV1,
            _ => todo!(),
        }
    }
}

// ============================================================================

#[derive(Debug, Clone)]
pub enum FieldVariant {}

#[derive(Debug, Clone)]
pub enum ObjectsVariant {}

#[derive(Debug, Clone)]
pub enum NetworkVariant {}
