/*
 * =============================================================================
 * Crate: Color
 * Author: Gael Zarco
 * Description: Methods for generating color values from integers for canvas
 * =============================================================================
*/

/// RGB color
///
/// Takes in r, g, and b values as 8-bit and spits out a 32-bit color integer
///
/// Utilizes bit-wise operations to calculate a final color value
pub fn u8_rgb_color(r: u8, g: u8, b: u8) -> u32 {
    let (r, g, b) = (r as u32, g as u32, b as u32);
    (r << 16) | (g << 8) | b
}

