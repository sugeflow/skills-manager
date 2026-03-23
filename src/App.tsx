import { startTransition, useDeferredValue, useEffect, useMemo, useState } from "react";

import "./App.css";
import { SkillDetail } from "./components/SkillDetail";
import { SkillList } from "./components/SkillList";
import { Sidebar } from "./components/Sidebar";
import { getSkill, listSkills, saveSkill, scanAll } from "./lib/api";
import type { SkillDetail as SkillDetailType, SkillRecord } from "./types";

function App() {
  const [skills, setSkills] = useState<SkillRecord[]>([]);
  const [query, setQuery] = useState("");
  const deferredQuery = useDeferredValue(query);
  const [activeTool, setActiveTool] = useState<string | null>(null);
  const [selectedId, setSelectedId] = useState<number | null>(null);
  const [detail, setDetail] = useState<SkillDetailType | null>(null);
  const [draft, setDraft] = useState("");
  const [isScanning, setIsScanning] = useState(false);
  const [isListLoading, setIsListLoading] = useState(false);
  const [isDetailLoading, setIsDetailLoading] = useState(false);
  const [isSaving, setIsSaving] = useState(false);
  const [error, setError] = useState<string | null>(null);

  useEffect(() => {
    void handleScan();
  }, []);

  useEffect(() => {
    void refreshList(deferredQuery);
  }, [deferredQuery]);

  useEffect(() => {
    if (selectedId === null) {
      setDetail(null);
      setDraft("");
      return;
    }

    let cancelled = false;
    setIsDetailLoading(true);
    void getSkill(selectedId)
      .then((value) => {
        if (cancelled) {
          return;
        }
        setDetail(value);
        setDraft(value.record.content);
      })
      .catch((reason) => {
        if (!cancelled) {
          setError(String(reason));
        }
      })
      .finally(() => {
        if (!cancelled) {
          setIsDetailLoading(false);
        }
      });

    return () => {
      cancelled = true;
    };
  }, [selectedId]);

  const visibleSkills = useMemo(() => {
    if (!activeTool) {
      return skills;
    }
    return skills.filter((skill) => skill.tool_source === activeTool);
  }, [activeTool, skills]);

  const isDirty = detail !== null && draft !== detail.record.content;

  async function refreshList(nextQuery: string) {
    setIsListLoading(true);
    try {
      const nextSkills = await listSkills(nextQuery);
      startTransition(() => {
        setSkills(nextSkills);
        if (selectedId && !nextSkills.some((skill) => skill.id === selectedId)) {
          setSelectedId(null);
        }
      });
    } catch (reason) {
      setError(String(reason));
    } finally {
      setIsListLoading(false);
    }
  }

  async function handleScan() {
    setIsScanning(true);
    setError(null);
    try {
      const nextSkills = await scanAll();
      startTransition(() => {
        setSkills(nextSkills);
        if (selectedId && !nextSkills.some((skill) => skill.id === selectedId)) {
          setSelectedId(null);
        }
      });
    } catch (reason) {
      setError(String(reason));
    } finally {
      setIsScanning(false);
    }
  }

  async function handleSave() {
    if (!detail) {
      return;
    }

    setIsSaving(true);
    setError(null);
    try {
      const nextDetail = await saveSkill(detail.record.id, draft);
      setDetail(nextDetail);
      setDraft(nextDetail.record.content);
      await refreshList(query);
    } catch (reason) {
      setError(String(reason));
    } finally {
      setIsSaving(false);
    }
  }

  return (
    <main className="app-shell">
      <Sidebar
        skills={skills}
        activeTool={activeTool}
        onToolChange={setActiveTool}
        onRescan={handleScan}
        loading={isScanning}
      />

      <SkillList
        skills={visibleSkills}
        query={query}
        isLoading={isListLoading}
        onQueryChange={setQuery}
        selectedId={selectedId}
        onSelect={setSelectedId}
      />

      <SkillDetail
        detail={detail}
        draft={draft}
        isDirty={Boolean(isDirty)}
        isLoading={isDetailLoading}
        isSaving={isSaving}
        onDraftChange={setDraft}
        onSave={handleSave}
      />

      {error ? <div className="error-banner">{error}</div> : null}
    </main>
  );
}

export default App;
