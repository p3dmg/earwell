import { useEffect } from "react";
import { useQuery } from "@tanstack/react-query";
import { BrowserRouter, Routes, Route } from "react-router-dom";
import { api } from "@/lib/api";
import { useSessionStore } from "@/store/session";
import { SetupForm }  from "@/components/SetupForm";
import { UnlockForm } from "@/components/UnlockForm";
import { LoginForm }  from "@/components/LoginForm";
import { AppShell }   from "@/components/AppShell";
import Dashboard from "@/pages/Dashboard";

export default function App() {
  const setStatus = useSessionStore((s) => s.setStatus);
  const status    = useSessionStore((s) => s.status);

  const q = useQuery({ queryKey: ["app_status"], queryFn: api.appStatus, staleTime: 0 });
  useEffect(() => { if (q.data) setStatus(q.data); }, [q.data, setStatus]);

  if (q.isPending) {
    return <div className="grid h-screen place-items-center text-sm text-[var(--color-muted-fg)]">Loading…</div>;
  }

  const phase = status?.phase ?? q.data?.phase;
  if (phase === "needs_setup") return <SetupForm />;
  if (phase === "locked")      return <UnlockForm />;
  if (phase === "unlocked")    return <LoginForm />;

  return (
    <BrowserRouter>
      <Routes>
        <Route element={<AppShell />}>
          <Route index element={<Dashboard />} />
          <Route path="*" element={<Dashboard />} />
        </Route>
      </Routes>
    </BrowserRouter>
  );
}
