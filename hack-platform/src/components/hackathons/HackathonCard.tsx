import React from "react";
import type { Hackathon } from "../../types";

export const HackathonCard: React.FC<{ hackathon: Hackathon; cta?: string }> = ({ hackathon, cta }) => {
  return (
    <div className="hackathon-card">
      <div className="hackathon-card__top">
        <div className="hackathon-logo" />
        <div>
          <div className="hackathon-title">{hackathon.title}</div>
          <div className="hackathon-level">{hackathon.level.toUpperCase()}</div>
        </div>
        <span className={`mode-badge ${hackathon.mode}`}>{hackathon.mode}</span>
      </div>
      <div className="hackathon-date">{hackathon.dateRange}</div>
      <div className="hackathon-tags">
        {hackathon.tags.map((tag) => (
          <span key={tag} className="tag">
            {tag}
          </span>
        ))}
      </div>
      <div className="hackathon-footer">
        <span className={`status-badge ${hackathon.status}`}>
          {hackathon.status === "active" ? "В процессе" : hackathon.status === "upcoming" ? "Скоро" : "Завершён"}
        </span>
        <button className="primary-btn alt">{cta ?? "Зарегистрироваться"}</button>
      </div>
    </div>
  );
};
