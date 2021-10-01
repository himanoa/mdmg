use crate::error::MdmbError;
use crate::Result;
use handlebars::{Context, Handlebars, Helper, JsonRender, Output, RenderContext, RenderError};
use inflector::Inflector;
use serde::Serialize;

#[derive(Debug, Serialize, Default)]
pub struct MdmgCtx {
    pub identify: String,
}

impl MdmgCtx {
    fn new(identify: &str) -> Self {
        Self {
            identify: identify.to_string(),
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
        .ok_or(RenderError::new(
            "Param 0 is required for pascal_case_decorator.",
        ))
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
        .ok_or(RenderError::new(
            "Param 0 is required for camel_case_decorator.",
        ))
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
        .ok_or(RenderError::new(
            "Param 0 is required for snake_case_decorator.",
        ))
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
        .ok_or(RenderError::new(
            "Param 0 is required for kebab_case_decorator.",
        ))
        .map(|s| s.value().render())?;
    let rendered = target.to_kebab_case();
    out.write(&rendered)?;
    Ok(())
}

pub fn render(template_str: &str, ctx: &MdmgCtx) -> Result<String> {
    let mut handlebars = Handlebars::new();

    handlebars.register_helper("pascal_case", Box::new(pascal_case_helper));
    handlebars.register_helper("camel_case", Box::new(camel_case_helper));
    handlebars.register_helper("kebab_case", Box::new(kebab_case_helper));
    handlebars.register_helper("snake_case", Box::new(snake_case_helper));

    handlebars
        .render_template(template_str, ctx).map_err(|e| MdmbError::TempalteRenderError { reason: e.desc })
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::default::Default;

    #[test]
    fn render_returning_the_piyopoyo() {
        assert_eq!(render("PIYOPIYP", &Default::default()).unwrap(), "PIYOPIYP")
    }

    #[test]
    fn render_returning_the_himanoa() {
        assert_eq!(
            render("{{identify}}", &MdmgCtx::new("himanoa")).unwrap(),
            "himanoa"
        )
    }

    #[test]
    fn render_returning_the_variable() {
        assert_eq!(
            render("{{identify}}", &MdmgCtx::new("himanoa")).unwrap(),
            "himanoa"
        )
    }

    #[test]
    fn render_returning_helper() {
        assert_eq!(
            render(
                "{{pascal_case identify}} {{camel_case identify}} {{kebab_case identify}} {{ snake_case identify }}", &MdmgCtx::new("exampleAccountRegister")
            )
            .unwrap(),
            "ExampleAccountRegister exampleAccountRegister example-account-register example_account_register"
        )
    }
}
