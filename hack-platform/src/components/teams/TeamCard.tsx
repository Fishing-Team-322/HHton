import React from "react";
import type { Team } from "../../types";

export const TeamCard: React.FC<{ team: Team }> = ({ team }) => (
  <div className="team-card">
    <div className="team-card__header">
      <div className="team-logo" />
      <div>
        <div className="team-name">{team.name}</div>
        <div className="team-meta">{team.country}</div>
      </div>
      <span className={`team-level ${team.level}`}>{team.level}</span>
    </div>
    <div className="team-slogan">{team.motto}</div>
    <div className="team-footer">
      <div>{team.membersCount} участников</div>
      <div>{team.hackathonsCount} хакатонов</div>
    </div>
  </div>
);
