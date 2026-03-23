import type { SkillRecord } from "../types";

interface SkillListProps {
  skills: SkillRecord[];
  query: string;
  isLoading: boolean;
  onQueryChange: (value: string) => void;
  selectedId: number | null;
  onSelect: (id: number) => void;
}

export function SkillList({
  skills,
  query,
  isLoading,
  onQueryChange,
  selectedId,
  onSelect,
}: SkillListProps) {
  return (
    <section className="skill-list-panel">
      <div className="toolbar">
        <input
          className="search-input"
          placeholder="Search name, description, or content"
          value={query}
          onChange={(event) => onQueryChange(event.currentTarget.value)}
        />
      </div>

      <div className="skill-list">
        {isLoading ? (
          <div className="panel-status">
            <div className="panel-status__spinner" />
            <p>Refreshing skills...</p>
          </div>
        ) : null}

        {skills.map((skill) => (
          <button
            key={skill.id}
            className={`skill-row ${selectedId === skill.id ? "skill-row--active" : ""}`}
            onClick={() => onSelect(skill.id)}
          >
            <div className="skill-row__head">
              <h2>{skill.name}</h2>
              <span className="pill">{skill.tool_source}</span>
            </div>
            <p>{skill.description || "No description"}</p>
            <small>{skill.primary_path}</small>
          </button>
        ))}

        {skills.length === 0 ? (
          <div className="empty-state">
            <h3>No skills found</h3>
            <p>Run a scan or change the current search and filter.</p>
          </div>
        ) : null}
      </div>
    </section>
  );
}
