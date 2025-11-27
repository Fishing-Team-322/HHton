import React, { useMemo, useState } from "react";
import "./App.css";

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
                <h2 className="page-title neon-text">Командные профили</h2>
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


export default App;
