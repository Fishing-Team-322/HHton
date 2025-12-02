import React, { useMemo, useState } from "react";
import styled from "@emotion/styled";
import type { ProfileTab } from "../types";
import {
  activityFeed,
  badges,
  overviewEntries,
  profileDetails,
  profileSeason,
  profileStats,
} from "../mockData";

const PageWrapper = styled.div`
  display: flex;
  flex-direction: column;
  gap: 18px;
`;

const ProfileHeader = styled.div`
  display: flex;
  justify-content: space-between;
  align-items: flex-end;
  gap: 12px;
`;

const HeaderTitles = styled.div`
  display: flex;
  flex-direction: column;
  gap: 6px;
`;

const ProfileTitle = styled.h1`
  margin: 0;
  font-size: 28px;
`;

const HeaderSubtitle = styled.div`
  color: ${({ theme }) => theme.colors.textMuted};
`;

const Tabs = styled.div`
  display: inline-flex;
  gap: 22px;
  border-bottom: 1px solid ${({ theme }) => theme.colors.borderSoft};
  padding: 0 4px;
`;

const TabButton = styled.button<{ active?: boolean }>`
  position: relative;
  border: none;
  background: transparent;
  color: ${({ active, theme }) => (active ? theme.colors.text : theme.colors.textMuted)};
  font-weight: 700;
  padding: 10px 6px;
  cursor: pointer;
  transition: color 0.16s ease;

  &:after {
    content: "";
    position: absolute;
    left: 0;
    right: 0;
    bottom: -1px;
    height: 3px;
    background: ${({ theme }) => theme.colors.accent};
    opacity: ${({ active }) => (active ? 1 : 0)};
    transition: opacity 0.16s ease;
    border-radius: ${({ theme }) => theme.radii.pill}px;
  }

  &:hover {
    color: ${({ theme }) => theme.colors.text};
  }
`;

const Layout = styled.div`
  display: grid;
  grid-template-columns: 1.7fr 1fr;
  gap: 14px;
`;

const Card = styled.div`
  background: ${({ theme }) => theme.colors.card};
  border-radius: ${({ theme }) => theme.radii.card}px;
  border: 1px solid ${({ theme }) => theme.colors.borderSoft};
  padding: 18px;
  box-shadow: 0 18px 45px rgba(2, 6, 23, 0.65);
`;

const CardHeader = styled.div`
  display: flex;
  align-items: center;
  justify-content: space-between;
  gap: 8px;
  margin-bottom: 12px;
`;

const CardTitle = styled.div`
  font-weight: 800;
  font-size: 18px;
`;

const Muted = styled.div`
  color: ${({ theme }) => theme.colors.textMuted};
  font-size: 13px;
`;

const ProgressBar = styled.div`
  background: ${({ theme }) => theme.colors.cardSoft};
  border-radius: ${({ theme }) => theme.radii.pill}px;
  height: 10px;
  overflow: hidden;
  border: 1px solid ${({ theme }) => theme.colors.borderSoft};
`;

const ProgressFill = styled.div<{ percent: number }>`
  height: 100%;
  width: ${({ percent }) => `${percent}%`};
  background: linear-gradient(90deg, #73ff52, #a3ff12);
`;

const ProfileCard = styled(Card)`
  display: flex;
  flex-direction: column;
  gap: 12px;
  background: radial-gradient(circle at 12% 18%, rgba(163, 255, 18, 0.08), transparent 34%),
    ${({ theme }) => theme.colors.card};
`;

const ProfileRow = styled.div`
  display: flex;
  align-items: center;
  justify-content: space-between;
`;

const NameBlock = styled.div`
  display: flex;
  flex-direction: column;
  gap: 4px;
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

const StatsGrid = styled.div`
  display: grid;
  grid-template-columns: repeat(auto-fit, minmax(150px, 1fr));
  gap: 10px;
`;

const StatTile = styled(Card)`
  padding: 12px 14px;
`;

const StatValue = styled.div`
  font-size: 22px;
  font-weight: 800;
`;

const StatLabel = styled.div`
  color: ${({ theme }) => theme.colors.textMuted};
  margin-top: 4px;
`;

const HackathonsGrid = styled.div`
  display: grid;
  grid-template-columns: repeat(auto-fit, minmax(240px, 1fr));
  gap: 12px;
`;

const HackathonCard = styled(Card)`
  display: flex;
  flex-direction: column;
  gap: 8px;
  box-shadow: 0 12px 30px rgba(15, 23, 42, 0.9);
