import React from "react";
import type { Page } from "../../types";

interface SidebarProps {
  activePage: Page;
  onChangePage: (page: Page) => void;
}

const SidebarItem: React.FC<{
  label: string;
  active?: boolean;
  onClick?: () => void;
}> = ({ label, active, onClick }) => (
  <button onClick={onClick} className={`sidebar-item ${active ? "active" : ""}`}>
    <span className="sidebar-dot" />
    <span>{label}</span>
  </button>
);

export const Sidebar: React.FC<SidebarProps> = ({ activePage, onChangePage }) => (
  <aside className="sidebar">
    <div className="logo">
      <div className="logo-mark" />
      <div className="logo-text">HACK</div>
    </div>
    <SidebarItem label="Главная" active={activePage === "dashboard"} onClick={() => onChangePage("dashboard")} />
    <SidebarItem label="Хакатоны" onClick={() => onChangePage("dashboard")} />
    <SidebarItem label="Команды" active={activePage === "teams"} onClick={() => onChangePage("teams")} />
    <SidebarItem label="Рейтинг" />
    <SidebarItem label="HR-панель" />
    <SidebarItem label="Профиль" active={activePage === "profile"} onClick={() => onChangePage("profile")} />
  </aside>
);
