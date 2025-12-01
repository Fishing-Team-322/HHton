import React, { useEffect, useState } from "react";
import { Sidebar } from "./components/layout/Sidebar";
import { Topbar } from "./components/layout/Topbar";
import { DashboardPage } from "./pages/DashboardPage";
import { ProfilePage } from "./pages/ProfilePage";
import { TeamsPage } from "./pages/TeamsPage";
import { AuthPage } from "./pages/AuthPage";
import type { Page } from "./types";
import { globalStyles } from "./styles/globalStyles";

type HackathonFormat = "online" | "offline" | "hybrid";
type HackathonStatus = "draft" | "published" | "archived" | string;

interface HackathonConfig {
  id: number;
  organizer_id: number;
  title: string;
  description: string;
  format: HackathonFormat;
  location?: string;
  start_at?: string;
  end_at?: string;
  status?: HackathonStatus;
}

interface HackathonConfigPageProps {
  organizerId: number;
  onBackToDashboard: () => void;
}

const initialFormState: {
  title: string;
  description: string;
  format: HackathonFormat;
  location: string;
  start_at: string;
  end_at: string;
  status: HackathonStatus;
} = {
  title: "",
  description: "",
  format: "online",
  location: "",
  start_at: "",
  end_at: "",
  status: "draft",
};

