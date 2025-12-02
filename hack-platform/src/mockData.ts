import type { Hackathon, Team, UserStats } from "./types";

export const hackathonsMock: Hackathon[] = [
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

export const userStats: UserStats = {
  totalHackathons: 18,
  wins: 4,
  podiums: 9,
  averagePlace: 3.2,
};

export const profileStats = userStats;

export const profileDetails = {
  name: "Алексей Волков",
  nickname: "@volkov.dev",
  city: "Москва",
  role: "Frontend · React / TypeScript",
  joined: "Октябрь 2024",
  bio: "Делаю живые прототипы, люблю командные спринты и быстрые демо для жюри.",
  activityProgress: 70,
  nextLevelHint: "Участвуйте в новых хакатонах, чтобы открыть дополнительные слоты для команд.",
};

export const profileSeason = {
  tier: "Bronze Tier",
  progress: 64,
  note: "Ранг растёт с участием в командных и соло хакатонах. Следующий чекпоинт через 2 участия.",
};

export const timeline = [
  { label: "Присоединился к команде Neon Ninjas", date: "18 фев 2025" },
  { label: "Команда заняла 2 место на HealthTech Surge", date: "4 фев 2025" },
  { label: "Участвовал в AI Red Team Challenge", date: "21 янв 2025" },
  { label: "Получил бейдж 'Ночной кодер'", date: "15 янв 2025" },
];

export const activityFeed = [
  {
    title: "Занял 2 место на HealthTech Surge · роль — Security",
    date: "27 ноября, 2025",
    ago: "5 days ago",
  },
  {
    title: "Релиз демо на GreenCode Sustainability Sprint · роль — Backend",
    date: "15 ноября, 2025",
    ago: "2 weeks ago",
  },
  {
    title: "Участвовал в AI Red Team Challenge · роль — Data Science",
    date: "21 октября, 2025",
    ago: "1 month ago",
  },
  {
    title: "Получил бейдж 'Ночной кодер'",
    date: "15 октября, 2025",
    ago: "1 month ago",
  },
];

export const overviewEntries = [
  { title: "GreenCode Sustainability Sprint", role: "Backend", result: "1 место", date: "май 2025" },
  { title: "Future Cities 48h Hack", role: "Data Science", result: "В процессе", date: "апр 2025" },
  { title: "FinBridge Open", role: "Frontend", result: "Приглашение", date: "июн 2025" },
];

export const badges = [
  { title: "Первый хакатон", unlocked: true },
  { title: "3 победы", unlocked: true },
  { title: "Ночной кодер", unlocked: true },
  { title: "Марафонец", unlocked: false },
  { title: "Дизайн-гуру", unlocked: false },
  { title: "Ментор", unlocked: false },
];

export const certificates = [
  { title: "AI Red Team Challenge", action: "Скачать PDF" },
  { title: "HealthTech Surge", action: "Скачать PDF" },
  { title: "GreenCode Sustainability Sprint", action: "Запросить" },
];

export const initialTeams: Team[] = [
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
