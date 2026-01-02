//! SVG conversion test for META_CHORD

use wmf_core::{
    converter::svg::SVGPlayer, parser::records::drawing::META_CHORD,
};

#[derive(Clone)]
struct ChordTestCase {
    desc: &'static str,
    record: META_CHORD,
    // If None, expect no <path> element in output
    expected_svg: Option<&'static str>,
}

#[test]
fn meta_chord_svg_table_test() {
    let cases = [
        ChordTestCase {
            desc: "Standard horizontal ellipse chord",
            record: META_CHORD {
                record_size: 0.into(),
                record_function: 0,
                y_radial2: 80,
                x_radial2: 120,
                y_radial1: 80,
                x_radial1: 40,
                bottom_rect: 120,
                right_rect: 120,
                top_rect: 40,
                left_rect: 40,
            },
            expected_svg: Some(
                r#"<svg xmlns=\"http://www.w3.org/2000/svg\" viewBox=\"0 0 1024 1024\">
<path fill=\"none\" fill-rule=\"evenodd\" d=\"M 40 80 A 40 40 0 0 1 120 80 L 80 80 Z\" stroke=\"black\" />
</svg>
"#,
            ),
        },
        ChordTestCase {
            desc: "Vertical ellipse chord",
            record: META_CHORD {
                record_size: 0.into(),
                record_function: 0,
                y_radial2: 120,
                x_radial2: 80,
                y_radial1: 40,
                x_radial1: 80,
                bottom_rect: 120,
                right_rect: 120,
                top_rect: 40,
                left_rect: 40,
            },
            expected_svg: Some(
                r#"<svg xmlns=\"http://www.w3.org/2000/svg\" viewBox=\"0 0 1024 1024\">
<path fill=\"none\" fill-rule=\"evenodd\" d=\"M 80 40 A 40 40 0 0 1 80 120 L 80 80 Z\" stroke=\"black\" />
</svg>
"#,
            ),
        },
        ChordTestCase {
            desc: "Skip when rx/ry is zero",
            record: META_CHORD {
                record_size: 0.into(),
                record_function: 0,
                y_radial2: 80,
                x_radial2: 120,
                y_radial1: 80,
                x_radial1: 40,
                bottom_rect: 40,
                right_rect: 40,
                top_rect: 40,
                left_rect: 40,
            },
            expected_svg: None,
        },
    ];
    for (i, case) in cases.iter().enumerate() {
        let mut player = SVGPlayer::new();
        let result = player.chord(i + 1, case.record.clone());
        assert!(result.is_ok(), "{}: Rendering failed", case.desc);
        let player = result.unwrap();
        let svg = player.generate().expect("SVG generation failed");
        let svg_str = String::from_utf8(svg).expect("SVG output is not UTF-8");
        match case.expected_svg {
            Some(expected_svg) => {
                assert_eq!(
                    svg_str.trim(),
                    expected_svg.trim(),
                    "{}: SVG output does not match expected\nOutput: \
                     {}\nExpected: {}",
                    case.desc,
                    svg_str,
                    expected_svg
                );
            }
            None => {
                assert!(
                    !svg_str.contains("<path"),
                    "{}: <path> element must not be present in SVG\nOutput: {}",
                    case.desc,
                    svg_str
                );
            }
        }
    }
}
