macro_rules! make_instructions {
    (
        $(
            (
                $name:ident, $code:ident, $mod_name:ident, $tag:literal
                $(, metadata_type: $metadata_type:ty)?
                $(, ($field_name:ident: $field_type:ty))*
                $(, optional: ($opt_field_name:ident: $opt_field_type:ty))*
                $(, boxed: ($boxed_field_name:ident: $boxed_field_type:ty))*
            ),
        )+
    ) => {
        use crate::{michelson::{Michelson, Data}, Result, Error, micheline::{Micheline, primitive_application::PrimitiveApplication}};
        pub use self::{
            sequence::{Sequence, sequence},
            $($mod_name::{$name, $mod_name},)*
        };

        #[derive(Debug, Clone, PartialEq)]
        pub enum Instruction {
            Sequence(Sequence),
            $($name($name),)*
        }

        impl Instruction {
            pub fn prim_values() -> &'static [&'static crate::michelson::Prim] {
                &PRIMS
            }
        }

        impl From<Instruction> for Data {
            fn from(value: Instruction) -> Self {
                Self::Instruction(value)
            }
        }

        impl From<Instruction> for Micheline {
            fn from(value: Instruction) -> Self {
                match value {
                    Instruction::Sequence(value) => value.into(),
                    $(
                        Instruction::$name(value) => value.into(),
                    )*
                }
            }
        }

        impl From<Instruction> for Michelson {
            fn from(value: Instruction) -> Self {
                Self::Data(Data::Instruction(value))
            }
        }

        impl TryFrom<Michelson> for Instruction {
            type Error = Error;

            fn try_from(value: Michelson) -> Result<Self> {
                if let Michelson::Data(Data::Instruction(value)) = value {
                    return Ok(value);
                }
                Err(Error::InvalidMichelsonInstruction)
            }
        }

        impl TryFrom<Micheline> for Instruction {
            type Error = Error;

            fn try_from(value: Micheline) -> Result<Self> {
                if value.is_micheline_sequence() {
                    return Ok(Instruction::Sequence(value.try_into()?));
                }
                let primitive_application: PrimitiveApplication = value.try_into()?;

                primitive_application.try_into()
            }
        }

        impl TryFrom<PrimitiveApplication> for Instruction {
            type Error = Error;

            fn try_from(value: PrimitiveApplication) -> Result<Self> {
                match value.prim() {
                    $(stringify!($code) => Ok(Instruction::$name(value.try_into()?)),)*
                    _ => Err(Error::InvalidPrimitiveApplication),
                }
            }
        }

        const PRIMS: &[&'static crate::michelson::Prim] = &[
            $(&$mod_name::PRIM,)*
        ];

        $(
            make_instruction!(
                $name, $code, $mod_name, $tag
                $(, metadata_type: $metadata_type)?
                $(, ($field_name: $field_type))*
                $(, optional: ($opt_field_name: $opt_field_type))*
                $(, boxed: ($boxed_field_name: $boxed_field_type))*
            );
        )+
    };
}

