use std::str::FromStr;

use crate::{
    equal_temperament_default, Fraction, ELEVEN_LIMIT, FIVE_LIMIT, FORTYTHREE_TONE, INDIAN_SCALE,
    INDIAN_SCALE_22, INDIA_SCALE_ALT, JUST_INTONATION, JUST_INTONATION_24, PYTHOGREAN_TUNING,
};
#[derive(Clone, Debug, PartialEq, Copy)]
pub enum TuningSystem {
    StepMethod,
    EqualTemperament,

    JustIntonation,
    JustIntonation24,
    PythogoreanTuning,

    FiveLimit,
    ElevenLimit,

    FortyThreeTone,

    //ethnic scales
    Indian,
    IndianAlt,
    Indian22,
}

impl FromStr for TuningSystem {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "JustIntonation" => Ok(TuningSystem::JustIntonation),
            "JustIntonation24" => Ok(TuningSystem::JustIntonation24),
            "StepMethod" => Ok(TuningSystem::StepMethod),
            "EqualTemperament" => Ok(TuningSystem::EqualTemperament),
            "PythogoreanTuning" => Ok(TuningSystem::PythogoreanTuning),
            "FiveLimit" => Ok(TuningSystem::FiveLimit),
            "ElevenLimit" => Ok(TuningSystem::ElevenLimit),
            "FortyThreeTone" => Ok(TuningSystem::FortyThreeTone),
            "Indian" => Ok(TuningSystem::Indian),
            "IndianFull" => Ok(TuningSystem::Indian22),
            _ => Err(()),
        }
    }
}

pub fn get_fraction(tuning_sytem: TuningSystem, index: usize) -> Fraction {
    match tuning_sytem {
        TuningSystem::StepMethod => todo!(),
        TuningSystem::EqualTemperament => equal_temperament_default(index),
        _ => get_fraction_from_table(tuning_sytem, index),
    }
}

fn get_fraction_from_table(tuning_sytem: TuningSystem, index: usize) -> Fraction {
    let lut: &[Fraction] = match tuning_sytem {
        TuningSystem::JustIntonation => &JUST_INTONATION,
        TuningSystem::JustIntonation24 => &JUST_INTONATION_24,
        TuningSystem::PythogoreanTuning => &PYTHOGREAN_TUNING,
        TuningSystem::FiveLimit => &FIVE_LIMIT,
        TuningSystem::ElevenLimit => &ELEVEN_LIMIT,
        TuningSystem::FortyThreeTone => &FORTYTHREE_TONE,
        TuningSystem::Indian => &INDIAN_SCALE,
        TuningSystem::IndianAlt => &INDIA_SCALE_ALT,
        TuningSystem::Indian22 => &INDIAN_SCALE_22,

        TuningSystem::StepMethod | TuningSystem::EqualTemperament => panic!(),
    };
    let len = lut.len();

    let octaves = (index / len) as u32;
    let mut fration = lut[index % len];
    fration.numerator += fration.denominator * octaves;
    fration
}
