import { Building2, ChevronDown } from "lucide-react";

export function EntitySwitcher() {
  return (
    <button
      disabled
      className="flex items-center gap-2 rounded-md border border-[var(--color-border)] px-2.5 py-1.5 text-sm text-[var(--color-muted-fg)] opacity-70"
      title="Entities load in Feature 2"
    >
      <Building2 className="h-4 w-4" /> All Entities <ChevronDown className="h-3.5 w-3.5" />
    </button>
  );
}
