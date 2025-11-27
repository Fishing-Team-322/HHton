import React, { useState } from "react";
import { Sidebar } from "./components/layout/Sidebar";
import { Topbar } from "./components/layout/Topbar";
import { DashboardPage } from "./pages/DashboardPage";
import { ProfilePage } from "./pages/ProfilePage";
import { TeamsPage } from "./pages/TeamsPage";
import { AuthPage } from "./pages/AuthPage";
import type { Page } from "./types";
import { globalStyles } from "./styles/globalStyles";

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
        {activePage === "dashboard" && <DashboardPage />}
        {activePage === "profile" && <ProfilePage />}
        {activePage === "teams" && <TeamsPage />}
      </main>
    </div>
  );
};

export default App;
