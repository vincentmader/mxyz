#![allow(unused_variables)]
#![allow(unreachable_code)]

// NOTE: use bincode::serialize instead

trait Protocol {
    fn to_bytes(&self) -> Vec<u8>;
    fn from_bytes(bin: &[u8]) -> Self;
}

// // ============================================================================

use mxyz_engine::entity::object::planet::Planet;
impl Protocol for Planet {
    fn to_bytes(&self) -> Vec<u8> {
        let mut bin: Vec<u8> = Vec::new();
        bin.extend_from_slice(&self.mass.to_be_bytes());
        bin.extend_from_slice(&self.position[0].to_be_bytes());
        bin.extend_from_slice(&self.position[1].to_be_bytes());
        bin.extend_from_slice(&self.position[2].to_be_bytes());
        bin.extend_from_slice(&self.velocity[0].to_be_bytes());
        bin.extend_from_slice(&self.velocity[1].to_be_bytes());
        bin.extend_from_slice(&self.velocity[2].to_be_bytes());
        bin
    }
    fn from_bytes(_bin: &[u8]) -> Self {
        let mass = todo!();
        let position = todo!();
        let velocity = todo!();
        Planet::new(mass, position, velocity)
    }
}

// ============================================================================

use mxyz_engine::entity::field::fluid_cell::FluidCell;
impl Protocol for FluidCell {
    fn to_bytes(&self) -> Vec<u8> {
        todo!()
    }
    fn from_bytes(_bin: &[u8]) -> Self {
        let velocity = todo!();
        let density = todo!();
        FluidCell::new(velocity, density)
    }
}

// ============================================================================

// mod testing {
//     pub trait Protocol {
//         fn to_bytes();
//         fn from_bytes(bytes: &[u8]) -> Self;
//     }
//     unsafe fn any_as_u8_slice<T: Sized>(p: &T) -> &[u8] {
//         ::std::slice::from_raw_parts((p as *const T) as *const u8, ::std::mem::size_of::<T>())
//     }

//     // impl Protocol for FluidCell {
//     //     fn to_bytes() {}
//     //     fn from_bytes(bytes: &[u8]) -> Self {
//     //         unsafe { std::mem::transmute(*bytes) }
//     //     }
//     // }

//     use mxyz_engine::entity::field::fluid_cell::FluidCell;
//     fn testo() {
//         let velocity = [0., 0., 0.];
//         let density = 0.;
//         let foo = FluidCell::new(velocity, density);

//         struct MyStruct {
//             id: u8,
//             data: [u8; 1024],
//         }
//         let my_struct = MyStruct {
//             id: 0,
//             data: [1; 1024],
//         };
//         let bytes: &[u8] = unsafe { any_as_u8_slice(&my_struct) };
//         // tcp_stream.write(bytes);
//         println!("{:?}", bytes);

//         let s: MyStruct = unsafe { std::mem::transmute(*bytes) };
//     }
// }
