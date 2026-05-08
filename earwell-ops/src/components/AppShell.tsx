import { Outlet } from "react-router-dom";
import { Sidebar } from "./Sidebar";
import { TopBar } from "./TopBar";

export function AppShell() {
  return (
    <div className="grid h-screen grid-cols-[240px_1fr] grid-rows-[auto_1fr]">
      <div className="col-span-2 border-b border-[var(--color-border)]">
        <TopBar />
      </div>
      <aside className="border-r border-[var(--color-border)] bg-[var(--color-card)]">
        <Sidebar />
      </aside>
      <main className="overflow-auto p-6">
        <Outlet />
      </main>
    </div>
  );
}
