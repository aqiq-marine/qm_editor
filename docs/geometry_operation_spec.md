# Geometry Operation Domain Spec

## 目的

QM Editor における幾何操作を、分子編集・AI エージェント・Gaussian 入力生成から独立したドメインモデルとして定義する。

この仕様は、実装詳細ではなく以下を対象とする。

- 型設計
- 責務分離
- モジュール境界
- Validation と Execution の関係
- 将来の Pipeline / Artifact モデルへの拡張方針

## 設計方針

- 1 回の幾何操作要求は `GeometryOperation` として表す
- 実行状態は `GeometryOperationExecution` に分離する
- 入力と出力は `Artifact` 中心で接続できるようにする
- `Workflow` は将来の多段 Pipeline 用に予約する
- Engine 差分はできるだけ隠蔽し、要求側のモデルを安定させる

## 用語

### GeometryOperation

単発の幾何操作要求。

例:

- 3D 構造生成
- 構造最適化
- Scan
- Conformer Search

### GeometryOperationExecution

GeometryOperation の実行状態を表す。

保持する情報の例:

- state
- validation
- progress
- started_at
- finished_at
- result
- failure

### GeometryArtifact

Pipeline 上で受け渡される成果物の抽象概念。

構造そのものだけではなく、以下も含める。

- 最適化構造
- Conformer 集合
- Scan 結果
- エネルギープロファイル
- 中間生成物

### GeometryInput

GeometryOperation が対象とする入力参照。

## コアモデル

### GeometryOperation

```rust
pub struct GeometryOperation {
    pub id: OperationId,
    pub input: GeometryInput,
    pub engine: GeometryEngineId,
    pub objective: GeometryObjective,
    pub constraints: Vec<GeometryConstraint>,
    pub options: GeometryOptions,
}
```

### GeometryInput

```rust
pub struct GeometryInput {
    pub source: GeometryInputSource,
}
```

### GeometryInputSource

```rust
pub enum GeometryInputSource {
    Molecule(MoleculeRef),
    Conformer(ConformerRef),
    Artifact(GeometryArtifactRef),
}
```

### GeometryObjective

```rust
pub enum GeometryObjective {
    Build3D,
    Optimize,
    Scan(ScanObjective),
    ConformerSearch,
    Custom { name: String },
}
```

### ScanObjective

```rust
pub struct ScanObjective {
    pub coordinate: ScanCoordinate,
    pub range: ScanRange,
    pub step_count: u32,
    pub step_size: Option<f64>,
}
```

### GeometryConstraint

```rust
pub enum GeometryConstraint {
    AtomFixed { atom: AtomRef },
    CoordinateFixed(GeometryCoordinateConstraint),
    Distance(DistanceConstraint),
    Plane(PlaneConstraint),
    Symmetry(SymmetryConstraint),
    Custom { name: String, payload: serde_json::Value },
}
```

### GeometryOptions

```rust
pub struct GeometryOptions {
    pub max_iterations: Option<u32>,
    pub convergence: Option<ConvergencePolicy>,
    pub allow_approximation: bool,
    pub coordinate_frame: Option<CoordinateFrameRef>,
}
```

## Artifact モデル

### GeometryArtifactRef

```rust
pub struct GeometryArtifactRef {
    pub id: ArtifactId,
    pub kind: GeometryArtifactKind,
}
```

### GeometryArtifactKind

```rust
pub enum GeometryArtifactKind {
    Structure,
    ConformerSet,
    ScanProfile,
    EnergyProfile,
    Intermediate,
    Custom { name: String },
}
```

## Execution モデル

### GeometryOperationExecution

```rust
pub struct GeometryOperationExecution {
    pub operation: GeometryOperation,
    pub resolved_engine: Option<GeometryEngineDescriptor>,
    pub state: GeometryExecutionState,
    pub validation: Option<GeometryValidationReport>,
    pub progress: Option<GeometryExecutionProgress>,
    pub started_at: Option<DateTime<Utc>>,
    pub finished_at: Option<DateTime<Utc>>,
    pub result: Option<GeometryResult>,
    pub failure: Option<GeometryExecutionError>,
}
```

### GeometryExecutionState

```rust
pub enum GeometryExecutionState {
    Draft,
    Validated,
    Running,
    Completed,
    Failed,
    Cancelled,
}
```

### GeometryExecutionProgress

```rust
pub struct GeometryExecutionProgress {
    pub stage: GeometryExecutionStage,
    pub phase: Option<String>,
    pub fraction: Option<f32>,
    pub message: Option<String>,
}
```

### GeometryExecutionStage

