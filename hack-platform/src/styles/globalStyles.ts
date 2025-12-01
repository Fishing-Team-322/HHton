export const globalStyles = `
:root {
  --bg: #05060a;
  --bg-strong: #060818;
  --panel: #0b0f19;
  --panel-strong: #111827;
  --border: #1c2333;
  --muted: #9ca3af;
  --text: #e5e7eb;
  --accent: #9fef00;
  --accent-2: #00b3ff;
  --shadow: 0 8px 24px rgba(0, 0, 0, 0.25);
}

* {
  box-sizing: border-box;
}

.app {
  display: flex;
  min-height: 100vh;
  background: var(--bg);
  color: var(--text);
}

.sidebar {
  width: 230px;
  background: #0a0d18;
  border-right: 1px solid var(--border);
  padding: 22px 16px;
  position: sticky;
  top: 0;
  align-self: flex-start;
  height: 100vh;
  display: flex;
  flex-direction: column;
  gap: 8px;
}

.logo {
  display: flex;
  align-items: center;
  gap: 12px;
  padding: 10px 10px 18px;
}

.logo-mark {
  width: 36px;
  height: 36px;
  border-radius: 10px;
  background: var(--accent);
  box-shadow: 0 0 0 1px #121829;
}

.logo-text {
  font-weight: 800;
  letter-spacing: 1px;
}

.sidebar-item {
  background: transparent;
  border: 1px solid transparent;
  color: var(--text);
  padding: 12px 12px 12px 10px;
  display: flex;
  align-items: center;
  gap: 10px;
  border-radius: 10px;
  cursor: pointer;
  position: relative;
  transition: all 0.12s ease-out;
  text-align: left;
}

.sidebar-item:hover {
  background: #0f1322;
  border-color: var(--border);
}

.sidebar-item.active {
  background: #0f1628;
  border-color: var(--accent);
  color: var(--accent);
  box-shadow: inset 3px 0 0 var(--accent);
  font-weight: 700;
}

.sidebar-item.active .sidebar-dot {
  background: var(--accent);
}

.sidebar-dot {
  width: 8px;
  height: 8px;
  border-radius: 50%;
  background: #2b3346;
}

.content {
  flex: 1;
  padding: 30px 32px;
  background: var(--bg-strong);
}

.topbar {
  display: flex;
  justify-content: space-between;
  align-items: center;
  margin-bottom: 26px;
  background: var(--panel-strong);
  border: 1px solid var(--border);
  padding: 14px 16px;
  border-radius: 10px;
  box-shadow: var(--shadow);
}

.page-title {
  margin: 6px 0 0;
  font-size: 24px;
}

.accent-text {
  color: var(--accent-2);
}

.breadcrumbs {
  color: var(--muted);
  font-size: 12px;
  letter-spacing: 0.5px;
}

.top-actions {
  display: flex;
  gap: 10px;
}

.page {
  display: flex;
  flex-direction: column;
  gap: 22px;
}

.grid.two {
  display: grid;
  grid-template-columns: 2fr 1.15fr;
  gap: 16px;
}

.hero {
  background: var(--panel-strong);
  border-radius: 10px;
  padding: 20px;
  position: relative;
  min-height: 240px;
  display: grid;
  grid-template-columns: 1.5fr 1fr;
  border: 1px solid var(--border);
  box-shadow: var(--shadow);
  overflow: hidden;
}

.hero-content {
  display: flex;
  flex-direction: column;
  gap: 10px;
  position: relative;
  z-index: 1;
}

.hero-label {
  text-transform: uppercase;
  letter-spacing: 1px;
  font-size: 12px;
  color: var(--muted);
}

.hero-title {
  margin: 0;
  font-size: 28px;
  font-weight: 800;
}

.hero-subtitle {
  margin: 0;
  max-width: 520px;
  line-height: 1.5;
  color: #cfd3dc;
}

.hero-tags {
  display: flex;
  flex-wrap: wrap;
  gap: 8px;
  margin: 6px 0 4px;
}

.hero-visual {
  border-radius: 10px;
  border: 1px solid var(--border);
  background: #0c1322;
  box-shadow: inset 0 0 0 1px #0f1628;
}

.profile-panel {
  background: var(--panel);
  border-radius: 10px;
  padding: 16px;
  box-shadow: var(--shadow);
  display: flex;
  flex-direction: column;
  gap: 14px;
  border: 1px solid var(--border);
}

.profile-compact {
  display: flex;
  gap: 12px;
  align-items: center;
}

.avatar {
  width: 58px;
  height: 58px;
  border-radius: 50%;
  background: #0f1628;
  border: 2px solid var(--accent);
  box-shadow: 0 8px 18px rgba(0, 0, 0, 0.2);
}

.name {
  font-weight: 700;
}

.meta {
  color: var(--muted);
  font-size: 13px;
}

.mini-note {
  color: var(--muted);
  font-size: 12px;
  margin-top: 6px;
}

.metrics {
  display: grid;
  grid-template-columns: repeat(3, 1fr);
  gap: 10px;
}

.progress {
  background: #0d1220;
  border: 1px solid var(--border);
  border-radius: 10px;
  padding: 12px;
}

.progress-header {
  display: flex;
  justify-content: space-between;
  font-size: 13px;
  color: var(--text);
}

.progress-bar {
  width: 100%;
  height: 10px;
  background: #0a0f1c;
  border: 1px solid var(--border);
  border-radius: 999px;
  margin: 10px 0;
  overflow: hidden;
}

.progress-bar span {
  display: block;
  height: 100%;
  background: var(--accent);
  border-radius: 999px;
}

.progress-note {
  color: var(--muted);
  font-size: 12px;
}

.tabs {
  display: flex;
  gap: 8px;
  margin-top: 6px;
  flex-wrap: wrap;
}

.tab {
  background: #0e1423;
  border: 1px solid var(--border);
  color: var(--text);
  padding: 10px 16px;
  border-radius: 10px;
  cursor: pointer;
  transition: all 0.12s ease-out;
}

.tab:hover {
  background: #121a2c;
}

.tab.active {
  border-color: var(--accent);
  color: var(--accent);
  box-shadow: inset 0 -2px 0 var(--accent);
}

.cards-grid {
  display: grid;
  grid-template-columns: repeat(auto-fit, minmax(250px, 1fr));
  gap: 12px;
}

.hackathon-card {
  background: var(--panel);
  border-radius: 10px;
  padding: 14px;
  display: flex;
  flex-direction: column;
  gap: 8px;
  box-shadow: var(--shadow);
  border: 1px solid var(--border);
  transition: all 0.15s ease-out;
}

.hackathon-card:hover {
  transform: translateY(-3px);
  border-color: var(--accent);
}

.hackathon-card__top {
  display: flex;
  align-items: center;
  gap: 10px;
}

.hackathon-logo {
  width: 44px;
  height: 44px;
  border-radius: 10px;
  background: #0f1628;
  border: 1px solid var(--border);
  box-shadow: inset 0 0 0 1px #0a0f1c;
}

.hackathon-title {
  font-weight: 700;
}

.hackathon-level {
  color: var(--muted);
  font-size: 12px;
  letter-spacing: 1px;
}

.hackathon-date {
  color: var(--muted);
  font-size: 13px;
}

.hackathon-tags {
  display: flex;
  gap: 6px;
  flex-wrap: wrap;
}

.tag {
  background: #0f1628;
  padding: 6px 10px;
  border-radius: 999px;
  color: var(--text);
  font-size: 12px;
  border: 1px solid var(--border);
}

.tag.ghost {
  background: transparent;
  border-color: #1f2a3d;
  color: var(--muted);
}

.mode-badge {
  margin-left: auto;
  text-transform: capitalize;
  padding: 6px 10px;
  border-radius: 8px;
  font-size: 12px;
  border: 1px solid var(--border);
}

.mode-badge.online {
  color: var(--accent-2);
  background: #0b1423;
}

.mode-badge.offline {
  color: #b1b8c7;
  background: #0c101d;
}

.mode-badge.hybrid {
  color: var(--accent);
  background: #0c131f;
  border-color: #243040;
}

.hackathon-footer {
  display: flex;
  justify-content: space-between;
  align-items: center;
}

.status-badge {
  padding: 6px 10px;
  border-radius: 8px;
  font-size: 12px;
  background: #0e1423;
  color: var(--text);
  border: 1px solid var(--border);
}

.status-badge.active {
  color: var(--accent);
  background: #0d161f;
}

.status-badge.upcoming {
  color: var(--accent-2);
}

.status-badge.finished {
  color: #7a8296;
}

.primary-btn {
  background: var(--accent);
  color: #05060a;
  border: 1px solid #6bb300;
  padding: 12px 18px;
  border-radius: 8px;
  font-weight: 700;
  cursor: pointer;
  transition: all 0.12s ease-out;
  box-shadow: 0 8px 18px rgba(0, 0, 0, 0.22);
}

.primary-btn:hover {
  transform: translateY(-1px) scale(1.01);
  background: #8ed800;
}

.primary-btn.dark {
  background: #101a0c;
  color: var(--accent);
  border-color: var(--accent);
}

.primary-btn.alt {
  padding: 10px 14px;
  box-shadow: none;
}

.ghost-btn {
  background: transparent;
  color: var(--text);
  border: 1px solid #2a3348;
  padding: 10px 14px;
  border-radius: 8px;
  cursor: pointer;
  transition: all 0.12s ease-out;
}

.ghost-btn:hover {
  border-color: var(--accent);
  color: var(--accent);
}

.stat-card {
  background: #0f1422;
  border-radius: 10px;
  padding: 14px;
  border: 1px solid var(--border);
  box-shadow: var(--shadow);
}

.stat-card.accent {
  border-color: var(--accent);
  box-shadow: 0 8px 20px rgba(159, 239, 0, 0.15);
}

.stat-title {
  color: var(--muted);
  font-size: 12px;
}

.stat-value {
  font-size: 22px;
  font-weight: 800;
  margin-top: 4px;
}

.stats-row {
  display: grid;
  grid-template-columns: repeat(auto-fit, minmax(180px, 1fr));
  gap: 12px;
}

.profile-header {
  background: var(--panel);
  border-radius: 10px;
  padding: 18px;
  display: flex;
  gap: 16px;
  align-items: center;
  box-shadow: var(--shadow);
  border: 1px solid var(--border);
}

.profile-avatar {
  width: 80px;
  height: 80px;
  border-radius: 20px;
  background: linear-gradient(135deg, #0f1628, #131a2c);
  border: 2px solid var(--border);
  box-shadow: 0 14px 22px rgba(0, 0, 0, 0.28);
}

.profile-info {
  display: flex;
  flex-direction: column;
  gap: 6px;
}

.profile-name {
  font-size: 22px;
  font-weight: 800;
}

.profile-name .nickname {
  color: var(--muted);
  font-size: 14px;
  font-weight: 500;
}

.profile-meta {
  color: var(--muted);
}

.profile-bio {
  margin: 4px 0 0;
  color: #cfd3dc;
  line-height: 1.5;
}

.badge-grid {
  display: grid;
  grid-template-columns: repeat(auto-fit, minmax(160px, 1fr));
  gap: 12px;
}

.badge-card {
  background: #0f1422;
  border: 1px solid var(--border);
  border-radius: 12px;
  padding: 14px;
  display: flex;
  flex-direction: column;
  gap: 8px;
  box-shadow: var(--shadow);
}

.badge-card.locked {
  opacity: 0.6;
  border-style: dashed;
}

.badge-icon {
  width: 48px;
  height: 48px;
  border-radius: 12px;
  background: linear-gradient(135deg, #0b1324, #0e1a2c);
  border: 1px solid var(--border);
}

.badge-title {
  font-weight: 700;
}

.badge-status {
  color: var(--muted);
  font-size: 12px;
}

.timeline {
  display: flex;
  flex-direction: column;
  gap: 12px;
}

.timeline-item {
  display: flex;
  gap: 12px;
  align-items: center;
  background: #0f1422;
  border: 1px solid var(--border);
  border-radius: 10px;
  padding: 12px;
  box-shadow: var(--shadow);
}

.timeline-dot {
  width: 12px;
  height: 12px;
  border-radius: 50%;
  background: var(--accent);
}

.timeline-title {
  font-weight: 700;
}

.timeline-date {
  color: var(--muted);
  font-size: 12px;
}

.list {
  display: flex;
  flex-direction: column;
  gap: 10px;
}

.list-row {
  display: flex;
  justify-content: space-between;
  align-items: center;
  background: #0f1422;
  border: 1px solid var(--border);
  border-radius: 10px;
  padding: 12px;
  box-shadow: var(--shadow);
}

.list-title {
  font-weight: 700;
}

.list-sub {
  color: var(--muted);
  font-size: 12px;
}

.status-pill {
  background: #101a2c;
  color: var(--accent);
  padding: 6px 12px;
  border-radius: 999px;
  border: 1px solid var(--border);
}

.teams-header {
  display: flex;
  justify-content: space-between;
  align-items: center;
}

.cards-grid.teams {
  grid-template-columns: repeat(auto-fit, minmax(260px, 1fr));
}

.team-card {
  background: var(--panel);
  border-radius: 10px;
  padding: 14px;
  border: 1px solid var(--border);
  box-shadow: var(--shadow);
  transition: all 0.12s ease-out;
}

.team-card:hover {
  transform: translateY(-2px);
  border-color: var(--accent);
}

.team-card__header {
  display: flex;
  align-items: center;
  gap: 10px;
}

.team-logo {
  width: 42px;
  height: 42px;
  border-radius: 10px;
  background: #0f1628;
  border: 1px solid var(--border);
  box-shadow: inset 0 0 0 1px #0c1220;
}

.team-name {
  font-weight: 700;
}

.team-meta {
  color: var(--muted);
  font-size: 13px;
}

.team-slogan {
  margin: 10px 0;
  color: #c7ccda;
}

.team-footer {
  display: flex;
  justify-content: space-between;
  color: var(--muted);
  font-size: 13px;
}

.team-level {
  margin-left: auto;
  padding: 6px 10px;
  border-radius: 8px;
  font-size: 12px;
  text-transform: capitalize;
  background: #0e1423;
  border: 1px solid var(--border);
}

.team-level.pro {
  color: var(--accent);
}

.team-level.mixed {
  color: var(--accent-2);
}

.team-level.beginner {
  color: #aeb6c7;
}

.form-card {
  margin-top: 18px;
  background: var(--panel);
  border-radius: 10px;
  padding: 18px;
  border: 1px solid var(--border);
  box-shadow: var(--shadow);
}

.form-header {
  display: flex;
  justify-content: space-between;
  gap: 12px;
  align-items: flex-start;
}

.form-badge {
  display: inline-flex;
  padding: 6px 10px;
  border-radius: 999px;
  background: #0e1626;
  color: var(--accent);
  font-size: 12px;
  letter-spacing: 0.5px;
  border: 1px solid var(--border);
}

.cover-placeholder {
  width: 180px;
  height: 80px;
  border-radius: 8px;
  background: #0d1220;
  display: grid;
  place-items: center;
  color: var(--muted);
  border: 1px dashed #2a3246;
}

.team-form {
  margin-top: 14px;
  display: flex;
  flex-direction: column;
  gap: 12px;
}

.form-grid {
  display: grid;
  grid-template-columns: repeat(auto-fit, minmax(220px, 1fr));
  gap: 12px;
}

label {
  display: flex;
  flex-direction: column;
  gap: 6px;
  color: var(--muted);
  font-size: 14px;
}

input,
select,
textarea {
  background: #0c1220;
  border: 1px solid var(--border);
  color: var(--text);
  padding: 12px 12px;
  border-radius: 10px;
  font-size: 14px;
  outline: none;
}

textarea {
  resize: vertical;
}

.upload-box {
  background: #0c1220;
  border: 1px dashed #2a3246;
  border-radius: 12px;
  padding: 20px;
  display: grid;
  place-items: center;
  color: var(--muted);
  text-align: center;
}

.upload-box.square {
  height: 140px;
}

.upload-box.rectangle {
  height: 120px;
}

.upload-note {
  font-size: 12px;
  color: #6b7280;
}

.switch-row {
  display: flex;
  justify-content: space-between;
  align-items: center;
  background: #0e1423;
  border: 1px solid var(--border);
  border-radius: 12px;
  padding: 12px;
}

.switch-title {
  color: var(--text);
  font-weight: 700;
}

.switch-sub {
  color: var(--muted);
  font-size: 13px;
}

.switch {
  width: 52px;
  height: 28px;
  border-radius: 999px;
  background: #121a2c;
  border: 1px solid var(--border);
  padding: 4px;
  display: flex;
  align-items: center;
  cursor: pointer;
  transition: all 0.12s ease-out;
}

.switch span {
  display: block;
  width: 18px;
  height: 18px;
  border-radius: 50%;
  background: #6b7280;
  transition: all 0.12s ease-out;
}

.switch.on {
  background: #101a0c;
  border-color: var(--accent);
}

.switch.on span {
  background: var(--accent);
  transform: translateX(20px);
}

.form-actions {
  display: flex;
  gap: 10px;
}

.hero-tags .tag,
.hackathon-tags .tag,
.badge-card,
.stat-card,
.profile-header,
.team-card {
  border: 1px solid var(--border);
}

.hero-visual,
.profile-panel,
.hackathon-card,
.list-row,
.timeline-item,
.badge-card,
.team-card,
.form-card {
  position: relative;
  overflow: hidden;
}

.hero-visual::after,
.profile-panel::after,
.hackathon-card::after,
.list-row::after,
.timeline-item::after,
.badge-card::after,
.team-card::after,
.form-card::after {
  content: "";
  position: absolute;
  inset: 0;
  background: radial-gradient(circle at top left, rgba(159, 239, 0, 0.08), transparent 45%);
  pointer-events: none;
  opacity: 0.7;
}

.hero-visual::before {
  content: "";
  position: absolute;
  inset: 0;
  background: linear-gradient(135deg, rgba(0, 179, 255, 0.08), transparent);
}

.form-badge,
.primary-btn,
.status-pill,
.switch.on,
.mode-badge.hybrid,
.switch.on span {
  box-shadow: 0 0 0 1px rgba(159, 239, 0, 0.1), 0 8px 20px rgba(159, 239, 0, 0.15);
}

.form-card .cover-placeholder,
.upload-box {
  position: relative;
}

.form-card .cover-placeholder::after,
.upload-box::after {
  content: "";
  position: absolute;
  inset: 0;
  background: radial-gradient(circle at center, rgba(0, 179, 255, 0.04), transparent 50%);
  pointer-events: none;
}

/* Auth styles */

.auth-root {
  width: 100%;
  min-height: 100vh;
  padding: 40px 24px;
  display: flex;
  align-items: center;
  justify-content: center;
  background: radial-gradient(circle at 20% 20%, rgba(0, 179, 255, 0.06), transparent 25%),
    radial-gradient(circle at 80% 0%, rgba(159, 239, 0, 0.08), transparent 30%),
    var(--bg);
}

.auth-card {
  width: 100%;
  max-width: 1120px;
  background: var(--panel);
  border-radius: 24px;
  box-shadow: var(--shadow);
  border: 1px solid rgba(255, 255, 255, 0.06);
  display: grid;
  grid-template-columns: 1.2fr 1fr;
  overflow: hidden;
  min-height: 520px;
}

.auth-left {
  padding: 32px 36px;
  background: linear-gradient(145deg, rgba(15, 22, 40, 0.95), rgba(10, 13, 24, 0.95));
  border-right: 1px solid var(--border);
  display: flex;
  flex-direction: column;
  gap: 14px;
  justify-content: center;
}

.auth-title {
  font-size: 32px;
  margin: 0;
}

.auth-subtitle {
  margin: 0;
  color: #cfd3dc;
  line-height: 1.6;
}

.auth-right {
  padding: 32px 36px;
  display: flex;
  flex-direction: column;
  gap: 16px;
  background: #0c1220;
}

.auth-mode-toggle {
  display: inline-flex;
  background: #0f1628;
  border: 1px solid var(--border);
  border-radius: 12px;
  padding: 6px;
  gap: 6px;
}

.auth-tab {
  padding: 10px 16px;
  border-radius: 8px;
  border: 1px solid transparent;
  background: transparent;
  color: var(--muted);
  cursor: pointer;
  font-weight: 700;
}

.auth-tab.active {
  background: #121a2c;
  border-color: var(--border);
  color: var(--text);
  box-shadow: inset 0 -2px 0 var(--accent);
}

.auth-form {
  display: flex;
  flex-direction: column;
  gap: 12px;
  background: #0f1422;
  border: 1px solid var(--border);
  border-radius: 12px;
  padding: 16px;
  box-shadow: var(--shadow);
}

.auth-error {
  background: rgba(255, 99, 71, 0.1);
  color: #f87171;
  border: 1px solid rgba(248, 113, 113, 0.4);
  padding: 10px 12px;
  border-radius: 10px;
  font-size: 14px;
}

.auth-submit {
  width: 100%;
  justify-content: center;
  display: inline-flex;
  align-items: center;
  text-align: center;
}

.auth-switch {
  background: transparent;
  border: none;
  color: var(--accent);
  text-align: left;
  cursor: pointer;
  font-weight: 700;
}

.hack-toolbar {
  display: flex;
  align-items: center;
  justify-content: space-between;
  gap: 12px;
  flex-wrap: wrap;
  margin-top: 8px;
}

.hack-config-layout {
  display: grid;
  grid-template-columns: 1fr 1.2fr;
  gap: 16px;
}

.hack-config-list {
  background: var(--panel);
  border: 1px solid var(--border);
  border-radius: 18px;
  padding: 16px;
  box-shadow: var(--shadow);
  display: flex;
  flex-direction: column;
  gap: 12px;
}

.hack-config-header {
  display: flex;
  align-items: center;
  justify-content: space-between;
  gap: 10px;
}

.hack-config-title {
  font-size: 18px;
  font-weight: 800;
}

.hack-config-counter {
  min-width: 36px;
  height: 36px;
  border-radius: 12px;
  background: var(--panel-strong);
  border: 1px solid var(--border);
  display: grid;
  place-items: center;
  font-weight: 700;
}

.hack-config-meta {
  color: var(--muted);
  font-size: 13px;
}

.hack-config-items {
  display: flex;
  flex-direction: column;
  gap: 10px;
}

.hack-config-item {
  background: linear-gradient(145deg, rgba(18, 24, 44, 0.9), rgba(11, 15, 25, 0.9)), var(--panel);
  border: 1px solid var(--border);
  border-radius: 16px;
  padding: 12px 14px;
  color: var(--text);
  text-align: left;
  cursor: pointer;
  transition: all 0.15s ease-out;
}

.hack-config-item:hover {
  border-color: var(--accent);
  transform: translateY(-2px);
}

.hack-config-item.active {
  border-color: var(--accent);
  box-shadow: 0 10px 30px rgba(159, 239, 0, 0.08);
}

.hack-config-item__row {
  display: flex;
  align-items: center;
  justify-content: space-between;
  gap: 12px;
}

.hack-config-item__title {
  font-weight: 800;
}

.hack-config-form {
  background: var(--panel);
  border-radius: 20px;
  border: 1px solid var(--border);
  box-shadow: var(--shadow);
  display: flex;
  flex-direction: column;
  gap: 12px;
}

.hack-config-status-label {
  font-size: 16px;
  font-weight: 800;
}

.hack-config-status {
  color: var(--muted);
  font-size: 13px;
}

.hack-config-status.inline {
  min-height: 18px;
}

.hack-config-status.info {
  color: var(--accent);
}

.hack-config-status.error {
  color: #f87171;
}

.hack-config-toolbar {
  display: flex;
  align-items: center;
  justify-content: space-between;
  gap: 12px;
  margin-bottom: 6px;
  flex-wrap: wrap;
}

.hack-config-form .form-grid {
  margin-top: 6px;
}

.hack-config-form .form-actions {
  margin-top: 6px;
}

@media (max-width: 768px) {
  .hack-toolbar {
    flex-direction: column;
    align-items: stretch;
  }

  .hack-toolbar .primary-btn {
    width: 100%;
  }
}

@media (max-width: 960px) {
  .auth-card {
    grid-template-columns: 1fr;
  }

  .auth-left {
    border-right: none;
    border-bottom: 1px solid var(--border);
  }

  .hack-config-layout {
    grid-template-columns: 1fr;
  }
}
`;
