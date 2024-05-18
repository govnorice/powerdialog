pub struct Images {
    pub logout: &'static [u8],
    pub poweroff: &'static [u8],
    pub reboot: &'static [u8],
    pub suspend: &'static [u8],
}
pub const IMAGES: Images = Images {
    logout: &[60, 115, 118, 103, 32, 102, 105, 108, 108, 61, 34, 35, 102, 102, 102, 102, 102, 102, 34, 32, 104, 101, 105, 103, 104, 116, 61, 34, 54, 57, 112, 120, 34, 32, 119, 105, 100, 116, 104, 61, 34, 54, 57, 112, 120, 34, 32, 118, 101, 114, 115, 105, 111, 110, 61, 34, 49, 46, 49, 34, 32, 105, 100, 61, 34, 67, 97, 112, 97, 95, 49, 34, 32, 120, 109, 108, 110, 115, 61, 34, 104, 116, 116, 112, 58, 47, 47, 119, 119, 119, 46, 119, 51, 46, 111, 114, 103, 47, 50, 48, 48, 48, 47, 115, 118, 103, 34, 32, 120, 109, 108, 110, 115, 58, 120, 108, 105, 110, 107, 61, 34, 104, 116, 116, 112, 58, 47, 47, 119, 119, 119, 46, 119, 51, 46, 111, 114, 103, 47, 49, 57, 57, 57, 47, 120, 108, 105, 110, 107, 34, 32, 118, 105, 101, 119, 66, 111, 120, 61, 34, 48, 32, 48, 32, 52, 55, 49, 46, 50, 32, 52, 55, 49, 46, 50, 34, 32, 120, 109, 108, 58, 115, 112, 97, 99, 101, 61, 34, 112, 114, 101, 115, 101, 114, 118, 101, 34, 62, 60, 103, 32, 105, 100, 61, 34, 83, 86, 71, 82, 101, 112, 111, 95, 98, 103, 67, 97, 114, 114, 105, 101, 114, 34, 32, 115, 116, 114, 111, 107, 101, 45, 119, 105, 100, 116, 104, 61, 34, 48, 34, 62, 60, 47, 103, 62, 60, 103, 32, 105, 100, 61, 34, 83, 86, 71, 82, 101, 112, 111, 95, 116, 114, 97, 99, 101, 114, 67, 97, 114, 114, 105, 101, 114, 34, 32, 115, 116, 114, 111, 107, 101, 45, 108, 105, 110, 101, 99, 97, 112, 61, 34, 114, 111, 117, 110, 100, 34, 32, 115, 116, 114, 111, 107, 101, 45, 108, 105, 110, 101, 106, 111, 105, 110, 61, 34, 114, 111, 117, 110, 100, 34, 62, 60, 47, 103, 62, 60, 103, 32, 105, 100, 61, 34, 83, 86, 71, 82, 101, 112, 111, 95, 105, 99, 111, 110, 67, 97, 114, 114, 105, 101, 114, 34, 62, 32, 60, 103, 62, 32, 60, 103, 62, 32, 60, 112, 97, 116, 104, 32, 100, 61, 34, 77, 50, 50, 55, 46, 54, 49, 57, 44, 52, 52, 52, 46, 50, 104, 45, 49, 50, 50, 46, 57, 99, 45, 51, 51, 46, 52, 44, 48, 45, 54, 48, 46, 53, 45, 50, 55, 46, 50, 45, 54, 48, 46, 53, 45, 54, 48, 46, 53, 86, 56, 55, 46, 53, 99, 48, 45, 51, 51, 46, 52, 44, 50, 55, 46, 50, 45, 54, 48, 46, 53, 44, 54, 48, 46, 53, 45, 54, 48, 46, 53, 104, 49, 50, 52, 46, 57, 99, 55, 46, 53, 44, 48, 44, 49, 51, 46, 53, 45, 54, 44, 49, 51, 46, 53, 45, 49, 51, 46, 53, 32, 115, 45, 54, 45, 49, 51, 46, 53, 45, 49, 51, 46, 53, 45, 49, 51, 46, 53, 104, 45, 49, 50, 52, 46, 57, 99, 45, 52, 56, 46, 51, 44, 48, 45, 56, 55, 46, 53, 44, 51, 57, 46, 51, 45, 56, 55, 46, 53, 44, 56, 55, 46, 53, 118, 50, 57, 54, 46, 50, 99, 48, 44, 52, 56, 46, 51, 44, 51, 57, 46, 51, 44, 56, 55, 46, 53, 44, 56, 55, 46, 53, 44, 56, 55, 46, 53, 104, 49, 50, 50, 46, 57, 99, 55, 46, 53, 44, 48, 44, 49, 51, 46, 53, 45, 54, 44, 49, 51, 46, 53, 45, 49, 51, 46, 53, 32, 83, 50, 51, 53, 46, 48, 49, 57, 44, 52, 52, 52, 46, 50, 44, 50, 50, 55, 46, 54, 49, 57, 44, 52, 52, 52, 46, 50, 122, 34, 62, 60, 47, 112, 97, 116, 104, 62, 32, 60, 112, 97, 116, 104, 32, 100, 61, 34, 77, 52, 53, 48, 46, 48, 49, 57, 44, 50, 50, 54, 46, 49, 108, 45, 56, 53, 46, 56, 45, 56, 53, 46, 56, 99, 45, 53, 46, 51, 45, 53, 46, 51, 45, 49, 51, 46, 56, 45, 53, 46, 51, 45, 49, 57, 46, 49, 44, 48, 99, 45, 53, 46, 51, 44, 53, 46, 51, 45, 53, 46, 51, 44, 49, 51, 46, 56, 44, 48, 44, 49, 57, 46, 49, 108, 54, 50, 46, 56, 44, 54, 50, 46, 56, 104, 45, 50, 55, 51, 46, 57, 99, 45, 55, 46, 53, 44, 48, 45, 49, 51, 46, 53, 44, 54, 45, 49, 51, 46, 53, 44, 49, 51, 46, 53, 32, 115, 54, 44, 49, 51, 46, 53, 44, 49, 51, 46, 53, 44, 49, 51, 46, 53, 104, 50, 55, 51, 46, 57, 108, 45, 54, 50, 46, 56, 44, 54, 50, 46, 56, 99, 45, 53, 46, 51, 44, 53, 46, 51, 45, 53, 46, 51, 44, 49, 51, 46, 56, 44, 48, 44, 49, 57, 46, 49, 99, 50, 46, 54, 44, 50, 46, 54, 44, 54, 46, 49, 44, 52, 44, 57, 46, 53, 44, 52, 115, 54, 46, 57, 45, 49, 46, 51, 44, 57, 46, 53, 45, 52, 108, 56, 53, 46, 56, 45, 56, 53, 46, 56, 32, 67, 52, 53, 53, 46, 51, 49, 57, 44, 50, 51, 57, 46, 57, 44, 52, 53, 53, 46, 51, 49, 57, 44, 50, 51, 49, 46, 51, 44, 52, 53, 48, 46, 48, 49, 57, 44, 50, 50, 54, 46, 49, 122, 34, 62, 60, 47, 112, 97, 116, 104, 62, 32, 60, 47, 103, 62, 32, 60, 47, 103, 62, 32, 60, 47, 103, 62, 60, 47, 115, 118, 103, 62],
    poweroff: &[60, 63, 120, 109, 108, 32, 118, 101, 114, 115, 105, 111, 110, 61, 34, 49, 46, 48, 34, 32, 101, 110, 99, 111, 100, 105, 110, 103, 61, 34, 85, 84, 70, 45, 56, 34, 63, 62, 10, 60, 115, 118, 103, 32, 120, 109, 108, 110, 115, 61, 34, 104, 116, 116, 112, 58, 47, 47, 119, 119, 119, 46, 119, 51, 46, 111, 114, 103, 47, 50, 48, 48, 48, 47, 115, 118, 103, 34, 32, 120, 109, 108, 110, 115, 58, 120, 108, 105, 110, 107, 61, 34, 104, 116, 116, 112, 58, 47, 47, 119, 119, 119, 46, 119, 51, 46, 111, 114, 103, 47, 49, 57, 57, 57, 47, 120, 108, 105, 110, 107, 34, 32, 119, 105, 100, 116, 104, 61, 34, 54, 57, 112, 120, 34, 32, 104, 101, 105, 103, 104, 116, 61, 34, 54, 57, 112, 120, 34, 32, 118, 105, 101, 119, 66, 111, 120, 61, 34, 48, 32, 48, 32, 54, 57, 32, 54, 57, 34, 32, 118, 101, 114, 115, 105, 111, 110, 61, 34, 49, 46, 49, 34, 62, 10, 60, 103, 32, 105, 100, 61, 34, 115, 117, 114, 102, 97, 99, 101, 49, 34, 62, 10, 60, 112, 97, 116, 104, 32, 115, 116, 121, 108, 101, 61, 34, 32, 115, 116, 114, 111, 107, 101, 58, 110, 111, 110, 101, 59, 102, 105, 108, 108, 45, 114, 117, 108, 101, 58, 110, 111, 110, 122, 101, 114, 111, 59, 102, 105, 108, 108, 58, 114, 103, 98, 40, 49, 48, 48, 37, 44, 49, 48, 48, 37, 44, 49, 48, 48, 37, 41, 59, 102, 105, 108, 108, 45, 111, 112, 97, 99, 105, 116, 121, 58, 49, 59, 34, 32, 100, 61, 34, 77, 32, 51, 54, 46, 56, 53, 49, 53, 54, 50, 32, 51, 46, 49, 51, 54, 55, 49, 57, 32, 67, 32, 51, 54, 46, 56, 53, 49, 53, 54, 50, 32, 49, 46, 56, 51, 53, 57, 51, 56, 32, 51, 53, 46, 56, 48, 48, 55, 56, 49, 32, 48, 46, 55, 56, 53, 49, 53, 54, 32, 51, 52, 46, 53, 32, 48, 46, 55, 56, 53, 49, 53, 54, 32, 67, 32, 51, 51, 46, 49, 57, 57, 50, 49, 57, 32, 48, 46, 55, 56, 53, 49, 53, 54, 32, 51, 50, 46, 49, 52, 56, 52, 51, 56, 32, 49, 46, 56, 51, 53, 57, 51, 56, 32, 51, 50, 46, 49, 52, 56, 52, 51, 56, 32, 51, 46, 49, 51, 54, 55, 49, 57, 32, 76, 32, 51, 50, 46, 49, 52, 56, 52, 51, 56, 32, 50, 53, 46, 48, 56, 57, 56, 52, 52, 32, 67, 32, 51, 50, 46, 49, 52, 56, 52, 51, 56, 32, 50, 54, 46, 51, 57, 48, 54, 50, 53, 32, 51, 51, 46, 49, 57, 57, 50, 49, 57, 32, 50, 55, 46, 52, 52, 49, 52, 48, 54, 32, 51, 52, 46, 53, 32, 50, 55, 46, 52, 52, 49, 52, 48, 54, 32, 67, 32, 51, 53, 46, 56, 48, 48, 55, 56, 49, 32, 50, 55, 46, 52, 52, 49, 52, 48, 54, 32, 51, 54, 46, 56, 53, 49, 53, 54, 50, 32, 50, 54, 46, 51, 57, 48, 54, 50, 53, 32, 51, 54, 46, 56, 53, 49, 53, 54, 50, 32, 50, 53, 46, 48, 56, 57, 56, 52, 52, 32, 90, 32, 77, 32, 51, 54, 46, 56, 53, 49, 53, 54, 50, 32, 51, 46, 49, 51, 54, 55, 49, 57, 32, 34, 47, 62, 10, 60, 112, 97, 116, 104, 32, 115, 116, 121, 108, 101, 61, 34, 32, 115, 116, 114, 111, 107, 101, 58, 110, 111, 110, 101, 59, 102, 105, 108, 108, 45, 114, 117, 108, 101, 58, 110, 111, 110, 122, 101, 114, 111, 59, 102, 105, 108, 108, 58, 114, 103, 98, 40, 49, 48, 48, 37, 44, 49, 48, 48, 37, 44, 49, 48, 48, 37, 41, 59, 102, 105, 108, 108, 45, 111, 112, 97, 99, 105, 116, 121, 58, 49, 59, 34, 32, 100, 61, 34, 77, 32, 49, 55, 46, 48, 57, 51, 55, 53, 32, 49, 49, 46, 50, 56, 57, 48, 54, 50, 32, 67, 32, 49, 56, 46, 49, 51, 50, 56, 49, 50, 32, 49, 48, 46, 53, 48, 55, 56, 49, 50, 32, 49, 56, 46, 51, 52, 51, 55, 53, 32, 57, 46, 48, 51, 53, 49, 53, 54, 32, 49, 55, 46, 53, 54, 50, 53, 32, 55, 46, 57, 57, 54, 48, 57, 52, 32, 67, 32, 49, 54, 46, 55, 56, 49, 50, 53, 32, 54, 46, 57, 53, 55, 48, 51, 49, 32, 49, 53, 46, 51, 48, 56, 53, 57, 52, 32, 54, 46, 55, 53, 32, 49, 52, 46, 50, 54, 57, 53, 51, 49, 32, 55, 46, 53, 50, 55, 51, 52, 52, 32, 67, 32, 54, 46, 48, 56, 53, 57, 51, 56, 32, 49, 51, 46, 54, 55, 53, 55, 56, 49, 32, 48, 46, 55, 56, 53, 49, 53, 54, 32, 50, 51, 46, 52, 54, 56, 55, 53, 32, 48, 46, 55, 56, 53, 49, 53, 54, 32, 51, 52, 46, 53, 32, 67, 32, 48, 46, 55, 56, 53, 49, 53, 54, 32, 53, 51, 46, 49, 50, 49, 48, 57, 52, 32, 49, 53, 46, 56, 55, 56, 57, 48, 54, 32, 54, 56, 46, 50, 49, 52, 56, 52, 52, 32, 51, 52, 46, 53, 32, 54, 56, 46, 50, 49, 52, 56, 52, 52, 32, 67, 32, 53, 51, 46, 49, 50, 49, 48, 57, 52, 32, 54, 56, 46, 50, 49, 52, 56, 52, 52, 32, 54, 56, 46, 50, 49, 52, 56, 52, 52, 32, 53, 51, 46, 49, 50, 49, 48, 57, 52, 32, 54, 56, 46, 50, 49, 52, 56, 52, 52, 32, 51, 52, 46, 53, 32, 67, 32, 54, 56, 46, 50, 49, 52, 56, 52, 52, 32, 50, 51, 46, 53, 53, 48, 55, 56, 49, 32, 54, 50, 46, 55, 56, 49, 50, 53, 32, 49, 51, 46, 54, 56, 51, 53, 57, 52, 32, 53, 52, 46, 55, 52, 54, 48, 57, 52, 32, 55, 46, 53, 51, 57, 48, 54, 50, 32, 67, 32, 53, 51, 46, 55, 49, 52, 56, 52, 52, 32, 54, 46, 55, 53, 32, 53, 50, 46, 50, 51, 56, 50, 56, 49, 32, 54, 46, 57, 52, 57, 50, 49, 57, 32, 53, 49, 46, 52, 52, 57, 50, 49, 57, 32, 55, 46, 57, 56, 48, 52, 54, 57, 32, 67, 32, 53, 48, 46, 54, 54, 48, 49, 53, 54, 32, 57, 46, 48, 49, 49, 55, 49, 57, 32, 53, 48, 46, 56, 53, 53, 52, 54, 57, 32, 49, 48, 46, 52, 56, 56, 50, 56, 49, 32, 53, 49, 46, 56, 57, 48, 54, 50, 53, 32, 49, 49, 46, 50, 55, 55, 51, 52, 52, 32, 67, 32, 53, 56, 46, 56, 52, 55, 54, 53, 54, 32, 49, 54, 46, 53, 57, 55, 54, 53, 54, 32, 54, 51, 46, 53, 49, 49, 55, 49, 57, 32, 50, 53, 46, 49, 50, 49, 48, 57, 52, 32, 54, 51, 46, 53, 49, 49, 55, 49, 57, 32, 51, 52, 46, 53, 32, 67, 32, 54, 51, 46, 53, 49, 49, 55, 49, 57, 32, 53, 48, 46, 53, 50, 51, 52, 51, 56, 32, 53, 48, 46, 53, 50, 51, 52, 51, 56, 32, 54, 51, 46, 53, 49, 49, 55, 49, 57, 32, 51, 52, 46, 53, 32, 54, 51, 46, 53, 49, 49, 55, 49, 57, 32, 67, 32, 49, 56, 46, 52, 55, 54, 53, 54, 50, 32, 54, 51, 46, 53, 49, 49, 55, 49, 57, 32, 53, 46, 52, 56, 56, 50, 56, 49, 32, 53, 48, 46, 53, 50, 51, 52, 51, 56, 32, 53, 46, 52, 56, 56, 50, 56, 49, 32, 51, 52, 46, 53, 32, 67, 32, 53, 46, 52, 56, 56, 50, 56, 49, 32, 50, 53, 46, 48, 49, 49, 55, 49, 57, 32, 49, 48, 46, 48, 52, 50, 57, 54, 57, 32, 49, 54, 46, 53, 56, 53, 57, 51, 56, 32, 49, 55, 46, 48, 57, 51, 55, 53, 32, 49, 49, 46, 50, 56, 57, 48, 54, 50, 32, 90, 32, 77, 32, 49, 55, 46, 48, 57, 51, 55, 53, 32, 49, 49, 46, 50, 56, 57, 48, 54, 50, 32, 34, 47, 62, 10, 60, 47, 103, 62, 10, 60, 47, 115, 118, 103, 62, 10],
    reboot: &[60, 63, 120, 109, 108, 32, 118, 101, 114, 115, 105, 111, 110, 61, 34, 49, 46, 48, 34, 32, 101, 110, 99, 111, 100, 105, 110, 103, 61, 34, 85, 84, 70, 45, 56, 34, 63, 62, 10, 60, 115, 118, 103, 32, 120, 109, 108, 110, 115, 61, 34, 104, 116, 116, 112, 58, 47, 47, 119, 119, 119, 46, 119, 51, 46, 111, 114, 103, 47, 50, 48, 48, 48, 47, 115, 118, 103, 34, 32, 120, 109, 108, 110, 115, 58, 120, 108, 105, 110, 107, 61, 34, 104, 116, 116, 112, 58, 47, 47, 119, 119, 119, 46, 119, 51, 46, 111, 114, 103, 47, 49, 57, 57, 57, 47, 120, 108, 105, 110, 107, 34, 32, 119, 105, 100, 116, 104, 61, 34, 54, 57, 112, 120, 34, 32, 104, 101, 105, 103, 104, 116, 61, 34, 54, 57, 112, 120, 34, 32, 118, 105, 101, 119, 66, 111, 120, 61, 34, 48, 32, 48, 32, 54, 57, 32, 55, 54, 34, 32, 118, 101, 114, 115, 105, 111, 110, 61, 34, 49, 46, 49, 34, 62, 10, 60, 103, 32, 105, 100, 61, 34, 115, 117, 114, 102, 97, 99, 101, 49, 34, 62, 10, 60, 112, 97, 116, 104, 32, 115, 116, 121, 108, 101, 61, 34, 32, 115, 116, 114, 111, 107, 101, 58, 110, 111, 110, 101, 59, 102, 105, 108, 108, 45, 114, 117, 108, 101, 58, 110, 111, 110, 122, 101, 114, 111, 59, 102, 105, 108, 108, 58, 114, 103, 98, 40, 49, 48, 48, 37, 44, 49, 48, 48, 37, 44, 49, 48, 48, 37, 41, 59, 102, 105, 108, 108, 45, 111, 112, 97, 99, 105, 116, 121, 58, 49, 59, 34, 32, 100, 61, 34, 77, 32, 52, 56, 46, 57, 57, 50, 49, 56, 56, 32, 49, 46, 55, 56, 53, 49, 53, 54, 32, 67, 32, 52, 55, 46, 56, 54, 55, 49, 56, 56, 32, 48, 46, 54, 55, 49, 56, 55, 53, 32, 52, 54, 46, 48, 52, 54, 56, 55, 53, 32, 48, 46, 54, 55, 49, 56, 55, 53, 32, 52, 52, 46, 57, 50, 53, 55, 56, 49, 32, 49, 46, 55, 56, 53, 49, 53, 54, 32, 67, 32, 52, 51, 46, 56, 48, 48, 55, 56, 49, 32, 50, 46, 56, 57, 56, 52, 51, 56, 32, 52, 51, 46, 56, 48, 48, 55, 56, 49, 32, 52, 46, 55, 48, 51, 49, 50, 53, 32, 52, 52, 46, 57, 50, 53, 55, 56, 49, 32, 53, 46, 56, 49, 54, 52, 48, 54, 32, 76, 32, 52, 55, 46, 54, 56, 51, 53, 57, 52, 32, 56, 46, 53, 53, 48, 55, 56, 49, 32, 76, 32, 51, 48, 46, 48, 52, 54, 56, 55, 53, 32, 56, 46, 53, 53, 48, 55, 56, 49, 32, 67, 32, 50, 51, 46, 53, 48, 51, 57, 48, 54, 32, 56, 46, 53, 53, 48, 55, 56, 49, 32, 49, 57, 46, 53, 49, 53, 54, 50, 53, 32, 56, 46, 53, 52, 54, 56, 55, 53, 32, 49, 54, 46, 49, 56, 55, 53, 32, 57, 46, 54, 50, 49, 48, 57, 52, 32, 67, 32, 57, 46, 52, 56, 48, 52, 54, 57, 32, 49, 49, 46, 55, 56, 49, 50, 53, 32, 52, 46, 50, 49, 56, 55, 53, 32, 49, 54, 46, 57, 57, 54, 48, 57, 52, 32, 50, 46, 48, 51, 57, 48, 54, 50, 32, 50, 51, 46, 54, 52, 56, 52, 51, 56, 32, 67, 32, 48, 46, 57, 53, 55, 48, 51, 49, 32, 50, 54, 46, 57, 52, 53, 51, 49, 50, 32, 48, 46, 57, 53, 55, 48, 51, 49, 32, 51, 48, 46, 57, 48, 50, 51, 52, 52, 32, 48, 46, 57, 53, 55, 48, 51, 49, 32, 51, 55, 46, 51, 56, 54, 55, 49, 57, 32, 76, 32, 48, 46, 57, 53, 55, 48, 51, 49, 32, 51, 56, 46, 54, 49, 51, 50, 56, 49, 32, 67, 32, 48, 46, 57, 53, 55, 48, 51, 49, 32, 52, 53, 46, 48, 57, 55, 54, 53, 54, 32, 48, 46, 57, 53, 55, 48, 51, 49, 32, 52, 57, 46, 48, 53, 52, 54, 56, 56, 32, 50, 46, 48, 51, 57, 48, 54, 50, 32, 53, 50, 46, 51, 53, 49, 53, 54, 50, 32, 67, 32, 50, 46, 53, 50, 55, 51, 52, 52, 32, 53, 51, 46, 56, 52, 55, 54, 53, 54, 32, 52, 46, 49, 52, 56, 52, 51, 56, 32, 53, 52, 46, 54, 54, 55, 57, 54, 57, 32, 53, 46, 54, 54, 48, 49, 53, 54, 32, 53, 52, 46, 49, 56, 51, 53, 57, 52, 32, 67, 32, 55, 46, 49, 55, 49, 56, 55, 53, 32, 53, 51, 46, 54, 57, 53, 51, 49, 50, 32, 55, 46, 57, 57, 54, 48, 57, 52, 32, 53, 50, 46, 48, 56, 53, 57, 51, 56, 32, 55, 46, 53, 48, 51, 57, 48, 54, 32, 53, 48, 46, 53, 56, 57, 56, 52, 52, 32, 67, 32, 54, 46, 55, 52, 50, 49, 56, 56, 32, 52, 56, 46, 50, 54, 53, 54, 50, 53, 32, 54, 46, 55, 48, 55, 48, 51, 49, 32, 52, 53, 46, 50, 57, 50, 57, 54, 57, 32, 54, 46, 55, 48, 55, 48, 51, 49, 32, 51, 56, 32, 67, 32, 54, 46, 55, 48, 55, 48, 51, 49, 32, 51, 48, 46, 55, 48, 55, 48, 51, 49, 32, 54, 46, 55, 52, 50, 49, 56, 56, 32, 50, 55, 46, 55, 51, 52, 51, 55, 53, 32, 55, 46, 53, 48, 51, 57, 48, 54, 32, 50, 53, 46, 52, 49, 48, 49, 53, 54, 32, 67, 32, 57, 46, 49, 49, 55, 49, 56, 56, 32, 50, 48, 46, 52, 57, 50, 49, 56, 56, 32, 49, 51, 46, 48, 48, 55, 56, 49, 50, 32, 49, 54, 46, 54, 51, 54, 55, 49, 57, 32, 49, 55, 46, 57, 54, 52, 56, 52, 52, 32, 49, 53, 46, 48, 51, 57, 48, 54, 50, 32, 67, 32, 50, 48, 46, 51, 49, 50, 53, 32, 49, 52, 46, 50, 56, 53, 49, 53, 54, 32, 50, 51, 46, 51, 48, 56, 53, 57, 52, 32, 49, 52, 46, 50, 53, 32, 51, 48, 46, 54, 54, 55, 57, 54, 57, 32, 49, 52, 46, 50, 53, 32, 76, 32, 52, 54, 46, 54, 56, 51, 53, 57, 52, 32, 49, 52, 46, 50, 53, 32, 76, 32, 52, 52, 46, 49, 54, 48, 49, 53, 54, 32, 49, 54, 46, 51, 51, 53, 57, 51, 56, 32, 67, 32, 52, 50, 46, 57, 52, 49, 52, 48, 54, 32, 49, 55, 46, 51, 52, 51, 55, 53, 32, 52, 50, 46, 55, 55, 51, 52, 51, 56, 32, 49, 57, 46, 49, 52, 48, 54, 50, 53, 32, 52, 51, 46, 55, 57, 50, 57, 54, 57, 32, 50, 48, 46, 51, 52, 55, 54, 53, 54, 32, 67, 32, 52, 52, 46, 56, 48, 56, 53, 57, 52, 32, 50, 49, 46, 53, 53, 56, 53, 57, 52, 32, 52, 54, 46, 54, 50, 49, 48, 57, 52, 32, 50, 49, 46, 55, 50, 50, 54, 53, 54, 32, 52, 55, 46, 56, 51, 57, 56, 52, 52, 32, 50, 48, 46, 55, 49, 52, 56, 52, 52, 32, 76, 32, 53, 51, 46, 50, 52, 50, 49, 56, 56, 32, 49, 54, 46, 50, 53, 51, 57, 48, 54, 32, 67, 32, 53, 52, 46, 55, 49, 52, 56, 52, 52, 32, 49, 53, 46, 48, 51, 53, 49, 53, 54, 32, 53, 53, 46, 53, 50, 51, 52, 51, 56, 32, 49, 51, 46, 51, 50, 56, 49, 50, 53, 32, 53, 53, 46, 54, 52, 48, 54, 50, 53, 32, 49, 49, 46, 53, 56, 57, 56, 52, 52, 32, 67, 32, 53, 53, 46, 55, 54, 53, 54, 50, 53, 32, 57, 46, 55, 53, 55, 56, 49, 50, 32, 53, 53, 46, 49, 50, 49, 48, 57, 52, 32, 55, 46, 56, 54, 51, 50, 56, 49, 32, 53, 51, 46, 54, 57, 49, 52, 48, 54, 32, 54, 46, 52, 52, 49, 52, 48, 54, 32, 90, 32, 77, 32, 52, 56, 46, 57, 57, 50, 49, 56, 56, 32, 49, 46, 55, 56, 53, 49, 53, 54, 32, 34, 47, 62, 10, 60, 112, 97, 116, 104, 32, 115, 116, 121, 108, 101, 61, 34, 32, 115, 116, 114, 111, 107, 101, 58, 110, 111, 110, 101, 59, 102, 105, 108, 108, 45, 114, 117, 108, 101, 58, 110, 111, 110, 122, 101, 114, 111, 59, 102, 105, 108, 108, 58, 114, 103, 98, 40, 49, 48, 48, 37, 44, 49, 48, 48, 37, 44, 49, 48, 48, 37, 41, 59, 102, 105, 108, 108, 45, 111, 112, 97, 99, 105, 116, 121, 58, 49, 59, 34, 32, 100, 61, 34, 77, 32, 54, 54, 46, 57, 54, 52, 56, 52, 52, 32, 50, 51, 46, 54, 52, 56, 52, 51, 56, 32, 67, 32, 54, 54, 46, 52, 55, 50, 54, 53, 54, 32, 50, 50, 46, 49, 53, 50, 51, 52, 52, 32, 54, 52, 46, 56, 53, 49, 53, 54, 50, 32, 50, 49, 46, 51, 51, 50, 48, 51, 49, 32, 54, 51, 46, 51, 51, 57, 56, 52, 52, 32, 50, 49, 46, 56, 49, 54, 52, 48, 54, 32, 67, 32, 54, 49, 46, 56, 51, 50, 48, 51, 49, 32, 50, 50, 46, 51, 48, 52, 54, 56, 56, 32, 54, 49, 46, 48, 48, 51, 57, 48, 54, 32, 50, 51, 46, 57, 49, 52, 48, 54, 50, 32, 54, 49, 46, 52, 57, 54, 48, 57, 52, 32, 50, 53, 46, 52, 49, 48, 49, 53, 54, 32, 67, 32, 54, 50, 46, 50, 53, 55, 56, 49, 50, 32, 50, 55, 46, 55, 51, 52, 51, 55, 53, 32, 54, 50, 46, 50, 57, 50, 57, 54, 57, 32, 51, 48, 46, 55, 48, 55, 48, 51, 49, 32, 54, 50, 46, 50, 57, 50, 57, 54, 57, 32, 51, 56, 32, 67, 32, 54, 50, 46, 50, 57, 50, 57, 54, 57, 32, 52, 53, 46, 50, 57, 50, 57, 54, 57, 32, 54, 50, 46, 50, 53, 55, 56, 49, 50, 32, 52, 56, 46, 50, 54, 53, 54, 50, 53, 32, 54, 49, 46, 52, 57, 54, 48, 57, 52, 32, 53, 48, 46, 53, 56, 57, 56, 52, 52, 32, 67, 32, 53, 57, 46, 56, 56, 50, 56, 49, 50, 32, 53, 53, 46, 53, 48, 55, 56, 49, 50, 32, 53, 53, 46, 57, 57, 54, 48, 57, 52, 32, 53, 57, 46, 51, 54, 51, 50, 56, 49, 32, 53, 49, 46, 48, 51, 53, 49, 53, 54, 32, 54, 48, 46, 57, 54, 48, 57, 51, 56, 32, 67, 32, 52, 56, 46, 54, 57, 49, 52, 48, 54, 32, 54, 49, 46, 55, 49, 52, 56, 52, 52, 32, 52, 53, 46, 54, 57, 49, 52, 48, 54, 32, 54, 49, 46, 55, 53, 32, 51, 56, 46, 51, 51, 50, 48, 51, 49, 32, 54, 49, 46, 55, 53, 32, 76, 32, 51, 48, 46, 54, 54, 55, 57, 54, 57, 32, 54, 49, 46, 55, 53, 32, 67, 32, 50, 54, 46, 57, 53, 55, 48, 51, 49, 32, 54, 49, 46, 55, 53, 32, 50, 52, 46, 51, 53, 57, 51, 55, 53, 32, 54, 49, 46, 55, 52, 50, 49, 56, 56, 32, 50, 50, 46, 51, 57, 48, 54, 50, 53, 32, 54, 49, 46, 54, 51, 54, 55, 49, 57, 32, 76, 32, 50, 53, 46, 48, 51, 49, 50, 53, 32, 53, 57, 46, 48, 49, 53, 54, 50, 53, 32, 67, 32, 50, 54, 46, 49, 53, 54, 50, 53, 32, 53, 55, 46, 57, 48, 50, 51, 52, 52, 32, 50, 54, 46, 49, 53, 54, 50, 53, 32, 53, 54, 46, 48, 57, 55, 54, 53, 54, 32, 50, 53, 46, 48, 51, 49, 50, 53, 32, 53, 52, 46, 57, 56, 52, 51, 55, 53, 32, 67, 32, 50, 51, 46, 57, 49, 48, 49, 53, 54, 32, 53, 51, 46, 56, 55, 49, 48, 57, 52, 32, 50, 50, 46, 48, 56, 57, 56, 52, 52, 32, 53, 51, 46, 56, 55, 49, 48, 57, 52, 32, 50, 48, 46, 57, 54, 56, 55, 53, 32, 53, 52, 46, 57, 56, 52, 51, 55, 53, 32, 76, 32, 49, 54, 46, 50, 54, 57, 53, 51, 49, 32, 53, 57, 46, 54, 52, 48, 54, 50, 53, 32, 67, 32, 49, 53, 46, 50, 52, 50, 49, 56, 56, 32, 54, 48, 46, 54, 54, 48, 49, 53, 54, 32, 49, 52, 46, 54, 49, 51, 50, 56, 49, 32, 54, 49, 46, 57, 51, 51, 53, 57, 52, 32, 49, 52, 46, 51, 57, 52, 53, 51, 49, 32, 54, 51, 46, 50, 53, 51, 57, 48, 54, 32, 67, 32, 49, 52, 46, 48, 49, 57, 53, 51, 49, 32, 54, 53, 46, 52, 56, 48, 52, 54, 57, 32, 49, 52, 46, 56, 48, 52, 54, 56, 56, 32, 54, 55, 46, 56, 55, 53, 32, 49, 54, 46, 55, 49, 56, 55, 53, 32, 54, 57, 46, 52, 53, 51, 49, 50, 53, 32, 76, 32, 50, 50, 46, 49, 49, 55, 49, 56, 56, 32, 55, 51, 46, 57, 49, 52, 48, 54, 50, 32, 67, 32, 50, 51, 46, 51, 51, 53, 57, 51, 56, 32, 55, 52, 46, 57, 50, 49, 56, 55, 53, 32, 50, 53, 46, 49, 53, 50, 51, 52, 52, 32, 55, 52, 46, 55, 53, 55, 56, 49, 50, 32, 50, 54, 46, 49, 54, 55, 57, 54, 57, 32, 55, 51, 46, 53, 53, 48, 55, 56, 49, 32, 67, 32, 50, 55, 46, 49, 56, 51, 53, 57, 52, 32, 55, 50, 46, 51, 51, 57, 56, 52, 52, 32, 50, 55, 46, 48, 49, 57, 53, 51, 49, 32, 55, 48, 46, 53, 52, 50, 57, 54, 57, 32, 50, 53, 46, 56, 48, 48, 55, 56, 49, 32, 54, 57, 46, 53, 51, 53, 49, 53, 54, 32, 76, 32, 50, 51, 46, 49, 56, 55, 53, 32, 54, 55, 46, 51, 55, 56, 57, 48, 54, 32, 67, 32, 50, 53, 46, 49, 52, 48, 54, 50, 53, 32, 54, 55, 46, 52, 52, 57, 50, 49, 57, 32, 50, 55, 46, 51, 57, 48, 54, 50, 53, 32, 54, 55, 46, 52, 52, 57, 50, 49, 57, 32, 51, 48, 46, 48, 55, 48, 51, 49, 50, 32, 54, 55, 46, 52, 52, 57, 50, 49, 57, 32, 76, 32, 51, 56, 46, 57, 53, 51, 49, 50, 53, 32, 54, 55, 46, 52, 52, 57, 50, 49, 57, 32, 67, 32, 52, 53, 46, 52, 57, 54, 48, 57, 52, 32, 54, 55, 46, 52, 52, 57, 50, 49, 57, 32, 52, 57, 46, 52, 56, 52, 51, 55, 53, 32, 54, 55, 46, 52, 53, 51, 49, 50, 53, 32, 53, 50, 46, 56, 49, 50, 53, 32, 54, 54, 46, 51, 55, 56, 57, 48, 54, 32, 67, 32, 53, 57, 46, 53, 50, 51, 52, 51, 56, 32, 54, 52, 46, 50, 49, 56, 55, 53, 32, 54, 52, 46, 55, 56, 49, 50, 53, 32, 53, 57, 46, 48, 48, 51, 57, 48, 54, 32, 54, 54, 46, 57, 54, 52, 56, 52, 52, 32, 53, 50, 46, 51, 53, 49, 53, 54, 50, 32, 67, 32, 54, 56, 46, 48, 52, 50, 57, 54, 57, 32, 52, 57, 46, 48, 53, 52, 54, 56, 56, 32, 54, 56, 46, 48, 52, 50, 57, 54, 57, 32, 52, 53, 46, 48, 57, 55, 54, 53, 54, 32, 54, 56, 46, 48, 52, 50, 57, 54, 57, 32, 51, 56, 46, 54, 49, 51, 50, 56, 49, 32, 76, 32, 54, 56, 46, 48, 52, 50, 57, 54, 57, 32, 51, 55, 46, 51, 56, 54, 55, 49, 57, 32, 67, 32, 54, 56, 46, 48, 52, 50, 57, 54, 57, 32, 51, 48, 46, 57, 48, 50, 51, 52, 52, 32, 54, 56, 46, 48, 52, 50, 57, 54, 57, 32, 50, 54, 46, 57, 52, 53, 51, 49, 50, 32, 54, 54, 46, 57, 54, 52, 56, 52, 52, 32, 50, 51, 46, 54, 52, 56, 52, 51, 56, 32, 90, 32, 77, 32, 54, 54, 46, 57, 54, 52, 56, 52, 52, 32, 50, 51, 46, 54, 52, 56, 52, 51, 56, 32, 34, 47, 62, 10, 60, 47, 103, 62, 10, 60, 47, 115, 118, 103, 62, 10],
    suspend: &[60, 63, 120, 109, 108, 32, 118, 101, 114, 115, 105, 111, 110, 61, 34, 49, 46, 48, 34, 32, 101, 110, 99, 111, 100, 105, 110, 103, 61, 34, 85, 84, 70, 45, 56, 34, 63, 62, 10, 60, 115, 118, 103, 32, 120, 109, 108, 110, 115, 61, 34, 104, 116, 116, 112, 58, 47, 47, 119, 119, 119, 46, 119, 51, 46, 111, 114, 103, 47, 50, 48, 48, 48, 47, 115, 118, 103, 34, 32, 120, 109, 108, 110, 115, 58, 120, 108, 105, 110, 107, 61, 34, 104, 116, 116, 112, 58, 47, 47, 119, 119, 119, 46, 119, 51, 46, 111, 114, 103, 47, 49, 57, 57, 57, 47, 120, 108, 105, 110, 107, 34, 32, 119, 105, 100, 116, 104, 61, 34, 54, 57, 112, 120, 34, 32, 104, 101, 105, 103, 104, 116, 61, 34, 54, 57, 112, 120, 34, 32, 118, 105, 101, 119, 66, 111, 120, 61, 34, 48, 32, 48, 32, 54, 57, 32, 54, 57, 34, 32, 118, 101, 114, 115, 105, 111, 110, 61, 34, 49, 46, 49, 34, 62, 10, 60, 103, 32, 105, 100, 61, 34, 115, 117, 114, 102, 97, 99, 101, 49, 34, 62, 10, 60, 112, 97, 116, 104, 32, 115, 116, 121, 108, 101, 61, 34, 32, 115, 116, 114, 111, 107, 101, 58, 110, 111, 110, 101, 59, 102, 105, 108, 108, 45, 114, 117, 108, 101, 58, 101, 118, 101, 110, 111, 100, 100, 59, 102, 105, 108, 108, 58, 114, 103, 98, 40, 49, 48, 48, 37, 44, 49, 48, 48, 37, 44, 49, 48, 48, 37, 41, 59, 102, 105, 108, 108, 45, 111, 112, 97, 99, 105, 116, 121, 58, 49, 59, 34, 32, 100, 61, 34, 77, 32, 51, 51, 46, 53, 53, 52, 54, 56, 56, 32, 49, 49, 46, 54, 48, 49, 53, 54, 50, 32, 67, 32, 51, 50, 46, 55, 54, 53, 54, 50, 53, 32, 49, 54, 46, 57, 50, 57, 54, 56, 56, 32, 51, 49, 46, 55, 54, 57, 53, 51, 49, 32, 50, 53, 46, 50, 51, 56, 50, 56, 49, 32, 51, 55, 46, 49, 52, 48, 54, 50, 53, 32, 51, 52, 46, 53, 51, 57, 48, 54, 50, 32, 67, 32, 52, 50, 46, 53, 49, 49, 55, 49, 57, 32, 52, 51, 46, 56, 52, 51, 55, 53, 32, 53, 48, 46, 50, 48, 51, 49, 50, 53, 32, 52, 55, 46, 49, 51, 54, 55, 49, 57, 32, 53, 53, 46, 50, 49, 52, 56, 52, 52, 32, 52, 57, 46, 49, 49, 55, 49, 56, 56, 32, 67, 32, 53, 53, 46, 51, 56, 54, 55, 49, 57, 32, 52, 57, 46, 49, 56, 51, 53, 57, 52, 32, 53, 53, 46, 53, 53, 56, 53, 57, 52, 32, 52, 57, 46, 50, 53, 51, 57, 48, 54, 32, 53, 53, 46, 55, 51, 48, 52, 54, 57, 32, 52, 57, 46, 51, 50, 48, 51, 49, 50, 32, 67, 32, 53, 54, 46, 55, 54, 57, 53, 51, 49, 32, 52, 57, 46, 55, 50, 54, 53, 54, 50, 32, 53, 55, 46, 55, 54, 53, 54, 50, 53, 32, 53, 48, 46, 49, 50, 49, 48, 57, 52, 32, 53, 56, 46, 53, 51, 57, 48, 54, 50, 32, 53, 48, 46, 53, 48, 51, 57, 48, 54, 32, 67, 32, 53, 57, 32, 53, 48, 46, 55, 51, 52, 51, 55, 53, 32, 53, 57, 46, 52, 56, 56, 50, 56, 49, 32, 53, 49, 46, 48, 49, 49, 55, 49, 57, 32, 53, 57, 46, 57, 49, 52, 48, 54, 50, 32, 53, 49, 46, 51, 53, 53, 52, 54, 57, 32, 67, 32, 54, 48, 46, 51, 51, 50, 48, 51, 49, 32, 53, 49, 46, 54, 57, 57, 50, 49, 57, 32, 54, 48, 46, 56, 52, 51, 55, 53, 32, 53, 50, 46, 50, 51, 52, 51, 55, 53, 32, 54, 49, 46, 48, 56, 53, 57, 51, 56, 32, 53, 51, 46, 48, 49, 53, 54, 50, 53, 32, 67, 32, 54, 49, 46, 54, 51, 50, 56, 49, 50, 32, 53, 52, 46, 55, 57, 54, 56, 55, 53, 32, 54, 48, 46, 52, 49, 52, 48, 54, 50, 32, 53, 54, 46, 49, 52, 56, 52, 51, 56, 32, 53, 57, 46, 54, 54, 55, 57, 54, 57, 32, 53, 54, 46, 56, 50, 56, 49, 50, 53, 32, 67, 32, 53, 56, 46, 55, 56, 49, 50, 53, 32, 53, 55, 46, 54, 51, 50, 56, 49, 50, 32, 53, 55, 46, 52, 52, 49, 52, 48, 54, 32, 53, 56, 46, 53, 48, 55, 56, 49, 50, 32, 53, 53, 46, 55, 48, 55, 48, 51, 49, 32, 53, 57, 46, 53, 48, 55, 56, 49, 50, 32, 67, 32, 52, 48, 46, 57, 50, 53, 55, 56, 49, 32, 54, 56, 46, 48, 52, 50, 57, 54, 57, 32, 50, 50, 46, 48, 50, 51, 52, 51, 56, 32, 54, 50, 46, 57, 56, 48, 52, 54, 57, 32, 49, 51, 46, 52, 56, 56, 50, 56, 49, 32, 52, 56, 46, 49, 57, 53, 51, 49, 50, 32, 67, 32, 52, 46, 57, 53, 51, 49, 50, 53, 32, 51, 51, 46, 52, 49, 52, 48, 54, 50, 32, 49, 48, 46, 48, 49, 57, 53, 51, 49, 32, 49, 52, 46, 53, 49, 49, 55, 49, 57, 32, 50, 52, 46, 56, 48, 48, 55, 56, 49, 32, 53, 46, 57, 55, 54, 53, 54, 50, 32, 67, 32, 50, 54, 46, 53, 51, 53, 49, 53, 54, 32, 52, 46, 57, 55, 54, 53, 54, 50, 32, 50, 55, 46, 57, 54, 48, 57, 51, 56, 32, 52, 46, 50, 53, 51, 57, 48, 54, 32, 50, 57, 46, 49, 48, 49, 53, 54, 50, 32, 51, 46, 56, 56, 54, 55, 49, 57, 32, 67, 32, 51, 48, 46, 48, 54, 54, 52, 48, 54, 32, 51, 46, 53, 56, 50, 48, 51, 49, 32, 51, 49, 46, 56, 52, 51, 55, 53, 32, 51, 46, 50, 48, 51, 49, 50, 53, 32, 51, 51, 46, 49, 49, 51, 50, 56, 49, 32, 52, 46, 53, 54, 54, 52, 48, 54, 32, 67, 32, 51, 51, 46, 54, 54, 55, 57, 54, 57, 32, 53, 46, 49, 54, 55, 57, 54, 57, 32, 51, 51, 46, 56, 55, 56, 57, 48, 54, 32, 53, 46, 56, 55, 56, 57, 48, 54, 32, 51, 51, 46, 57, 54, 52, 56, 52, 52, 32, 54, 46, 52, 49, 48, 49, 53, 54, 32, 67, 32, 51, 52, 46, 48, 53, 48, 55, 56, 49, 32, 54, 46, 57, 53, 51, 49, 50, 53, 32, 51, 52, 46, 48, 52, 54, 56, 55, 53, 32, 55, 46, 53, 49, 53, 54, 50, 53, 32, 51, 52, 46, 48, 49, 53, 54, 50, 53, 32, 56, 46, 48, 50, 55, 51, 52, 52, 32, 67, 32, 51, 51, 46, 57, 54, 48, 57, 51, 56, 32, 56, 46, 56, 57, 48, 54, 50, 53, 32, 51, 51, 46, 56, 48, 48, 55, 56, 49, 32, 57, 46, 57, 52, 57, 50, 49, 57, 32, 51, 51, 46, 54, 51, 54, 55, 49, 57, 32, 49, 49, 46, 48, 53, 48, 55, 56, 49, 32, 67, 32, 51, 51, 46, 54, 48, 57, 51, 55, 53, 32, 49, 49, 46, 50, 51, 52, 51, 55, 53, 32, 51, 51, 46, 53, 56, 50, 48, 51, 49, 32, 49, 49, 46, 52, 49, 55, 57, 54, 57, 32, 51, 51, 46, 53, 53, 52, 54, 56, 56, 32, 49, 49, 46, 54, 48, 49, 53, 54, 50, 32, 90, 32, 77, 32, 50, 57, 46, 54, 54, 52, 48, 54, 50, 32, 56, 46, 50, 56, 57, 48, 54, 50, 32, 67, 32, 50, 57, 46, 48, 49, 49, 55, 49, 57, 32, 56, 46, 53, 55, 56, 49, 50, 53, 32, 50, 56, 46, 49, 51, 50, 56, 49, 50, 32, 57, 46, 48, 51, 53, 49, 53, 54, 32, 50, 54, 46, 57, 53, 55, 48, 51, 49, 32, 57, 46, 55, 49, 48, 57, 51, 56, 32, 67, 32, 49, 52, 46, 50, 51, 56, 50, 56, 49, 32, 49, 55, 46, 48, 53, 52, 54, 56, 56, 32, 57, 46, 56, 55, 56, 57, 48, 54, 32, 51, 51, 46, 51, 50, 48, 51, 49, 50, 32, 49, 55, 46, 50, 50, 50, 54, 53, 54, 32, 52, 54, 46, 48, 51, 57, 48, 54, 50, 32, 67, 32, 50, 52, 46, 53, 54, 54, 52, 48, 54, 32, 53, 56, 46, 55, 53, 55, 56, 49, 50, 32, 52, 48, 46, 56, 51, 50, 48, 51, 49, 32, 54, 51, 46, 49, 49, 55, 49, 56, 56, 32, 53, 51, 46, 53, 53, 48, 55, 56, 49, 32, 53, 53, 46, 55, 55, 51, 52, 51, 56, 32, 67, 32, 53, 52, 46, 55, 50, 54, 53, 54, 50, 32, 53, 53, 46, 48, 57, 55, 54, 53, 54, 32, 53, 53, 46, 53, 54, 50, 53, 32, 53, 52, 46, 53, 54, 50, 53, 32, 53, 54, 46, 49, 51, 54, 55, 49, 57, 32, 53, 52, 46, 49, 52, 48, 54, 50, 53, 32, 67, 32, 53, 53, 46, 54, 49, 55, 49, 56, 56, 32, 53, 51, 46, 57, 49, 48, 49, 53, 54, 32, 53, 52, 46, 57, 55, 50, 54, 53, 54, 32, 53, 51, 46, 54, 53, 54, 50, 53, 32, 53, 52, 46, 49, 55, 53, 55, 56, 49, 32, 53, 51, 46, 51, 52, 51, 55, 53, 32, 67, 32, 53, 52, 32, 53, 51, 46, 50, 55, 51, 52, 51, 56, 32, 53, 51, 46, 56, 50, 48, 51, 49, 50, 32, 53, 51, 46, 50, 48, 51, 49, 50, 53, 32, 53, 51, 46, 54, 50, 56, 57, 48, 54, 32, 53, 51, 46, 49, 50, 56, 57, 48, 54, 32, 67, 32, 52, 56, 46, 51, 57, 56, 52, 51, 56, 32, 53, 49, 46, 48, 53, 56, 53, 57, 52, 32, 51, 57, 46, 53, 51, 53, 49, 53, 54, 32, 52, 55, 46, 51, 49, 50, 53, 32, 51, 51, 46, 52, 48, 54, 50, 53, 32, 51, 54, 46, 54, 57, 53, 51, 49, 50, 32, 67, 32, 50, 55, 46, 50, 55, 55, 51, 52, 52, 32, 50, 54, 46, 48, 56, 50, 48, 51, 49, 32, 50, 56, 46, 52, 54, 52, 56, 52, 52, 32, 49, 54, 46, 53, 51, 49, 50, 53, 32, 50, 57, 46, 50, 56, 57, 48, 54, 50, 32, 49, 48, 46, 57, 54, 56, 55, 53, 32, 67, 32, 50, 57, 46, 51, 50, 48, 51, 49, 50, 32, 49, 48, 46, 55, 54, 53, 54, 50, 53, 32, 50, 57, 46, 51, 52, 55, 54, 53, 54, 32, 49, 48, 46, 53, 55, 48, 51, 49, 50, 32, 50, 57, 46, 51, 55, 53, 32, 49, 48, 46, 51, 56, 54, 55, 49, 57, 32, 67, 32, 50, 57, 46, 53, 32, 57, 46, 53, 51, 57, 48, 54, 50, 32, 50, 57, 46, 54, 48, 53, 52, 54, 57, 32, 56, 46, 56, 53, 53, 52, 54, 57, 32, 50, 57, 46, 54, 54, 52, 48, 54, 50, 32, 56, 46, 50, 56, 57, 48, 54, 50, 32, 90, 32, 77, 32, 51, 48, 46, 56, 49, 54, 52, 48, 54, 32, 55, 46, 57, 48, 50, 51, 52, 52, 32, 67, 32, 51, 48, 46, 56, 49, 54, 52, 48, 54, 32, 55, 46, 57, 48, 50, 51, 52, 52, 32, 51, 48, 46, 55, 57, 50, 57, 54, 57, 32, 55, 46, 57, 48, 54, 50, 53, 32, 51, 48, 46, 55, 53, 55, 56, 49, 50, 32, 55, 46, 57, 48, 54, 50, 53, 32, 67, 32, 51, 48, 46, 55, 57, 54, 56, 55, 53, 32, 55, 46, 56, 57, 56, 52, 51, 56, 32, 51, 48, 46, 56, 49, 54, 52, 48, 54, 32, 55, 46, 57, 48, 50, 51, 52, 52, 32, 51, 48, 46, 56, 49, 54, 52, 48, 54, 32, 55, 46, 57, 48, 50, 51, 52, 52, 32, 90, 32, 77, 32, 53, 55, 46, 48, 49, 53, 54, 50, 53, 32, 53, 51, 46, 51, 56, 50, 56, 49, 50, 32, 67, 32, 53, 55, 46, 48, 51, 53, 49, 53, 54, 32, 53, 51, 46, 51, 53, 53, 52, 54, 57, 32, 53, 55, 46, 48, 52, 54, 56, 55, 53, 32, 53, 51, 46, 51, 51, 57, 56, 52, 52, 32, 53, 55, 46, 48, 52, 54, 56, 55, 53, 32, 53, 51, 46, 51, 51, 57, 56, 52, 52, 32, 67, 32, 53, 55, 46, 48, 52, 54, 56, 55, 53, 32, 53, 51, 46, 51, 51, 57, 56, 52, 52, 32, 53, 55, 46, 48, 52, 50, 57, 54, 57, 32, 53, 51, 46, 51, 53, 53, 52, 54, 57, 32, 53, 55, 46, 48, 49, 53, 54, 50, 53, 32, 53, 51, 46, 51, 56, 50, 56, 49, 50, 32, 90, 32, 77, 32, 53, 55, 46, 48, 49, 53, 54, 50, 53, 32, 53, 51, 46, 51, 56, 50, 56, 49, 50, 32, 34, 47, 62, 10, 60, 47, 103, 62, 10, 60, 47, 115, 118, 103, 62, 10],
};