```rust
pub enum GeometryExecutionStage {
    Preparing,
    Embedding,
    Optimizing,
    Scanning,
    Evaluating,
    WritingOutput,
    Completed,
}
```

### GeometryExecutionEvent

実行中のイベントは Result ではなく Execution 側のランタイム責務として扱う。

```rust
pub enum GeometryExecutionEvent {
    Progress(GeometryExecutionProgress),
    Warning(GeometryRuntimeWarning),
    Log { level: LogLevel, message: String },
}
```

## Validation

### GeometryValidationReport

```rust
pub struct GeometryValidationReport {
    pub status: GeometryValidationStatus,
    pub issues: Vec<GeometryValidationIssue>,
}
```

### GeometryValidationStatus

```rust
pub enum GeometryValidationStatus {
    Valid,
    Invalid,
    Unsupported,
}
```

### Validation の責務

- `status` は大分類のみを表す
- `issues` は理由と詳細を表す
- `Warning` や `Approximate` は issue 側で表現する
- `Unsupported` は独立した状態として残す

## Result

### GeometryResult

```rust
pub struct GeometryResult {
    pub primary_structure: Option<GeometryStructure>,
    pub conformers: Vec<ConformerResult>,
    pub energies: Vec<EnergyRecord>,
    pub run: Option<GeometryRunSummary>,
    pub metadata: GeometryResultMetadata,
}
```

### GeometryRunSummary

```rust
pub struct GeometryRunSummary {
    pub iteration_count: Option<u64>,
    pub elapsed: Option<std::time::Duration>,
    pub termination: GeometryTermination,
    pub warnings: Vec<GeometryRuntimeWarning>,
}
```

### GeometryTermination

```rust
pub enum GeometryTermination {
    Converged,
    MaxIterationsReached,
    NumericalFailure,
    Cancelled,
    CompletedWithoutConvergence,
}
```

### Result の責務

- 最終成果物を表す
- 実行要約を含む
- 実行途中のイベントは含めない

## Engine

### GeometryEngineDescriptor

```rust
pub struct GeometryEngineDescriptor {
    pub id: GeometryEngineId,
    pub display_name: String,
    pub version: String,
}
```

### GeometryEngine

Engine は実行主体であり、Registry は持たない。

```rust
pub trait GeometryEngine: Send + Sync {
    fn descriptor(&self) -> &GeometryEngineDescriptor;
    fn validate(
        &self,
        operation: &GeometryOperation,
        ctx: &GeometryValidationContext,
    ) -> GeometryValidationReport;
    fn execute(
        &self,
        operation: GeometryOperation,
        ctx: &GeometryExecutionContext,
    ) -> Result<GeometryResult, GeometryError>;
}
```

## Registry

Registry の責務は Engine の登録・検索・取得に限定する。

- 登録
- 解決
- 列挙

Registry は以下を担当しない。

- Validation
- Execution
- Planning

## Planner / Validator / Runner

`service.rs` を単一ファサードにする場合でも、責務は内部で分ける。

- Planner: Operation から実行方針を組み立てる
- Validator: 実行可能性を判定する
- Runner: 実行を担当する

将来の拡張を考えると、論理的にはこの 3 分割が望ましい。

## モジュール構成

```text
geometry/
  mod.rs
  input.rs
  artifact.rs
  operation.rs
  objective.rs
  constraint.rs
  options.rs
  execution.rs
  progress.rs
  validation.rs
  result.rs
  engine.rs
  registry.rs
  planner.rs
  validator.rs
  runner.rs
  error.rs
```

## 責務分離の要点

- `Operation` は要求
- `Execution` は状態
- `Result` は出力
- `Artifact` は Pipeline 接続点
- `Registry` は解決
- `Planner / Validator / Runner` は処理の分離

## 将来の Pipeline 拡張

将来は以下のモデルを上位概念として追加できる。

```text
Artifact -> Operation -> Artifact -> Operation -> Artifact
```

追加候補:

- GeometryPipeline
- GeometryPipelineExecution
- GeometryStep
- GeometryArtifactGraph

このときの基本方針は以下。

- 前段の `GeometryResult` を `GeometryArtifactRef` に変換する
- 次段は `GeometryInputSource::Artifact(...)` を受け取る
- 単発 `GeometryOperation` はそのまま再利用する
- `Workflow` は多段処理のために予約しておく

## 採用方針の要約

- 単発要求は `GeometryOperation`
- 実行状態は `GeometryOperationExecution`
- 入力は `GeometryInputSource`
- 成果物は `GeometryArtifact`
- 進捗は `stage + phase + fraction + message`
- 実行中イベントは `GeometryExecutionEvent`
- `Registry` は解決専用
- `service.rs` は Planner / Validator / Runner に分離可能な構造にする

