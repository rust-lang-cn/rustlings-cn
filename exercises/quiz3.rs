// quiz3.rs
// 这个测试要测验：
// - 泛型
// - Traits
// 一个想象中的魔法学校有一个用 Rust 编写的成绩单生成系统！
// 现在这个系统只支持创建学生的成绩是数字（比如 1.0 -> 5.5）的成绩单。
// 然而，学校也会使用字母发布成绩 (A+ -> F-)，并需求能够打印这两种类型的成绩单！

// 按需更改结构体 ReportCard 中的代码和 impl 块，使其
// 支持字母成绩单。更改第二个测试中的成绩为 "A+"
// 以表明你的更改允许了字母成绩。

// 执行 `rustlings hint quiz3` 或在观察模式下使用 `hint` 子命令来获取提示。

// I AM NOT DONE

pub struct ReportCard {
    pub grade: f32,
    pub student_name: String,
    pub student_age: u8,
}

impl ReportCard {
    pub fn print(&self) -> String {
        format!("{} ({}) - achieved a grade of {}",
            &self.student_name, &self.student_age, &self.grade)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn generate_numeric_report_card() {
        let report_card = ReportCard {
            grade: 2.1,
            student_name: "Tom Wriggle".to_string(),
            student_age: 12,
        };
        assert_eq!(
            report_card.print(),
            "Tom Wriggle (12) - achieved a grade of 2.1"
        );
    }

    #[test]
    fn generate_alphabetic_report_card() {
        // TODO: 完成练习后，要修改这个成绩
        let report_card = ReportCard {
            grade: 2.1,
            student_name: "Gary Plotter".to_string(),
            student_age: 11,
        };
        assert_eq!(
            report_card.print(),
            "Gary Plotter (11) - achieved a grade of A+"
        );
    }
}
