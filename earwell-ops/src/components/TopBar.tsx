import { Lock, LogOut } from "lucide-react";
import { useMutation } from "@tanstack/react-query";
import { api } from "@/lib/api";
import { useSessionStore } from "@/store/session";
import { EntitySwitcher } from "./EntitySwitcher";

export function TopBar() {
  const session = useSessionStore((s) => s.session);
  const setStatus = useSessionStore((s) => s.setStatus);

  const lockMut = useMutation({ mutationFn: api.lock, onSuccess: setStatus });
  const logoutMut = useMutation({
    mutationFn: api.logout,
    onSuccess: () => useSessionStore.getState().setSession(null),
  });

  return (
    <div className="flex items-center justify-between px-4 py-2.5">
      <div className="flex items-center gap-3">
        <div className="text-sm font-semibold tracking-tight">Earwell Ops</div>
        <div className="h-5 w-px bg-[var(--color-border)]" />
        <EntitySwitcher />
      </div>
      <div className="flex items-center gap-2 text-sm">
        {session && (
          <span className="text-[var(--color-muted-fg)]">
            {session.full_name} · <span className="text-[var(--color-fg)]">{labelForRole(session.role)}</span>
          </span>
        )}
        <button onClick={() => logoutMut.mutate()} title="Log out"
          className="flex items-center gap-1.5 rounded-md px-2 py-1.5 text-[var(--color-muted-fg)] hover:bg-black/5 hover:text-[var(--color-fg)] dark:hover:bg-white/5">
          <LogOut className="h-4 w-4" /> Log out
        </button>
        <button onClick={() => lockMut.mutate()} title="Lock the database"
          className="flex items-center gap-1.5 rounded-md px-2 py-1.5 text-[var(--color-muted-fg)] hover:bg-black/5 hover:text-[var(--color-fg)] dark:hover:bg-white/5">
          <Lock className="h-4 w-4" /> Lock
        </button>
      </div>
    </div>
  );
}

function labelForRole(r: string) {
  return r === "admin" ? "Admin" : r === "bookkeeper" ? "Bookkeeper" : "Partner";
}
