/*
-----------------------------------------
Code without bonus
-----------------------------------------

pub fn reverse(input: &str) -> String {
    input.chars().rev().collect()
}
 */

use unicode_segmentation::UnicodeSegmentation;

pub fn reverse(input: &str) -> String {
    input.graphemes(true).rev().collect()
}