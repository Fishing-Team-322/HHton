import React, { useState } from "react";
import type { Team } from "../types";
import { TeamCard } from "../components/teams/TeamCard";
import { initialTeams } from "../mockData";

export const TeamsPage: React.FC = () => {
  const [teams, setTeams] = useState<Team[]>(initialTeams);
  const [isPublic, setIsPublic] = useState(true);
  const [formData, setFormData] = useState({
    name: "",
    country: "Россия",
    motto: "",
    description: "",
    website: "",
    telegram: "",
    github: "",
    vk: "",
  });

  const handleCreateTeam = (e: React.FormEvent) => {
    e.preventDefault();
    if (!formData.name.trim()) return;
    const newTeam: Team = {
      id: Date.now(),
      name: formData.name,
      country: formData.country,
      motto: formData.motto || "Новый слоган в пути",
      description: formData.description,
      membersCount: 3,
      hackathonsCount: 0,
      level: "beginner",
    };
    setTeams((prev) => [newTeam, ...prev]);
    setFormData({
      name: "",
      country: "Россия",
      motto: "",
      description: "",
      website: "",
      telegram: "",
      github: "",
      vk: "",
    });
  };

  return (
    <section className="page">
      <div className="teams-header">
        <div>
          <div className="breadcrumbs">Мои команды</div>
          <h2 className="page-title accent-text">Командные профили</h2>
        </div>
        <button className="primary-btn">Создать команду</button>
      </div>
      <div className="cards-grid teams">
        {teams.map((team) => (
          <TeamCard key={team.id} team={team} />
        ))}
      </div>

      <div className="form-card">
        <div className="form-header">
          <div>
            <div className="form-badge">Create Team</div>
            <h3>Новая команда</h3>
            <p>Заполните профиль, чтобы быть заметнее на предстоящих хакатонах</p>
          </div>
          <div className="cover-placeholder">Team Cover</div>
        </div>
        <form onSubmit={handleCreateTeam} className="team-form">
          <div className="form-grid">
            <label>
              <span>Team name *</span>
              <input
                value={formData.name}
                onChange={(e) => setFormData({ ...formData, name: e.target.value })}
                placeholder="Например, Aurora Stack"
                required
              />
            </label>
            <label>
              <span>Country</span>
              <select value={formData.country} onChange={(e) => setFormData({ ...formData, country: e.target.value })}>
                <option>Россия</option>
                <option>Казахстан</option>
                <option>Беларусь</option>
                <option>Украина</option>
                <option>Грузия</option>
              </select>
            </label>
          </div>

          <label>
            <span>Team motto</span>
            <input
              value={formData.motto}
              onChange={(e) => setFormData({ ...formData, motto: e.target.value })}
              placeholder="Слоган, который вдохновит команду"
            />
          </label>
          <label>
            <span>Team description</span>
            <textarea
              value={formData.description}
              onChange={(e) => setFormData({ ...formData, description: e.target.value })}
              placeholder="Пару предложений о стэке, сильных сторонах и опыте"
              rows={3}
            />
          </label>

          <div className="form-grid">
            <div>
              <div className="upload-box square">
                Team Avatar
                <div className="upload-note">Загрузить jpg/png</div>
              </div>
            </div>
            <div>
              <div className="upload-box rectangle">
                Team Cover
                <div className="upload-note">1920x150 — 2880x225</div>
              </div>
            </div>
          </div>

          <label className="switch-row">
            <div>
              <div className="switch-title">Make team profile public</div>
              <div className="switch-sub">Команда появится в поиске и сможет получать приглашения</div>
            </div>
            <div className={`switch ${isPublic ? "on" : ""}`} onClick={() => setIsPublic((p) => !p)}>
              <span />
            </div>
          </label>

          <div className="form-grid">
            <label>
              <span>Website</span>
              <input
                value={formData.website}
                onChange={(e) => setFormData({ ...formData, website: e.target.value })}
                placeholder="https://"
              />
            </label>
            <label>
              <span>Telegram</span>
              <input
                value={formData.telegram}
                onChange={(e) => setFormData({ ...formData, telegram: e.target.value })}
                placeholder="@team"
              />
            </label>
          </div>
          <div className="form-grid">
            <label>
              <span>GitHub</span>
              <input
                value={formData.github}
                onChange={(e) => setFormData({ ...formData, github: e.target.value })}
                placeholder="github.com/"
              />
            </label>
            <label>
              <span>VK</span>
              <input
                value={formData.vk}
                onChange={(e) => setFormData({ ...formData, vk: e.target.value })}
                placeholder="vk.com/"
              />
            </label>
          </div>

          <div className="form-actions">
            <button type="submit" className="primary-btn">
              Создать
            </button>
            <button type="button" className="ghost-btn" onClick={() => setFormData({ ...formData, name: "" })}>
              Сбросить
            </button>
          </div>
        </form>
      </div>
    </section>
  );
};
