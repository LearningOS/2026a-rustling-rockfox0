// quiz3.rs
//
//这个测验测试：
//-泛型
//-特征
//
//虚构的魔法学校编写了新的成绩单生成系统
//在 Rust 中！目前系统仅支持创建成绩单
//学生的成绩以数字表示（例如 1.0 -> 5.5）。然而，
//学校还发布按字母顺序排列的成绩（A+ -> F-）并且需要能够
//打印两种类型的报告卡！
//
//在 struct ReportCard 和 impl 块中进行必要的代码更改
//支持按字母顺序排列的报告卡。将第二次测试的成绩更改为
//“A+”表示您的更改允许按字母顺序评分。
//
//执行 `rusdlingshintquiz3` 或使用 `hint`watch 子命令来获取提示。
use std::fmt::Display;
pub struct ReportCard<T> {
    pub grade: T,
    pub student_name: String,
    pub student_age: u8,
}

impl<T:Display> ReportCard<T> {
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
//TODO：完成练习后，请务必更改此处的成绩。     
   let report_card = ReportCard {
            grade: "A+".to_string(),
            student_name: "Gary Plotter".to_string(),
            student_age: 11,
        };
        assert_eq!(
            report_card.print(),
            "Gary Plotter (11) - achieved a grade of A+"
        );
    }
}
