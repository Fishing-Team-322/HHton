import React, { useEffect, useMemo, useState } from "react";
import { css, Global, ThemeProvider } from "@emotion/react";
import styled from "@emotion/styled";
import type { Hackathon, HackathonTab, Page, Team } from "./types";
import { currentUser, hackathonsMock, initialTeams, userStats } from "./mockData";
import ProfilePage from "./pages/ProfilePage";

type HackathonFormat = "online" | "offline" | "hybrid";
type HackathonStatus = "draft" | "published" | "archived" | string;

interface HackathonConfig {
  id: number;
  organizer_id: number;
  title: string;
  description: string;
  format: HackathonFormat;
  location?: string;
  start_at?: string;
  end_at?: string;
  status?: HackathonStatus;
}

interface HackathonConfigPageProps {
  organizerId: number;
  onBackToDashboard: () => void;
}

const theme = {
  colors: {
    bg: "#020617",
    bgElevated: "#020817",
    sidebar: "#020617",
    card: "#020818",
    cardSoft: "#050b1f",
    borderSoft: "rgba(148, 163, 184, 0.18)",
    text: "#E5E7EB",
    textMuted: "#9CA3AF",
    accent: "#A3FF12",
    accentSoft: "rgba(163,255,18,0.15)",
    danger: "#F97373",
  },
  radii: {
    card: 14,
    pill: 999,
  },
};

type AppTheme = typeof theme;

declare module "@emotion/react" {
  export interface Theme extends AppTheme {}
}

const globalResetStyles = css`
  * {
    box-sizing: border-box;
  }

  body {
    margin: 0;
    padding: 0;
    background: ${theme.colors.bg};
    color: ${theme.colors.text};
    font-family: "Inter", system-ui, -apple-system, BlinkMacSystemFont, "Segoe UI", sans-serif;
  }

  button,
  input,
  textarea,
  select {
    font-family: inherit;
  }
`;

const AppShell = styled.div`
  display: flex;
  min-height: 100vh;
  background: ${({ theme }) => theme.colors.bg};
  color: ${({ theme }) => theme.colors.text};
`;

const Sidebar = styled.aside`
  width: 240px;
  background: ${({ theme }) => theme.colors.sidebar};
  border-right: 1px solid ${({ theme }) => theme.colors.borderSoft};
  padding: 24px 18px;
  display: flex;
  flex-direction: column;
  gap: 20px;
  position: sticky;
  top: 0;
  height: 100vh;
`;

const SidebarHeader = styled.div`
  display: flex;
  align-items: center;
  gap: 12px;
  padding: 6px 10px 10px;
`;

const LogoMark = styled.div`
  width: 42px;
  height: 42px;
  border-radius: 12px;
  background: ${({ theme }) => theme.colors.accent};
  box-shadow: 0 0 0 1px rgba(163, 255, 18, 0.3);
`;

const LogoText = styled.div`
  font-weight: 800;
  letter-spacing: 1px;
  font-size: 18px;
`;

const SidebarModeSwitcher = styled.div`
  display: grid;
  grid-template-columns: repeat(3, 1fr);
  gap: 6px;
  background: ${({ theme }) => theme.colors.cardSoft};
  border-radius: ${({ theme }) => theme.radii.card}px;
  border: 1px solid ${({ theme }) => theme.colors.borderSoft};
  padding: 6px;
`;

const SidebarModeButton = styled.button<{ active?: boolean }>`
  border: none;
  border-radius: ${({ theme }) => theme.radii.pill}px;
  padding: 10px 0;
  background: ${({ active, theme }) => (active ? theme.colors.accent : theme.colors.card)};
  color: ${({ active, theme }) => (active ? "#031004" : theme.colors.textMuted)};
  font-weight: 700;
  cursor: pointer;
  transition: all 0.12s ease;
  display: flex;
  align-items: center;
  justify-content: center;
  gap: 6px;

  &:hover {
    color: ${({ theme, active }) => (active ? "#031004" : theme.colors.text)};
    border: 1px solid ${({ theme }) => theme.colors.accentSoft};
  }
`;

const TierCard = styled.div`
  background: ${({ theme }) => theme.colors.card};
  border-radius: ${({ theme }) => theme.radii.card}px;
  border: 1px solid ${({ theme }) => theme.colors.borderSoft};
  padding: 14px 12px;
  display: flex;
  flex-direction: column;
  gap: 8px;
  box-shadow: 0 12px 28px rgba(2, 6, 23, 0.55);
`;

const TierRow = styled.div`
  display: flex;
  align-items: center;
  justify-content: space-between;
`;

const TierBadge = styled.div`
  background: ${({ theme }) => theme.colors.accentSoft};
  color: ${({ theme }) => theme.colors.accent};
  border-radius: ${({ theme }) => theme.radii.pill}px;
  padding: 6px 10px;
  font-weight: 700;
  font-size: 13px;
`;

const SidebarNav = styled.nav`
  display: flex;
  flex-direction: column;
  gap: 4px;
`;

interface NavItemProps {
  active?: boolean;
}

const SidebarNavItem = styled.button<NavItemProps>`
  background: ${({ active, theme }) => (active ? theme.colors.card : "transparent")};
  border: 1px solid
    ${({ active, theme }) => (active ? theme.colors.accentSoft : theme.colors.borderSoft)};
  color: ${({ active, theme }) => (active ? theme.colors.accent : theme.colors.textMuted)};
  border-left: 4px solid ${({ active, theme }) => (active ? theme.colors.accent : "transparent")};
  padding: 12px 12px 12px 14px;
  border-radius: ${({ theme }) => theme.radii.card}px;
  text-align: left;
  display: flex;
  align-items: center;
  gap: 10px;
  cursor: pointer;
  transition: color 0.16s ease, background-color 0.16s ease, border-color 0.16s ease, box-shadow 0.16s ease;
  width: 100%;
  transform: none;

  &:hover {
    color: ${({ theme }) => theme.colors.text};
    border-color: ${({ theme }) => theme.colors.accentSoft};
    box-shadow: 0 8px 18px rgba(2, 6, 23, 0.55);
  }
`;

