//! # SIMICS API SYS
//!
//! Low level bindings to the SIMICS API
//!
//! This crate provides raw bindings to the SIMICS api built directly from the header files of the
//! SIMICS base package using `bindgen`. In general, you should prefer to use the `simics-api`
//! crate over this one, as it provides higher level safe bindings to the SIMICS API.
//!

#![deny(clippy::unwrap_used)]

mod bindings;

pub use bindings::*;

#[cfg(feature = "6.0.28")]
pub const SIMICS_VERSION: &str = "6.0.28";
#[cfg(feature = "6.0.31")]
pub const SIMICS_VERSION: &str = "6.0.31";
#[cfg(feature = "6.0.33")]
pub const SIMICS_VERSION: &str = "6.0.33";
#[cfg(feature = "6.0.34")]
pub const SIMICS_VERSION: &str = "6.0.34";
#[cfg(feature = "6.0.35")]
pub const SIMICS_VERSION: &str = "6.0.35";
#[cfg(feature = "6.0.36")]
pub const SIMICS_VERSION: &str = "6.0.36";
#[cfg(feature = "6.0.38")]
pub const SIMICS_VERSION: &str = "6.0.38";
#[cfg(feature = "6.0.39")]
pub const SIMICS_VERSION: &str = "6.0.39";
#[cfg(feature = "6.0.40")]
pub const SIMICS_VERSION: &str = "6.0.40";
#[cfg(feature = "6.0.41")]
pub const SIMICS_VERSION: &str = "6.0.41";
#[cfg(feature = "6.0.42")]
pub const SIMICS_VERSION: &str = "6.0.42";
#[cfg(feature = "6.0.43")]
pub const SIMICS_VERSION: &str = "6.0.43";
#[cfg(feature = "6.0.44")]
pub const SIMICS_VERSION: &str = "6.0.44";
#[cfg(feature = "6.0.45")]
pub const SIMICS_VERSION: &str = "6.0.45";
#[cfg(feature = "6.0.46")]
pub const SIMICS_VERSION: &str = "6.0.46";
#[cfg(feature = "6.0.47")]
pub const SIMICS_VERSION: &str = "6.0.47";
#[cfg(feature = "6.0.48")]
pub const SIMICS_VERSION: &str = "6.0.48";
#[cfg(feature = "6.0.49")]
pub const SIMICS_VERSION: &str = "6.0.49";
#[cfg(feature = "6.0.50")]
pub const SIMICS_VERSION: &str = "6.0.50";
#[cfg(feature = "6.0.51")]
pub const SIMICS_VERSION: &str = "6.0.51";
#[cfg(feature = "6.0.52")]
pub const SIMICS_VERSION: &str = "6.0.52";
#[cfg(feature = "6.0.53")]
pub const SIMICS_VERSION: &str = "6.0.53";
#[cfg(feature = "6.0.54")]
pub const SIMICS_VERSION: &str = "6.0.54";
#[cfg(feature = "6.0.55")]
pub const SIMICS_VERSION: &str = "6.0.55";
#[cfg(feature = "6.0.56")]
pub const SIMICS_VERSION: &str = "6.0.56";
#[cfg(feature = "6.0.57")]
pub const SIMICS_VERSION: &str = "6.0.57";
#[cfg(feature = "6.0.58")]
pub const SIMICS_VERSION: &str = "6.0.58";
#[cfg(feature = "6.0.59")]
pub const SIMICS_VERSION: &str = "6.0.59";
#[cfg(feature = "6.0.60")]
pub const SIMICS_VERSION: &str = "6.0.60";
#[cfg(feature = "6.0.61")]
pub const SIMICS_VERSION: &str = "6.0.61";
#[cfg(feature = "6.0.62")]
pub const SIMICS_VERSION: &str = "6.0.62";
#[cfg(feature = "6.0.63")]
pub const SIMICS_VERSION: &str = "6.0.63";
#[cfg(feature = "6.0.64")]
pub const SIMICS_VERSION: &str = "6.0.64";
#[cfg(feature = "6.0.65")]
pub const SIMICS_VERSION: &str = "6.0.65";
#[cfg(feature = "6.0.66")]
pub const SIMICS_VERSION: &str = "6.0.66";
#[cfg(feature = "6.0.67")]
pub const SIMICS_VERSION: &str = "6.0.67";
#[cfg(feature = "6.0.68")]
pub const SIMICS_VERSION: &str = "6.0.68";
#[cfg(feature = "6.0.69")]
pub const SIMICS_VERSION: &str = "6.0.69";
#[cfg(feature = "6.0.70")]
pub const SIMICS_VERSION: &str = "6.0.70";
#[cfg(feature = "6.0.71")]
pub const SIMICS_VERSION: &str = "6.0.71";
#[cfg(feature = "6.0.72")]
pub const SIMICS_VERSION: &str = "6.0.72";
#[cfg(feature = "6.0.73")]
pub const SIMICS_VERSION: &str = "6.0.73";
#[cfg(feature = "6.0.74")]
pub const SIMICS_VERSION: &str = "6.0.74";
#[cfg(feature = "6.0.75")]
pub const SIMICS_VERSION: &str = "6.0.75";
#[cfg(feature = "6.0.76")]
pub const SIMICS_VERSION: &str = "6.0.76";
#[cfg(feature = "6.0.77")]
pub const SIMICS_VERSION: &str = "6.0.77";
#[cfg(feature = "6.0.78")]
pub const SIMICS_VERSION: &str = "6.0.78";
#[cfg(feature = "6.0.79")]
pub const SIMICS_VERSION: &str = "6.0.79";
#[cfg(feature = "6.0.80")]
pub const SIMICS_VERSION: &str = "6.0.80";
#[cfg(feature = "6.0.81")]
pub const SIMICS_VERSION: &str = "6.0.81";
#[cfg(feature = "6.0.82")]
pub const SIMICS_VERSION: &str = "6.0.82";
#[cfg(feature = "6.0.83")]
pub const SIMICS_VERSION: &str = "6.0.83";
#[cfg(feature = "6.0.84")]
pub const SIMICS_VERSION: &str = "6.0.84";
#[cfg(feature = "6.0.85")]
pub const SIMICS_VERSION: &str = "6.0.85";
#[cfg(feature = "6.0.86")]
pub const SIMICS_VERSION: &str = "6.0.86";
#[cfg(feature = "6.0.87")]
pub const SIMICS_VERSION: &str = "6.0.87";
#[cfg(feature = "6.0.88")]
pub const SIMICS_VERSION: &str = "6.0.88";
#[cfg(feature = "6.0.89")]
pub const SIMICS_VERSION: &str = "6.0.89";
#[cfg(feature = "6.0.90")]
pub const SIMICS_VERSION: &str = "6.0.90";
#[cfg(feature = "6.0.91")]
pub const SIMICS_VERSION: &str = "6.0.91";
#[cfg(feature = "6.0.92")]
pub const SIMICS_VERSION: &str = "6.0.92";
#[cfg(feature = "6.0.93")]
pub const SIMICS_VERSION: &str = "6.0.93";
#[cfg(feature = "6.0.94")]
pub const SIMICS_VERSION: &str = "6.0.94";
#[cfg(feature = "6.0.95")]
pub const SIMICS_VERSION: &str = "6.0.95";
#[cfg(feature = "6.0.96")]
pub const SIMICS_VERSION: &str = "6.0.96";
#[cfg(feature = "6.0.97")]
pub const SIMICS_VERSION: &str = "6.0.97";
#[cfg(feature = "6.0.98")]
pub const SIMICS_VERSION: &str = "6.0.98";
#[cfg(feature = "6.0.99")]
pub const SIMICS_VERSION: &str = "6.0.99";
#[cfg(feature = "6.0.100")]
pub const SIMICS_VERSION: &str = "6.0.100";
#[cfg(feature = "6.0.101")]
pub const SIMICS_VERSION: &str = "6.0.101";
#[cfg(feature = "6.0.102")]
pub const SIMICS_VERSION: &str = "6.0.102";
#[cfg(feature = "6.0.103")]
pub const SIMICS_VERSION: &str = "6.0.103";
#[cfg(feature = "6.0.104")]
pub const SIMICS_VERSION: &str = "6.0.104";
#[cfg(feature = "6.0.105")]
pub const SIMICS_VERSION: &str = "6.0.105";
#[cfg(feature = "6.0.106")]
pub const SIMICS_VERSION: &str = "6.0.106";
#[cfg(feature = "6.0.107")]
pub const SIMICS_VERSION: &str = "6.0.107";
#[cfg(feature = "6.0.108")]
pub const SIMICS_VERSION: &str = "6.0.108";
#[cfg(feature = "6.0.109")]
pub const SIMICS_VERSION: &str = "6.0.109";
#[cfg(feature = "6.0.110")]
pub const SIMICS_VERSION: &str = "6.0.110";
#[cfg(feature = "6.0.111")]
pub const SIMICS_VERSION: &str = "6.0.111";
#[cfg(feature = "6.0.112")]
pub const SIMICS_VERSION: &str = "6.0.112";
#[cfg(feature = "6.0.113")]
pub const SIMICS_VERSION: &str = "6.0.113";
#[cfg(feature = "6.0.114")]
pub const SIMICS_VERSION: &str = "6.0.114";
#[cfg(feature = "6.0.115")]
pub const SIMICS_VERSION: &str = "6.0.115";
#[cfg(feature = "6.0.116")]
pub const SIMICS_VERSION: &str = "6.0.116";
#[cfg(feature = "6.0.117")]
pub const SIMICS_VERSION: &str = "6.0.117";
#[cfg(feature = "6.0.118")]
pub const SIMICS_VERSION: &str = "6.0.118";
#[cfg(feature = "6.0.119")]
pub const SIMICS_VERSION: &str = "6.0.119";
#[cfg(feature = "6.0.120")]
pub const SIMICS_VERSION: &str = "6.0.120";
#[cfg(feature = "6.0.121")]
pub const SIMICS_VERSION: &str = "6.0.121";
#[cfg(feature = "6.0.122")]
pub const SIMICS_VERSION: &str = "6.0.122";
#[cfg(feature = "6.0.123")]
pub const SIMICS_VERSION: &str = "6.0.123";
#[cfg(feature = "6.0.124")]
pub const SIMICS_VERSION: &str = "6.0.124";
#[cfg(feature = "6.0.125")]
pub const SIMICS_VERSION: &str = "6.0.125";
#[cfg(feature = "6.0.126")]
pub const SIMICS_VERSION: &str = "6.0.126";
#[cfg(feature = "6.0.127")]
pub const SIMICS_VERSION: &str = "6.0.127";
#[cfg(feature = "6.0.128")]
pub const SIMICS_VERSION: &str = "6.0.128";
#[cfg(feature = "6.0.129")]
pub const SIMICS_VERSION: &str = "6.0.129";
#[cfg(feature = "6.0.130")]
pub const SIMICS_VERSION: &str = "6.0.130";
#[cfg(feature = "6.0.131")]
pub const SIMICS_VERSION: &str = "6.0.131";
#[cfg(feature = "6.0.132")]
pub const SIMICS_VERSION: &str = "6.0.132";
#[cfg(feature = "6.0.133")]
pub const SIMICS_VERSION: &str = "6.0.133";
#[cfg(feature = "6.0.134")]
pub const SIMICS_VERSION: &str = "6.0.134";
#[cfg(feature = "6.0.135")]
pub const SIMICS_VERSION: &str = "6.0.135";
#[cfg(feature = "6.0.136")]
pub const SIMICS_VERSION: &str = "6.0.136";
#[cfg(feature = "6.0.137")]
pub const SIMICS_VERSION: &str = "6.0.137";
#[cfg(feature = "6.0.138")]
pub const SIMICS_VERSION: &str = "6.0.138";
#[cfg(feature = "6.0.139")]
pub const SIMICS_VERSION: &str = "6.0.139";
#[cfg(feature = "6.0.140")]
pub const SIMICS_VERSION: &str = "6.0.140";
#[cfg(feature = "6.0.141")]
pub const SIMICS_VERSION: &str = "6.0.141";
#[cfg(feature = "6.0.142")]
pub const SIMICS_VERSION: &str = "6.0.142";
#[cfg(feature = "6.0.143")]
pub const SIMICS_VERSION: &str = "6.0.143";
#[cfg(feature = "6.0.144")]
pub const SIMICS_VERSION: &str = "6.0.144";
#[cfg(feature = "6.0.145")]
pub const SIMICS_VERSION: &str = "6.0.145";
#[cfg(feature = "6.0.146")]
pub const SIMICS_VERSION: &str = "6.0.146";
#[cfg(feature = "6.0.147")]
pub const SIMICS_VERSION: &str = "6.0.147";
#[cfg(feature = "6.0.148")]
pub const SIMICS_VERSION: &str = "6.0.148";
#[cfg(feature = "6.0.149")]
pub const SIMICS_VERSION: &str = "6.0.149";
#[cfg(feature = "6.0.150")]
pub const SIMICS_VERSION: &str = "6.0.150";
#[cfg(feature = "6.0.151")]
pub const SIMICS_VERSION: &str = "6.0.151";
#[cfg(feature = "6.0.152")]
pub const SIMICS_VERSION: &str = "6.0.152";
#[cfg(feature = "6.0.153")]
pub const SIMICS_VERSION: &str = "6.0.153";
#[cfg(feature = "6.0.154")]
pub const SIMICS_VERSION: &str = "6.0.154";
#[cfg(feature = "6.0.155")]
pub const SIMICS_VERSION: &str = "6.0.155";
#[cfg(feature = "6.0.156")]
pub const SIMICS_VERSION: &str = "6.0.156";
#[cfg(feature = "6.0.157")]
pub const SIMICS_VERSION: &str = "6.0.157";
#[cfg(feature = "6.0.158")]
pub const SIMICS_VERSION: &str = "6.0.158";
#[cfg(feature = "6.0.159")]
pub const SIMICS_VERSION: &str = "6.0.159";
#[cfg(feature = "6.0.160")]
pub const SIMICS_VERSION: &str = "6.0.160";
#[cfg(feature = "6.0.161")]
pub const SIMICS_VERSION: &str = "6.0.161";
#[cfg(feature = "6.0.162")]
pub const SIMICS_VERSION: &str = "6.0.162";
#[cfg(feature = "6.0.163")]
pub const SIMICS_VERSION: &str = "6.0.163";
#[cfg(feature = "6.0.164")]
pub const SIMICS_VERSION: &str = "6.0.164";
#[cfg(feature = "6.0.165")]
pub const SIMICS_VERSION: &str = "6.0.165";
#[cfg(feature = "6.0.166")]
pub const SIMICS_VERSION: &str = "6.0.166";
#[cfg(feature = "6.0.167")]
pub const SIMICS_VERSION: &str = "6.0.167";
