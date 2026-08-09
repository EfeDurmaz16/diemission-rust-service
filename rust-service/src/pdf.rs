use printpdf::{BuiltinFont, Mm, PdfDocument};

use crate::{error::AppError, models::StudentReportData};

/// Build a simple A4 PDF report from student JSON.
pub fn generate_student_report(student: &StudentReportData) -> Result<Vec<u8>, AppError> {
    let (doc, page, layer) = PdfDocument::new("Student Report", Mm(210.0), Mm(297.0), "Layer 1");

    let font = doc
        .add_builtin_font(BuiltinFont::Helvetica)
        .map_err(|err| AppError::Pdf(err.to_string()))?;
    let font_bold = doc
        .add_builtin_font(BuiltinFont::HelveticaBold)
        .map_err(|err| AppError::Pdf(err.to_string()))?;

    let layer = doc.get_page(page).get_layer(layer);

    let mut y = 280.0;
    let left = 20.0;
    let line = 8.0;

    layer.use_text("Student Report", 18.0, Mm(left), Mm(y), &font_bold);
    y -= 14.0;
    layer.use_text("School Management System", 11.0, Mm(left), Mm(y), &font);
    y -= 16.0;

    let rows = [
        ("ID", student.id.to_string()),
        ("Name", StudentReportData::display(&student.name)),
        ("Email", StudentReportData::display(&student.email)),
        ("Phone", StudentReportData::display(&student.phone)),
        ("Gender", StudentReportData::display(&student.gender)),
        (
            "Date of Birth",
            StudentReportData::display_value(&student.dob),
        ),
        ("Class", StudentReportData::display(&student.class)),
        ("Section", StudentReportData::display(&student.section)),
        ("Roll", StudentReportData::display_value(&student.roll)),
        (
            "Admission Date",
            StudentReportData::display_value(&student.admission_date),
        ),
        (
            "System Access",
            student
                .system_access
                .map(|v| v.to_string())
                .unwrap_or_else(|| "-".to_string()),
        ),
        ("Father", StudentReportData::display(&student.father_name)),
        (
            "Father Phone",
            StudentReportData::display(&student.father_phone),
        ),
        ("Mother", StudentReportData::display(&student.mother_name)),
        (
            "Mother Phone",
            StudentReportData::display(&student.mother_phone),
        ),
        (
            "Guardian",
            StudentReportData::display(&student.guardian_name),
        ),
        (
            "Guardian Phone",
            StudentReportData::display(&student.guardian_phone),
        ),
        (
            "Relation of Guardian",
            StudentReportData::display(&student.relation_of_guardian),
        ),
        (
            "Current Address",
            StudentReportData::display(&student.current_address),
        ),
        (
            "Permanent Address",
            StudentReportData::display(&student.permanent_address),
        ),
        (
            "Reporter",
            StudentReportData::display(&student.reporter_name),
        ),
    ];

    for (label, value) in rows {
        let text = truncate(&format!("{label}: {value}"), 90);
        layer.use_text(text, 11.0, Mm(left), Mm(y), &font);
        y -= line;
        if y < 20.0 {
            break;
        }
    }

    doc.save_to_bytes()
        .map_err(|err| AppError::Pdf(err.to_string()))
}

/// The built-in fonts are written with WinAnsi encoding, which covers Latin-1.
/// Anything outside it would be dropped silently by the PDF writer, so it is
/// replaced with a visible marker instead.
fn renderable(c: char) -> bool {
    matches!(c, ' '..='~' | '\u{a0}'..='\u{ff}')
}

fn truncate(value: &str, max: usize) -> String {
    let mut out: String = value
        .chars()
        .take(max)
        .map(|c| if renderable(c) { c } else { '?' })
        .collect();
    if value.chars().count() > max {
        out.push_str("...");
    }
    out
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn truncate_keeps_latin1_and_marks_cuts() {
        assert_eq!(truncate("José Müller", 90), "José Müller");
        assert_eq!(truncate("Ünal Çetin", 90), "Ünal Çetin");
        assert_eq!(truncate("你好", 90), "??");
        assert_eq!(
            truncate(&"a".repeat(95), 90),
            format!("{}...", "a".repeat(90))
        );
    }
}
