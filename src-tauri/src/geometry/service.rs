use super::{
    execution::GeometryExecutionContext, runner::GeometryRunner,
    validation::GeometryValidationContext, GeometryEngineRegistry, GeometryError, GeometryOperation,
    GeometryOperationExecution, GeometryValidationReport,
};

pub struct GeometryService {
    registry: GeometryEngineRegistry,
}

impl GeometryService {
    pub fn new() -> Self {
        Self {
            registry: GeometryEngineRegistry::new(),
        }
    }

    pub fn registry(&self) -> &GeometryEngineRegistry {
        &self.registry
    }

    pub fn registry_mut(&mut self) -> &mut GeometryEngineRegistry {
        &mut self.registry
    }

    pub fn validate(
        &self,
        operation: &GeometryOperation,
        ctx: &GeometryValidationContext,
    ) -> Result<GeometryValidationReport, GeometryError> {
        let validator = super::validator::GeometryValidator::new(&self.registry);
        validator.validate(operation, ctx)
    }

    pub fn execute(
        &self,
        operation: GeometryOperation,
        validation_ctx: &GeometryValidationContext,
        execution_ctx: &GeometryExecutionContext,
    ) -> Result<GeometryOperationExecution, GeometryError> {
        let runner = GeometryRunner::new(&self.registry);
        runner.execute(operation, validation_ctx, execution_ctx)
    }
}
