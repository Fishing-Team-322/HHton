import React, { useMemo, useState } from "react";
import type { HackathonTab } from "../types";
import { HackathonCard } from "../components/hackathons/HackathonCard";
import { StatCard } from "../components/common/StatCard";
import { hackathonsMock, userStats } from "../mockData";

export const DashboardPage: React.FC = () => {
  const [hackathonTab, setHackathonTab] = useState<HackathonTab>("recommended");

  const filteredHackathons = useMemo(() => {
    if (hackathonTab === "recommended") return hackathonsMock.filter((h) => h.status !== "finished");
    if (hackathonTab === "inprogress") return hackathonsMock.filter((h) => h.status === "active");
    return hackathonsMock.filter((h) => h.status !== "finished");
  }, [hackathonTab]);

  return (
    <section className="page">
      <div className="grid two">
        <div className="hero">
          <div className="hero-content">
            <div className="hero-label">Ближайший хакатон</div>
            <h2 className="hero-title">Aurora AI Hack 2025</h2>
            <p className="hero-subtitle">
              48 часов, чтобы собрать MVP умных решений для города. Собери команду, выбери направление и покажи, на что ты способен.
            </p>
            <div className="hero-tags">
              <span className="tag ghost">Онлайн / офлайн</span>
              <span className="tag ghost">Санкт-Петербург</span>
              <span className="tag ghost">12–14 апреля</span>
            </div>
            <button className="primary-btn dark">Перейти к регистрации</button>
          </div>
          <div className="hero-visual" />
        </div>
        <div className="profile-panel">
          <div className="profile-compact">
            <div className="avatar" />
            <div>
              <div className="name">Алексей Волков</div>
              <div className="meta">@volkov.dev · Москва</div>
              <div className="mini-note">Индивидуальная статистика — учитывается независимо от команды</div>
            </div>
          </div>
          <div className="metrics">
            <StatCard title="Участий" value={userStats.totalHackathons} />
            <StatCard title="Побед" value={userStats.wins} />
            <StatCard title="Призовых" value={userStats.podiums} />
          </div>
          <div className="progress">
            <div className="progress-header">
              <span>До следующего уровня активности</span>
              <span>70%</span>
            </div>
            <div className="progress-bar">
              <span style={{ width: "70%" }} />
            </div>
            <div className="progress-note">Продолжайте участвовать в офлайн хакатонах, чтобы улучшить средний результат.</div>
          </div>
        </div>
      </div>

      <div className="tabs">
        <button className={`tab ${hackathonTab === "recommended" ? "active" : ""}`} onClick={() => setHackathonTab("recommended")}>
          Рекомендуемые
        </button>
        <button className={`tab ${hackathonTab === "inprogress" ? "active" : ""}`} onClick={() => setHackathonTab("inprogress")}>
          В прогрессе
        </button>
        <button className={`tab ${hackathonTab === "favorites" ? "active" : ""}`} onClick={() => setHackathonTab("favorites")}>
          Избранные
        </button>
      </div>

      <div className="cards-grid">
        {filteredHackathons.map((h) => (
          <HackathonCard key={h.id} hackathon={h} cta={h.status === "active" ? "Продолжить участие" : "Зарегистрироваться"} />
        ))}
      </div>
    </section>
  );
};
