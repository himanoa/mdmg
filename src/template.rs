use crate::error::MdmgError;
use crate::Result;
use handlebars::{Context, Handlebars, Helper, JsonRender, Output, RenderContext, RenderError};
use inflector::Inflector;
use serde::Serialize;
use std::env::var;

#[derive(Debug, Serialize, Default)]
pub struct MdmgCtx {
    pub identify: String,
}

#[derive(Debug, Serialize, Default, PartialEq)]
pub struct Template {
    body: String,
}

impl Template {
    pub fn new<T: Into<String>>(body: T) -> Self {
        Template { body: body.into() }
    }
}

impl MdmgCtx {
    pub fn new<T: Into<String>>(identify: T) -> Self {
        Self {
            identify: identify.into(),
        }
    }
}

fn pascal_case_helper(
    h: &Helper,
    _: &Handlebars,
    _: &Context,
    _: &mut RenderContext,
    out: &mut dyn Output,
) -> std::result::Result<(), RenderError> {
    let target = h
        .param(0)
        .ok_or_else(|| RenderError::new("Param 0 is required for pascal_case_decorator."))
        .map(|s| s.value().render())?;
    let rendered = target.to_pascal_case();
    out.write(&rendered)?;
    Ok(())
}

fn camel_case_helper(
    h: &Helper,
    _: &Handlebars,
    _: &Context,
    _: &mut RenderContext,
    out: &mut dyn Output,
) -> std::result::Result<(), RenderError> {
    let target = h
        .param(0)
        .ok_or_else(|| RenderError::new("Param 0 is required for camel_case_decorator."))
        .map(|s| s.value().render())?;
    let rendered = target.to_camel_case();
    out.write(&rendered)?;
    Ok(())
}

fn snake_case_helper(
    h: &Helper,
    _: &Handlebars,
    _: &Context,
    _: &mut RenderContext,
    out: &mut dyn Output,
) -> std::result::Result<(), RenderError> {
    let target = h
        .param(0)
        .ok_or_else(|| RenderError::new("Param 0 is required for snake_case_decorator."))
        .map(|s| s.value().render())?;
    let rendered = target.to_snake_case();
    out.write(&rendered)?;
    Ok(())
}

fn kebab_case_helper(
    h: &Helper,
    _: &Handlebars,
    _: &Context,
    _: &mut RenderContext,
    out: &mut dyn Output,
) -> std::result::Result<(), RenderError> {
    let target = h
        .param(0)
        .ok_or_else(|| RenderError::new("Param 0 is required for kebab_case_decorator."))
        .map(|s| s.value().render())?;
    let rendered = target.to_kebab_case();
    out.write(&rendered)?;
    Ok(())
}

fn env_helper(
    h: &Helper,
    _: &Handlebars,
    _: &Context,
    _: &mut RenderContext,
    out: &mut dyn Output,
) -> std::result::Result<(), RenderError> {
    let target = h
        .param(0)
        .ok_or_else(|| RenderError::new("Param 0 is required for kebab_case_decorator."))
        .map(|s| s.value().render())?;
    let rendered = var(&target).unwrap_or_else(|_| panic!("env({}) is not defined.", &target));
    out.write(&rendered)?;
    Ok(())
}

pub fn render(template: Template, ctx: &MdmgCtx) -> Result<String> {
    let mut handlebars = Handlebars::new();

    handlebars.register_helper("pascal_case", Box::new(pascal_case_helper));
    handlebars.register_helper("camel_case", Box::new(camel_case_helper));
    handlebars.register_helper("kebab_case", Box::new(kebab_case_helper));
    handlebars.register_helper("snake_case", Box::new(snake_case_helper));
    handlebars.register_helper("env", Box::new(env_helper));

    handlebars
        .render_template(template.body.as_str(), ctx)
        .map_err(|e| MdmgError::TempalteRenderError { reason: e.desc })
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::default::Default;
    use std::env::{remove_var, set_var};

    #[test]
    fn render_returning_the_piyopoyo() {
        assert_eq!(
            render(Template::new("PIYOPIYP"), &Default::default()).unwrap(),
            "PIYOPIYP"
        )
    }

    #[test]
    fn render_returning_the_himanoa() {
        assert_eq!(
            render(Template::new("{{identify}}"), &MdmgCtx::new("himanoa")).unwrap(),
            "himanoa"
        )
    }

    #[test]
    fn render_returning_the_variable() {
        assert_eq!(
            render(Template::new("{{identify}}"), &MdmgCtx::new("himanoa")).unwrap(),
            "himanoa"
        )
    }

    #[test]
    fn render_returning_the_foo() {
        set_var("MDMG_TEST_VALUE1", "FOO");
        let actual = render(
            Template::new("{{ env \"MDMG_TEST_VALUE1\"}}"),
            &MdmgCtx::new("himanoa"),
        );
        remove_var("MDMG_TEST_VALUE1");
        assert_eq!(actual.unwrap(), "FOO")
    }

    #[test]
    fn render_returning_the_foo_adapter() {
        set_var("MDMG_TEST_VALUE2", "FooAdapter");
        let actual = render(
            Template::new("{{ snake_case (env \"MDMG_TEST_VALUE2\")}}"),
            &MdmgCtx::new("himanoa"),
        );
        remove_var("MDMG_TEST_VALUE2");
        assert_eq!(actual.unwrap(), "foo_adapter")
    }

    #[test]
    fn render_returning_helper() {
        assert_eq!(
            render(
                Template::new("{{pascal_case identify}} {{camel_case identify}} {{kebab_case identify}} {{ snake_case identify }}"), &MdmgCtx::new("exampleAccountRegister")
            )
            .unwrap(),
            "ExampleAccountRegister exampleAccountRegister example-account-register example_account_register"
        )
    }
}
