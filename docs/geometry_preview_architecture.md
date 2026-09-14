# Geometry preview architecture

幾何編集のリアクティブなプレビューは、確定済みの `AppState` と分離する。
プレビュー中の座標変更は `StateManager` の `past` / `future` に記録しない。

## Lifecycle

```text
beginGeometryEdit
  Rust: topology を解析し、移動対象原子と変換の基準を返す
  TS: committed molecule を preview base として保持

updateGeometryPreview(value)
  TS: preview base に Quaternion / Translation を適用して表示
  Rust IPC は呼ばない

commitGeometryEdit(value)
  Rust: revision と入力を検証し、既存 reducer で正規座標を計算
  StateManager: 1 回だけ Undo 履歴へ登録

cancelGeometryEdit
  TS: preview を破棄し、committed molecule を表示
```

## API boundary

将来のTauri APIは次の2つを追加する。

```text
prepare_geometry_edit(molecule, request) -> GeometryEditPlan
commit_geometry_edit(request) -> AppState
```

`GeometryEditPlan` はトポロジー解析結果を含み、編集値の更新ごとに再計算しない。
最低限、次の情報を持たせる。

- `baseRevision`
- `operation`（bond length / angle / dihedral）
- `movingAtomIds`
- `pivot`
- `axis`
- `initialValue`
- `mode`

`movingAtomIds` はRust側で `connected_component_without_bond` を用いて決定する。
TypeScript側は分子グラフを再解析せず、対象原子へ剛体変換を適用する。

## Preview transform

- 結合長：並進ベクトル
- 結合角：角度差を回転角とする Quaternion
- 二面角：B-C軸を回転軸とする Quaternion

プレビューは常に編集開始時のスナップショットに対して適用する。逐次更新後の座標を次の基準にしないことで、丸め誤差の蓄積とスライダー操作によるドリフトを防ぐ。

## Consistency rules

`baseRevision` は確定時にRust側で検証する。不一致の場合はコミットせず、最新状態で計画を再生成する。

プレビュー中に原子選択、分子編集、Undo/Redoが発生した場合はプレビューを破棄する。確定後はRustが返した座標を表示へ採用し、TypeScript側の予測結果を正規結果で置き換える。

## Migration order

1. `GeometryEditPlan` と `prepare_geometry_edit` を追加
2. 二面角のプレビューで計画モデルを検証
3. `commit_geometry_edit` を追加し、既存 `SET_DIHEDRAL_ANGLE` を内部利用
4. 結合長、結合角へ展開
5. プレビューセッションをUndo/Redoと統合

