import React from "react";

export const Topbar: React.FC = () => (
  <header className="topbar">
    <div>
      <div className="breadcrumbs">/ Хакатон-платформа</div>
      <h1 className="page-title">Добро пожаловать, Алексей</h1>
    </div>
    <div className="top-actions">
      <button className="ghost-btn">Центр помощи</button>
      <button className="primary-btn">Новая заявка</button>
    </div>
  </header>
);
