// This quiz tests:
// - Generics
// - Traits
//
// An imaginary magical school has a new report card generation system written
// in Rust! Currently, the system only supports creating report cards where the
// student's grade is represented numerically (e.g. 1.0 -> 5.5). However, the
// school also issues alphabetical grades (A+ -> F-) and needs to be able to
// print both types of report card!
//
// Make the necessary code changes in the struct `ReportCard` and the impl
// block to support alphabetical report cards in addition to numerical ones.

// TODO: Adjust the struct as described above.
struct ReportCard {
    grade: Grade,
    student_name: String,
    student_age: u8,
}

enum Grade {
    Numeric(f32),
    Alphabetic(String),
}

// TODO: Adjust the impl block as described above.
impl ReportCard {
    fn print(&self) -> String {
        let grade_str = match &self.grade {
            Grade::Numeric(grade) => format!("{:.1}", grade),
            Grade::Alphabetic(grade) => grade.clone(),
        };
        format!(
            "{} ({}) - achieved a grade of {}",
            &self.student_name, &self.student_age, grade_str,
        )
    }
}

fn main() {
    // You can optionally experiment here.
}

//Don't change this function!
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn generate_numeric_report_card() {
        let report_card = ReportCard {
            grade: Grade::Numeric(2.1),
            student_name: "Tom Wriggle".to_string(),
            student_age: 12,
        };
        assert_eq!(
            report_card.print(),
            "Tom Wriggle (12) - achieved a grade of 2.1",
        );
    }

    #[test]
    fn generate_alphabetic_report_card() {
        let report_card = ReportCard {
            grade: Grade::Alphabetic("A+".to_string()),
            student_name: "Gary Plotter".to_string(),
            student_age: 11,
        };
        assert_eq!(
            report_card.print(),
            "Gary Plotter (11) - achieved a grade of A+",
        );
    }
}
