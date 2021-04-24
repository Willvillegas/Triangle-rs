//! runtime entities
//! These entities are used by the encoder to
//! generate the correct bytecode for the TAM (Triangle Abstract Machine).

#[derive(Debug, Clone)]
pub enum RuntimeEntity {
    None,
    KnownAddress(KnownAddressState),
    UnknownAddress(UnknownAddressState),
    KnownValue(KnownValueState),
    UnknownValue(UnknownValueState),
    PrimitiveRoutine(PrimitiveRoutineState),
    EqualityRoutine(EqualityRoutineState),
    UnknownRoutine(UnknownRoutineState),
    Field(FieldState),
    TypeRepresentation(TypeRepresentationState),
}

/// represents the declared address of a runtime entity
#[derive(Debug, Clone)]
pub struct EntityAddress {
    pub level: usize,
    pub displacement: isize,
}

impl EntityAddress {
    pub fn new(level: usize, displacement: isize) -> Self {
        EntityAddress {
            level,
            displacement,
        }
    }
}

#[derive(Debug, Clone)]
pub struct KnownAddressState {
    pub size: usize,
    pub address: EntityAddress,
}

impl KnownAddressState {
    pub fn new(size: usize, level: usize, displacement: isize) -> Self {
        KnownAddressState {
            size: size,
            address: EntityAddress::new(level, displacement),
        }
    }
}

#[derive(Debug, Clone)]
pub struct UnknownAddressState {
    size: usize,
    address: EntityAddress,
}

impl UnknownAddressState {
    pub fn new(size: usize, level: usize, displacement: isize) -> Self {
        UnknownAddressState {
            size: size,
            address: EntityAddress::new(level, displacement),
        }
    }
}

#[derive(Debug, Clone)]
pub struct KnownValueState {
    size: usize,
    value: i32,
}

impl KnownValueState {
    pub fn new(size: usize, value: i32) -> Self {
        KnownValueState { size, value }
    }
}

#[derive(Debug, Clone)]
pub struct UnknownValueState {
    size: usize,
    address: EntityAddress,
}

impl UnknownValueState {
    pub fn new(size: usize, level: usize, displacement: isize) -> Self {
        UnknownValueState {
            size: size,
            address: EntityAddress::new(level, displacement),
        }
    }
}

#[derive(Debug, Clone)]
pub struct KnownRoutineState {
    size: usize,
    address: EntityAddress,
}

impl KnownRoutineState {
    pub fn new(size: usize, level: usize, displacement: isize) -> Self {
        KnownRoutineState {
            size: size,
            address: EntityAddress::new(level, displacement),
        }
    }
}

#[derive(Debug, Clone)]
pub struct UnknownRoutineState {
    size: usize,
    address: EntityAddress,
}

impl UnknownRoutineState {
    pub fn new(size: usize, level: usize, displacement: isize) -> Self {
        UnknownRoutineState {
            size: size,
            address: EntityAddress::new(level, displacement),
        }
    }
}

#[derive(Debug, Clone)]
pub struct PrimitiveRoutineState {
    size: usize,
    displacement: isize,
}

impl PrimitiveRoutineState {
    pub fn new(size: usize, displacement: isize) -> Self {
        PrimitiveRoutineState { size, displacement }
    }
}

#[derive(Debug, Clone)]
pub struct EqualityRoutineState {
    size: usize,
    displacement: isize,
}

impl EqualityRoutineState {
    pub fn new(size: usize, displacement: isize) -> Self {
        EqualityRoutineState { size, displacement }
    }
}

#[derive(Debug, Clone)]
pub struct FieldState {
    size: usize,
    offset: isize,
}

impl FieldState {
    pub fn new(size: usize, offset: isize) -> Self {
        FieldState { size, offset }
    }
}

#[derive(Debug, Clone)]
pub struct TypeRepresentationState {
    size: usize,
}

impl TypeRepresentationState {
    pub fn new(size: usize) -> Self {
        TypeRepresentationState { size }
    }
}
