use wmf_core::{
    converter::{Player, SVGPlayer},
    parser::META_CHORD,
};

#[test]
fn meta_chord_svg_table_test() {
    struct TestCase {
        desc: &'static str,
        record: META_CHORD,
        expected_svg: &'static str,
    }

    let cases = [
        TestCase {
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
            expected_svg: r##"<svg viewBox="0 0 1024 1024" xmlns="http://www.w3.org/2000/svg"><path d="M 40 80 A 40 40 0 0 1 120 80 L 80 80 Z" fill="none" fill-rule="evenodd" id="elem1" stroke="#000000" stroke-dasharray="none" stroke-linecap="butt" stroke-linejoin="miter" stroke-opacity="1.00" stroke-width="1"></path></svg>"##,
        },
        TestCase {
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
            expected_svg: r##"<svg viewBox="0 0 1024 1024" xmlns="http://www.w3.org/2000/svg"><path d="M 80 40 A 40 40 0 0 1 80 120 L 80 80 Z" fill="none" fill-rule="evenodd" id="elem1" stroke="#000000" stroke-dasharray="none" stroke-linecap="butt" stroke-linejoin="miter" stroke-opacity="1.00" stroke-width="1"></path></svg>"##,
        },
        TestCase {
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
            expected_svg: r#"<svg viewBox="0 0 1024 1024" xmlns="http://www.w3.org/2000/svg"></svg>"#,
        },
    ];

    for (i, case) in cases.iter().enumerate() {
        let player = SVGPlayer::new();
        let result = player.chord(1, case.record.clone());

        assert!(result.is_ok(), "case {i}: {}: Rendering failed", case.desc);

        let player = result.unwrap();
        let svg = player.generate().expect("SVG generation failed");
        let svg_str = String::from_utf8(svg).expect("SVG output is not UTF-8");

        assert_eq!(
            svg_str.trim(),
            case.expected_svg.trim(),
            "case {i}: {}: SVG output does not match expected",
            case.desc,
        );
    }
}
