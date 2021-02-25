// This file was generated by gir (https://github.com/gtk-rs/gir)
// from gir-files (https://github.com/gtk-rs/gir-files)
// DO NOT EDIT

use pango_sys::*;
use std::env;
use std::error::Error;
use std::ffi::OsString;
use std::mem::{align_of, size_of};
use std::path::Path;
use std::process::Command;
use std::str;
use tempfile::Builder;

static PACKAGES: &[&str] = &["pango"];

#[derive(Clone, Debug)]
struct Compiler {
    pub args: Vec<String>,
}

impl Compiler {
    pub fn new() -> Result<Compiler, Box<dyn Error>> {
        let mut args = get_var("CC", "cc")?;
        args.push("-Wno-deprecated-declarations".to_owned());
        // For _Generic
        args.push("-std=c11".to_owned());
        // For %z support in printf when using MinGW.
        args.push("-D__USE_MINGW_ANSI_STDIO".to_owned());
        args.extend(get_var("CFLAGS", "")?);
        args.extend(get_var("CPPFLAGS", "")?);
        args.extend(pkg_config_cflags(PACKAGES)?);
        Ok(Compiler { args })
    }

    pub fn compile(&self, src: &Path, out: &Path) -> Result<(), Box<dyn Error>> {
        let mut cmd = self.to_command();
        cmd.arg(src);
        cmd.arg("-o");
        cmd.arg(out);
        let status = cmd.spawn()?.wait()?;
        if !status.success() {
            return Err(format!("compilation command {:?} failed, {}", &cmd, status).into());
        }
        Ok(())
    }

    fn to_command(&self) -> Command {
        let mut cmd = Command::new(&self.args[0]);
        cmd.args(&self.args[1..]);
        cmd
    }
}

fn get_var(name: &str, default: &str) -> Result<Vec<String>, Box<dyn Error>> {
    match env::var(name) {
        Ok(value) => Ok(shell_words::split(&value)?),
        Err(env::VarError::NotPresent) => Ok(shell_words::split(default)?),
        Err(err) => Err(format!("{} {}", name, err).into()),
    }
}

fn pkg_config_cflags(packages: &[&str]) -> Result<Vec<String>, Box<dyn Error>> {
    if packages.is_empty() {
        return Ok(Vec::new());
    }
    let pkg_config = env::var_os("PKG_CONFIG").unwrap_or_else(|| OsString::from("pkg-config"));
    let mut cmd = Command::new(pkg_config);
    cmd.arg("--cflags");
    cmd.args(packages);
    let out = cmd.output()?;
    if !out.status.success() {
        return Err(format!("command {:?} returned {}", &cmd, out.status).into());
    }
    let stdout = str::from_utf8(&out.stdout)?;
    Ok(shell_words::split(stdout.trim())?)
}

#[derive(Copy, Clone, Debug, Eq, PartialEq)]
struct Layout {
    size: usize,
    alignment: usize,
}

#[derive(Copy, Clone, Debug, Default, Eq, PartialEq)]
struct Results {
    /// Number of successfully completed tests.
    passed: usize,
    /// Total number of failed tests (including those that failed to compile).
    failed: usize,
}

impl Results {
    fn record_passed(&mut self) {
        self.passed += 1;
    }
    fn record_failed(&mut self) {
        self.failed += 1;
    }
    fn summary(&self) -> String {
        format!("{} passed; {} failed", self.passed, self.failed)
    }
    fn expect_total_success(&self) {
        if self.failed == 0 {
            println!("OK: {}", self.summary());
        } else {
            panic!("FAILED: {}", self.summary());
        };
    }
}

#[test]
fn cross_validate_constants_with_c() {
    let mut c_constants: Vec<(String, String)> = Vec::new();

    for l in get_c_output("constant").unwrap().lines() {
        let mut words = l.trim().split(';');
        let name = words.next().expect("Failed to parse name").to_owned();
        let value = words
            .next()
            .and_then(|s| s.parse().ok())
            .expect("Failed to parse value");
        c_constants.push((name, value));
    }

    let mut results = Results::default();

    for ((rust_name, rust_value), (c_name, c_value)) in
        RUST_CONSTANTS.iter().zip(c_constants.iter())
    {
        if rust_name != c_name {
            results.record_failed();
            eprintln!("Name mismatch:\nRust: {:?}\nC:    {:?}", rust_name, c_name,);
            continue;
        }

        if rust_value != c_value {
            results.record_failed();
            eprintln!(
                "Constant value mismatch for {}\nRust: {:?}\nC:    {:?}",
                rust_name, rust_value, &c_value
            );
            continue;
        }

        results.record_passed();
    }

    results.expect_total_success();
}

