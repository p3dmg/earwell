export function AuthCard(p: { title: string; subtitle?: string; icon?: React.ReactNode; children: React.ReactNode }) {
  return (
    <div className="grid h-screen place-items-center bg-[var(--color-bg)] p-6">
      <div className="w-full max-w-md rounded-xl border border-[var(--color-border)] bg-[var(--color-card)] p-6 shadow-sm">
        <div className="mb-1 flex items-center gap-2 text-sm font-semibold tracking-tight">
          {p.icon}{p.title}
        </div>
        {p.subtitle && <p className="mb-4 text-sm text-[var(--color-muted-fg)]">{p.subtitle}</p>}
        {p.children}
      </div>
    </div>
  );
}

export const inputCls =
  "rounded-md border border-[var(--color-border)] bg-[var(--color-card)] px-3 py-2 text-sm outline-none focus:border-[var(--color-primary)]";

export function Field({ label, error, children }: { label: string; error?: string; children: React.ReactNode }) {
  return (
    <label className="flex flex-col gap-1 text-sm">
      <span className="text-[var(--color-muted-fg)]">{label}</span>
      {children}
      {error && <span className="text-xs text-[var(--color-danger)]">{error}</span>}
    </label>
  );
}
