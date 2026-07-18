use super::{
    execution::GeometryExecutionContext, validation::GeometryValidationContext,
    GeometryEngineRegistry, GeometryError, GeometryOperation, GeometryValidationReport,
};

pub struct GeometryValidator<'a> {
    registry: &'a GeometryEngineRegistry,
}

impl<'a> GeometryValidator<'a> {
    pub fn new(registry: &'a GeometryEngineRegistry) -> Self {
        Self { registry }
    }

    pub fn validate(
        &self,
        operation: &GeometryOperation,
        ctx: &GeometryValidationContext,
    ) -> Result<GeometryValidationReport, GeometryError> {
        let engine = self
            .registry
            .get(&operation.engine)
            .ok_or_else(|| GeometryError::EngineNotFound {
                message: operation.engine.0.clone(),
            })?;
        Ok(engine.validate(operation, ctx))
    }

    pub fn validate_for_execution(
        &self,
        operation: &GeometryOperation,
        ctx: &GeometryValidationContext,
    ) -> Result<GeometryValidationReport, GeometryError> {
        self.validate(operation, ctx)
    }
}

#[allow(dead_code)]
pub fn _execution_context_example() -> GeometryExecutionContext {
    GeometryExecutionContext::new()
}