#[test]
fn cross_validate_layout_with_c() {
    let mut c_layouts = Vec::new();

    for l in get_c_output("layout").unwrap().lines() {
        let mut words = l.trim().split(';');
        let name = words.next().expect("Failed to parse name").to_owned();
        let size = words
            .next()
            .and_then(|s| s.parse().ok())
            .expect("Failed to parse size");
        let alignment = words
            .next()
            .and_then(|s| s.parse().ok())
            .expect("Failed to parse alignment");
        c_layouts.push((name, Layout { size, alignment }));
    }

    let mut results = Results::default();

    for ((rust_name, rust_layout), (c_name, c_layout)) in RUST_LAYOUTS.iter().zip(c_layouts.iter())
    {
        if rust_name != c_name {
            results.record_failed();
            eprintln!("Name mismatch:\nRust: {:?}\nC:    {:?}", rust_name, c_name,);
            continue;
        }

        if rust_layout != c_layout {
            results.record_failed();
            eprintln!(
                "Layout mismatch for {}\nRust: {:?}\nC:    {:?}",
                rust_name, rust_layout, &c_layout
            );
            continue;
        }

        results.record_passed();
    }

    results.expect_total_success();
}

fn get_c_output(name: &str) -> Result<String, Box<dyn Error>> {
    let tmpdir = Builder::new().prefix("abi").tempdir()?;
    let exe = tmpdir.path().join(name);
    let c_file = Path::new("tests").join(name).with_extension("c");

    let cc = Compiler::new().expect("configured compiler");
    cc.compile(&c_file, &exe)?;

    let mut abi_cmd = Command::new(exe);
    let output = abi_cmd.output()?;
    if !output.status.success() {
        return Err(format!("command {:?} failed, {:?}", &abi_cmd, &output).into());
    }

    Ok(String::from_utf8(output.stdout)?)
}

