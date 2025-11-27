import React, { useMemo, useState } from "react";

interface Hackathon {
  id: number;
  title: string;
  status: "upcoming" | "active" | "finished";
  level: "school" | "student" | "pro";
  mode: "online" | "offline" | "hybrid";
  dateRange: string;
  tags: string[];
}

interface UserStats {
  totalHackathons: number;
  wins: number;
  podiums: number;
  averagePlace: number;
}

interface Team {
  id: number;
  name: string;
  country: string;
  motto?: string;
  description?: string;
  membersCount: number;
  hackathonsCount: number;
  level: "beginner" | "mixed" | "pro";
}

type Page = "dashboard" | "profile" | "teams";

type HackathonTab = "recommended" | "inprogress" | "favorites";
type ProfileTab = "overview" | "activity" | "badges" | "certificates";

const hackathonsMock: Hackathon[] = [
  {
    id: 1,
    title: "Future Cities 48h Hack",
    status: "upcoming",
    level: "pro",
    mode: "hybrid",
    dateRange: "12–14 апреля 2025",
    tags: ["AI", "SmartCity", "IoT"],
  },
  {
    id: 2,
    title: "GreenCode Sustainability Sprint",
    status: "active",
    level: "student",
    mode: "online",
    dateRange: "3–5 мая 2025",
    tags: ["Climate", "Data", "Web"],
  },
  {
    id: 3,
    title: "FinBridge Open",
    status: "upcoming",
    level: "pro",
    mode: "offline",
    dateRange: "25–27 июня 2025",
    tags: ["FinTech", "Security", "API"],
  },
  {
    id: 4,
    title: "Indie Game Jam Vol.7",
    status: "active",
    level: "school",
    mode: "online",
    dateRange: "15–17 марта 2025",
    tags: ["GameDev", "Art", "Unity"],
  },
  {
    id: 5,
    title: "HealthTech Surge",
    status: "finished",
    level: "pro",
    mode: "hybrid",
    dateRange: "2–4 февраля 2025",
    tags: ["Med", "AI", "Wearables"],
  },
];

const userStats: UserStats = {
  totalHackathons: 18,
  wins: 4,
  podiums: 9,
  averagePlace: 3.2,
};

const timeline = [
  { label: "Присоединился к команде Neon Ninjas", date: "18 фев 2025" },
  { label: "Команда заняла 2 место на HealthTech Surge", date: "4 фев 2025" },
  { label: "Участвовал в AI Red Team Challenge", date: "21 янв 2025" },
  { label: "Получил бейдж 'Ночной кодер'", date: "15 янв 2025" },
];

const overviewEntries = [
  { title: "GreenCode Sustainability Sprint", role: "Backend", result: "1 место", date: "май 2025" },
  { title: "Future Cities 48h Hack", role: "Data Science", result: "В процессе", date: "апр 2025" },
  { title: "FinBridge Open", role: "Frontend", result: "Приглашение", date: "июн 2025" },
];

const badges = [
  { title: "Первый хакатон", unlocked: true },
  { title: "3 победы", unlocked: true },
  { title: "Ночной кодер", unlocked: true },
  { title: "Марафонец", unlocked: false },
  { title: "Дизайн-гуру", unlocked: false },
  { title: "Ментор", unlocked: false },
];

const certificates = [
  { title: "AI Red Team Challenge", action: "Скачать PDF" },
  { title: "HealthTech Surge", action: "Скачать PDF" },
  { title: "GreenCode Sustainability Sprint", action: "Запросить" },
];

const initialTeams: Team[] = [
  {
    id: 1,
    name: "Neon Ninjas",
    country: "Россия",
    motto: "Мы shipping, пока другие думают",
    description: "Фулстек-сквад, любим сложные интеграции и ночные демо",
    membersCount: 6,
    hackathonsCount: 12,
    level: "pro",
  },
  {
    id: 2,
    name: "Aqua Sparks",
    country: "Казахстан",
    motto: "Данные → решения → вау",
    description: "Data/ML команда со вкусом к визуализациям",
    membersCount: 5,
    hackathonsCount: 9,
    level: "mixed",
  },
  {
    id: 3,
    name: "Pixel Pulse",
    country: "Беларусь",
    motto: "UX без компромиссов",
    description: "Делаем красивые демо и быстрый фронт",
    membersCount: 4,
    hackathonsCount: 7,
    level: "beginner",
  },
];