const SidebarIcon = styled.span`
  width: 26px;
  height: 26px;
  border-radius: 10px;
  background: ${({ theme }) => theme.colors.cardSoft};
  display: grid;
  place-items: center;
  font-size: 13px;
`;

const MainArea = styled.div`
  flex: 1;
  display: flex;
  flex-direction: column;
  min-height: 100vh;
`;

const Topbar = styled.header`
  display: flex;
  align-items: center;
  gap: 16px;
  background: ${({ theme }) => theme.colors.bgElevated};
  border-bottom: 1px solid ${({ theme }) => theme.colors.borderSoft};
  padding: 14px 24px;
`;

const TopbarLeft = styled.div`
  display: flex;
  align-items: center;
  gap: 12px;
  flex: 1;
`;

const SearchBox = styled.div`
  flex: 1;
  background: ${({ theme }) => theme.colors.card};
  border: 1px solid ${({ theme }) => theme.colors.borderSoft};
  border-radius: ${({ theme }) => theme.radii.card}px;
  display: flex;
  align-items: center;
  gap: 10px;
  padding: 10px 12px;
  color: ${({ theme }) => theme.colors.textMuted};
`;

const SearchInput = styled.input`
  flex: 1;
  background: transparent;
  border: none;
  outline: none;
  color: ${({ theme }) => theme.colors.text};
  font-size: 14px;
`;

const TopbarRight = styled.div`
  display: flex;
  align-items: center;
  gap: 10px;
`;

const UserMenuRoot = styled.div`
  position: relative;
  display: flex;
  align-items: center;
`;

const UserMenuButton = styled.button`
  display: flex;
  align-items: center;
  gap: 10px;
  background: ${({ theme }) => theme.colors.card};
  border: 1px solid ${({ theme }) => theme.colors.borderSoft};
  border-radius: ${({ theme }) => theme.radii.card}px;
  padding: 8px 12px;
  cursor: pointer;
  color: ${({ theme }) => theme.colors.text};
  font-weight: 700;
  transition: background-color 0.12s ease, border-color 0.12s ease, color 0.12s ease, box-shadow 0.12s ease;

  &:hover {
    background: ${({ theme }) => theme.colors.cardSoft};
    border-color: ${({ theme }) => theme.colors.accentSoft};
    color: ${({ theme }) => theme.colors.text};
  }

  &:focus-visible {
    outline: 2px solid ${({ theme }) => theme.colors.accentSoft};
    outline-offset: 2px;
  }
`;

const UserAvatar = styled.div`
  width: 30px;
  height: 30px;
  border-radius: 10px;
  background: linear-gradient(135deg, #a3ff12, #6ee7b7, #7dd3fc);
  box-shadow: 0 0 0 1px rgba(163, 255, 18, 0.4);
`;

const CaretIcon = styled.span<{ open: boolean }>`
  font-size: 12px;
  color: ${({ theme }) => theme.colors.textMuted};
  transition: transform 0.14s ease;
  transform: rotate(${({ open }) => (open ? "180deg" : "0deg")});
  display: inline-flex;
  align-items: center;
`;

const UserMenuDropdown = styled.div`
  position: absolute;
  right: 0;
  top: calc(100% + 8px);
  background: ${({ theme }) => theme.colors.card};
  border: 1px solid ${({ theme }) => theme.colors.borderSoft};
  border-radius: ${({ theme }) => theme.radii.card}px;
  box-shadow: 0 16px 40px rgba(2, 6, 23, 0.6);
  min-width: 200px;
  overflow: hidden;
  z-index: 10;
`;

const UserMenuItem = styled.button`
  width: 100%;
  background: transparent;
  border: none;
  color: ${({ theme }) => theme.colors.text};
  padding: 12px 14px;
  text-align: left;
  cursor: pointer;
  font-size: 14px;
  transition: background-color 0.12s ease, color 0.12s ease;

  &:hover {
    background: ${({ theme }) => theme.colors.cardSoft};
    color: ${({ theme }) => theme.colors.text};
  }

  & + & {
    border-top: 1px solid ${({ theme }) => theme.colors.borderSoft};
  }
`;

const GhostButton = styled.button`
  background: ${({ theme }) => theme.colors.card};
  color: ${({ theme }) => theme.colors.text};
  border: 1px solid ${({ theme }) => theme.colors.borderSoft};
  border-radius: ${({ theme }) => theme.radii.card}px;
  padding: 10px 14px;
  cursor: pointer;
  transition: all 0.12s ease;

  &:hover {
    border-color: ${({ theme }) => theme.colors.accentSoft};
  }
`;

const AccentButton = styled.button`
  background: ${({ theme }) => theme.colors.accent};
  color: #031004;
  border: none;
  border-radius: ${({ theme }) => theme.radii.card}px;
  padding: 10px 14px;
  font-weight: 700;
  cursor: pointer;
  transition: transform 0.12s ease, box-shadow 0.12s ease;

  &:hover {
    transform: translateY(-1px);
    box-shadow: 0 8px 20px rgba(163, 255, 18, 0.35);
  }
`;

const Content = styled.main`
  padding: 26px 32px 34px;
  display: flex;
  flex-direction: column;
  gap: 22px;
  background: ${({ theme }) => theme.colors.bgElevated};
  flex: 1;
`;

const SectionGrid = styled.div`
  display: grid;
  grid-template-columns: 1.4fr 1fr;
  gap: 18px;
`;

const Card = styled.div`
  background: ${({ theme }) => theme.colors.card};
  border-radius: ${({ theme }) => theme.radii.card}px;
  border: 1px solid ${({ theme }) => theme.colors.borderSoft};
  padding: 18px;
  box-shadow: 0 18px 45px rgba(2, 6, 23, 0.8);
`;

const CardHeader = styled.div`
  display: flex;
  justify-content: space-between;
  align-items: center;
  gap: 10px;
  margin-bottom: 12px;
`;

const CardTitle = styled.div`
  font-size: 18px;
  font-weight: 700;
`;

const StrongText = styled.div`
  font-weight: 700;
`;

const Muted = styled.div`
  color: ${({ theme }) => theme.colors.textMuted};
  font-size: 13px;
`;