const RUST_LAYOUTS: &[(&str, Layout)] = &[
    (
        "PangoAlignment",
        Layout {
            size: size_of::<PangoAlignment>(),
            alignment: align_of::<PangoAlignment>(),
        },
    ),
    (
        "PangoAnalysis",
        Layout {
            size: size_of::<PangoAnalysis>(),
            alignment: align_of::<PangoAnalysis>(),
        },
    ),
    (
        "PangoAttrClass",
        Layout {
            size: size_of::<PangoAttrClass>(),
            alignment: align_of::<PangoAttrClass>(),
        },
    ),
    (
        "PangoAttrColor",
        Layout {
            size: size_of::<PangoAttrColor>(),
            alignment: align_of::<PangoAttrColor>(),
        },
    ),
    (
        "PangoAttrFloat",
        Layout {
            size: size_of::<PangoAttrFloat>(),
            alignment: align_of::<PangoAttrFloat>(),
        },
    ),
    (
        "PangoAttrFontDesc",
        Layout {
            size: size_of::<PangoAttrFontDesc>(),
            alignment: align_of::<PangoAttrFontDesc>(),
        },
    ),
    (
        "PangoAttrFontFeatures",
        Layout {
            size: size_of::<PangoAttrFontFeatures>(),
            alignment: align_of::<PangoAttrFontFeatures>(),
        },
    ),
    (
        "PangoAttrInt",
        Layout {
            size: size_of::<PangoAttrInt>(),
            alignment: align_of::<PangoAttrInt>(),
        },
    ),
    (
        "PangoAttrLanguage",
        Layout {
            size: size_of::<PangoAttrLanguage>(),
            alignment: align_of::<PangoAttrLanguage>(),
        },
    ),
    (
        "PangoAttrShape",
        Layout {
            size: size_of::<PangoAttrShape>(),
            alignment: align_of::<PangoAttrShape>(),
        },
    ),
    (
        "PangoAttrSize",
        Layout {
            size: size_of::<PangoAttrSize>(),
            alignment: align_of::<PangoAttrSize>(),
        },
    ),
    (
        "PangoAttrString",
        Layout {
            size: size_of::<PangoAttrString>(),
            alignment: align_of::<PangoAttrString>(),
        },
    ),
    (
        "PangoAttrType",
        Layout {
            size: size_of::<PangoAttrType>(),
            alignment: align_of::<PangoAttrType>(),
        },
    ),
    (
        "PangoAttribute",
        Layout {
            size: size_of::<PangoAttribute>(),
            alignment: align_of::<PangoAttribute>(),
        },
    ),
    (
        "PangoBidiType",
        Layout {
            size: size_of::<PangoBidiType>(),
            alignment: align_of::<PangoBidiType>(),
        },
    ),
    (
        "PangoColor",
        Layout {
            size: size_of::<PangoColor>(),
            alignment: align_of::<PangoColor>(),
        },
    ),
    (
        "PangoCoverageLevel",
        Layout {
            size: size_of::<PangoCoverageLevel>(),
            alignment: align_of::<PangoCoverageLevel>(),
        },
    ),
    (
        "PangoDirection",
        Layout {
            size: size_of::<PangoDirection>(),
            alignment: align_of::<PangoDirection>(),
        },
    ),
    (
        "PangoEllipsizeMode",
        Layout {
            size: size_of::<PangoEllipsizeMode>(),
            alignment: align_of::<PangoEllipsizeMode>(),
        },
    ),
    (
        "PangoEngine",
        Layout {
            size: size_of::<PangoEngine>(),
            alignment: align_of::<PangoEngine>(),
        },
    ),
    (
        "PangoEngineClass",
        Layout {
            size: size_of::<PangoEngineClass>(),
            alignment: align_of::<PangoEngineClass>(),
        },
    ),
    (
        "PangoEngineInfo",
        Layout {
            size: size_of::<PangoEngineInfo>(),
            alignment: align_of::<PangoEngineInfo>(),
        },
    ),
    (
        "PangoEngineLang",
        Layout {
            size: size_of::<PangoEngineLang>(),
            alignment: align_of::<PangoEngineLang>(),
        },
    ),
    (
        "PangoEngineLangClass",
        Layout {
            size: size_of::<PangoEngineLangClass>(),
            alignment: align_of::<PangoEngineLangClass>(),
        },
    ),
    (
        "PangoEngineScriptInfo",
        Layout {
            size: size_of::<PangoEngineScriptInfo>(),
            alignment: align_of::<PangoEngineScriptInfo>(),
        },
    ),
    (
        "PangoEngineShape",
        Layout {
            size: size_of::<PangoEngineShape>(),
            alignment: align_of::<PangoEngineShape>(),
        },
    ),
    (
        "PangoEngineShapeClass",
        Layout {
            size: size_of::<PangoEngineShapeClass>(),
            alignment: align_of::<PangoEngineShapeClass>(),
        },
    ),
    (
        "PangoFont",
        Layout {
            size: size_of::<PangoFont>(),
            alignment: align_of::<PangoFont>(),
        },
    ),
    (
        "PangoFontClass",
        Layout {
            size: size_of::<PangoFontClass>(),
            alignment: align_of::<PangoFontClass>(),
        },
    ),
    (
        "PangoFontFace",
        Layout {
            size: size_of::<PangoFontFace>(),
            alignment: align_of::<PangoFontFace>(),
        },
    ),
    (
        "PangoFontFaceClass",
        Layout {
            size: size_of::<PangoFontFaceClass>(),
            alignment: align_of::<PangoFontFaceClass>(),
        },
    ),
    (
        "PangoFontFamily",
        Layout {
            size: size_of::<PangoFontFamily>(),
            alignment: align_of::<PangoFontFamily>(),
        },
    ),
    (
        "PangoFontFamilyClass",
        Layout {
            size: size_of::<PangoFontFamilyClass>(),
            alignment: align_of::<PangoFontFamilyClass>(),
        },
    ),
    (
        "PangoFontMap",
        Layout {
            size: size_of::<PangoFontMap>(),
            alignment: align_of::<PangoFontMap>(),
        },
    ),
    (
        "PangoFontMapClass",
        Layout {
            size: size_of::<PangoFontMapClass>(),
            alignment: align_of::<PangoFontMapClass>(),
        },
    ),
    (
        "PangoFontMask",
        Layout {
            size: size_of::<PangoFontMask>(),
            alignment: align_of::<PangoFontMask>(),
        },
    ),
    (
        "PangoFontMetrics",
        Layout {
            size: size_of::<PangoFontMetrics>(),
            alignment: align_of::<PangoFontMetrics>(),
        },
    ),
    (
        "PangoFontset",
        Layout {
            size: size_of::<PangoFontset>(),
            alignment: align_of::<PangoFontset>(),
        },
    ),
    (
        "PangoFontsetClass",
        Layout {
            size: size_of::<PangoFontsetClass>(),
            alignment: align_of::<PangoFontsetClass>(),
        },
    ),
    (
        "PangoGlyph",
        Layout {
            size: size_of::<PangoGlyph>(),
            alignment: align_of::<PangoGlyph>(),
        },
    ),
    (
        "PangoGlyphGeometry",
        Layout {
            size: size_of::<PangoGlyphGeometry>(),
            alignment: align_of::<PangoGlyphGeometry>(),
        },
    ),
    (
        "PangoGlyphInfo",
        Layout {
            size: size_of::<PangoGlyphInfo>(),
            alignment: align_of::<PangoGlyphInfo>(),
        },
    ),
    (
        "PangoGlyphItem",
        Layout {
            size: size_of::<PangoGlyphItem>(),
            alignment: align_of::<PangoGlyphItem>(),
        },
    ),
    (
        "PangoGlyphItemIter",
        Layout {
            size: size_of::<PangoGlyphItemIter>(),
            alignment: align_of::<PangoGlyphItemIter>(),
        },
    ),
    (
        "PangoGlyphString",
        Layout {
            size: size_of::<PangoGlyphString>(),
            alignment: align_of::<PangoGlyphString>(),
        },
    ),
    (
        "PangoGlyphUnit",
        Layout {
            size: size_of::<PangoGlyphUnit>(),
            alignment: align_of::<PangoGlyphUnit>(),
        },
    ),
    (
        "PangoGlyphVisAttr",
        Layout {
            size: size_of::<PangoGlyphVisAttr>(),
            alignment: align_of::<PangoGlyphVisAttr>(),
        },
    ),
    (
        "PangoGravity",
        Layout {
            size: size_of::<PangoGravity>(),
            alignment: align_of::<PangoGravity>(),
        },
    ),
    (
        "PangoGravityHint",
        Layout {
            size: size_of::<PangoGravityHint>(),
            alignment: align_of::<PangoGravityHint>(),
        },
    ),
    (
        "PangoIncludedModule",
        Layout {
            size: size_of::<PangoIncludedModule>(),
            alignment: align_of::<PangoIncludedModule>(),
        },
    ),
    (
        "PangoItem",
        Layout {
            size: size_of::<PangoItem>(),
            alignment: align_of::<PangoItem>(),
        },
    ),
    (
        "PangoLayoutRun",
        Layout {
            size: size_of::<PangoLayoutRun>(),
            alignment: align_of::<PangoLayoutRun>(),
        },
    ),
    (
        "PangoMatrix",
        Layout {
            size: size_of::<PangoMatrix>(),
            alignment: align_of::<PangoMatrix>(),
        },
    ),
    (
        "PangoOverline",
        Layout {
            size: size_of::<PangoOverline>(),
            alignment: align_of::<PangoOverline>(),
        },
    ),
    (
        "PangoRectangle",
        Layout {
            size: size_of::<PangoRectangle>(),
            alignment: align_of::<PangoRectangle>(),
        },
    ),
    (
        "PangoRenderPart",
        Layout {
            size: size_of::<PangoRenderPart>(),
            alignment: align_of::<PangoRenderPart>(),
        },
    ),
    (
        "PangoRenderer",
        Layout {
            size: size_of::<PangoRenderer>(),
            alignment: align_of::<PangoRenderer>(),
        },
    ),
    (
        "PangoRendererClass",
        Layout {
            size: size_of::<PangoRendererClass>(),
            alignment: align_of::<PangoRendererClass>(),
        },
    ),
    (
        "PangoScript",
        Layout {
            size: size_of::<PangoScript>(),
            alignment: align_of::<PangoScript>(),
        },
    ),
    (
        "PangoShapeFlags",
        Layout {
            size: size_of::<PangoShapeFlags>(),
            alignment: align_of::<PangoShapeFlags>(),
        },
    ),
    (
        "PangoShowFlags",
        Layout {
            size: size_of::<PangoShowFlags>(),
            alignment: align_of::<PangoShowFlags>(),
        },
    ),
    (
        "PangoStretch",
        Layout {
            size: size_of::<PangoStretch>(),
            alignment: align_of::<PangoStretch>(),
        },
    ),
    (
        "PangoStyle",
        Layout {
            size: size_of::<PangoStyle>(),
            alignment: align_of::<PangoStyle>(),
        },
    ),
    (
        "PangoTabAlign",
        Layout {
            size: size_of::<PangoTabAlign>(),
            alignment: align_of::<PangoTabAlign>(),
        },
    ),
    (
        "PangoUnderline",
        Layout {
            size: size_of::<PangoUnderline>(),
            alignment: align_of::<PangoUnderline>(),
        },
    ),
    (
        "PangoVariant",
        Layout {
            size: size_of::<PangoVariant>(),
            alignment: align_of::<PangoVariant>(),
        },
    ),
    (
        "PangoWeight",
        Layout {
            size: size_of::<PangoWeight>(),
            alignment: align_of::<PangoWeight>(),
        },
    ),
    (
        "PangoWrapMode",
        Layout {
            size: size_of::<PangoWrapMode>(),
            alignment: align_of::<PangoWrapMode>(),
        },
    ),
];

