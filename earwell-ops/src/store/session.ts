import { create } from "zustand";
import type { AppStatus, Session } from "@/lib/types";

interface S {
  status: AppStatus | null;
  session: Session | null;
  setStatus: (s: AppStatus) => void;
  setSession: (s: Session | null) => void;
}
export const useSessionStore = create<S>((set) => ({
  status: null,
  session: null,
  setStatus: (status) => set({ status, session: status.session }),
  setSession: (session) => set({ session }),
}));