`;

const HackathonHeader = styled.div`
  display: flex;
  justify-content: space-between;
  align-items: center;
  gap: 8px;
`;

const StatusBadge = styled.span<{ tone?: "success" | "muted" }>`
  background: ${({ tone, theme }) => (tone === "success" ? theme.colors.accentSoft : theme.colors.cardSoft)};
  color: ${({ tone, theme }) => (tone === "success" ? theme.colors.accent : theme.colors.textMuted)};
  border-radius: ${({ theme }) => theme.radii.pill}px;
  padding: 6px 10px;
  font-weight: 700;
  font-size: 12px;
  border: 1px solid ${({ tone, theme }) => (tone === "success" ? theme.colors.accentSoft : theme.colors.borderSoft)};
`;

const RightColumn = styled.div`
  display: grid;
  gap: 12px;
`;

const AboutList = styled.div`
  display: grid;
  gap: 10px;
`;

const AboutRow = styled.div`
  display: flex;
  justify-content: space-between;
  align-items: center;
`;

const BadgeGrid = styled.div`
  display: grid;
  grid-template-columns: repeat(auto-fit, minmax(120px, 1fr));
  gap: 10px;
`;

const BadgeCard = styled(Card)<{ locked?: boolean }>`
  text-align: center;
  opacity: ${({ locked }) => (locked ? 0.5 : 1)};
  border-style: solid;
`;

const Timeline = styled.div`
  display: grid;
  gap: 12px;
`;

const TimelineItem = styled(Card)`
  display: grid;
  grid-template-columns: auto 1fr;
  gap: 12px;
  align-items: center;
