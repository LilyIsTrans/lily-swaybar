use std::ops::RangeBounds;

use serde::de::Visitor;

use serde::Deserializer;

use serde::Deserialize;

impl<'de> Deserialize<'de> for Color {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct ColorVisitor;
        impl<'de> Visitor<'de> for ColorVisitor {
            type Value = Color;

            fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
                todo!()
            }

            fn visit_u32<E>(self, v: u32) -> Result<Self::Value, E>
            where
                E: serde::de::Error,
            {
                Ok(Color { color: v })
            }

            fn visit_str<E>(self, v: &str) -> Result<Self::Value, E>
            where
                E: serde::de::Error,
            {
                if let Some(starting_index) = v.find(|c| char::is_digit(c, 16)) {
                    let ending_index = v.rfind(|c| char::is_digit(c, 16)).unwrap();
                    let digits = v.get(starting_index..=ending_index);
                    let prefix = v.get(..starting_index);
                    let suffix = v.get(ending_index + 1..);

                    todo!()
                } else {
                    Err(serde::de::Error::invalid_value(
                        serde::de::Unexpected::Str(v),
                        &self,
                    ))
                }
            }
        }

        todo!()
    }
}

#[derive(Clone, Copy, PartialEq, Eq, Hash)]
#[repr(transparent)]
pub struct Color {
    pub(crate) color: u32,
}

#[derive(Deserialize, Clone)]
pub struct Colors {}