const HackathonConfigPage: React.FC<HackathonConfigPageProps> = ({ organizerId, onBackToDashboard }) => {
  const [hackathons, setHackathons] = useState<HackathonConfig[]>([]);
  const [editingId, setEditingId] = useState<number | null>(null);
  const [form, setForm] = useState(initialFormState);
  const [loading, setLoading] = useState(false);
  const [saving, setSaving] = useState(false);
  const [error, setError] = useState<string | null>(null);
  const [successMessage, setSuccessMessage] = useState<string | null>(null);

  const fetchHackathons = async () => {
    setLoading(true);
    setError(null);
    try {
      const response = await fetch(`http://localhost:8081/api/v1/organizers/${organizerId}/hackathons`);
      if (!response.ok) {
        throw new Error("Не удалось загрузить список хакатонов");
      }
      const data: HackathonConfig[] = await response.json();
      setHackathons(data);
    } catch (err) {
      setError(err instanceof Error ? err.message : "Ошибка загрузки данных");
    } finally {
      setLoading(false);
    }
  };

  useEffect(() => {
    void fetchHackathons();
  }, [organizerId]);

  const handleChange = (
    field: keyof typeof form,
    e: React.ChangeEvent<HTMLInputElement | HTMLTextAreaElement | HTMLSelectElement>,
  ) => {
    setForm((prev) => ({
      ...prev,
      [field]: e.target.value,
    }));
  };

  const handleSelectHackathon = (hackathon: HackathonConfig) => {
    setEditingId(hackathon.id);
    setForm({
      title: hackathon.title || "",
      description: hackathon.description || "",
      format: hackathon.format || "online",
      location: hackathon.location || "",
      start_at: hackathon.start_at || "",
      end_at: hackathon.end_at || "",
      status: hackathon.status || "draft",
    });
    setSuccessMessage(null);
    setError(null);
  };

  const resetForm = () => {
    setEditingId(null);
    setForm(initialFormState);
    setSuccessMessage(null);
    setError(null);
  };

  const handleCreate = async () => {
    setSaving(true);
    setError(null);
    setSuccessMessage(null);
    try {
      const response = await fetch("http://localhost:8081/api/v1/hackathons", {
        method: "POST",
        headers: { "Content-Type": "application/json" },
        body: JSON.stringify({ ...form, organizer_id: organizerId }),
      });
      if (!response.ok) {
        throw new Error("Не удалось создать хакатон");
      }
      const created: HackathonConfig = await response.json();
      setHackathons((prev) => [created, ...prev]);
      setEditingId(created.id ?? null);
      setSuccessMessage("Хакатон создан");
    } catch (err) {
      setError(err instanceof Error ? err.message : "Ошибка при создании");
    } finally {
      setSaving(false);
    }
  };

  const handleUpdate = async () => {
    if (!editingId) return;
    setSaving(true);
    setError(null);
    setSuccessMessage(null);
    try {
      const response = await fetch(`http://localhost:8081/api/v1/hackathons/${editingId}`, {
        method: "PUT",
        headers: { "Content-Type": "application/json" },
        body: JSON.stringify({ ...form, organizer_id: organizerId }),
      });
      if (!response.ok) {
        throw new Error("Не удалось обновить хакатон");
      }
      const updated: HackathonConfig = await response.json();
      setHackathons((prev) => prev.map((item) => (item.id === updated.id ? updated : item)));
      setSuccessMessage("Изменения сохранены");
    } catch (err) {
      setError(err instanceof Error ? err.message : "Ошибка при обновлении");
    } finally {
      setSaving(false);
    }
  };

  const formatDate = (value?: string) => {
    if (!value) return "Без даты";
    const parsed = new Date(value);
    return Number.isNaN(parsed.getTime()) ? value : parsed.toLocaleDateString("ru-RU");
  };

  const selectedTitle = editingId
    ? hackathons.find((item) => item.id === editingId)?.title || form.title || "Без названия"
    : "Новый хакатон";

  return (
    <section className="page">
      <div className="hack-config-toolbar">
        <button className="ghost-btn" onClick={onBackToDashboard}>
          Назад на дашборд
        </button>
        <div className="hack-config-status inline">
          {loading ? "Загрузка..." : error ? error : successMessage || ""}
        </div>
        <button className="primary-btn alt" onClick={resetForm}>
          Создать новый
        </button>
      </div>

      <div className="hack-config-layout">
        <div className="hack-config-list">
          <div className="hack-config-header">
            <div>
              <div className="hack-config-title">Мои хакатоны</div>
              <div className="hack-config-meta">Организатор #{organizerId}</div>
            </div>
            <div className="hack-config-counter">{hackathons.length}</div>
          </div>

          {loading && <div className="hack-config-meta">Загружаем список...</div>}
          {!loading && hackathons.length === 0 && <div className="hack-config-meta">Пока нет хакатонов</div>}
          {error && <div className="hack-config-status error">{error}</div>}

          <div className="hack-config-items">
            {hackathons.map((hackathon) => (
              <button
                key={hackathon.id}
                className={`hack-config-item ${editingId === hackathon.id ? "active" : ""}`}
                onClick={() => handleSelectHackathon(hackathon)}
              >
                <div className="hack-config-item__row">
                  <div>
                    <div className="hack-config-item__title">{hackathon.title}</div>
                    <div className="hack-config-meta">{hackathon.status || "draft"}</div>
                  </div>
                  <span className={`mode-badge ${hackathon.format}`}>{hackathon.format}</span>
                </div>
                <div className="hack-config-meta">
                  {formatDate(hackathon.start_at)} — {formatDate(hackathon.end_at)}
                </div>
              </button>
            ))}
          </div>
        </div>

        <div className="hack-config-form form-card">
          <div className="hack-config-header">
            <div>
              <div className="hack-config-status-label">{editingId ? `Редактирование: ${selectedTitle}` : "Новый хакатон"}</div>
              <div className="hack-config-meta">Заполните поля и сохраните изменения</div>
            </div>
            <button className="ghost-btn" onClick={resetForm}>
              Создать новый
            </button>
          </div>

          <div className="hack-config-status info">
            {saving ? "Сохраняем..." : successMessage || ""}
          </div>

          <div className="form-grid">
            <label>
              Название хакатона*
              <input value={form.title} onChange={(e) => handleChange("title", e)} placeholder="Future Cities 48h Hack" required />
            </label>
            <label>
              Формат
              <select value={form.format} onChange={(e) => handleChange("format", e)}>
                <option value="online">Онлайн</option>
                <option value="offline">Офлайн</option>
                <option value="hybrid">Гибрид</option>
              </select>
            </label>
            <label>
              Локация
              <input value={form.location} onChange={(e) => handleChange("location", e)} placeholder="Ростов-на-Дону" />
            </label>
            <label>
              Статус
              <select value={form.status} onChange={(e) => handleChange("status", e)}>
                <option value="draft">Черновик</option>
                <option value="published">Опубликован</option>
                <option value="archived">Архив</option>
              </select>
            </label>
            <label>
              Дата/время начала
              <input value={form.start_at} onChange={(e) => handleChange("start_at", e)} placeholder="2025-04-12T09:00:00Z" />
            </label>
            <label>
              Дата/время конца
              <input value={form.end_at} onChange={(e) => handleChange("end_at", e)} placeholder="2025-04-14T18:00:00Z" />
            </label>
          </div>

          <label>
            Краткое описание*
            <textarea
              value={form.description}
              onChange={(e) => handleChange("description", e)}
              placeholder="Опишите правила, призы и ключевые направления"
              rows={4}
              required
            />
          </label>

          <div className="form-actions">
            <button className="primary-btn" onClick={handleCreate} disabled={saving}>
              Создать хакатон
            </button>
            <button className="ghost-btn" onClick={handleUpdate} disabled={!editingId || saving}>
              Сохранить изменения
            </button>
          </div>

          {error && <div className="hack-config-status error">{error}</div>}
        </div>
      </div>
    </section>
  );
};

const App: React.FC = () => {
  const [activePage, setActivePage] = useState<Page>("dashboard");
  const [isAuthenticated, setIsAuthenticated] = useState(false);

  if (!isAuthenticated) {
    return (
      <div className="app">
        <style>{globalStyles}</style>
        <AuthPage onAuthSuccess={() => setIsAuthenticated(true)} />
      </div>
    );
  }

  return (
    <div className="app">
      <style>{globalStyles}</style>

      <Sidebar activePage={activePage} onChangePage={setActivePage} />

      <main className="content">
        <Topbar />
        {activePage === "dashboard" && <DashboardPage onOpenHackConfig={() => setActivePage("hackathon-config")} />}
        {activePage === "profile" && <ProfilePage />}
        {activePage === "teams" && <TeamsPage />}
        {activePage === "hackathon-config" && (
          <HackathonConfigPage organizerId={1} onBackToDashboard={() => setActivePage("dashboard")} />
        )}
      </main>
    </div>
  );
};

export default App;