const RUST_CONSTANTS: &[(&str, &str)] = &[
    ("(gint) PANGO_ALIGN_CENTER", "1"),
    ("(gint) PANGO_ALIGN_LEFT", "0"),
    ("(gint) PANGO_ALIGN_RIGHT", "2"),
    ("PANGO_ANALYSIS_FLAG_CENTERED_BASELINE", "1"),
    ("PANGO_ANALYSIS_FLAG_IS_ELLIPSIS", "2"),
    ("PANGO_ANALYSIS_FLAG_NEED_HYPHEN", "4"),
    ("(gint) PANGO_ATTR_ABSOLUTE_SIZE", "20"),
    ("(gint) PANGO_ATTR_ALLOW_BREAKS", "26"),
    ("(gint) PANGO_ATTR_BACKGROUND", "10"),
    ("(gint) PANGO_ATTR_BACKGROUND_ALPHA", "25"),
    ("(gint) PANGO_ATTR_FALLBACK", "16"),
    ("(gint) PANGO_ATTR_FAMILY", "2"),
    ("(gint) PANGO_ATTR_FONT_DESC", "8"),
    ("(gint) PANGO_ATTR_FONT_FEATURES", "23"),
    ("(gint) PANGO_ATTR_FOREGROUND", "9"),
    ("(gint) PANGO_ATTR_FOREGROUND_ALPHA", "24"),
    ("(gint) PANGO_ATTR_GRAVITY", "21"),
    ("(gint) PANGO_ATTR_GRAVITY_HINT", "22"),
    ("PANGO_ATTR_INDEX_FROM_TEXT_BEGINNING", "0"),
    ("(gint) PANGO_ATTR_INSERT_HYPHENS", "28"),
    ("(gint) PANGO_ATTR_INVALID", "0"),
    ("(gint) PANGO_ATTR_LANGUAGE", "1"),
    ("(gint) PANGO_ATTR_LETTER_SPACING", "17"),
    ("(gint) PANGO_ATTR_OVERLINE", "29"),
    ("(gint) PANGO_ATTR_OVERLINE_COLOR", "30"),
    ("(gint) PANGO_ATTR_RISE", "13"),
    ("(gint) PANGO_ATTR_SCALE", "15"),
    ("(gint) PANGO_ATTR_SHAPE", "14"),
    ("(gint) PANGO_ATTR_SHOW", "27"),
    ("(gint) PANGO_ATTR_SIZE", "7"),
    ("(gint) PANGO_ATTR_STRETCH", "6"),
    ("(gint) PANGO_ATTR_STRIKETHROUGH", "12"),
    ("(gint) PANGO_ATTR_STRIKETHROUGH_COLOR", "19"),
    ("(gint) PANGO_ATTR_STYLE", "3"),
    ("(gint) PANGO_ATTR_UNDERLINE", "11"),
    ("(gint) PANGO_ATTR_UNDERLINE_COLOR", "18"),
    ("(gint) PANGO_ATTR_VARIANT", "5"),
    ("(gint) PANGO_ATTR_WEIGHT", "4"),
    ("(gint) PANGO_BIDI_TYPE_AL", "4"),
    ("(gint) PANGO_BIDI_TYPE_AN", "11"),
    ("(gint) PANGO_BIDI_TYPE_B", "15"),
    ("(gint) PANGO_BIDI_TYPE_BN", "14"),
    ("(gint) PANGO_BIDI_TYPE_CS", "12"),
    ("(gint) PANGO_BIDI_TYPE_EN", "8"),
    ("(gint) PANGO_BIDI_TYPE_ES", "9"),
    ("(gint) PANGO_BIDI_TYPE_ET", "10"),
    ("(gint) PANGO_BIDI_TYPE_L", "0"),
    ("(gint) PANGO_BIDI_TYPE_LRE", "1"),
    ("(gint) PANGO_BIDI_TYPE_LRO", "2"),
    ("(gint) PANGO_BIDI_TYPE_NSM", "13"),
    ("(gint) PANGO_BIDI_TYPE_ON", "18"),
    ("(gint) PANGO_BIDI_TYPE_PDF", "7"),
    ("(gint) PANGO_BIDI_TYPE_R", "3"),
    ("(gint) PANGO_BIDI_TYPE_RLE", "5"),
    ("(gint) PANGO_BIDI_TYPE_RLO", "6"),
    ("(gint) PANGO_BIDI_TYPE_S", "16"),
    ("(gint) PANGO_BIDI_TYPE_WS", "17"),
    ("(gint) PANGO_COVERAGE_APPROXIMATE", "2"),
    ("(gint) PANGO_COVERAGE_EXACT", "3"),
    ("(gint) PANGO_COVERAGE_FALLBACK", "1"),
    ("(gint) PANGO_COVERAGE_NONE", "0"),
    ("(gint) PANGO_DIRECTION_LTR", "0"),
    ("(gint) PANGO_DIRECTION_NEUTRAL", "6"),
    ("(gint) PANGO_DIRECTION_RTL", "1"),
    ("(gint) PANGO_DIRECTION_TTB_LTR", "2"),
    ("(gint) PANGO_DIRECTION_TTB_RTL", "3"),
    ("(gint) PANGO_DIRECTION_WEAK_LTR", "4"),
    ("(gint) PANGO_DIRECTION_WEAK_RTL", "5"),
    ("(gint) PANGO_ELLIPSIZE_END", "3"),
    ("(gint) PANGO_ELLIPSIZE_MIDDLE", "2"),
    ("(gint) PANGO_ELLIPSIZE_NONE", "0"),
    ("(gint) PANGO_ELLIPSIZE_START", "1"),
    ("PANGO_ENGINE_TYPE_LANG", "PangoEngineLang"),
    ("PANGO_ENGINE_TYPE_SHAPE", "PangoEngineShape"),
    ("(guint) PANGO_FONT_MASK_FAMILY", "1"),
    ("(guint) PANGO_FONT_MASK_GRAVITY", "64"),
    ("(guint) PANGO_FONT_MASK_SIZE", "32"),
    ("(guint) PANGO_FONT_MASK_STRETCH", "16"),
    ("(guint) PANGO_FONT_MASK_STYLE", "2"),
    ("(guint) PANGO_FONT_MASK_VARIANT", "4"),
    ("(guint) PANGO_FONT_MASK_VARIATIONS", "128"),
    ("(guint) PANGO_FONT_MASK_WEIGHT", "8"),
    ("PANGO_GLYPH_EMPTY", "268435455"),
    ("PANGO_GLYPH_INVALID_INPUT", "4294967295"),
    ("PANGO_GLYPH_UNKNOWN_FLAG", "268435456"),
    ("(gint) PANGO_GRAVITY_AUTO", "4"),
    ("(gint) PANGO_GRAVITY_EAST", "1"),
    ("(gint) PANGO_GRAVITY_HINT_LINE", "2"),
    ("(gint) PANGO_GRAVITY_HINT_NATURAL", "0"),
    ("(gint) PANGO_GRAVITY_HINT_STRONG", "1"),
    ("(gint) PANGO_GRAVITY_NORTH", "2"),
    ("(gint) PANGO_GRAVITY_SOUTH", "0"),
    ("(gint) PANGO_GRAVITY_WEST", "3"),
    ("(gint) PANGO_OVERLINE_NONE", "0"),
    ("(gint) PANGO_OVERLINE_SINGLE", "1"),
    ("(gint) PANGO_RENDER_PART_BACKGROUND", "1"),
    ("(gint) PANGO_RENDER_PART_FOREGROUND", "0"),
    ("(gint) PANGO_RENDER_PART_OVERLINE", "4"),
    ("(gint) PANGO_RENDER_PART_STRIKETHROUGH", "3"),
    ("(gint) PANGO_RENDER_PART_UNDERLINE", "2"),
    ("PANGO_RENDER_TYPE_NONE", "PangoRenderNone"),
    ("PANGO_SCALE", "1024"),
    ("(gint) PANGO_SCRIPT_AHOM", "111"),
    ("(gint) PANGO_SCRIPT_ANATOLIAN_HIEROGLYPHS", "112"),
    ("(gint) PANGO_SCRIPT_ARABIC", "2"),
    ("(gint) PANGO_SCRIPT_ARMENIAN", "3"),
    ("(gint) PANGO_SCRIPT_BALINESE", "62"),
    ("(gint) PANGO_SCRIPT_BASSA_VAH", "88"),
    ("(gint) PANGO_SCRIPT_BATAK", "78"),
    ("(gint) PANGO_SCRIPT_BENGALI", "4"),
    ("(gint) PANGO_SCRIPT_BOPOMOFO", "5"),
    ("(gint) PANGO_SCRIPT_BRAHMI", "79"),
    ("(gint) PANGO_SCRIPT_BRAILLE", "46"),
    ("(gint) PANGO_SCRIPT_BUGINESE", "55"),
    ("(gint) PANGO_SCRIPT_BUHID", "44"),
    ("(gint) PANGO_SCRIPT_CANADIAN_ABORIGINAL", "40"),
    ("(gint) PANGO_SCRIPT_CARIAN", "75"),
    ("(gint) PANGO_SCRIPT_CAUCASIAN_ALBANIAN", "89"),
    ("(gint) PANGO_SCRIPT_CHAKMA", "81"),
    ("(gint) PANGO_SCRIPT_CHAM", "72"),
    ("(gint) PANGO_SCRIPT_CHEROKEE", "6"),
    ("(gint) PANGO_SCRIPT_COMMON", "0"),
    ("(gint) PANGO_SCRIPT_COPTIC", "7"),
    ("(gint) PANGO_SCRIPT_CUNEIFORM", "63"),
    ("(gint) PANGO_SCRIPT_CYPRIOT", "47"),
    ("(gint) PANGO_SCRIPT_CYRILLIC", "8"),
    ("(gint) PANGO_SCRIPT_DESERET", "9"),
    ("(gint) PANGO_SCRIPT_DEVANAGARI", "10"),
    ("(gint) PANGO_SCRIPT_DUPLOYAN", "90"),
    ("(gint) PANGO_SCRIPT_ELBASAN", "91"),
    ("(gint) PANGO_SCRIPT_ETHIOPIC", "11"),
    ("(gint) PANGO_SCRIPT_GEORGIAN", "12"),
    ("(gint) PANGO_SCRIPT_GLAGOLITIC", "56"),
    ("(gint) PANGO_SCRIPT_GOTHIC", "13"),
    ("(gint) PANGO_SCRIPT_GRANTHA", "92"),
    ("(gint) PANGO_SCRIPT_GREEK", "14"),
    ("(gint) PANGO_SCRIPT_GUJARATI", "15"),
    ("(gint) PANGO_SCRIPT_GURMUKHI", "16"),
    ("(gint) PANGO_SCRIPT_HAN", "17"),
    ("(gint) PANGO_SCRIPT_HANGUL", "18"),
    ("(gint) PANGO_SCRIPT_HANUNOO", "43"),
    ("(gint) PANGO_SCRIPT_HATRAN", "113"),
    ("(gint) PANGO_SCRIPT_HEBREW", "19"),
    ("(gint) PANGO_SCRIPT_HIRAGANA", "20"),
    ("(gint) PANGO_SCRIPT_INHERITED", "1"),
    ("(gint) PANGO_SCRIPT_INVALID_CODE", "-1"),
    ("(gint) PANGO_SCRIPT_KANNADA", "21"),
    ("(gint) PANGO_SCRIPT_KATAKANA", "22"),
    ("(gint) PANGO_SCRIPT_KAYAH_LI", "67"),
    ("(gint) PANGO_SCRIPT_KHAROSHTHI", "60"),
    ("(gint) PANGO_SCRIPT_KHMER", "23"),
    ("(gint) PANGO_SCRIPT_KHOJKI", "93"),
    ("(gint) PANGO_SCRIPT_KHUDAWADI", "94"),
    ("(gint) PANGO_SCRIPT_LAO", "24"),
    ("(gint) PANGO_SCRIPT_LATIN", "25"),
    ("(gint) PANGO_SCRIPT_LEPCHA", "68"),
    ("(gint) PANGO_SCRIPT_LIMBU", "48"),
    ("(gint) PANGO_SCRIPT_LINEAR_A", "95"),
    ("(gint) PANGO_SCRIPT_LINEAR_B", "51"),
    ("(gint) PANGO_SCRIPT_LYCIAN", "76"),
    ("(gint) PANGO_SCRIPT_LYDIAN", "77"),
    ("(gint) PANGO_SCRIPT_MAHAJANI", "96"),
    ("(gint) PANGO_SCRIPT_MALAYALAM", "26"),
    ("(gint) PANGO_SCRIPT_MANDAIC", "80"),
    ("(gint) PANGO_SCRIPT_MANICHAEAN", "97"),
    ("(gint) PANGO_SCRIPT_MENDE_KIKAKUI", "98"),
    ("(gint) PANGO_SCRIPT_MEROITIC_CURSIVE", "82"),
    ("(gint) PANGO_SCRIPT_MEROITIC_HIEROGLYPHS", "83"),
    ("(gint) PANGO_SCRIPT_MIAO", "84"),
    ("(gint) PANGO_SCRIPT_MODI", "99"),
    ("(gint) PANGO_SCRIPT_MONGOLIAN", "27"),
    ("(gint) PANGO_SCRIPT_MRO", "100"),
    ("(gint) PANGO_SCRIPT_MULTANI", "114"),
    ("(gint) PANGO_SCRIPT_MYANMAR", "28"),
    ("(gint) PANGO_SCRIPT_NABATAEAN", "101"),
    ("(gint) PANGO_SCRIPT_NEW_TAI_LUE", "54"),
    ("(gint) PANGO_SCRIPT_NKO", "66"),
    ("(gint) PANGO_SCRIPT_OGHAM", "29"),
    ("(gint) PANGO_SCRIPT_OLD_HUNGARIAN", "115"),
    ("(gint) PANGO_SCRIPT_OLD_ITALIC", "30"),
    ("(gint) PANGO_SCRIPT_OLD_NORTH_ARABIAN", "102"),
    ("(gint) PANGO_SCRIPT_OLD_PERMIC", "103"),
    ("(gint) PANGO_SCRIPT_OLD_PERSIAN", "59"),
    ("(gint) PANGO_SCRIPT_OL_CHIKI", "73"),
    ("(gint) PANGO_SCRIPT_ORIYA", "31"),
    ("(gint) PANGO_SCRIPT_OSMANYA", "49"),
    ("(gint) PANGO_SCRIPT_PAHAWH_HMONG", "104"),
    ("(gint) PANGO_SCRIPT_PALMYRENE", "105"),
    ("(gint) PANGO_SCRIPT_PAU_CIN_HAU", "106"),
    ("(gint) PANGO_SCRIPT_PHAGS_PA", "65"),
    ("(gint) PANGO_SCRIPT_PHOENICIAN", "64"),
    ("(gint) PANGO_SCRIPT_PSALTER_PAHLAVI", "107"),
    ("(gint) PANGO_SCRIPT_REJANG", "69"),
    ("(gint) PANGO_SCRIPT_RUNIC", "32"),
    ("(gint) PANGO_SCRIPT_SAURASHTRA", "71"),
    ("(gint) PANGO_SCRIPT_SHARADA", "85"),
    ("(gint) PANGO_SCRIPT_SHAVIAN", "50"),
    ("(gint) PANGO_SCRIPT_SIDDHAM", "108"),
    ("(gint) PANGO_SCRIPT_SIGNWRITING", "116"),
    ("(gint) PANGO_SCRIPT_SINHALA", "33"),
    ("(gint) PANGO_SCRIPT_SORA_SOMPENG", "86"),
    ("(gint) PANGO_SCRIPT_SUNDANESE", "70"),
    ("(gint) PANGO_SCRIPT_SYLOTI_NAGRI", "58"),
    ("(gint) PANGO_SCRIPT_SYRIAC", "34"),
    ("(gint) PANGO_SCRIPT_TAGALOG", "42"),
    ("(gint) PANGO_SCRIPT_TAGBANWA", "45"),
    ("(gint) PANGO_SCRIPT_TAI_LE", "52"),
    ("(gint) PANGO_SCRIPT_TAKRI", "87"),
    ("(gint) PANGO_SCRIPT_TAMIL", "35"),
    ("(gint) PANGO_SCRIPT_TELUGU", "36"),
    ("(gint) PANGO_SCRIPT_THAANA", "37"),
    ("(gint) PANGO_SCRIPT_THAI", "38"),
    ("(gint) PANGO_SCRIPT_TIBETAN", "39"),
    ("(gint) PANGO_SCRIPT_TIFINAGH", "57"),
    ("(gint) PANGO_SCRIPT_TIRHUTA", "109"),
    ("(gint) PANGO_SCRIPT_UGARITIC", "53"),
    ("(gint) PANGO_SCRIPT_UNKNOWN", "61"),
    ("(gint) PANGO_SCRIPT_VAI", "74"),
    ("(gint) PANGO_SCRIPT_WARANG_CITI", "110"),
    ("(gint) PANGO_SCRIPT_YI", "41"),
    ("(guint) PANGO_SHAPE_NONE", "0"),
    ("(guint) PANGO_SHAPE_ROUND_POSITIONS", "1"),
    ("(guint) PANGO_SHOW_IGNORABLES", "4"),
    ("(guint) PANGO_SHOW_LINE_BREAKS", "2"),
    ("(guint) PANGO_SHOW_NONE", "0"),
    ("(guint) PANGO_SHOW_SPACES", "1"),
    ("(gint) PANGO_STRETCH_CONDENSED", "2"),
    ("(gint) PANGO_STRETCH_EXPANDED", "6"),
    ("(gint) PANGO_STRETCH_EXTRA_CONDENSED", "1"),
    ("(gint) PANGO_STRETCH_EXTRA_EXPANDED", "7"),
    ("(gint) PANGO_STRETCH_NORMAL", "4"),
    ("(gint) PANGO_STRETCH_SEMI_CONDENSED", "3"),
    ("(gint) PANGO_STRETCH_SEMI_EXPANDED", "5"),
    ("(gint) PANGO_STRETCH_ULTRA_CONDENSED", "0"),
    ("(gint) PANGO_STRETCH_ULTRA_EXPANDED", "8"),
    ("(gint) PANGO_STYLE_ITALIC", "2"),
    ("(gint) PANGO_STYLE_NORMAL", "0"),
    ("(gint) PANGO_STYLE_OBLIQUE", "1"),
    ("(gint) PANGO_TAB_LEFT", "0"),
    ("(gint) PANGO_UNDERLINE_DOUBLE", "2"),
    ("(gint) PANGO_UNDERLINE_DOUBLE_LINE", "6"),
    ("(gint) PANGO_UNDERLINE_ERROR", "4"),
    ("(gint) PANGO_UNDERLINE_ERROR_LINE", "7"),
    ("(gint) PANGO_UNDERLINE_LOW", "3"),
    ("(gint) PANGO_UNDERLINE_NONE", "0"),
    ("(gint) PANGO_UNDERLINE_SINGLE", "1"),
    ("(gint) PANGO_UNDERLINE_SINGLE_LINE", "5"),
    ("PANGO_UNKNOWN_GLYPH_HEIGHT", "14"),
    ("PANGO_UNKNOWN_GLYPH_WIDTH", "10"),
    ("(gint) PANGO_VARIANT_NORMAL", "0"),
    ("(gint) PANGO_VARIANT_SMALL_CAPS", "1"),
    ("PANGO_VERSION_MIN_REQUIRED", "2"),
    ("(gint) PANGO_WEIGHT_BOLD", "700"),
    ("(gint) PANGO_WEIGHT_BOOK", "380"),
    ("(gint) PANGO_WEIGHT_HEAVY", "900"),
    ("(gint) PANGO_WEIGHT_LIGHT", "300"),
    ("(gint) PANGO_WEIGHT_MEDIUM", "500"),
    ("(gint) PANGO_WEIGHT_NORMAL", "400"),
    ("(gint) PANGO_WEIGHT_SEMIBOLD", "600"),
    ("(gint) PANGO_WEIGHT_SEMILIGHT", "350"),
    ("(gint) PANGO_WEIGHT_THIN", "100"),
    ("(gint) PANGO_WEIGHT_ULTRABOLD", "800"),
    ("(gint) PANGO_WEIGHT_ULTRAHEAVY", "1000"),
    ("(gint) PANGO_WEIGHT_ULTRALIGHT", "200"),
    ("(gint) PANGO_WRAP_CHAR", "1"),
    ("(gint) PANGO_WRAP_WORD", "0"),
    ("(gint) PANGO_WRAP_WORD_CHAR", "2"),
];