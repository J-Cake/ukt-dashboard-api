use serde::{Deserialize, Serialize};
use std::time::SystemTime;

nestify::nest! {
    #[derive(Debug, Clone, Serialize, Deserialize)]*
    pub struct WeatherState {
        pub time: SystemTime,
        pub response: pub struct WeatherResponse {
            pub is_day: bool,
            pub current: pub struct WeatherDay {
                pub wind_speed: f64,
                pub precipitation: f64,
                pub humidity: f64,
                pub temperature: f64,
                pub code: u64,
                pub weather: #[repr(u8)] pub enum PresentWeather {
                    // 00-19 No precipitation etc
                    CloudDevelopmentNotObserved                               = 0,   // 00: Cloud development not observed or not observable
                    CloudsDissolving                                          = 1,   // 01: Clouds generally dissolving or becoming less developed
                    SkyStateUnchanged                                         = 2,   // 02: State of sky on the whole unchanged
                    CloudsForming                                             = 3,   // 03: Clouds generally forming or developing
                    VisibilityReducedBySmoke                                  = 4,   // 04: Visibility reduced by smoke, e.g. veldt/forest fires, industrial smoke or volcanic ashes
                    Haze                                                      = 5,   // 05: Haze
                    WidespreadDustSuspended                                   = 6,   // 06: Widespread dust in suspension in the air, not raised by wind at or near the station
                    DustOrSandRaisedByWind                                    = 7,   // 07: Dust or sand raised by wind at or near the station, but no well-developed dust whirl(s) or sand whirl(s), and no duststorm or sandstorm
                    DustOrSandWhirls                                          = 8,   // 08: Well developed dust whirl(s) or sand whirl(s) seen at or near the station during preceding hour or at time of observation, but no duststorm or sandstorm
                    DuststormOrSandstorm                                      = 9,   // 09: Duststorm or sandstorm within sight at the time of observation, or at the station during the preceding hour
                    Mist                                                      = 10,  // 10: Mist
                    PatchesShallowFogOrIceFog                                 = 11,  // 11: Patches shallow fog or ice fog at the station, whether on land or sea, not deeper than ~2 m on land or 10 m at sea
                    ContinuousFogOrIceFog                                     = 12,  // 12: More or less continuous (shallow) fog or ice fog
                    LightningVisibleNoThunder                                 = 13,  // 13: Lightning visible, no thunder heard
                    PrecipitationInSightNotReachingGround                     = 14,  // 14: Precipitation within sight, not reaching the ground or surface of the sea
                    PrecipitationInSightGroundDistant                         = 15,  // 15: Precipitation within sight, reaching the ground or the surface of the sea, but distant (>5 km from station)
                    PrecipitationInSightNearStation                           = 16,  // 16: Precipitation within sight, reaching the ground or the surface of the sea, near to, but not at the station
                    ThunderstormNoPrecipitation                               = 17,  // 17: Thunderstorm, but no precipitation at time of observation
                    Squalls                                                   = 18,  // 18: Squalls at or within sight of station during the preceding hour or at time of observation
                    FunnelClouds                                              = 19,  // 19: Funnel cloud(s) — Tornado cloud or water‐spout

                    // 20-29 Precip/fog/thunder at station during preceding hour but not at observation time
                    DrizzleNotFreezing                                        = 20,  // 20: Drizzle (not freezing) or snow grains, not falling as shower(s)
                    RainNotFreezing                                           = 21,  // 21: Rain (not freezing)
                    Snow                                                      = 22,  // 22: Snow
                    RainAndSnowOrIcePellets                                   = 23,  // 23: Rain and snow or ice pellets
                    FreezingDrizzleOrFreezingRain                             = 24,  // 24: Freezing drizzle or freezing rain
                    ShowerOfRain                                              = 25,  // 25: Shower(s) of rain
                    ShowerOfSnowOrRainAndSnow                                 = 26,  // 26: Shower(s) of snow, or of rain and snow
                    ShowerOfHailOrRainAndHail                                 = 27,  // 27: Shower(s) of hail, or of rain and hail
                    FogOrIceFog                                               = 28,  // 28: Fog or ice fog
                    ThunderstormWithOrWithoutPrecipitation                    = 29,  // 29: Thunderstorm (with or without precipitation)

                    // 30-39 Duststorm, sandstorm, drifting/blowing snow
                    SlightOrModerateDuststormHasDecreased                     = 30,  // 30: Slight or moderate duststorm or sandstorm – has decreased during preceding hour
                    DuststormNoChange                                         = 31,  // 31: – no appreciable change during preceding hour
                    DuststormHasBegunOrIncreased                              = 32,  // 32: – has begun or has increased during preceding hour
                    SevereDuststormHasDecreased                               = 33,  // 33: Severe duststorm or sandstorm – has decreased during preceding hour
                    SevereDuststormNoChange                                   = 34,  // 34: – no appreciable change during preceding hour
                    SevereDuststormHasBegunOrIncreased                        = 35,  // 35: – has begun or has increased during preceding hour
                    SlightOrModerateBlowingSnowLow                            = 36,  // 36: Slight or moderate blowing snow generally low (below eye level)
                    HeavyDriftingSnow                                         = 37,  // 37: Heavy drifting snow
                    SlightOrModerateBlowingSnowHigh                           = 38,  // 38: Slight or moderate blowing snow generally high (above eye level)
                    HeavyDriftingSnow2                                        = 39,  // 39: Heavy drifting snow

                    // 40-49 Fog or ice fog at time of observation
                    FogOrIceFogDistance                                       = 40,  // 40: Fog or ice fog at a distance at the time of observation, but not at the station during the preceding hour, extending to a level above observer
                    FogOrIceFogInPatches                                      = 41,  // 41: Fog or ice fog in patches
                    FogOrIceFogSkyVisibleHasThinned                           = 42,  // 42: Fog or ice fog, sky visible has become thinner during preceding hour
                    FogOrIceFogSkyInvisible1                                  = 43,  // 43: Fog or ice fog, sky invisible
                    FogOrIceFogSkyVisibleNoChange                             = 44,  // 44: Fog or ice fog, sky visible, no appreciable change during preceding hour
                    FogOrIceFogSkyInvisible2                                  = 45,  // 45: Fog or ice fog, sky invisible
                    FogOrIceFogSkyVisibleHasThickened                         = 46,  // 46: Fog or ice fog, sky visible, has begun or become thicker during preceding hour
                    FogOrIceFogSkyInvisible3                                  = 47,  // 47: Fog or ice fog, sky invisible
                    FogDepositingRimeSkyVisible                               = 48,  // 48: Fog, depositing rime, sky visible
                    FogDepositingRimeSkyInvisible                             = 49,  // 49: Fog, depositing rime, sky invisible

                    // 50-59 Precipitation at station at observation time — Drizzle
                    DrizzleNotFreezingIntermittentSlight                      = 50,  // 50: Drizzle, not freezing, intermittent slight at time of observation
                    DrizzleNotFreezingContinuous                              = 51,  // 51: Drizzle, not freezing, continuous
                    DrizzleNotFreezingIntermittentModerate                    = 52,  // 52: Drizzle, not freezing, intermittent moderate at time of observation
                    DrizzleNotFreezingContinuousHeavy                         = 53,  // 53: Drizzle, not freezing, continuous
                    DrizzleNotFreezingIntermittentHeavy                       = 54,  // 54: Drizzle, not freezing, intermittent heavy (dense) at time of observation
                    DrizzleNotFreezingContinuousAgain                         = 55,  // 55: Drizzle, not freezing, continuous (again)
                    DrizzleFreezingSlight                                     = 56,  // 56: Drizzle, freezing, slight
                    DrizzleFreezingModerateOrHeavy                            = 57,  // 57: Drizzle, freezing, moderate or heavy (dense)
                    DrizzleAndRainSlight                                      = 58,  // 58: Drizzle and rain, slight
                    DrizzleAndRainModerateOrHeavy                             = 59,  // 59: Drizzle and rain, moderate or heavy

                    // 60-69 Rain
                    RainNotFreezingIntermittentSlight                         = 60,  // 60: Rain, not freezing, intermittent slight at time of observation
                    RainNotFreezingContinuous                                 = 61,  // 61: Rain, not freezing, continuous
                    RainNotFreezingIntermittentModerate                       = 62,  // 62: Rain, not freezing, intermittent moderate at time of observation
                    RainNotFreezingContinuousAgain                            = 63,  // 63: Rain, not freezing, continuous
                    RainNotFreezingIntermittentHeavy                          = 64,  // 64: Rain, not freezing, intermittent heavy at time of observation
                    RainNotFreezingContinuousHeavy                            = 65,  // 65: Rain, not freezing, continuous
                    RainFreezingSlight                                        = 66,  // 66: Rain, freezing, slight
                    RainFreezingModerateOrHeavy                               = 67,  // 67: Rain, freezing, moderate or heavy (dense)
                    RainOrDrizzleAndSnowSlight                                = 68,  // 68: Rain or drizzle and snow, slight
                    RainOrDrizzleAndSnowModerateOrHeavy                       = 69,  // 69: Rain or drizzle and snow, moderate or heavy

                    // 70-79 Solid precipitation not in showers
                    SnowflakesIntermittentSlight                              = 70,  // 70: Intermittent fall of snowflakes slight at time of observation
                    SnowflakesContinuous                                      = 71,  // 71: Continuous fall of snowflakes
                    SnowflakesIntermittentModerate                            = 72,  // 72: Intermittent fall of snowflakes moderate at time of observation
                    SnowflakesContinuousAgain                                 = 73,  // 73: Continuous fall of snowflakes
                    SnowflakesIntermittentHeavy                               = 74,  // 74: Intermittent fall of snowflakes heavy at time of observation
                    SnowflakesContinuousHeavy                                 = 75,  // 75: Continuous fall of snowflakes
                    DiamondDust                                               = 76,  // 76: Diamond dust (with or without fog)
                    SnowGrains                                                = 77,  // 77: Snow grains (with or without fog)
                    StarLikeSnowCrystals                                      = 78,  // 78: Isolated star-like snow crystals (with or without fog)
                    IcePellets                                                = 79,  // 79: Ice pellets

                    // 80-99 Showery precipitation / precipitation with thunderstorm
                    RainShowersSlight                                         = 80,  // 80: Rain shower(s), slight
                    RainShowersModerateOrHeavy                                = 81,  // 81: Rain shower(s), moderate or heavy
                    RainShowerViolent                                         = 82,  // 82: Rain shower(s), violent
                    RainAndSnowMixedShowersSlight                             = 83,  // 83: Shower(s) of rain and snow mixed, slight
                    RainAndSnowMixedShowersModerateOrHeavy                    = 84,  // 84: Shower(s) of rain and snow mixed, moderate or heavy
                    SnowShowersSlight                                         = 85,  // 85: Snow shower(s), slight
                    SnowShowersModerateOrHeavy                                = 86,  // 86: Snow shower(s), moderate or heavy
                    SnowPelletsOrSmallHailShowersSlight                       = 87,  // 87: Shower(s) of snow pellets or small hail, with or without rain or rain and snow mixed - slight
                    SnowPelletsOrSmallHailShowersModerateOrHeavy              = 88,  // 88: – moderate or heavy
                    HailShowersWithoutThunderSlight                           = 89,  // 89: Shower(s) of hail, with or without rain and/or snow mixed, not associated with thunder - slight
                    HailShowersWithoutThunderModerateOrHeavy                  = 90,  // 90: – moderate or heavy
                    SlightRainAtObservationThunderPriorHour                   = 91,  // 91: Slight rain at time of observation; thunderstorm during the preceding hour but not at time of observation
                    ModerateOrHeavyRainAtObservation                          = 92,  // 92: Moderate or heavy rain at time of observation
                    SlightSnowOrRainAndSnowMixedOrHailAtObservation           = 93,  // 93: Slight snow, or rain and snow mixed or hail at time of observation
                    ModerateOrHeavySnowOrRainAndSnowMixedOrHail               = 94,  // 94: Moderate or heavy snow, or rain and snow mixed or hail at time of observation
                    ThunderstormSlightOrModerateWithoutHail                   = 95,  // 95: Thunderstorm, slight or moderate, without hail but with rain and/or snow at time of observation
                    ThunderstormSlightOrModerateWithHail                      = 96,  // 96: Thunderstorm, slight or moderate, with hail at time of observation
                    ThunderstormHeavyWithoutHail                              = 97,  // 97: Thunderstorm, heavy, without hail but with rain and/or snow at time of observation
                    ThunderstormWithDuststormOrSandstorm                      = 98,  // 98: Thunderstorm combined with duststorm or sandstorm at time of observation
                    ThunderstormHeavyWithHail                                 = 99,  // 99: Thunderstorm, heavy, with hail at time of observation
                }
            },
            pub daily: Vec<WeatherDay>,
        }
    }
}

