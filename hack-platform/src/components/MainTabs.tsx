import React from "react";
import styled from "@emotion/styled";

export type MainTab = "home" | "profile" | "teams";

interface MainTabsProps {
  activeTab: MainTab;
  onChange: (tab: MainTab) => void;
}

const TabsWrapper = styled.div`
  display: flex;
  justify-content: center;
  margin: 10px 0 6px;
`;

const TabPill = styled.div`
  display: inline-flex;
  padding: 6px;
  border-radius: ${({ theme }) => theme.radii.pill}px;
  background: ${({ theme }) => theme.colors.cardSoft};
  border: 1px solid ${({ theme }) => theme.colors.borderSoft};
  box-shadow: 0 10px 24px rgba(2, 6, 23, 0.6);
`;

const TabButton = styled.button<{ active?: boolean }>`
  border: none;
  background: ${({ active, theme }) => (active ? theme.colors.accent : theme.colors.card)};
  color: ${({ active, theme }) => (active ? "#031004" : theme.colors.text)};
  padding: 10px 20px;
  border-radius: ${({ theme }) => theme.radii.pill}px;
  font-weight: 800;
  cursor: pointer;
  transition: color 0.16s ease, background-color 0.16s ease, box-shadow 0.16s ease;
  box-shadow: ${({ active }) => (active ? "0 12px 28px rgba(163, 255, 18, 0.35)" : "none")};

  &:hover {
    color: ${({ active, theme }) => (active ? "#031004" : theme.colors.text)};
    background: ${({ active, theme }) => (active ? theme.colors.accent : theme.colors.cardSoft)};
  }
`;

export const MainTabs: React.FC<MainTabsProps> = ({ activeTab, onChange }) => (
  <TabsWrapper>
    <TabPill>
      <TabButton active={activeTab === "home"} onClick={() => onChange("home")}>
        Home
      </TabButton>
      <TabButton active={activeTab === "profile"} onClick={() => onChange("profile")}>
        Profile
      </TabButton>
      <TabButton active={activeTab === "teams"} onClick={() => onChange("teams")}>
        Teams
      </TabButton>
    </TabPill>
  </TabsWrapper>
);

export default MainTabs;
