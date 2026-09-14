#[cfg(test)]
use rsubstitute::*;
use std::path::PathBuf;

pub struct SourceCodeId {
    relative_file_path: PathBuf,
    line: usize,
    column: usize,
}

impl SourceCodeId {
    pub fn new(relative_file_path: PathBuf, line: usize, column: usize) -> Self {
        Self {
            relative_file_path,
            line,
            column,
        }
    }
}

impl SourceCodeId {
    pub fn to_fn_ident_prefix(self) -> String {
        let result = self
            .relative_file_path
            .iter()
            .map(|x| x.to_str().unwrap())
            .map(normalize_path_part)
            .chain(vec![self.line.to_string(), self.column.to_string()])
            .collect::<Vec<_>>()
            .join("_");

        return result;
    }
}

#[cfg_attr(test, mock(base))]
fn normalize_path_part(path_part: &str) -> String {
    path_part.replace(|x| !char::is_alphanumeric(x), "_")
}

#[cfg(test)]
mod tests {
    #![allow(non_snake_case)]

    use super::*;

    #[test]
    fn SourceCodeId_to_fn_ident_prefix_Ok() {
        // Arrange
        let line = 10;
        let column = 20;
        let source_code_id = SourceCodeId::new("a/b/c.rs".into(), line, column);
        normalize_path_part::setup(Arg::Any).returns_many([
            "quo".to_owned(),
            "vadis".to_owned(),
            "veridis".to_owned(),
        ]);

        // Act
        let result = source_code_id.to_fn_ident_prefix();

        // Assert
        let expected = format!("quo_vadis_veridis_{}_{}", line, column);
        assert_eq!(result, expected);

        normalize_path_part::received("a", 1.time())
            .received("b", 1.time())
            .received("c.rs", 1.time())
            .no_other_calls();
    }
}