const ProfileCard = styled(Card)`
  position: relative;
  overflow: hidden;
  background: radial-gradient(circle at 20% 20%, rgba(163, 255, 18, 0.1), transparent 32%),
    ${({ theme }) => theme.colors.card};
`;

const ProfileName = styled.h2`
  margin: 0;
  font-size: 26px;
`;

const ProgressBar = styled.div`
  background: ${({ theme }) => theme.colors.cardSoft};
  border-radius: ${({ theme }) => theme.radii.pill}px;
  height: 10px;
  overflow: hidden;
  border: 1px solid ${({ theme }) => theme.colors.borderSoft};
`;

interface ProgressFillProps {
  percent: number;
}

const ProgressFill = styled.span<ProgressFillProps>`
  display: block;
  height: 100%;
  width: ${({ percent }) => `${percent}%`};
  background: linear-gradient(90deg, #73ff52, #a3ff12);
`;

const BadgePill = styled.span`
  display: inline-flex;
  align-items: center;
  gap: 6px;
  background: ${({ theme }) => theme.colors.accentSoft};
  color: ${({ theme }) => theme.colors.accent};
  border-radius: ${({ theme }) => theme.radii.pill}px;
  padding: 6px 10px;
  font-weight: 700;
  font-size: 12px;
`;

const StatsRow = styled.div`
  display: grid;
  grid-template-columns: repeat(auto-fit, minmax(140px, 1fr));
  gap: 12px;
`;

const StatTile = styled(Card)`
  padding: 14px 16px;
`;

const StatValue = styled.div`
  font-size: 20px;
  font-weight: 800;
`;

const StatLabel = styled.div`
  color: ${({ theme }) => theme.colors.textMuted};
  font-size: 13px;
  margin-top: 4px;
`;

const Tabs = styled.div`
  display: inline-flex;
  gap: 6px;
  background: ${({ theme }) => theme.colors.cardSoft};
  padding: 6px;
  border-radius: ${({ theme }) => theme.radii.pill}px;
  border: 1px solid ${({ theme }) => theme.colors.borderSoft};
`;

interface TabButtonProps {
  active?: boolean;
}

const TabButton = styled.button<TabButtonProps>`
  border: none;
  background: ${({ active, theme }) => (active ? theme.colors.card : "transparent")};
  color: ${({ active, theme }) => (active ? theme.colors.text : theme.colors.textMuted)};
  border-radius: ${({ theme }) => theme.radii.pill}px;
  padding: 8px 14px;
  cursor: pointer;
  font-weight: 600;
  transition: all 0.12s ease;
  border: 1px solid ${({ active, theme }) => (active ? theme.colors.accentSoft : "transparent")};

  &:hover {
    color: ${({ theme }) => theme.colors.text};
  }
`;

const CardsGrid = styled.div`
  display: grid;
  grid-template-columns: repeat(auto-fit, minmax(260px, 1fr));
  gap: 14px;
`;

interface HackathonCardProps {
  active?: boolean;
}

const HackathonCard = styled.div<HackathonCardProps>`
  background: ${({ theme }) => theme.colors.card};
  border-radius: ${({ theme }) => theme.radii.card}px;
  border: 1px solid ${({ theme }) => theme.colors.borderSoft};
  padding: 16px;
  display: flex;
  flex-direction: column;
  gap: 10px;
  box-shadow: 0 18px 45px rgba(15, 23, 42, 0.8);
  transition: transform 0.15s ease, box-shadow 0.15s ease, border-color 0.15s ease;

  &:hover {
    transform: translateY(-2px);
    border-color: ${({ theme }) => theme.colors.accentSoft};
    box-shadow: 0 22px 60px rgba(15, 23, 42, 1);
  }
`;

const HackathonHeader = styled.div`
  display: flex;
  justify-content: space-between;
  gap: 8px;
  align-items: center;
`;

const Pill = styled.span`
  background: ${({ theme }) => theme.colors.cardSoft};
  border-radius: ${({ theme }) => theme.radii.pill}px;
  padding: 6px 10px;
  border: 1px solid ${({ theme }) => theme.colors.borderSoft};
  color: ${({ theme }) => theme.colors.textMuted};
  font-size: 12px;
`;

const CTAButton = styled.button`
  background: ${({ theme }) => theme.colors.accentSoft};
  color: ${({ theme }) => theme.colors.accent};
  border: 1px solid ${({ theme }) => theme.colors.accentSoft};
  border-radius: ${({ theme }) => theme.radii.card}px;
  padding: 10px 12px;
  cursor: pointer;
  font-weight: 700;
  transition: all 0.12s ease;

  &:hover {
    border-color: ${({ theme }) => theme.colors.accent};
    background: rgba(163, 255, 18, 0.22);
  }
`;

const SectionTitleRow = styled.div`
  display: flex;
  justify-content: space-between;
  align-items: center;
  gap: 10px;
`;

const TeamGrid = styled.div`
  display: grid;
  grid-template-columns: repeat(auto-fit, minmax(260px, 1fr));
  gap: 12px;
`;

const TeamCard = styled(Card)`
  display: flex;
  flex-direction: column;
  gap: 8px;
`;

const LevelBadge = styled(Pill)`
  color: ${({ theme }) => theme.colors.text};
`;

const FormCard = styled(Card)`
  display: flex;
  flex-direction: column;
  gap: 14px;
`;

const FormGrid = styled.div`
  display: grid;
  grid-template-columns: repeat(auto-fit, minmax(220px, 1fr));
  gap: 12px;
`;

const Label = styled.label`
  display: flex;
  flex-direction: column;
  gap: 6px;
  font-size: 14px;
  color: ${({ theme }) => theme.colors.text};

  input,
  textarea,
  select {
    background: ${({ theme }) => theme.colors.cardSoft};
    border: 1px solid ${({ theme }) => theme.colors.borderSoft};
    color: ${({ theme }) => theme.colors.text};
    border-radius: ${({ theme }) => theme.radii.card}px;
    padding: 10px 12px;
    outline: none;
    transition: border-color 0.12s ease;

    &:focus {
      border-color: ${({ theme }) => theme.colors.accentSoft};
    }
  }

  textarea {
    resize: vertical;
  }
`;