`;

const Avatar = styled.div`
  width: 42px;
  height: 42px;
  border-radius: 50%;
  background: linear-gradient(135deg, #111827, #0f172a);
  border: 1px solid ${({ theme }) => theme.colors.borderSoft};
  display: grid;
  place-items: center;
  font-weight: 800;
`;

const PlaceholderCard = styled(Card)`
  text-align: center;
  color: ${({ theme }) => theme.colors.textMuted};
`;

const ProfilePage: React.FC = () => {
  const [profileTab, setProfileTab] = useState<ProfileTab>("profile");

  const statTiles = useMemo(
    () => [
      { label: "Участий в хакатонах", value: profileStats.totalHackathons },
      { label: "Побед", value: profileStats.wins },
      { label: "Призовых мест", value: profileStats.podiums },
      { label: "Среднее место", value: profileStats.averagePlace },
    ],
    [],
  );

  return (
    <PageWrapper>
      <ProfileHeader>
        <HeaderTitles>
          <ProfileTitle>{profileDetails.nickname}</ProfileTitle>
          <HeaderSubtitle>
            {profileDetails.city} · {profileDetails.role}
          </HeaderSubtitle>
        </HeaderTitles>
        <Tabs>
          {(
            [
              { key: "profile", label: "Profile" },
              { key: "activity", label: "Activity" },
              { key: "badges", label: "Badges" },
              { key: "published", label: "Published" },
            ] as const
          ).map((tab) => (
            <TabButton key={tab.key} active={profileTab === tab.key} onClick={() => setProfileTab(tab.key)}>
              {tab.label}
            </TabButton>
          ))}
        </Tabs>
      </ProfileHeader>

      {profileTab === "profile" && (
        <Layout>
          <div style={{ display: "grid", gap: 12 }}>
            <ProfileCard>
              <ProfileRow>
                <NameBlock>
                  <BadgePill>Profile</BadgePill>
                  <ProfileTitle style={{ fontSize: 26, marginTop: 6 }}>{profileDetails.name}</ProfileTitle>
                  <Muted>
                    {profileDetails.nickname} · {profileDetails.city} · {profileDetails.role}
                  </Muted>
                </NameBlock>
                <div style={{ minWidth: 220 }}>
                  <CardHeader>
                    <Muted>До следующего уровня активности</Muted>
                    <Muted>{profileDetails.activityProgress}%</Muted>
                  </CardHeader>
                  <ProgressBar>
                    <ProgressFill percent={profileDetails.activityProgress} />
                  </ProgressBar>
                  <Muted style={{ marginTop: 6 }}>{profileDetails.nextLevelHint}</Muted>
                </div>
              </ProfileRow>

              <StatsGrid>
                {statTiles.map((stat) => (
                  <StatTile key={stat.label}>
                    <StatLabel>{stat.label}</StatLabel>
                    <StatValue>{stat.value}</StatValue>
                  </StatTile>
                ))}
              </StatsGrid>
            </ProfileCard>

            <div>
              <CardHeader>
                <div>
                  <Muted>История участия</Muted>
                  <CardTitle>Хакатоны</CardTitle>
                </div>
              </CardHeader>
              <HackathonsGrid>
                {overviewEntries.map((hackathon) => (
                  <HackathonCard key={hackathon.title}>
                    <HackathonHeader>
                      <div>
                        <CardTitle style={{ fontSize: 16 }}>{hackathon.title}</CardTitle>
                        <Muted>
                          {hackathon.role} · {hackathon.date}
                        </Muted>
                      </div>
                      <StatusBadge tone={hackathon.result.includes("место") ? "success" : "muted"}>
                        {hackathon.result}
                      </StatusBadge>
                    </HackathonHeader>
                  </HackathonCard>
                ))}
              </HackathonsGrid>
            </div>
          </div>

          <RightColumn>
            <Card>
              <CardHeader>
                <div>
                  <Muted>Текущий сезон</Muted>
                  <CardTitle>{profileSeason.tier}</CardTitle>
                </div>
                <StatusBadge tone="muted">Season 9</StatusBadge>
              </CardHeader>
              <div>
                <ProgressBar>
                  <ProgressFill percent={profileSeason.progress} />
                </ProgressBar>
                <Muted style={{ marginTop: 6 }}>{profileSeason.note}</Muted>
              </div>
            </Card>

            <Card>
              <CardHeader>
                <div>
                  <Muted>About</Muted>
                  <CardTitle>Обо мне</CardTitle>
                </div>
              </CardHeader>
              <AboutList>
                <AboutRow>
                  <Muted>Город</Muted>
                  <div>{profileDetails.city}</div>
                </AboutRow>
                <AboutRow>
                  <Muted>Стек / роль</Muted>
                  <div>{profileDetails.role}</div>
                </AboutRow>
                <AboutRow>
                  <Muted>Дата регистрации</Muted>
                  <div>{profileDetails.joined}</div>
                </AboutRow>
                <AboutRow>
                  <Muted>Bio</Muted>
                  <div>{profileDetails.bio}</div>
                </AboutRow>
              </AboutList>
            </Card>

            <Card>
              <CardHeader>
                <div>
                  <Muted>Badges</Muted>
                  <CardTitle>Достижения</CardTitle>
                </div>
                <StatusBadge tone="muted">View all</StatusBadge>
              </CardHeader>
              <BadgeGrid>
                {badges.slice(0, 6).map((badge) => (
                  <BadgeCard key={badge.title} locked={!badge.unlocked}>
                    <div style={{ fontSize: 28 }}>🏅</div>
                    <div style={{ fontWeight: 700 }}>{badge.title}</div>
                    <Muted>{badge.unlocked ? "Получен" : "Заблокирован"}</Muted>
                  </BadgeCard>
                ))}
              </BadgeGrid>
            </Card>
          </RightColumn>
        </Layout>
      )}

      {profileTab === "activity" && (
        <Timeline>
          {activityFeed.map((entry) => (
            <TimelineItem key={entry.title}>
              <Avatar>AV</Avatar>
              <div>
                <CardTitle style={{ fontSize: 16 }}>{entry.title}</CardTitle>
                <Muted>
                  {entry.date} · {entry.ago}
                </Muted>
              </div>
            </TimelineItem>
          ))}
        </Timeline>
      )}

      {profileTab === "badges" && (
        <BadgeGrid>
          {badges.map((badge) => (
            <BadgeCard key={badge.title} locked={!badge.unlocked}>
              <div style={{ fontSize: 30 }}>🎖️</div>
              <div style={{ fontWeight: 800 }}>{badge.title}</div>
              <Muted>{badge.unlocked ? "Активен" : "Не открыт"}</Muted>
            </BadgeCard>
          ))}
        </BadgeGrid>
      )}

      {profileTab === "published" && (
        <PlaceholderCard>
          <CardTitle>Нет опубликованного контента</CardTitle>
          <Muted>У вас пока нет публикаций. Делитесь кейсами и статьями после хакатонов.</Muted>
        </PlaceholderCard>
      )}
    </PageWrapper>
  );
};

export default ProfilePage;
