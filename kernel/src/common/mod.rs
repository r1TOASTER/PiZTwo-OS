use crate::memory::mmio::RegSized;

// TODO: Better names
#[derive(Clone, Debug, Copy)]
pub enum CommonErr {
    OffRange,
    OffSize,
    NoSize,
    OffValue,
    RegOverflow
}

pub(crate) fn set_reg_val<T: RegSized>(addr: *mut T, value: T, field_offset: u8, field_size: u8) -> Result<(), CommonErr> {
    if field_offset >= T::BITS { 
        // if the offset is bigger then the amount of bits in T (for example, T:u32 and field_offset is 64)
        return Err(CommonErr::OffRange);
    }

    if field_size > T::BITS {
        // if the field size is bigger then the amount of bits in T (for example, T:u32 and field_size is 33)
        return Err(CommonErr::OffSize);
    } 

    if field_size == 0 {
        // can't have no field
        return Err(CommonErr::NoSize);
    }

    if (field_offset as u32 + field_size as u32) > T::BITS as u32 {
        // can't have offset = 10 and size = 22 when the register is 32 bit for example
        return Err(CommonErr::RegOverflow);
    }

    // get original register value
    let mut org_val = unsafe { T::mmio_read(addr) };

    // set field mask - bit is on field_size times (field_size = 3 -> field_mask = 0b111)
    let field_mask = (T::from(1u8) << field_size.into()) - T::from(1u8);

    // check the value doesnt overflow the field_mask (can't have value 8 when field_mask can be mas 7 (0b111))
    if value > field_mask {
        return Err(CommonErr::OffValue);
    }

    // clear the field_mask offset
    org_val &= !(field_mask << field_offset.into());
    // put the new value on the right offset
    org_val |= value << field_offset.into();

    unsafe { T::mmio_write(addr, org_val); }

    Ok(())
}