const SwitchRow = styled(Label)`
  flex-direction: row;
  justify-content: space-between;
  align-items: center;
`;

const Switch = styled.div<{ on?: boolean }>`
  width: 56px;
  height: 30px;
  border-radius: 20px;
  border: 1px solid ${({ theme }) => theme.colors.borderSoft};
  background: ${({ on, theme }) => (on ? theme.colors.accentSoft : theme.colors.cardSoft)};
  display: flex;
  align-items: center;
  padding: 4px;
  cursor: pointer;
  transition: all 0.12s ease;

  span {
    width: 22px;
    height: 22px;
    border-radius: 50%;
    background: ${({ theme }) => theme.colors.text};
    transform: translateX(${({ on }) => (on ? "22px" : "0")});
    transition: transform 0.12s ease;
  }
`;

const FormActions = styled.div`
  display: flex;
  gap: 10px;
  flex-wrap: wrap;
`;

const ConfigLayout = styled.div`
  display: grid;
  grid-template-columns: 1.1fr 1.6fr;
  gap: 16px;
`;

const ConfigListCard = styled(Card)`
  display: flex;
  flex-direction: column;
  gap: 12px;
`;

const ConfigListItem = styled.button<{ active?: boolean }>`
  background: ${({ active, theme }) => (active ? theme.colors.accentSoft : theme.colors.cardSoft)};
  border: 1px solid ${({ active, theme }) => (active ? theme.colors.accent : theme.colors.borderSoft)};
  color: ${({ theme }) => theme.colors.text};
  border-radius: ${({ theme }) => theme.radii.card}px;
  padding: 12px;
  text-align: left;
  display: flex;
  flex-direction: column;
  gap: 4px;
  cursor: pointer;
  transition: all 0.12s ease;

  &:hover {
    border-color: ${({ theme }) => theme.colors.accentSoft};
  }
`;

const StatusText = styled.div<{ tone?: "error" | "info" }>`
  color: ${({ tone, theme }) => (tone === "error" ? theme.colors.danger : theme.colors.textMuted)};
  background: ${({ tone, theme }) => (tone === "error" ? "rgba(249,115,115,0.08)" : theme.colors.cardSoft)};
  border: 1px solid
    ${({ tone, theme }) => (tone === "error" ? "rgba(249,115,115,0.35)" : theme.colors.borderSoft)};
  border-radius: ${({ theme }) => theme.radii.card}px;
  padding: 10px 12px;
`;

const AuthRoot = styled.div`
  min-height: 100vh;
  display: grid;
  place-items: center;
  background: radial-gradient(circle at 10% 20%, rgba(163, 255, 18, 0.06), transparent 32%),
    ${({ theme }) => theme.colors.bg};
`;

const AuthCard = styled(Card)`
  width: min(960px, 92vw);
  display: grid;
  grid-template-columns: 1fr 1fr;
  gap: 0;
  padding: 0;
  overflow: hidden;
`;

const AuthLeft = styled.div`
  padding: 26px 22px;
  border-right: 1px solid ${({ theme }) => theme.colors.borderSoft};
  background: linear-gradient(135deg, rgba(163, 255, 18, 0.1), rgba(2, 6, 23, 0.95));
`;

const AuthRight = styled.div`
  padding: 22px;
`;

const AuthTabs = styled.div`
  display: inline-flex;
  background: ${({ theme }) => theme.colors.cardSoft};
  border: 1px solid ${({ theme }) => theme.colors.borderSoft};
  border-radius: ${({ theme }) => theme.radii.pill}px;
  margin-bottom: 12px;
`;

const AuthTab = styled.button<{ active?: boolean }>`
  border: none;
  background: ${({ active, theme }) => (active ? theme.colors.card : "transparent")};
  color: ${({ active, theme }) => (active ? theme.colors.text : theme.colors.textMuted)};
  padding: 10px 16px;
  border-radius: ${({ theme }) => theme.radii.pill}px;
  cursor: pointer;
  font-weight: 700;
`;

const AuthForm = styled.form`
  display: flex;
  flex-direction: column;
  gap: 10px;
`;

const AuthError = styled.div`
  color: ${({ theme }) => theme.colors.danger};
  background: rgba(249, 115, 115, 0.08);
  border: 1px solid rgba(249, 115, 115, 0.35);
  border-radius: ${({ theme }) => theme.radii.card}px;
  padding: 10px 12px;
`;

interface UserMenuProps {
  nickname: string;
  onProfileClick: () => void;
}

const UserMenu: React.FC<UserMenuProps> = ({ nickname, onProfileClick }) => {
  const [open, setOpen] = useState(false);

  const toggle = () => setOpen((v) => !v);
  const close = () => setOpen(false);

  return (
    <UserMenuRoot tabIndex={0} onBlur={close}>
      <UserMenuButton type="button" onClick={toggle}>
        <UserAvatar />
        <span>{nickname}</span>
        <CaretIcon open={open}>▾</CaretIcon>
      </UserMenuButton>

      {open && (
        <UserMenuDropdown>
          <UserMenuItem
            type="button"
            onClick={() => {
              onProfileClick();
              close();
            }}
          >
            Профиль
          </UserMenuItem>
          <UserMenuItem
            type="button"
            onClick={() => {
              console.log("open account settings");
              close();
            }}
          >
            Настройки аккаунта
          </UserMenuItem>
        </UserMenuDropdown>
      )}
    </UserMenuRoot>
  );
};

const initialFormState: {
  title: string;
  description: string;
  format: HackathonFormat;
  location: string;
  start_at: string;
  end_at: string;
  status: HackathonStatus;
} = {
  title: "",
  description: "",
  format: "online",
  location: "",
  start_at: "",
  end_at: "",
  status: "draft",
};