const gradientText = { background: "linear-gradient(120deg, #7C3AED, #22D3EE)", WebkitBackgroundClip: "text", color: "transparent" } as const;

const SidebarItem: React.FC<{
  label: string;
  active?: boolean;
  onClick?: () => void;
}> = ({ label, active, onClick }) => {
  return (
    <button
      onClick={onClick}
      className={`sidebar-item ${active ? "active" : ""}`}
    >
      <span className="sidebar-dot" />
      <span>{label}</span>
    </button>
  );
};

const HackathonCard: React.FC<{ hackathon: Hackathon; cta?: string }> = ({ hackathon, cta }) => {
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
        <span className={`status-badge ${hackathon.status}`}>{hackathon.status === "active" ? "В процессе" : hackathon.status === "upcoming" ? "Скоро" : "Завершён"}</span>
        <button className="primary-btn alt">{cta ?? "Зарегистрироваться"}</button>
      </div>
    </div>
  );
};

const StatCard: React.FC<{ title: string; value: string | number; accent?: boolean }> = ({ title, value, accent }) => (
  <div className={`stat-card ${accent ? "accent" : ""}`}>
    <div className="stat-title">{title}</div>
    <div className="stat-value">{value}</div>
  </div>
);

const ProfileHeader: React.FC = () => {
  return (
    <div className="profile-header">
      <div className="profile-avatar" />
      <div className="profile-info">
        <div className="profile-name">Алексей Волков <span className="nickname">@volkov.dev</span></div>
        <div className="profile-meta">Москва · Frontend · React / TypeScript</div>
        <p className="profile-bio">
          Создаю интерфейсы для команд, люблю быстрые демо и понятные дашборды. Часто беру на себя фасилитацию и проверку UX-гипотез прямо во время хакатона.
        </p>
        <button className="primary-btn">Редактировать профиль</button>
      </div>
    </div>
  );
};

