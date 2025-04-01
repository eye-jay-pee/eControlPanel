// fn test_height(subject: &Specimen) {
//     assert_eq!(, subject.2)
//     match super::Height::new(subject.0, subject.1) {
//         Resu
//     }
// }
//
// #[test]
// fn test_bounds() {
//     for subject in [
//         Specimen(0, 0, Err(HeightError::TooShort)),
//         Specimen(0, 11, Err(HeightError::TooShort)),
//         Specimen(1, 0, Ok(Height { feet: 1, inches: 0 })),
//         Specimen(
//             9,
//             11,
//             Ok(Height {
//                 feet: 9,
//                 inches: 11,
//             }),
//         ),
//         Specimen(10, 0, Err(HeightError::TooTall)),
//     ] {
//         test_height(&s);
//     }
// }
// #[test]
// fn test_normalization() {
//     for subject in [
//         Specimen(0, (2 * 12), Ok(Height { feet: 2, inches: 0 })),
//         Specimen(1, (3 * 12), Ok(Height { feet: 3, inches: 0 })),
//         Specimen(8, 30, Err(HeightError::TooTall)),
//     ] {
//         test_height(&s);
//     }
// }