const HackathonConfigPage: React.FC<HackathonConfigPageProps> = ({ organizerId, onBackToDashboard }) => {
  const [hackathons, setHackathons] = useState<HackathonConfig[]>([]);
  const [editingId, setEditingId] = useState<number | null>(null);
  const [form, setForm] = useState(initialFormState);
  const [loading, setLoading] = useState(false);
  const [saving, setSaving] = useState(false);
  const [error, setError] = useState<string | null>(null);
  const [successMessage, setSuccessMessage] = useState<string | null>(null);

  const fetchHackathons = async () => {
    setLoading(true);
    setError(null);
    try {
      const response = await fetch(`http://localhost:8081/api/v1/organizers/${organizerId}/hackathons`);
      if (!response.ok) {
        throw new Error("Не удалось загрузить список хакатонов");
      }
      const data: HackathonConfig[] = await response.json();
      setHackathons(data);
    } catch (err) {
      setError(err instanceof Error ? err.message : "Ошибка загрузки данных");
    } finally {
      setLoading(false);
    }
  };

  useEffect(() => {
    void fetchHackathons();
  }, [organizerId]);

  const handleChange = (
    field: keyof typeof form,
    e: React.ChangeEvent<HTMLInputElement | HTMLTextAreaElement | HTMLSelectElement>,
  ) => {
    setForm((prev) => ({
      ...prev,
      [field]: e.target.value,
    }));
  };

  const handleSelectHackathon = (hackathon: HackathonConfig) => {
    setEditingId(hackathon.id);
    setForm({
      title: hackathon.title || "",
      description: hackathon.description || "",
      format: hackathon.format || "online",
      location: hackathon.location || "",
      start_at: hackathon.start_at || "",
      end_at: hackathon.end_at || "",
      status: hackathon.status || "draft",
    });
    setSuccessMessage(null);
    setError(null);
  };

  const resetForm = () => {
    setEditingId(null);
    setForm(initialFormState);
    setSuccessMessage(null);
    setError(null);
  };

  const handleCreate = async () => {
    setSaving(true);
    setError(null);
    setSuccessMessage(null);
    try {
      const response = await fetch("http://localhost:8081/api/v1/hackathons", {
        method: "POST",
        headers: { "Content-Type": "application/json" },
        body: JSON.stringify({ ...form, organizer_id: organizerId }),
      });
      if (!response.ok) {
        throw new Error("Не удалось создать хакатон");
      }
      const created: HackathonConfig = await response.json();
      setHackathons((prev) => [created, ...prev]);
      setEditingId(created.id ?? null);
      setSuccessMessage("Хакатон создан");
    } catch (err) {
      setError(err instanceof Error ? err.message : "Ошибка при создании");
    } finally {
      setSaving(false);
    }
  };

  const handleUpdate = async () => {
    if (!editingId) return;
    setSaving(true);
    setError(null);
    setSuccessMessage(null);
    try {
      const response = await fetch(`http://localhost:8081/api/v1/hackathons/${editingId}`, {
        method: "PUT",
        headers: { "Content-Type": "application/json" },
        body: JSON.stringify({ ...form, organizer_id: organizerId }),
      });
      if (!response.ok) {
        throw new Error("Не удалось обновить хакатон");
      }
      const updated: HackathonConfig = await response.json();
      setHackathons((prev) => prev.map((item) => (item.id === updated.id ? updated : item)));
      setSuccessMessage("Изменения сохранены");
    } catch (err) {
      setError(err instanceof Error ? err.message : "Ошибка при обновлении");
    } finally {
      setSaving(false);
    }
  };

  const formatDate = (value?: string) => {
    if (!value) return "Без даты";
    const parsed = new Date(value);
    return Number.isNaN(parsed.getTime()) ? value : parsed.toLocaleDateString("ru-RU");
  };

  const selectedTitle = editingId
    ? hackathons.find((item) => item.id === editingId)?.title || form.title || "Без названия"
    : "Новый хакатон";

  return (
    <section>
      <SectionTitleRow>
        <div>
          <Muted>Конфигуратор</Muted>
          <CardTitle>Настройка хакатона</CardTitle>
        </div>
        <FormActions>
          <GhostButton onClick={onBackToDashboard}>Назад на дашборд</GhostButton>
          <AccentButton onClick={resetForm}>Создать новый</AccentButton>
        </FormActions>
      </SectionTitleRow>

      <ConfigLayout>
        <ConfigListCard>
          <CardHeader>
            <div>
              <Muted>Мои хакатоны · организатор #{organizerId}</Muted>
              <CardTitle>Список</CardTitle>
            </div>
            <BadgePill>{hackathons.length}</BadgePill>
          </CardHeader>
          {loading && <Muted>Загружаем список...</Muted>}
          {!loading && hackathons.length === 0 && <Muted>Пока нет хакатонов</Muted>}
          {error && <StatusText tone="error">{error}</StatusText>}
          <div style={{ display: "grid", gap: 10 }}>
            {hackathons.map((hackathon) => (
              <ConfigListItem
                key={hackathon.id}
                active={editingId === hackathon.id}
                onClick={() => handleSelectHackathon(hackathon)}
              >
                <div style={{ display: "flex", justifyContent: "space-between", alignItems: "center" }}>
                  <div>
                    <div style={{ fontWeight: 700 }}>{hackathon.title}</div>
                    <Muted>{hackathon.status || "draft"}</Muted>
                  </div>
                  <Pill>{hackathon.format}</Pill>
                </div>
                <Muted>
                  {formatDate(hackathon.start_at)} — {formatDate(hackathon.end_at)}
                </Muted>
              </ConfigListItem>
            ))}
          </div>
        </ConfigListCard>

        <FormCard>
          <CardHeader>
            <div>
              <Muted>{editingId ? `Редактирование: ${selectedTitle}` : "Новый хакатон"}</Muted>
              <CardTitle>Форма настройки</CardTitle>
            </div>
            <GhostButton onClick={resetForm}>Создать новый</GhostButton>
          </CardHeader>

          {saving ? <StatusText tone="info">Сохраняем...</StatusText> : successMessage && <StatusText tone="info">{successMessage}</StatusText>}

          <FormGrid>
            <Label>
              Название хакатона*
              <input value={form.title} onChange={(e) => handleChange("title", e)} placeholder="Future Cities 48h Hack" required />
            </Label>
            <Label>
              Формат
              <select value={form.format} onChange={(e) => handleChange("format", e)}>
                <option value="online">Онлайн</option>
                <option value="offline">Офлайн</option>
                <option value="hybrid">Гибрид</option>
              </select>
            </Label>
            <Label>
              Локация
              <input value={form.location} onChange={(e) => handleChange("location", e)} placeholder="Ростов-на-Дону" />
            </Label>
            <Label>
              Статус
              <select value={form.status} onChange={(e) => handleChange("status", e)}>
                <option value="draft">Черновик</option>
                <option value="published">Опубликован</option>
                <option value="archived">Архив</option>
              </select>
            </Label>
            <Label>
              Дата/время начала
              <input value={form.start_at} onChange={(e) => handleChange("start_at", e)} placeholder="2025-04-12T09:00:00Z" />
            </Label>
            <Label>
              Дата/время конца
              <input value={form.end_at} onChange={(e) => handleChange("end_at", e)} placeholder="2025-04-14T18:00:00Z" />
            </Label>
          </FormGrid>

          <Label>
            Краткое описание*
            <textarea
              value={form.description}
              onChange={(e) => handleChange("description", e)}
              placeholder="Опишите правила, призы и ключевые направления"
              rows={4}
              required
            />
          </Label>

          <FormActions>
            <AccentButton onClick={handleCreate} disabled={saving}>
              Создать хакатон
            </AccentButton>
            <GhostButton onClick={handleUpdate} disabled={!editingId || saving}>
              Сохранить изменения
            </GhostButton>
          </FormActions>

          {error && <StatusText tone="error">{error}</StatusText>}
        </FormCard>
      </ConfigLayout>
    </section>
  );
};

