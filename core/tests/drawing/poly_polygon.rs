use wmf_core::{
    converter::{Player, SVGPlayer},
    parser::{
        META_POLYPOLYGON, META_SETPOLYFILLMODE, META_SETWINDOWEXT, PointS,
        PolyFillMode, PolyPolygon,
    },
};

#[test]
fn meta_poly_polygon_svg_table_test() {
    struct TestCase {
        desc: &'static str,
        fill_mode: PolyFillMode,
        record: META_POLYPOLYGON,
        expected_svg: &'static str,
    }

    // Outer square + inner square — the "donut" / glyph-hole case that
    // regressed numbers like "0" in CHEQBOOK.WMF. The two sub-polygons MUST
    // be emitted as one <path> with two subpaths so fill-rule can subtract
    // the inner ring from the outer.
    let donut = || META_POLYPOLYGON {
        record_size: 0.into(),
        record_function: 0,
        poly_polygon: PolyPolygon {
            number_of_polygons: 2,
            a_points_per_polygon: vec![4, 4],
            a_points: vec![
                PointS { x: 10, y: 10 },
                PointS { x: 90, y: 10 },
                PointS { x: 90, y: 90 },
                PointS { x: 10, y: 90 },
                PointS { x: 30, y: 30 },
                PointS { x: 70, y: 30 },
                PointS { x: 70, y: 70 },
                PointS { x: 30, y: 70 },
            ],
        },
    };

    let cases = [
        TestCase {
            desc: "Outer + inner ring (evenodd) emits one path with two \
                   subpaths",
            fill_mode: PolyFillMode::ALTERNATE,
            record: donut(),
            expected_svg: r##"<svg viewBox="0 0 1024 1024" xmlns="http://www.w3.org/2000/svg"><path d="M 10,10 L 90,10 L 90,90 L 10,90 Z M 30,30 L 70,30 L 70,70 L 30,70 Z" fill="none" fill-rule="evenodd" id="elem1" stroke="#000000" stroke-dasharray="none" stroke-linecap="butt" stroke-linejoin="miter" stroke-opacity="1.00" stroke-width="1"></path></svg>"##,
        },
        TestCase {
            desc: "Outer + inner ring (nonzero) carries the winding \
                   fill-rule",
            fill_mode: PolyFillMode::WINDING,
            record: donut(),
            expected_svg: r##"<svg viewBox="0 0 1024 1024" xmlns="http://www.w3.org/2000/svg"><path d="M 10,10 L 90,10 L 90,90 L 10,90 Z M 30,30 L 70,30 L 70,70 L 30,70 Z" fill="none" fill-rule="nonzero" id="elem1" stroke="#000000" stroke-dasharray="none" stroke-linecap="butt" stroke-linejoin="miter" stroke-opacity="1.00" stroke-width="1"></path></svg>"##,
        },
        TestCase {
            desc: "Single sub-polygon still produces a single path",
            fill_mode: PolyFillMode::ALTERNATE,
            record: META_POLYPOLYGON {
                record_size: 0.into(),
                record_function: 0,
                poly_polygon: PolyPolygon {
                    number_of_polygons: 1,
                    a_points_per_polygon: vec![3],
                    a_points: vec![
                        PointS { x: 0, y: 0 },
                        PointS { x: 100, y: 0 },
                        PointS { x: 50, y: 100 },
                    ],
                },
            },
            expected_svg: r##"<svg viewBox="0 0 1024 1024" xmlns="http://www.w3.org/2000/svg"><path d="M 0,0 L 100,0 L 50,100 Z" fill="none" fill-rule="evenodd" id="elem1" stroke="#000000" stroke-dasharray="none" stroke-linecap="butt" stroke-linejoin="miter" stroke-opacity="1.00" stroke-width="1"></path></svg>"##,
        },
        TestCase {
            desc: "Zero-point sub-polygon is skipped without consuming \
                   subsequent points",
            fill_mode: PolyFillMode::ALTERNATE,
            record: META_POLYPOLYGON {
                record_size: 0.into(),
                record_function: 0,
                poly_polygon: PolyPolygon {
                    number_of_polygons: 2,
                    a_points_per_polygon: vec![0, 3],
                    a_points: vec![
                        PointS { x: 0, y: 0 },
                        PointS { x: 10, y: 0 },
                        PointS { x: 5, y: 10 },
                    ],
                },
            },
            expected_svg: r##"<svg viewBox="0 0 1024 1024" xmlns="http://www.w3.org/2000/svg"><path d="M 0,0 L 10,0 L 5,10 Z" fill="none" fill-rule="evenodd" id="elem1" stroke="#000000" stroke-dasharray="none" stroke-linecap="butt" stroke-linejoin="miter" stroke-opacity="1.00" stroke-width="1"></path></svg>"##,
        },
    ];

    for (i, case) in cases.iter().enumerate() {
        let player = SVGPlayer::new();
        let player = player
            .set_window_ext(0, META_SETWINDOWEXT {
                record_size: 0.into(),
                record_function: 0,
                y: 1024,
                x: 1024,
            })
            .expect("set_window_ext failed");
        let player = player
            .set_polyfill_mode(0, META_SETPOLYFILLMODE {
                record_size: 0.into(),
                record_function: 0,
                poly_fill_mode: case.fill_mode,
                reserved: None,
            })
            .expect("set_polyfill_mode failed");
        let result = player.poly_polygon(1, case.record.clone());

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
