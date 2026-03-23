import type { SkillDetail as SkillDetailType } from "../types";

interface SkillDetailProps {
  detail: SkillDetailType | null;
  draft: string;
  isDirty: boolean;
  isLoading: boolean;
  isSaving: boolean;
  onDraftChange: (value: string) => void;
  onSave: () => void;
}

export function SkillDetail({
  detail,
  draft,
  isDirty,
  isLoading,
  isSaving,
  onDraftChange,
  onSave,
}: SkillDetailProps) {
  if (isLoading) {
    return (
      <section className="detail-panel detail-panel--empty">
        <div className="panel-status">
          <div className="panel-status__spinner" />
          <p>Loading skill content...</p>
        </div>
      </section>
    );
  }

  if (!detail) {
    return (
      <section className="detail-panel detail-panel--empty">
        <h2>Select a skill</h2>
        <p>Pick a skill from the middle column to view and edit its source file.</p>
      </section>
    );
  }

  return (
    <section className="detail-panel">
      <div className="detail-panel__header">
        <div>
          <p className="eyebrow">Editing</p>
          <h2>{detail.record.name}</h2>
          <p className="detail-panel__path">{detail.record.primary_path}</p>
        </div>

        <button
          className="button"
          disabled={!isDirty || isSaving}
          onClick={onSave}
        >
          {isSaving ? "Saving..." : isDirty ? "Save" : "Saved"}
        </button>
      </div>

      <div className="meta-grid">
        <div>
          <span>Tool</span>
          <strong>{detail.record.tool_source}</strong>
        </div>
        <div>
          <span>Format</span>
          <strong>{detail.record.format}</strong>
        </div>
        <div>
          <span>Installations</span>
          <strong>{detail.installations.length}</strong>
        </div>
        <div>
          <span>Size</span>
          <strong>{detail.record.file_size} bytes</strong>
        </div>
      </div>

      {isSaving ? (
        <div className="detail-saving-indicator">
          <div className="panel-status__spinner panel-status__spinner--small" />
          <span>Saving changes...</span>
        </div>
      ) : null}

      <textarea
        className="editor"
        value={draft}
        onChange={(event) => onDraftChange(event.currentTarget.value)}
        spellCheck={false}
      />
    </section>
  );
}
