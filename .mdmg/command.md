# Command

## src/commands/{{snake_case identify}}.rs

```rust
use crate::Result;

pub trait {{pascal_case identify}}Command {
    fn run(&self) -> Result<()>;
}

pub struct {{pascal_case identify}}CommandImpl;

impl {{pascal_case identify}}CommandImpl {
    pub fn new() -> Self {
        {{pascal_case identify}}CommandImpl
    }
}

impl {{pascal_case identify}}Command for {{pascal_case identify}}CommandImpl {
    fn run(&self) -> Result<()> {
      unimplemented!()
    }
}
```