const TeamCard: React.FC<{ team: Team }> = ({ team }) => (
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

const App: React.FC = () => {
  const [activePage, setActivePage] = useState<Page>("dashboard");
  const [hackathonTab, setHackathonTab] = useState<HackathonTab>("recommended");
  const [profileTab, setProfileTab] = useState<ProfileTab>("overview");
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

  const filteredHackathons = useMemo(() => {
    if (hackathonTab === "recommended") return hackathonsMock.filter((h) => h.status !== "finished");
    if (hackathonTab === "inprogress") return hackathonsMock.filter((h) => h.status === "active");
    return hackathonsMock.filter((h) => h.status !== "finished");
  }, [hackathonTab]);

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
    <div className="app">
      <style>{globalStyles}</style>
      <aside className="sidebar">
        <div className="logo">
          <div className="logo-mark" />
          <div className="logo-text">HACK</div>
        </div>
        <SidebarItem label="Главная" active={activePage === "dashboard"} onClick={() => setActivePage("dashboard")} />
        <SidebarItem label="Хакатоны" />
        <SidebarItem label="Команды" active={activePage === "teams"} onClick={() => setActivePage("teams")} />
        <SidebarItem label="Рейтинг" />
        <SidebarItem label="HR-панель" />
        <SidebarItem label="Профиль" active={activePage === "profile"} onClick={() => setActivePage("profile")} />
      </aside>

      <main className="content">
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

        {activePage === "dashboard" && (
          <section className="page">
            <div className="grid two">
              <div className="hero">
                <div className="hero-content">
                  <div className="hero-label">Ближайший хакатон</div>
                  <h2 className="hero-title">Aurora AI Hack 2025</h2>
                  <p className="hero-subtitle">48 часов, чтобы собрать MVP умных решений для города. Собери команду, выбери направление и покажи, на что ты способен.</p>
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
              <button className={`tab ${hackathonTab === "recommended" ? "active" : ""}`} onClick={() => setHackathonTab("recommended")}>Рекомендуемые</button>
              <button className={`tab ${hackathonTab === "inprogress" ? "active" : ""}`} onClick={() => setHackathonTab("inprogress")}>В прогрессе</button>
              <button className={`tab ${hackathonTab === "favorites" ? "active" : ""}`} onClick={() => setHackathonTab("favorites")}>Избранные</button>
            </div>

            <div className="cards-grid">
              {filteredHackathons.map((h) => (
                <HackathonCard
                  key={h.id}
                  hackathon={h}
                  cta={h.status === "active" ? "Продолжить участие" : "Зарегистрироваться"}
                />
              ))}
            </div>
          </section>
        )}

        {activePage === "profile" && (
          <section className="page">
            <ProfileHeader />
            <div className="stats-row">
              <StatCard title="Участий в хакатонах" value={userStats.totalHackathons} accent />
              <StatCard title="Побед" value={userStats.wins} />
              <StatCard title="Призовых мест" value={userStats.podiums} />
              <StatCard title="Среднее место" value={userStats.averagePlace} />
            </div>
            <div className="tabs">
              <button className={`tab ${profileTab === "overview" ? "active" : ""}`} onClick={() => setProfileTab("overview")}>Обзор</button>
              <button className={`tab ${profileTab === "activity" ? "active" : ""}`} onClick={() => setProfileTab("activity")}>Активность</button>
              <button className={`tab ${profileTab === "badges" ? "active" : ""}`} onClick={() => setProfileTab("badges")}>Значки</button>
              <button className={`tab ${profileTab === "certificates" ? "active" : ""}`} onClick={() => setProfileTab("certificates")}>Сертификаты</button>
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
        )}

        {activePage === "teams" && (
          <section className="page">
            <div className="teams-header">
              <div>
                <div className="breadcrumbs">Мои команды</div>
                <h2 className="page-title" style={gradientText}>Командные профили</h2>
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
                    <select
                      value={formData.country}
                      onChange={(e) => setFormData({ ...formData, country: e.target.value })}
                    >
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
                    <div className="upload-box square">Team Avatar
                      <div className="upload-note">Загрузить jpg/png</div>
                    </div>
                  </div>
                  <div>
                    <div className="upload-box rectangle">Team Cover
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
                  <button type="submit" className="primary-btn">Создать</button>
                  <button type="button" className="ghost-btn" onClick={() => setFormData({ ...formData, name: "" })}>Сбросить</button>
                </div>
              </form>
            </div>
          </section>
        )}
      </main>
    </div>
  );
};

