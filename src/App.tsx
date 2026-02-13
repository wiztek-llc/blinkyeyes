import { useEffect } from "react";
import { Routes, Route, NavLink, useLocation } from "react-router-dom";
import { listen } from "@tauri-apps/api/event";
import Dashboard from "./pages/Dashboard";
import Settings from "./pages/Settings";
import MiniOverlay from "./components/MiniOverlay";
import type { UserSettings } from "./lib/types";
import { getSettings } from "./lib/commands";
import { useChime } from "./hooks/useChime";

function applyTheme(theme: string) {
  const root = document.documentElement;
  if (theme === "dark") {
    root.classList.add("dark");
  } else if (theme === "light") {
    root.classList.remove("dark");
  } else {
    // system
    if (window.matchMedia("(prefers-color-scheme: dark)").matches) {
      root.classList.add("dark");
    } else {
      root.classList.remove("dark");
    }
  }
}

function AppShell() {
  useChime();

  useEffect(() => {
    getSettings()
      .then((s) => applyTheme(s.theme))
      .catch(console.error);

    const unlistenSettings = listen<UserSettings>(
      "settings-changed",
      (event) => {
        applyTheme(event.payload.theme);
      },
    );

    const mq = window.matchMedia("(prefers-color-scheme: dark)");
    const handleSystemChange = () => {
      getSettings()
        .then((s) => {
          if (s.theme === "system") applyTheme("system");
        })
        .catch(console.error);
    };
    mq.addEventListener("change", handleSystemChange);

    return () => {
      unlistenSettings.then((fn) => fn());
      mq.removeEventListener("change", handleSystemChange);
    };
  }, []);

  return (
    <div className="min-h-screen bg-gray-50 text-gray-900 dark:bg-gray-900 dark:text-gray-100 transition-colors">
      <nav className="flex items-center justify-between px-5 py-3 border-b border-gray-200 dark:border-gray-700 bg-white/80 dark:bg-gray-800/80 backdrop-blur-sm">
        <span className="text-lg font-semibold tracking-tight">Blinky</span>
        <div className="flex gap-1">
          <NavLink
            to="/"
            className={({ isActive }) =>
              `px-3 py-1.5 rounded-lg text-sm font-medium transition-colors ${
                isActive
                  ? "bg-blue-100 text-blue-700 dark:bg-blue-900/40 dark:text-blue-300"
                  : "text-gray-500 hover:text-gray-700 dark:text-gray-400 dark:hover:text-gray-200"
              }`
            }
          >
            Dashboard
          </NavLink>
          <NavLink
            to="/settings"
            className={({ isActive }) =>
              `px-3 py-1.5 rounded-lg text-sm font-medium transition-colors ${
                isActive
                  ? "bg-blue-100 text-blue-700 dark:bg-blue-900/40 dark:text-blue-300"
                  : "text-gray-500 hover:text-gray-700 dark:text-gray-400 dark:hover:text-gray-200"
              }`
            }
          >
            Settings
          </NavLink>
        </div>
      </nav>
      <main className="p-5">
        <Routes>
          <Route path="/" element={<Dashboard />} />
          <Route path="/settings" element={<Settings />} />
        </Routes>
      </main>
    </div>
  );
}

function App() {
  const location = useLocation();

  if (location.pathname === "/overlay") {
    return <MiniOverlay />;
  }

  return <AppShell />;
}

export default App;
