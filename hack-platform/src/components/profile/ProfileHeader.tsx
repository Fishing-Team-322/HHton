import React from "react";

export const ProfileHeader: React.FC = () => {
  return (
    <div className="profile-header">
      <div className="profile-avatar" />
      <div className="profile-info">
        <div className="profile-name">
          Алексей Волков <span className="nickname">@volkov.dev</span>
        </div>
        <div className="profile-meta">Москва · Frontend · React / TypeScript</div>
        <p className="profile-bio">
          Создаю интерфейсы для команд, люблю быстрые демо и понятные дашборды. Часто беру на себя фасилитацию и проверку UX-
          гипотез прямо во время хакатона.
        </p>
        <button className="primary-btn">Редактировать профиль</button>
      </div>
    </div>
  );
};
