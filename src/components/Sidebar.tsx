import type { SkillRecord } from "../types";

interface SidebarProps {
  skills: SkillRecord[];
  activeTool: string | null;
  onToolChange: (tool: string | null) => void;
  onRescan: () => void;
  loading: boolean;
}

function toolLabel(tool: string) {
  switch (tool) {
    case "claude":
      return "Claude";
    case "cursor":
      return "Cursor";
    case "codex":
      return "Codex";
    case "custom":
      return "Custom";
    default:
      return tool;
  }
}

export function Sidebar({
  skills,
  activeTool,
  onToolChange,
  onRescan,
  loading,
}: SidebarProps) {
  const counts = skills.reduce<Record<string, number>>((acc, skill) => {
    acc[skill.tool_source] = (acc[skill.tool_source] ?? 0) + 1;
    return acc;
  }, {});

  const tools = Object.keys(counts).sort();

  return (
    <aside className="sidebar">
      <div className="sidebar__brand">
        <div>
          <p className="eyebrow">Chops Core</p>
          <h1>Skill Index</h1>
        </div>
        <button className="button button--secondary" onClick={onRescan}>
          {loading ? "Scanning..." : "Rescan"}
        </button>
      </div>

      {loading ? (
        <div className="panel-status panel-status--compact">
          <div className="panel-status__spinner panel-status__spinner--small" />
          <p>Scanning configured directories...</p>
        </div>
      ) : null}

      <div className="sidebar__section">
        <button
          className={`filter-chip ${activeTool === null ? "filter-chip--active" : ""}`}
          onClick={() => onToolChange(null)}
        >
          <span>All Skills</span>
          <strong>{skills.length}</strong>
        </button>

        {tools.map((tool) => (
          <button
            key={tool}
            className={`filter-chip ${activeTool === tool ? "filter-chip--active" : ""}`}
            onClick={() => onToolChange(tool)}
          >
            <span>{toolLabel(tool)}</span>
            <strong>{counts[tool]}</strong>
          </button>
        ))}
      </div>

      <div className="sidebar__footnote">
        <p>Current MVP scans local skill directories and edits source files directly.</p>
      </div>
    </aside>
  );
}