impl PresentWeather {
    /// Try to convert a u8 code into the enum variant.
    pub fn from_code(code: u8) -> Option<Self> {
        match code {
            0 => Some(PresentWeather::CloudDevelopmentNotObserved),
            1 => Some(PresentWeather::CloudsDissolving),
            2 => Some(PresentWeather::SkyStateUnchanged),
            3 => Some(PresentWeather::CloudsForming),
            4 => Some(PresentWeather::VisibilityReducedBySmoke),
            5 => Some(PresentWeather::Haze),
            6 => Some(PresentWeather::WidespreadDustSuspended),
            7 => Some(PresentWeather::DustOrSandRaisedByWind),
            8 => Some(PresentWeather::DustOrSandWhirls),
            9 => Some(PresentWeather::DuststormOrSandstorm),
            10 => Some(PresentWeather::Mist),
            11 => Some(PresentWeather::PatchesShallowFogOrIceFog),
            12 => Some(PresentWeather::ContinuousFogOrIceFog),
            13 => Some(PresentWeather::LightningVisibleNoThunder),
            14 => Some(PresentWeather::PrecipitationInSightNotReachingGround),
            15 => Some(PresentWeather::PrecipitationInSightGroundDistant),
            16 => Some(PresentWeather::PrecipitationInSightNearStation),
            17 => Some(PresentWeather::ThunderstormNoPrecipitation),
            18 => Some(PresentWeather::Squalls),
            19 => Some(PresentWeather::FunnelClouds),
            20 => Some(PresentWeather::DrizzleNotFreezing),
            21 => Some(PresentWeather::RainNotFreezing),
            22 => Some(PresentWeather::Snow),
            23 => Some(PresentWeather::RainAndSnowOrIcePellets),
            24 => Some(PresentWeather::FreezingDrizzleOrFreezingRain),
            25 => Some(PresentWeather::ShowerOfRain),
            26 => Some(PresentWeather::ShowerOfSnowOrRainAndSnow),
            27 => Some(PresentWeather::ShowerOfHailOrRainAndHail),
            28 => Some(PresentWeather::FogOrIceFog),
            29 => Some(PresentWeather::ThunderstormWithOrWithoutPrecipitation),
            30 => Some(PresentWeather::SlightOrModerateDuststormHasDecreased),
            31 => Some(PresentWeather::DuststormNoChange),
            32 => Some(PresentWeather::DuststormHasBegunOrIncreased),
            33 => Some(PresentWeather::SevereDuststormHasDecreased),
            34 => Some(PresentWeather::SevereDuststormNoChange),
            35 => Some(PresentWeather::SevereDuststormHasBegunOrIncreased),
            36 => Some(PresentWeather::SlightOrModerateBlowingSnowLow),
            37 => Some(PresentWeather::HeavyDriftingSnow),
            38 => Some(PresentWeather::SlightOrModerateBlowingSnowHigh),
            39 => Some(PresentWeather::HeavyDriftingSnow2),
            40 => Some(PresentWeather::FogOrIceFogDistance),
            41 => Some(PresentWeather::FogOrIceFogInPatches),
            42 => Some(PresentWeather::FogOrIceFogSkyVisibleHasThinned),
            43 => Some(PresentWeather::FogOrIceFogSkyInvisible1),
            44 => Some(PresentWeather::FogOrIceFogSkyVisibleNoChange),
            45 => Some(PresentWeather::FogOrIceFogSkyInvisible2),
            46 => Some(PresentWeather::FogOrIceFogSkyVisibleHasThickened),
            47 => Some(PresentWeather::FogOrIceFogSkyInvisible3),
            48 => Some(PresentWeather::FogDepositingRimeSkyVisible),
            49 => Some(PresentWeather::FogDepositingRimeSkyInvisible),
            50 => Some(PresentWeather::DrizzleNotFreezingIntermittentSlight),
            51 => Some(PresentWeather::DrizzleNotFreezingContinuous),
            52 => Some(PresentWeather::DrizzleNotFreezingIntermittentModerate),
            53 => Some(PresentWeather::DrizzleNotFreezingContinuousHeavy),
            54 => Some(PresentWeather::DrizzleNotFreezingIntermittentHeavy),
            55 => Some(PresentWeather::DrizzleNotFreezingContinuousAgain),
            56 => Some(PresentWeather::DrizzleFreezingSlight),
            57 => Some(PresentWeather::DrizzleFreezingModerateOrHeavy),
            58 => Some(PresentWeather::DrizzleAndRainSlight),
            59 => Some(PresentWeather::DrizzleAndRainModerateOrHeavy),
            60 => Some(PresentWeather::RainNotFreezingIntermittentSlight),
            61 => Some(PresentWeather::RainNotFreezingContinuous),
            62 => Some(PresentWeather::RainNotFreezingIntermittentModerate),
            63 => Some(PresentWeather::RainNotFreezingContinuousAgain),
            64 => Some(PresentWeather::RainNotFreezingIntermittentHeavy),
            65 => Some(PresentWeather::RainNotFreezingContinuousHeavy),
            66 => Some(PresentWeather::RainFreezingSlight),
            67 => Some(PresentWeather::RainFreezingModerateOrHeavy),
            68 => Some(PresentWeather::RainOrDrizzleAndSnowSlight),
            69 => Some(PresentWeather::RainOrDrizzleAndSnowModerateOrHeavy),
            70 => Some(PresentWeather::SnowflakesIntermittentSlight),
            71 => Some(PresentWeather::SnowflakesContinuous),
            72 => Some(PresentWeather::SnowflakesIntermittentModerate),
            73 => Some(PresentWeather::SnowflakesContinuousAgain),
            74 => Some(PresentWeather::SnowflakesIntermittentHeavy),
            75 => Some(PresentWeather::SnowflakesContinuousHeavy),
            76 => Some(PresentWeather::DiamondDust),
            77 => Some(PresentWeather::SnowGrains),
            78 => Some(PresentWeather::StarLikeSnowCrystals),
            79 => Some(PresentWeather::IcePellets),
            80 => Some(PresentWeather::RainShowersSlight),
            81 => Some(PresentWeather::RainShowersModerateOrHeavy),
            82 => Some(PresentWeather::RainShowerViolent),
            83 => Some(PresentWeather::RainAndSnowMixedShowersSlight),
            84 => Some(PresentWeather::RainAndSnowMixedShowersModerateOrHeavy),
            85 => Some(PresentWeather::SnowShowersSlight),
            86 => Some(PresentWeather::SnowShowersModerateOrHeavy),
            87 => Some(PresentWeather::SnowPelletsOrSmallHailShowersSlight),
            88 => Some(PresentWeather::SnowPelletsOrSmallHailShowersModerateOrHeavy),
            89 => Some(PresentWeather::HailShowersWithoutThunderSlight),
            90 => Some(PresentWeather::HailShowersWithoutThunderModerateOrHeavy),
            91 => Some(PresentWeather::SlightRainAtObservationThunderPriorHour),
            92 => Some(PresentWeather::ModerateOrHeavyRainAtObservation),
            93 => Some(PresentWeather::SlightSnowOrRainAndSnowMixedOrHailAtObservation),
            94 => Some(PresentWeather::ModerateOrHeavySnowOrRainAndSnowMixedOrHail),
            95 => Some(PresentWeather::ThunderstormSlightOrModerateWithoutHail),
            96 => Some(PresentWeather::ThunderstormSlightOrModerateWithHail),
            97 => Some(PresentWeather::ThunderstormHeavyWithoutHail),
            98 => Some(PresentWeather::ThunderstormWithDuststormOrSandstorm),
            99 => Some(PresentWeather::ThunderstormHeavyWithHail),
            _ => None,
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WeatherSchema {
    latitude: f64,
    longitude: f64,
    generationtime_ms: f64,
    utc_offset_seconds: f64,
    timezone: String,
    timezone_abbreviation: String,
    elevation: f64,
    pub(crate) current: serde_json::Value,
    pub(crate) daily: serde_json::Value,
}