const globalStyles = `
  :root {
    --bg: #020617;
    --panel: rgba(11, 17, 32, 0.9);
    --muted: #9CA3AF;
    --text: #E5E7EB;
    --shadow: 0 16px 50px rgba(0,0,0,0.35);
    --gradient: linear-gradient(120deg, #7C3AED, #22D3EE);
  }

  * { box-sizing: border-box; }
  body { margin: 0; font-family: 'Inter', system-ui, -apple-system, sans-serif; background: var(--bg); color: var(--text); }
  .app { display: flex; min-height: 100vh; background: radial-gradient(circle at 10% 20%, rgba(124,58,237,0.06), transparent 25%), radial-gradient(circle at 80% 0%, rgba(34,211,238,0.06), transparent 20%), var(--bg); }

  .sidebar { width: 230px; background: #050816; border-right: 1px solid rgba(255,255,255,0.04); padding: 24px 16px; position: sticky; top: 0; align-self: flex-start; height: 100vh; display: flex; flex-direction: column; gap: 8px; }
  .logo { display: flex; align-items: center; gap: 12px; padding: 12px 12px 20px; }
  .logo-mark { width: 36px; height: 36px; border-radius: 12px; background: var(--gradient); box-shadow: var(--shadow); }
  .logo-text { font-weight: 800; letter-spacing: 1px; }
  .sidebar-item { background: transparent; border: none; color: var(--text); padding: 12px 14px; display: flex; align-items: center; gap: 10px; border-radius: 14px; cursor: pointer; position: relative; transition: all 0.15s ease-out; }
  .sidebar-item:hover { background: rgba(255,255,255,0.04); transform: translateX(2px); }
  .sidebar-item.active { background: var(--gradient); color: #0B1120; font-weight: 700; box-shadow: 0 10px 30px rgba(124,58,237,0.25); }
  .sidebar-item.active .sidebar-dot { background: #0B1120; }
  .sidebar-dot { width: 8px; height: 8px; border-radius: 50%; background: rgba(255,255,255,0.3); }

  .content { flex: 1; padding: 32px 36px; }
  .topbar { display: flex; justify-content: space-between; align-items: center; margin-bottom: 28px; }
  .page-title { margin: 6px 0 0; font-size: 24px; }
  .breadcrumbs { color: var(--muted); font-size: 12px; letter-spacing: 0.5px; }
  .top-actions { display: flex; gap: 10px; }

  .page { display: flex; flex-direction: column; gap: 24px; }
  .grid.two { display: grid; grid-template-columns: 2fr 1.2fr; gap: 18px; }
  .hero { background: var(--gradient); border-radius: 20px; padding: 22px; position: relative; overflow: hidden; min-height: 240px; display: grid; grid-template-columns: 1.5fr 1fr; box-shadow: var(--shadow); }
  .hero:before { content: ""; position: absolute; inset: 0; background: radial-gradient(circle at 20% 20%, rgba(255,255,255,0.1), transparent 35%); }
  .hero-content { position: relative; z-index: 1; display: flex; flex-direction: column; gap: 10px; }
  .hero-label { text-transform: uppercase; letter-spacing: 1px; font-size: 12px; opacity: 0.9; }
  .hero-title { margin: 0; font-size: 28px; font-weight: 800; }
  .hero-subtitle { margin: 0; max-width: 520px; line-height: 1.5; }
  .hero-tags { display: flex; flex-wrap: wrap; gap: 8px; margin: 6px 0 4px; }
  .hero-visual { background: rgba(255,255,255,0.14); backdrop-filter: blur(6px); border-radius: 18px; border: 1px solid rgba(255,255,255,0.25); box-shadow: var(--shadow); }

  .profile-panel { background: var(--panel); border-radius: 20px; padding: 18px; box-shadow: var(--shadow); display: flex; flex-direction: column; gap: 14px; }
  .profile-compact { display: flex; gap: 12px; align-items: center; }
  .avatar { width: 58px; height: 58px; border-radius: 50%; background: linear-gradient(135deg, #22D3EE, #7C3AED); border: 3px solid rgba(255,255,255,0.15); box-shadow: var(--shadow); }
  .name { font-weight: 700; }
  .meta { color: var(--muted); font-size: 13px; }
  .mini-note { color: var(--muted); font-size: 12px; margin-top: 6px; }
  .metrics { display: grid; grid-template-columns: repeat(3, 1fr); gap: 10px; }
  .progress { background: rgba(255,255,255,0.02); border: 1px solid rgba(255,255,255,0.05); border-radius: 14px; padding: 12px; }
  .progress-header { display: flex; justify-content: space-between; font-size: 13px; color: var(--text); }
  .progress-bar { width: 100%; height: 10px; background: rgba(255,255,255,0.08); border-radius: 999px; margin: 10px 0; overflow: hidden; }
  .progress-bar span { display: block; height: 100%; background: var(--gradient); border-radius: 999px; }
  .progress-note { color: var(--muted); font-size: 12px; }

  .tabs { display: flex; gap: 8px; margin-top: 6px; flex-wrap: wrap; }
  .tab { background: rgba(255,255,255,0.04); border: 1px solid rgba(255,255,255,0.05); color: var(--text); padding: 10px 16px; border-radius: 14px; cursor: pointer; transition: all 0.15s ease-out; }
  .tab:hover { transform: translateY(-2px); }
  .tab.active { background: var(--gradient); border-color: transparent; color: #0B1120; font-weight: 700; }

  .cards-grid { display: grid; grid-template-columns: repeat(auto-fit, minmax(250px, 1fr)); gap: 14px; }
  .hackathon-card { background: var(--panel); border-radius: 18px; padding: 14px; display: flex; flex-direction: column; gap: 8px; box-shadow: var(--shadow); border: 1px solid rgba(255,255,255,0.05); transition: all 0.2s ease-out; }
  .hackathon-card:hover { transform: translateY(-4px); border-color: rgba(34,211,238,0.35); }
  .hackathon-card__top { display: flex; align-items: center; gap: 10px; }
  .hackathon-logo { width: 44px; height: 44px; border-radius: 12px; background: linear-gradient(135deg, rgba(34,211,238,0.4), rgba(124,58,237,0.4)); border: 1px solid rgba(255,255,255,0.08); }
  .hackathon-title { font-weight: 700; }
  .hackathon-level { color: var(--muted); font-size: 12px; letter-spacing: 1px; }
  .hackathon-date { color: var(--muted); font-size: 13px; }
  .hackathon-tags { display: flex; gap: 6px; flex-wrap: wrap; }
  .tag { background: rgba(255,255,255,0.06); padding: 6px 10px; border-radius: 999px; color: var(--text); font-size: 12px; }
  .tag.ghost { background: rgba(255,255,255,0.15); color: #0B1120; font-weight: 600; }
  .mode-badge { margin-left: auto; text-transform: capitalize; padding: 6px 10px; border-radius: 10px; font-size: 12px; }
  .mode-badge.online { background: rgba(34,211,238,0.2); color: #8ef; }
  .mode-badge.offline { background: rgba(124,58,237,0.2); color: #bfa2ff; }
  .mode-badge.hybrid { background: rgba(34,211,238,0.2); color: #c5f9ff; border: 1px solid rgba(124,58,237,0.35); }
  .hackathon-footer { display: flex; justify-content: space-between; align-items: center; }
  .status-badge { padding: 6px 10px; border-radius: 10px; font-size: 12px; background: rgba(255,255,255,0.06); color: var(--text); }
  .status-badge.active { color: #22D3EE; }
  .status-badge.upcoming { color: #7C3AED; }
  .status-badge.finished { color: #9CA3AF; }

  .primary-btn { background: var(--gradient); color: #0B1120; border: none; padding: 12px 18px; border-radius: 14px; font-weight: 700; cursor: pointer; transition: all 0.15s ease-out; box-shadow: var(--shadow); }
  .primary-btn:hover { transform: translateY(-2px) scale(1.01); filter: brightness(1.05); }
  .primary-btn.dark { color: #fff; background: rgba(0,0,0,0.25); border: 1px solid rgba(255,255,255,0.2); box-shadow: none; }
  .primary-btn.alt { padding: 10px 14px; box-shadow: none; }
  .ghost-btn { background: rgba(255,255,255,0.05); color: var(--text); border: 1px solid rgba(255,255,255,0.07); padding: 10px 14px; border-radius: 12px; cursor: pointer; transition: all 0.15s ease-out; }
  .ghost-btn:hover { border-color: rgba(34,211,238,0.4); color: #d9fbff; }

  .stat-card { background: rgba(255,255,255,0.04); border-radius: 16px; padding: 14px; border: 1px solid rgba(255,255,255,0.05); }
  .stat-card.accent { background: linear-gradient(135deg, rgba(124,58,237,0.12), rgba(34,211,238,0.12)); border-color: rgba(34,211,238,0.3); }
  .stat-title { color: var(--muted); font-size: 12px; }
  .stat-value { font-size: 22px; font-weight: 800; margin-top: 4px; }

  .stats-row { display: grid; grid-template-columns: repeat(auto-fit, minmax(180px, 1fr)); gap: 12px; }
  .profile-header { background: var(--panel); border-radius: 20px; padding: 20px; display: flex; gap: 16px; align-items: center; box-shadow: var(--shadow); }
  .profile-avatar { width: 90px; height: 90px; border-radius: 24px; background: linear-gradient(135deg, #7C3AED, #22D3EE); border: 3px solid rgba(255,255,255,0.12); box-shadow: var(--shadow); }
  .profile-info { display: flex; flex-direction: column; gap: 8px; }
  .profile-name { font-size: 24px; font-weight: 800; }
  .nickname { color: var(--muted); font-size: 16px; margin-left: 6px; }
  .profile-meta { color: var(--muted); }
  .profile-bio { margin: 0; max-width: 720px; color: #d5d9e2; }

  .list { background: var(--panel); border-radius: 16px; padding: 12px; display: flex; flex-direction: column; gap: 10px; border: 1px solid rgba(255,255,255,0.05); box-shadow: var(--shadow); }
  .list-row { display: flex; justify-content: space-between; align-items: center; padding: 12px; border-radius: 12px; background: rgba(255,255,255,0.02); }
  .list-title { font-weight: 700; }
  .list-sub { color: var(--muted); font-size: 13px; }
  .status-pill { background: rgba(124,58,237,0.15); color: #d6b8ff; padding: 8px 12px; border-radius: 12px; }

  .timeline { background: var(--panel); border-radius: 16px; padding: 14px; border: 1px solid rgba(255,255,255,0.05); box-shadow: var(--shadow); display: flex; flex-direction: column; gap: 12px; }
  .timeline-item { display: flex; gap: 12px; align-items: center; }
  .timeline-dot { width: 12px; height: 12px; border-radius: 50%; background: var(--gradient); box-shadow: 0 0 0 6px rgba(124,58,237,0.12); }
  .timeline-title { font-weight: 600; }
  .timeline-date { color: var(--muted); font-size: 12px; }

  .badge-grid { display: grid; grid-template-columns: repeat(auto-fit, minmax(160px, 1fr)); gap: 10px; }
  .badge-card { background: rgba(255,255,255,0.03); border-radius: 14px; padding: 12px; text-align: center; border: 1px solid rgba(255,255,255,0.05); box-shadow: var(--shadow); }
  .badge-card.locked { filter: grayscale(1); opacity: 0.6; }
  .badge-icon { width: 48px; height: 48px; border-radius: 14px; margin: 0 auto 10px; background: linear-gradient(135deg, rgba(124,58,237,0.3), rgba(34,211,238,0.3)); }
  .badge-title { font-weight: 700; }
  .badge-status { color: var(--muted); font-size: 12px; }

  .teams-header { display: flex; justify-content: space-between; align-items: center; }
  .cards-grid.teams { grid-template-columns: repeat(auto-fit, minmax(260px, 1fr)); }
  .team-card { background: var(--panel); border-radius: 16px; padding: 14px; border: 1px solid rgba(255,255,255,0.05); box-shadow: var(--shadow); transition: all 0.15s ease-out; }
  .team-card:hover { transform: translateY(-3px); border-color: rgba(34,211,238,0.25); }
  .team-card__header { display: flex; align-items: center; gap: 10px; }
  .team-logo { width: 42px; height: 42px; border-radius: 12px; background: linear-gradient(135deg, #22D3EE, #7C3AED); }
  .team-name { font-weight: 700; }
  .team-meta { color: var(--muted); font-size: 13px; }
  .team-slogan { margin: 10px 0; color: #d9dce5; }
  .team-footer { display: flex; justify-content: space-between; color: var(--muted); font-size: 13px; }
  .team-level { margin-left: auto; padding: 6px 10px; border-radius: 10px; font-size: 12px; text-transform: capitalize; background: rgba(255,255,255,0.06); }
  .team-level.pro { border: 1px solid rgba(124,58,237,0.4); }
  .team-level.mixed { border: 1px solid rgba(34,211,238,0.4); }
  .team-level.beginner { border: 1px solid rgba(255,255,255,0.1); }

  .form-card { margin-top: 18px; background: var(--panel); border-radius: 18px; padding: 18px; border: 1px solid rgba(255,255,255,0.05); box-shadow: var(--shadow); }
  .form-header { display: flex; justify-content: space-between; gap: 12px; }
  .form-badge { display: inline-flex; padding: 6px 10px; border-radius: 999px; background: rgba(124,58,237,0.2); color: #d6b8ff; font-size: 12px; letter-spacing: 0.5px; }
  .cover-placeholder { width: 180px; height: 80px; border-radius: 12px; background: rgba(255,255,255,0.04); display: grid; place-items: center; color: var(--muted); border: 1px dashed rgba(255,255,255,0.1); }
  .team-form { margin-top: 14px; display: flex; flex-direction: column; gap: 12px; }
  .form-grid { display: grid; grid-template-columns: repeat(auto-fit, minmax(240px, 1fr)); gap: 12px; }
  label { display: flex; flex-direction: column; gap: 6px; color: var(--text); font-size: 14px; }
  input, textarea, select { background: rgba(255,255,255,0.04); border: 1px solid rgba(255,255,255,0.07); border-radius: 12px; padding: 12px; color: var(--text); font-size: 14px; }
  input:focus, textarea:focus, select:focus { outline: 1px solid rgba(34,211,238,0.5); }
  textarea { resize: vertical; }
  .upload-box { background: rgba(255,255,255,0.03); border: 1px dashed rgba(255,255,255,0.15); border-radius: 12px; padding: 16px; display: grid; place-items: center; color: var(--muted); }
  .upload-box.square { aspect-ratio: 1; }
  .upload-box.rectangle { min-height: 120px; }
  .upload-note { color: var(--muted); font-size: 12px; margin-top: 6px; }
  .switch-row { background: rgba(255,255,255,0.03); border: 1px solid rgba(255,255,255,0.06); border-radius: 12px; padding: 12px; display: flex; justify-content: space-between; align-items: center; }
  .switch-title { font-weight: 700; }
  .switch-sub { color: var(--muted); font-size: 12px; }
  .switch { width: 50px; height: 26px; border-radius: 999px; background: rgba(255,255,255,0.1); border: 1px solid rgba(255,255,255,0.15); cursor: pointer; display: flex; align-items: center; padding: 4px; transition: all 0.15s ease-out; }
  .switch span { width: 18px; height: 18px; background: #fff; border-radius: 50%; transition: all 0.15s ease-out; }
  .switch.on { background: var(--gradient); border-color: transparent; }
  .switch.on span { transform: translateX(22px); background: #0B1120; }
  .form-actions { display: flex; gap: 10px; }

  .hero, .hackathon-card, .team-card, .profile-panel, .profile-header, .form-card { backdrop-filter: blur(6px); }

  @media (max-width: 1100px) {
    .grid.two { grid-template-columns: 1fr; }
    .hero { grid-template-columns: 1fr; }
    .sidebar { position: fixed; left: -260px; top: 0; height: auto; width: 100%; flex-direction: row; overflow-x: auto; padding: 12px; z-index: 10; }
    .sidebar-item { min-width: 120px; justify-content: center; }
    .content { padding-top: 120px; }
  }

  @media (max-width: 768px) {
    .topbar { flex-direction: column; align-items: flex-start; gap: 12px; }
    .metrics { grid-template-columns: repeat(auto-fit, minmax(120px, 1fr)); }
    .hero { padding: 18px; }
    .team-footer { flex-direction: column; align-items: flex-start; gap: 6px; }
    .form-actions { flex-direction: column; }
    .top-actions { width: 100%; justify-content: flex-start; }
  }
`;

export default App;
