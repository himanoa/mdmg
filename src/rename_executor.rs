use crate::logger::Logger;
use crate::scaffold::Scaffold;
use crate::Result;

use derive_more::{Constructor, Display, Deref, From, Into};
use inflector::Inflector;

#[derive(Debug, Clone, Copy, Constructor, PartialEq, Eq, Display, Deref, From, Into)]
pub struct BeforeRenameName<'a>(&'a str);

#[derive(Debug, Clone, Constructor, PartialEq, Eq, Display, Deref, From, Into)]
pub struct RenamedName(String);

pub fn create_new_name(
    before_name: &str,
    before_identify: &str,
    after_identify: &str
) -> RenamedName {
    let renamed_file_name: String = before_name
        .replace(&before_identify.to_pascal_case(),&after_identify.to_pascal_case())
        .replace(&before_identify.to_camel_case(), &after_identify.to_camel_case())
        .replace(&before_identify.to_kebab_case(), &after_identify.to_kebab_case())
        .replace(&before_identify.to_snake_case(), &after_identify.to_snake_case())
        .to_string();
    RenamedName::new(renamed_file_name)
}

#[derive(Debug, Clone, Constructor, PartialEq, Eq)]
pub struct RenameFile<'a> { 
    id: BeforeRenameName<'a>,
    renamed_file_name: RenamedName,
    replaced_file_body: &'a str
}

impl<'a> RenameFile<'a> {
    pub fn create_rename_file(
        logger: impl Logger,
        scaffold: &'a Scaffold,
        before_identify: &'a str,
        after_identify: &'a str
    ) -> RenamedName {
        let (file_name, _) = match scaffold {
            Scaffold::Pending { file_name: _ } => panic!("received pending file"),
            Scaffold::Complete { file_name, file_body } => (file_name, file_body)
        };
        logger.debug(format!("before name: {}", file_name).as_str());
        let renamed_file = create_new_name(&file_name, before_identify, after_identify);
        logger.debug(format!("renamed name: {}", renamed_file).as_str());
        renamed_file
    }
}

pub trait RenameExecutor {
    fn execute(self, scaffold: &Scaffold) -> Result<()>;
}

#[cfg(not(tarpaulin_include))]
#[cfg(test)]
mod tests {
    use super::RenamedName;
    use crate::rename_executor::create_new_name;

    #[test]
    fn test_create_new_name() {
        assert_eq!(create_new_name("ExampleService", "Example", "Himanoa"), RenamedName::new("HimanoaService".to_string()), "Pascal case test");
        assert_eq!(create_new_name("exampleService", "example", "himanoa"), RenamedName::new("himanoaService".to_string()), "Camel case test");
        assert_eq!(create_new_name("example-service", "example", "himanoa"), RenamedName::new("himanoa-service".to_string()), "Kebab case test");
        assert_eq!(create_new_name("example_service", "example", "himanoa"), RenamedName::new("himanoa_service".to_string()), "Snake case test");
        assert_eq!(create_new_name("example_service", "adfadf", "himanoa"), RenamedName::new("example_service".to_string()), "No replace");
    }
}