const App: React.FC = () => {
  const [activePage, setActivePage] = useState<Page>("dashboard");
  const [isAuthenticated, setIsAuthenticated] = useState(false);
  const [hackathonTab, setHackathonTab] = useState<HackathonTab>("recommended");
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
  const [authMode, setAuthMode] = useState<"login" | "register">("login");
  const [authErrors, setAuthErrors] = useState<string | null>(null);
  const [authForm, setAuthForm] = useState({
    fullName: "",
    nickname: "",
    city: "",
    email: "",
    password: "",
    confirmPassword: "",
  });

  const filteredHackathons = useMemo(() => {
    if (hackathonTab === "recommended") return hackathonsMock.filter((h) => h.status !== "finished");
    if (hackathonTab === "inprogress") return hackathonsMock.filter((h) => h.status === "active");
    return hackathonsMock.filter((h) => h.status !== "finished");
  }, [hackathonTab]);

  const handleChangePage = (page: Page) => {
    setActivePage(page);
  };

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

  const navItems: { key: Page; label: string; icon: string }[] = [
    { key: "dashboard", label: "Главная", icon: "🏠" },
    { key: "hackathon-config", label: "Конструктор", icon: "🧭" },
    { key: "profile", label: "Профиль", icon: "🎯" },
    { key: "teams", label: "Команды", icon: "👥" },
  ];

  const handleAuthSubmit = (e: React.FormEvent) => {
    e.preventDefault();
    setAuthErrors(null);

    if (!authForm.email || !authForm.password) {
      setAuthErrors("Заполните email и пароль");
      return;
    }

    if (authMode === "register") {
      if (!authForm.fullName || !authForm.nickname || !authForm.city) {
        setAuthErrors("Заполните все поля профиля участника");
        return;
      }
      if (authForm.password !== authForm.confirmPassword) {
        setAuthErrors("Пароли не совпадают");
        return;
      }
    }

    setIsAuthenticated(true);
  };

  if (!isAuthenticated) {
    return (
      <ThemeProvider theme={theme}>
        <Global styles={globalResetStyles} />
        <AuthRoot>
          <AuthCard>
            <AuthLeft>
              <SidebarHeader>
                <LogoMark />
                <LogoText>HACK</LogoText>
              </SidebarHeader>
              <h1 style={{ margin: "10px 0 6px" }}>Платформа хакатонов</h1>
              <Muted>
                Участвуй в онлайн и офлайн хакатонах, собирай команды и копи личную статистику. Все новые аккаунты создаются
                как участники, роль организатора выдаётся позже через админ-панель.
              </Muted>
            </AuthLeft>
            <AuthRight>
              <AuthTabs>
                <AuthTab active={authMode === "login"} onClick={() => setAuthMode("login")}>
                  Вход
                </AuthTab>
                <AuthTab active={authMode === "register"} onClick={() => setAuthMode("register")}>
                  Регистрация
                </AuthTab>
              </AuthTabs>
              <AuthForm onSubmit={handleAuthSubmit}>
                {authMode === "register" && (
                  <>
                    <Label>
                      Полное имя
                      <input
                        value={authForm.fullName}
                        onChange={(e) => setAuthForm({ ...authForm, fullName: e.target.value })}
                        placeholder="Как к вам обращаться"
                      />
                    </Label>
                    <Label>
                      Никнейм
                      <input
                        value={authForm.nickname}
                        onChange={(e) => setAuthForm({ ...authForm, nickname: e.target.value })}
                        placeholder="@ник для платформы"
                      />
                    </Label>
                    <Label>
                      Город
                      <input
                        value={authForm.city}
                        onChange={(e) => setAuthForm({ ...authForm, city: e.target.value })}
                        placeholder="Например, Ростов-на-Дону"
                      />
                    </Label>
                  </>
                )}
                <Label>
                  Email
                  <input
                    type="email"
                    value={authForm.email}
                    onChange={(e) => setAuthForm({ ...authForm, email: e.target.value })}
                    placeholder="you@example.com"
                  />
                </Label>
                <Label>
                  Пароль
                  <input
                    type="password"
                    value={authForm.password}
                    onChange={(e) => setAuthForm({ ...authForm, password: e.target.value })}
                    placeholder="••••••••"
                  />
                </Label>
                {authMode === "register" && (
                  <Label>
                    Повтор пароля
                    <input
                      type="password"
                      value={authForm.confirmPassword}
                      onChange={(e) => setAuthForm({ ...authForm, confirmPassword: e.target.value })}
                      placeholder="••••••••"
                    />
                  </Label>
                )}
                {authErrors && <AuthError>{authErrors}</AuthError>}
                <AccentButton type="submit">{authMode === "login" ? "Войти" : "Создать аккаунт участника"}</AccentButton>
              </AuthForm>
              <GhostButton type="button" onClick={() => setAuthMode(authMode === "login" ? "register" : "login")}>
                {authMode === "login" ? "Нет аккаунта? Зарегистрироваться" : "Уже есть аккаунт? Войти"}
              </GhostButton>
            </AuthRight>
          </AuthCard>
        </AuthRoot>
      </ThemeProvider>
    );
  }

  return (
    <ThemeProvider theme={theme}>
      <Global styles={globalResetStyles} />
      <AppShell>
        <Sidebar>
          <SidebarHeader>
            <LogoMark />
            <LogoText>HACK</LogoText>
          </SidebarHeader>
          <SidebarModeSwitcher>
            <SidebarModeButton active={activePage === "dashboard"} onClick={() => handleChangePage("dashboard")}>
              🏠 Home
            </SidebarModeButton>
            <SidebarModeButton active={activePage === "teams"} onClick={() => handleChangePage("teams")}>
              👥 Teams
            </SidebarModeButton>
            <SidebarModeButton active={activePage === "profile"} onClick={() => handleChangePage("profile")}>
              👤 Profile
            </SidebarModeButton>
          </SidebarModeSwitcher>
          <TierCard>
            <TierRow>
              <div>
                <Muted>Ваш тир</Muted>
                <div style={{ fontWeight: 800 }}>Bronze</div>
              </div>
              <TierBadge>#7283</TierBadge>
            </TierRow>
            <ProgressBar>
              <ProgressFill percent={64} />
            </ProgressBar>
            <Muted>2 участия до Silver Tier</Muted>
          </TierCard>
          <SidebarNav>
            {navItems.map((item) => (
              <SidebarNavItem key={item.key} active={activePage === item.key} onClick={() => handleChangePage(item.key)}>
                <SidebarIcon>{item.icon}</SidebarIcon>
                {item.label}
              </SidebarNavItem>
            ))}
          </SidebarNav>
        </Sidebar>

        <MainArea>
          <Topbar>
            <TopbarLeft>
              <SearchBox>
                <span role="img" aria-label="search">
                  🔍
                </span>
                <SearchInput placeholder="Search hackathons..." />
              </SearchBox>
            </TopbarLeft>
            <TopbarRight>
              <UserMenu
                nickname={currentUser.nickname}
                onProfileClick={() => handleChangePage("profile")}
              />
            </TopbarRight>
          </Topbar>

          <Content>
            {activePage === "dashboard" && (
              <section>
                <SectionGrid>
                  <ProfileCard>
                    <BadgePill>Профиль участника</BadgePill>
                    <ProfileName>Алексей Волков</ProfileName>
                    <Muted>@volkov.dev · Москва · Продуктовый аналитик</Muted>
                    <StatsRow>
                      <div>
                        <StatValue>{userStats.totalHackathons}</StatValue>
                        <StatLabel>Участий</StatLabel>
                      </div>
                      <div>
                        <StatValue>{userStats.wins}</StatValue>
                        <StatLabel>Побед</StatLabel>
                      </div>
                      <div>
                        <StatValue>{userStats.podiums}</StatValue>
                        <StatLabel>Призовых</StatLabel>
                      </div>
                    </StatsRow>
                    <div>
                      <CardHeader>
                        <Muted>До следующего достижения</Muted>
                        <Muted>70%</Muted>
                      </CardHeader>
                      <ProgressBar>
                        <ProgressFill percent={70} />
                      </ProgressBar>
                      <Muted style={{ marginTop: 6 }}>
                        Продолжайте участвовать в хакатонах, чтобы повысить активность и открыть новые уровни.
                      </Muted>
                    </div>
                  </ProfileCard>

                  <Card>
                    <CardHeader>
                      <div>
                        <Muted>Сезонный рейтинг</Muted>
                        <CardTitle>#7283 Bronze Tier</CardTitle>
                      </div>
                      <Pill>В процессе</Pill>
                    </CardHeader>
                    <StatsRow>
                      <StatTile>
                        <StatLabel>Участий</StatLabel>
                        <StatValue>{userStats.totalHackathons}</StatValue>
                      </StatTile>
                      <StatTile>
                        <StatLabel>Победы</StatLabel>
                        <StatValue>{userStats.wins}</StatValue>
                      </StatTile>
                      <StatTile>
                        <StatLabel>Призовые</StatLabel>
                        <StatValue>{userStats.podiums}</StatValue>
                      </StatTile>
                    </StatsRow>
                    <div style={{ marginTop: 12 }}>
                      <Muted>До следующего достижения</Muted>
                      <ProgressBar style={{ marginTop: 8 }}>
                        <ProgressFill percent={45} />
                      </ProgressBar>
                      <Muted style={{ marginTop: 6 }}>Сделайте 2 участия, чтобы перейти в Silver Tier.</Muted>
                    </div>
                  </Card>
                </SectionGrid>

                <SectionTitleRow style={{ marginTop: 10, marginBottom: 6 }}>
                  <div>
                    <Muted>Подборка</Muted>
                    <CardTitle>Хакатоны</CardTitle>
                  </div>
                  <Tabs>
                    <TabButton active={hackathonTab === "recommended"} onClick={() => setHackathonTab("recommended")}>
                      For you
                    </TabButton>
                    <TabButton active={hackathonTab === "inprogress"} onClick={() => setHackathonTab("inprogress")}>
                      In progress
                    </TabButton>
                    <TabButton active={hackathonTab === "favorites"} onClick={() => setHackathonTab("favorites")}>
                      Favorites
                    </TabButton>
                  </Tabs>
                </SectionTitleRow>

                <CardsGrid>
                  {filteredHackathons.map((h: Hackathon) => (
                    <HackathonCard key={h.id}>
                      <HackathonHeader>
                        <div style={{ display: "flex", flexDirection: "column", gap: 4 }}>
                          <div style={{ fontWeight: 700 }}>{h.title}</div>
                          <Muted>{h.dateRange}</Muted>
                        </div>
                        <Pill>{h.mode}</Pill>
                      </HackathonHeader>
                      <div style={{ display: "flex", gap: 6, flexWrap: "wrap" }}>
                        {h.tags.map((tag) => (
                          <Pill key={tag}>{tag}</Pill>
                        ))}
                      </div>
                      <div style={{ display: "flex", justifyContent: "space-between", alignItems: "center" }}>
                        <Muted>{h.status === "active" ? "В прогрессе" : "Скоро"}</Muted>
                        <CTAButton>{h.status === "active" ? "Продолжить" : "Join"}</CTAButton>
                      </div>
                    </HackathonCard>
                  ))}
                </CardsGrid>
              </section>
            )}

            {activePage === "profile" && <ProfilePage />}

            {activePage === "teams" && (
              <section>
                <SectionTitleRow>
                  <div>
                    <Muted>Мои команды</Muted>
                    <CardTitle>Командные профили</CardTitle>
                  </div>
                  <AccentButton>Создать команду</AccentButton>
                </SectionTitleRow>
                <TeamGrid>
                  {teams.map((team) => (
                    <TeamCard key={team.id}>
                      <CardHeader>
                        <CardTitle>{team.name}</CardTitle>
                        <LevelBadge>{team.level}</LevelBadge>
                      </CardHeader>
                      <Muted>{team.motto}</Muted>
                      <StatsRow>
                        <div>
                          <Muted>Участников</Muted>
                          <StatValue>{team.membersCount}</StatValue>
                        </div>
                        <div>
                          <Muted>Хакатонов</Muted>
                          <StatValue>{team.hackathonsCount}</StatValue>
                        </div>
                        <div>
                          <Muted>Страна</Muted>
                          <StatValue>{team.country}</StatValue>
                        </div>
                      </StatsRow>
                    </TeamCard>
                  ))}
                </TeamGrid>

                <FormCard>
                  <CardHeader>
                    <div>
                      <BadgePill>Create Team</BadgePill>
                      <CardTitle>Новая команда</CardTitle>
                      <Muted>Заполните профиль, чтобы быть заметнее на предстоящих хакатонах</Muted>
                    </div>
                    <div style={{ background: theme.colors.cardSoft, padding: 12, borderRadius: theme.radii.card, border: `1px solid ${theme.colors.borderSoft}` }}>
                      Team Cover
                    </div>
                  </CardHeader>
                  <form onSubmit={handleCreateTeam}>
                    <FormGrid>
                      <Label>
                        <span>Team name *</span>
                        <input
                          value={formData.name}
                          onChange={(e) => setFormData({ ...formData, name: e.target.value })}
                          placeholder="Например, Aurora Stack"
                          required
                        />
                      </Label>
                      <Label>
                        <span>Country</span>
                        <select value={formData.country} onChange={(e) => setFormData({ ...formData, country: e.target.value })}>
                          <option>Россия</option>
                          <option>Казахстан</option>
                          <option>Беларусь</option>
                          <option>Украина</option>
                          <option>Грузия</option>
                        </select>
                      </Label>
                    </FormGrid>
                    <Label>
                      <span>Team motto</span>
                      <input
                        value={formData.motto}
                        onChange={(e) => setFormData({ ...formData, motto: e.target.value })}
                        placeholder="Слоган, который вдохновит команду"
                      />
                    </Label>
                    <Label>
                      <span>Team description</span>
                      <textarea
                        value={formData.description}
                        onChange={(e) => setFormData({ ...formData, description: e.target.value })}
                        placeholder="Пару предложений о стэке, сильных сторонах и опыте"
                        rows={3}
                      />
                    </Label>

                    <FormGrid>
                      <div>
                        <div
                          style={{
                            background: theme.colors.cardSoft,
                            border: `1px dashed ${theme.colors.borderSoft}`,
                            borderRadius: theme.radii.card,
                            padding: "24px",
                            textAlign: "center",
                          }}
                        >
                          Team Avatar
                          <Muted>Загрузить jpg/png</Muted>
                        </div>
                      </div>
                      <div>
                        <div
                          style={{
                            background: theme.colors.cardSoft,
                            border: `1px dashed ${theme.colors.borderSoft}`,
                            borderRadius: theme.radii.card,
                            padding: "24px",
                            textAlign: "center",
                          }}
                        >
                          Team Cover
                          <Muted>1920x150 — 2880x225</Muted>
                        </div>
                      </div>
                    </FormGrid>

                    <SwitchRow>
                      <div>
                        <StrongText>Make team profile public</StrongText>
                        <Muted>Команда появится в поиске и сможет получать приглашения</Muted>
                      </div>
                      <Switch on={isPublic} onClick={() => setIsPublic((p) => !p)}>
                        <span />
                      </Switch>
                    </SwitchRow>

                    <FormGrid>
                      <Label>
                        <span>Website</span>
                        <input
                          value={formData.website}
                          onChange={(e) => setFormData({ ...formData, website: e.target.value })}
                          placeholder="https://"
                        />
                      </Label>
                      <Label>
                        <span>Telegram</span>
                        <input
                          value={formData.telegram}
                          onChange={(e) => setFormData({ ...formData, telegram: e.target.value })}
                          placeholder="@team"
                        />
                      </Label>
                    </FormGrid>
                    <FormGrid>
                      <Label>
                        <span>GitHub</span>
                        <input
                          value={formData.github}
                          onChange={(e) => setFormData({ ...formData, github: e.target.value })}
                          placeholder="github.com/"
                        />
                      </Label>
                      <Label>
                        <span>VK</span>
                        <input
                          value={formData.vk}
                          onChange={(e) => setFormData({ ...formData, vk: e.target.value })}
                          placeholder="vk.com/"
                        />
                      </Label>
                    </FormGrid>

                    <FormActions>
                      <AccentButton type="submit">Создать</AccentButton>
                      <GhostButton type="button" onClick={() => setFormData({ ...formData, name: "" })}>
                        Сбросить
                      </GhostButton>
                    </FormActions>
                  </form>
                </FormCard>
              </section>
            )}

            {activePage === "hackathon-config" && (
              <HackathonConfigPage organizerId={1} onBackToDashboard={() => handleChangePage("dashboard")} />
            )}
          </Content>
        </MainArea>
      </AppShell>
    </ThemeProvider>
  );
};

export default App;
