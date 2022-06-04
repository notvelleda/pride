//! miscellaneous functions and types

use lazy_static::lazy_static;
use regex::Regex;
use serde::Deserialize;

/// size of an element that can be represented as a percentage, a fraction, or a number
#[derive(Debug, Deserialize)]
pub struct PartialSize(pub String);

impl PartialSize {
    /// convert this PartialSize into a floating-point number
    pub fn as_number(&self) -> Option<f64> {
        if let Some(f) = percent_to_float(&self.0) {
            Some(f)
        } else if let Some(f) = fraction_to_float(&self.0) {
            Some(f)
        } else if let Ok(f) = self.0.parse::<f64>() {
            if f >= 0.0 { // we don't want negatives
                Some(f)
            } else {
                None
            }
        } else {
            None
        }
    }
}

/// parse float value from percentage string
pub fn percent_to_float(string: &str) -> Option<f64> {
    lazy_static! { // avoid compiling the regex multiple times
        static ref RE: Regex = Regex::new(r"^([\d.]+)%$").unwrap();
    }

    // match regex against string, return None if no matches
    let cap = RE.captures(string)?;

    // make sure we have the right amount of captures
    if cap.len() != 2 {
        return None;
    }

    // try parsing first capture as f64, return None if failure
    let num = if let Ok(num) = cap[1].parse::<f64>() {
        num
    } else {
        return None;
    };

    // return number as a floating point percentage
    Some(num / 100.0)
}

/// parse float value from fraction string
pub fn fraction_to_float(string: &str) -> Option<f64> {
    lazy_static! { // avoid compiling the regex multiple times
        static ref RE: Regex = Regex::new(r"^([\d.]+)/([\d.]+)$").unwrap();
    }

    // match regex against string, return None if no matches
    let cap = RE.captures(string)?;

    // make sure we have the right amount of captures
    if cap.len() != 3 {
        return None;
    }

    // try parsing numerator and denominator as f64, return None if failure
    let numerator = if let Ok(num) = cap[1].parse::<f64>() {
        num
    } else {
        return None;
    };

    let denominator = if let Ok(num) = cap[2].parse::<f64>() {
        num
    } else {
        return None;
    };

    // turn fraction into float
    Some(numerator / denominator)
}
