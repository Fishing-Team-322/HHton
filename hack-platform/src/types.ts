export interface Hackathon {
  id: number;
  title: string;
  status: "upcoming" | "active" | "finished";
  level: "school" | "student" | "pro";
  mode: "online" | "offline" | "hybrid";
  dateRange: string;
  tags: string[];
}

export interface UserStats {
  totalHackathons: number;
  wins: number;
  podiums: number;
  averagePlace: number;
}

export interface Team {
  id: number;
  name: string;
  country: string;
  motto?: string;
  description?: string;
  membersCount: number;
  hackathonsCount: number;
  level: "beginner" | "mixed" | "pro";
}

export type Page = "dashboard" | "profile" | "teams" | "hackathon-config";
export type HackathonTab = "recommended" | "inprogress" | "favorites";
export type ProfileTab = "overview" | "activity" | "badges" | "certificates";