macro_rules! make_instruction {
    (
        $name:ident, $code:ident, $mod_name:ident, $tag:literal
        $(, metadata_type: $metadata_type:ty)?
        $(, ($field_name:ident: $field_type:ty))*
        $(, optional: ($opt_field_name:ident: $opt_field_type:ty))*
        $(, boxed: ($boxed_field_name:ident: $boxed_field_type:ty))*
    ) => {
        mod $mod_name {
            use crate::{
                michelson::{Prim, PrimType, Michelson, Data, Instruction},
                micheline::{Micheline, primitive_application::PrimitiveApplication},
                Error, Result,
            };

            #[derive(Debug, Clone, PartialEq)]
            pub struct $name {
                $(
                    $field_name: $field_type,
                )*
                $(
                    $opt_field_name: Option<$opt_field_type>,
                )*
                $(
                    $boxed_field_name: Box<$boxed_field_type>,
                )*
                $(
                    metadata: $metadata_type,
                )?
            }

            impl $name {
                $(
                    pub fn $field_name(&self) -> &$field_type {
                        &self.$field_name
                    }
                )*

                $(
                    pub fn $opt_field_name(&self) -> &Option<$opt_field_type> {
                        &self.$opt_field_name
                    }
                )*

                $(
                    pub fn $boxed_field_name(&self) -> &Box<$boxed_field_type> {
                        &self.$boxed_field_name
                    }
                )*

                $(
                    pub fn metadata(&self) -> &$metadata_type {
                        &self.metadata
                    }

                    pub fn annotations(&self) -> Vec<&crate::michelson::Annotation> {
                        self.metadata.annotations()
                    }
                )?

                pub fn new($($field_name: $field_type,)* $($opt_field_name: Option<$opt_field_type>,)* $($boxed_field_name: $boxed_field_type,)* $(metadata: $metadata_type)?) -> Self {
                    Self {
                        $($field_name,)*
                        $($opt_field_name,)*
                        $($boxed_field_name: Box::new($boxed_field_name),)*
                        $(
                            metadata: metadata as $metadata_type
                        )?
                    }
                }
            }

            impl PrimType for $name {
                fn prim_value() -> &'static Prim {
                    &PRIM
                }
            }

            pub const PRIM: Prim = Prim::new(stringify!($code), &[$tag]);

            impl From<$name> for Instruction {
                fn from(value: $name) -> Self {
                    Instruction::$name(value)
                }
            }

            impl TryFrom<Instruction> for $name {
                type Error = Error;

                fn try_from(value: Instruction) -> Result<Self> {
                    if let Instruction::$name(value) = value {
                        return Ok(value);
                    }
                    Err(Error::InvalidMichelsonInstruction)
                }
            }

            impl From<$name> for Michelson {
                fn from(value: $name) -> Self {
                    Self::Data(Data::Instruction(value.into()))
                }
            }

            impl TryFrom<Michelson> for $name {
                type Error = Error;

                fn try_from(value: Michelson) -> Result<Self> {
                    let instruction: Instruction = value.try_into()?;

                    if let Instruction::$name(value) = instruction {
                        return Ok(value);
                    }
                    Err(Error::InvalidMichelsonInstruction)
                }
            }

            impl From<$name> for Micheline {
                #[allow(unused)]
                fn from(value: $name) -> Self {
                    let mut args: Vec<Micheline> = vec![];
                    $(
                        args.push(value.$field_name.into());
                    )*
                    $(
                        if let Some(value) = value.$opt_field_name {
                            args.push(value.into());
                        }
                    )*
                    $(
                        args.push((*value.$boxed_field_name).into());
                    )*
                    let mut annots: Vec<String> = vec![];
                    $(
                        let metadata: $metadata_type = value.metadata;
                        annots = metadata.annotations().into_iter().map(|annot| annot.value().into()).collect();
                    )?
                    let primitive_application = PrimitiveApplication::new(PRIM.name().into(), Some(args), Some(annots));

                    primitive_application.into()
                }
            }

            impl TryFrom<PrimitiveApplication> for $name {
                type Error = Error;

                #[allow(unused)]
                fn try_from(value: PrimitiveApplication) -> Result<Self> {
                    if value.prim() != PRIM.name() {
                        return Err(Error::InvalidPrimitiveApplication);
                    }
                    $(
                        let metadata: $metadata_type = (&value).try_into()?;
                    )?
                    let mut args = value.to_args().unwrap_or(vec![]);
                    Ok(Self {
                        $(
                            $field_name: if !args.is_empty() { args.remove(0).try_into()? } else { Err(Error::InvalidPrimitiveApplication)? },
                        )*
                        $(
                            $opt_field_name: if !args.is_empty() { Some(args.remove(0).try_into()?) } else { None },
                        )*
                        $(
                            $boxed_field_name: if !args.is_empty() { Box::new(args.remove(0).try_into()?) } else { Err(Error::InvalidPrimitiveApplication)? },
                        )*
                        $(
                            metadata: metadata as $metadata_type,
                        )?
                    })
                }
            }

            pub fn $mod_name($($field_name: $field_type,)* $($opt_field_name: Option<$opt_field_type>,)* $($boxed_field_name: $boxed_field_type,)*) -> Michelson {
                $name::new(
                    $($field_name, )*
                    $($opt_field_name, )*
                    $($boxed_field_name,)*
                    $(<$metadata_type>::default(), )?
                ).into()
            }
        }
    };
}

pub(crate) use {make_instruction, make_instructions};
