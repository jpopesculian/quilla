import { create } from "zustand"

type Theme = "dark" | "light" | "system"

interface ThemeState {
  theme: Theme
  setTheme: (theme: Theme) => void
}

function applyTheme(theme: Theme) {
  const root = document.documentElement
  root.classList.remove("light", "dark")

  if (theme === "system") {
    const system = window.matchMedia("(prefers-color-scheme: dark)").matches
      ? "dark"
      : "light"
    root.classList.add(system)
  } else {
    root.classList.add(theme)
  }
}

function resolveTheme(theme: Theme): "dark" | "light" {
  if (theme === "system") {
    return window.matchMedia("(prefers-color-scheme: dark)").matches
      ? "dark"
      : "light"
  }
  return theme
}

export function useResolvedTheme(): "dark" | "light" {
  return resolveTheme(useThemeStore((s) => s.theme))
}

export const useThemeStore = create<ThemeState>((set) => {
  const stored = localStorage.getItem("theme") as Theme | null
  const initial = stored ?? "system"
  applyTheme(initial)

  return {
    theme: initial,
    setTheme: (theme) => {
      localStorage.setItem("theme", theme)
      applyTheme(theme)
      set({ theme })
    },
  }
})
