import React from "react";

export const StatCard: React.FC<{ title: string; value: string | number; accent?: boolean }> = ({ title, value, accent }) => (
  <div className={`stat-card ${accent ? "accent" : ""}`}>
    <div className="stat-title">{title}</div>
    <div className="stat-value">{value}</div>
  </div>
);
