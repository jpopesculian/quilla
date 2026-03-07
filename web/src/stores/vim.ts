import { create } from "zustand";

interface VimState {
  enabled: boolean;
  toggle: () => void;
}

export const useVimStore = create<VimState>((set) => ({
  enabled: localStorage.getItem("vim") === "true",
  toggle: () =>
    set((s) => {
      const enabled = !s.enabled;
      localStorage.setItem("vim", String(enabled));
      return { enabled };
    }),
}));
