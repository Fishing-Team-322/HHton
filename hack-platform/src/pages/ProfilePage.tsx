import React, { useState } from "react";
import type { ProfileTab } from "../types";
import { ProfileHeader } from "../components/profile/ProfileHeader";
import { StatCard } from "../components/common/StatCard";
import { badges, certificates, overviewEntries, timeline, userStats } from "../mockData";

export const ProfilePage: React.FC = () => {
  const [profileTab, setProfileTab] = useState<ProfileTab>("overview");

  return (
    <section className="page">
      <ProfileHeader />
      <div className="stats-row">
        <StatCard title="Участий в хакатонах" value={userStats.totalHackathons} accent />
        <StatCard title="Побед" value={userStats.wins} />
        <StatCard title="Призовых мест" value={userStats.podiums} />
        <StatCard title="Среднее место" value={userStats.averagePlace} />
      </div>
      <div className="tabs">
        <button className={`tab ${profileTab === "overview" ? "active" : ""}`} onClick={() => setProfileTab("overview")}>
          Обзор
        </button>
        <button className={`tab ${profileTab === "activity" ? "active" : ""}`} onClick={() => setProfileTab("activity")}>
          Активность
        </button>
        <button className={`tab ${profileTab === "badges" ? "active" : ""}`} onClick={() => setProfileTab("badges")}>
          Значки
        </button>
        <button className={`tab ${profileTab === "certificates" ? "active" : ""}`} onClick={() => setProfileTab("certificates")}>
          Сертификаты
        </button>
      </div>

      {profileTab === "overview" && (
        <div className="list">
          {overviewEntries.map((item) => (
            <div key={item.title} className="list-row">
              <div>
                <div className="list-title">{item.title}</div>
                <div className="list-sub">{item.date} · Роль: {item.role}</div>
              </div>
              <span className="status-pill">{item.result}</span>
            </div>
          ))}
        </div>
      )}

      {profileTab === "activity" && (
        <div className="timeline">
          {timeline.map((item) => (
            <div key={item.label} className="timeline-item">
              <div className="timeline-dot" />
              <div>
                <div className="timeline-title">{item.label}</div>
                <div className="timeline-date">{item.date}</div>
              </div>
            </div>
          ))}
        </div>
      )}

      {profileTab === "badges" && (
        <div className="badge-grid">
          {badges.map((badge) => (
            <div key={badge.title} className={`badge-card ${badge.unlocked ? "" : "locked"}`}>
              <div className="badge-icon" />
              <div className="badge-title">{badge.title}</div>
              <div className="badge-status">{badge.unlocked ? "Получен" : "Заблокирован"}</div>
            </div>
          ))}
        </div>
      )}

      {profileTab === "certificates" && (
        <div className="list">
          {certificates.map((cert) => (
            <div key={cert.title} className="list-row">
              <div className="list-title">{cert.title}</div>
              <button className="ghost-btn">{cert.action}</button>
            </div>
          ))}
        </div>
      )}
    </section>
  );
};
