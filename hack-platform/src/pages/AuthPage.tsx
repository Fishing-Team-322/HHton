import React, { useState } from "react";

interface AuthPageProps {
  onAuthSuccess: () => void;
}

type AuthMode = "login" | "register";

export const AuthPage: React.FC<AuthPageProps> = ({ onAuthSuccess }) => {
  const [mode, setMode] = useState<AuthMode>("login");
  const [form, setForm] = useState({
    fullName: "",
    nickname: "",
    city: "",
    email: "",
    password: "",
    confirmPassword: "",
  });
  const [errors, setErrors] = useState<string | null>(null);

  const handleSubmit = (e: React.FormEvent) => {
    e.preventDefault();
    setErrors(null);

    if (!form.email || !form.password) {
      setErrors("Заполните email и пароль");
      return;
    }

    if (mode === "register") {
      if (!form.fullName || !form.nickname || !form.city) {
        setErrors("Заполните все поля профиля участника");
        return;
      }
      if (form.password !== form.confirmPassword) {
        setErrors("Пароли не совпадают");
        return;
      }
    }

    onAuthSuccess();
  };

  const toggleMode = () => {
    setErrors(null);
    setMode(mode === "login" ? "register" : "login");
  };

  return (
    <div className="auth-root">
      <div className="auth-card">
        <div className="auth-left">
          <div className="logo">
            <div className="logo-mark" />
            <div className="logo-text">HACK</div>
          </div>
          <h1 className="auth-title">Платформа хакатонов</h1>
          <p className="auth-subtitle">
            Участвуй в онлайн и офлайн хакатонах, собирай команды и копи личную статистику. Все новые аккаунты создаются как
            <strong> участники</strong>, роль организатора выдаётся позже через админ-панель.
          </p>
        </div>

        <div className="auth-right">
          <div className="auth-mode-toggle">
            <button
              type="button"
              className={`auth-tab ${mode === "login" ? "active" : ""}`}
              onClick={() => setMode("login")}
            >
              Вход
            </button>
            <button
              type="button"
              className={`auth-tab ${mode === "register" ? "active" : ""}`}
              onClick={() => setMode("register")}
            >
              Регистрация
            </button>
          </div>

          <form className="auth-form" onSubmit={handleSubmit}>
            {mode === "register" && (
              <>
                <label>
                  <span>Полное имя</span>
                  <input
                    value={form.fullName}
                    onChange={(e) => setForm({ ...form, fullName: e.target.value })}
                    placeholder="Как к вам обращаться"
                  />
                </label>
                <label>
                  <span>Никнейм</span>
                  <input
                    value={form.nickname}
                    onChange={(e) => setForm({ ...form, nickname: e.target.value })}
                    placeholder="@ник для платформы"
                  />
                </label>
                <label>
                  <span>Город</span>
                  <input
                    value={form.city}
                    onChange={(e) => setForm({ ...form, city: e.target.value })}
                    placeholder="Например, Ростов-на-Дону"
                  />
                </label>
              </>
            )}

            <label>
              <span>Email</span>
              <input
                type="email"
                value={form.email}
                onChange={(e) => setForm({ ...form, email: e.target.value })}
                placeholder="you@example.com"
              />
            </label>
            <label>
              <span>Пароль</span>
              <input
                type="password"
                value={form.password}
                onChange={(e) => setForm({ ...form, password: e.target.value })}
                placeholder="••••••••"
              />
            </label>

            {mode === "register" && (
              <label>
                <span>Повтор пароля</span>
                <input
                  type="password"
                  value={form.confirmPassword}
                  onChange={(e) => setForm({ ...form, confirmPassword: e.target.value })}
                  placeholder="••••••••"
                />
              </label>
            )}

            {errors && <div className="auth-error">{errors}</div>}

            <button type="submit" className="primary-btn auth-submit">
              {mode === "login" ? "Войти" : "Создать аккаунт участника"}
            </button>
          </form>

          <button type="button" className="auth-switch" onClick={toggleMode}>
            {mode === "login"
              ? "Нет аккаунта? Зарегистрироваться как участник"
              : "Уже есть аккаунт? Войти"}
          </button>
        </div>
      </div>
    </div>
  );
};
