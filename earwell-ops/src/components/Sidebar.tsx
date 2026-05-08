import { NavLink } from "react-router-dom";
import {
  LayoutDashboard, ListChecks, FileInput, Scale,
  Receipt, Wallet, PieChart, FileText, ShieldCheck, Settings,
} from "lucide-react";

const items = [
  { to: "/",                icon: LayoutDashboard, label: "Dashboard" },
  { to: "/closeout",        icon: ListChecks,      label: "Closeout" },
  { to: "/imports",         icon: FileInput,       label: "Imports" },
  { to: "/reconciliation",  icon: Scale,           label: "Reconciliation" },
  { to: "/refunds",         icon: Receipt,         label: "Refunds" },
  { to: "/payables",        icon: Wallet,          label: "Payables" },
  { to: "/distributions",   icon: PieChart,        label: "Distributions" },
  { to: "/reports",         icon: FileText,        label: "Reports" },
  { to: "/audit",           icon: ShieldCheck,     label: "Audit" },
  { to: "/settings",        icon: Settings,        label: "Settings" },
];

export function Sidebar() {
  return (
    <nav className="flex h-full flex-col gap-1 p-3 text-sm">
      {items.map(({ to, icon: Icon, label }) => (
        <NavLink
          key={to} to={to} end={to === "/"}
          className={({ isActive }) =>
            [
              "flex items-center gap-2.5 rounded-md px-2.5 py-2 transition-colors",
              isActive
                ? "bg-[var(--color-primary)]/10 text-[var(--color-primary)]"
                : "text-[var(--color-muted-fg)] hover:bg-black/5 hover:text-[var(--color-fg)] dark:hover:bg-white/5",
            ].join(" ")
          }
        >
          <Icon className="h-4 w-4" /> {label}
        </NavLink>
      ))}
    </nav>
  );
}
