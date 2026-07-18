use super::{
    execution::GeometryExecutionContext, validation::GeometryValidationContext,
    GeometryEngineRegistry, GeometryError, GeometryOperation, GeometryOperationExecution,
};

pub struct GeometryRunner<'a> {
    registry: &'a GeometryEngineRegistry,
}

impl<'a> GeometryRunner<'a> {
    pub fn new(registry: &'a GeometryEngineRegistry) -> Self {
        Self { registry }
    }

    pub fn execute(
        &self,
        operation: GeometryOperation,
        validation_ctx: &GeometryValidationContext,
        execution_ctx: &GeometryExecutionContext,
    ) -> Result<GeometryOperationExecution, GeometryError> {
        let engine = self
            .registry
            .get(&operation.engine)
            .ok_or_else(|| GeometryError::EngineNotFound {
                message: operation.engine.0.clone(),
            })?;
        let validation = engine.validate(&operation, validation_ctx);
        let result = engine.execute(operation.clone(), execution_ctx)?;
        Ok(GeometryOperationExecution {
            operation,
            resolved_engine: Some(engine.descriptor().clone()),
            state: super::execution::GeometryExecutionState::Completed,
            validation: Some(validation),
            progress: None,
            started_at_ms: None,
            finished_at_ms: None,
            result: Some(result),
            failure: None,
            artifacts: Vec::new(),
        })
    }
}
