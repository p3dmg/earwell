import { useSessionStore } from "@/store/session";

export default function Dashboard() {
  const session = useSessionStore((s) => s.session);
  const status  = useSessionStore((s) => s.status);
  return (
    <div className="flex flex-col gap-4">
      <header>
        <h1 className="text-xl font-semibold tracking-tight">Welcome, {session?.full_name}</h1>
        <p className="text-sm text-[var(--color-muted-fg)]">
          {status?.user_count} user(s). Database: <code>{status?.db_path}</code>
        </p>
      </header>
      <div className="rounded-lg border border-dashed border-[var(--color-border)] p-8 text-center text-sm text-[var(--color-muted-fg)]">
        Dashboard tiles arrive in Feature 2 (entities + closeout summary).
      </div>
    </div>
  );
}
