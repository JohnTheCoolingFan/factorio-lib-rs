pub mod as_f32 {
    use fixed::traits::Fixed;
    use serde::{Deserialize, Deserializer, Serialize, Serializer};

    pub fn serialize<S, F>(val: &F, ser: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
        F: Fixed,
    {
        val.to_num::<f32>().serialize(ser)
    }

    pub fn deserialize<'de, D, F>(des: D) -> Result<F, D::Error>
    where
        D: Deserializer<'de>,
        F: Fixed,
    {
        f32::deserialize(des).map(|val| F::from_num(val))
    }
}

pub mod as_f64 {
    use fixed::traits::Fixed;
    use serde::{Deserialize, Deserializer, Serialize, Serializer};

    pub fn serialize<S, F>(val: &F, ser: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
        F: Fixed,
    {
        val.to_num::<f64>().serialize(ser)
    }

    pub fn deserialize<'de, D, F>(des: D) -> Result<F, D::Error>
    where
        D: Deserializer<'de>,
        F: Fixed,
    {
        f64::deserialize(des).map(|val| F::from_num(val))
    }
}
