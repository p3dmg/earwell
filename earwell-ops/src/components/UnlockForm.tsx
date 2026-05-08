import { useForm } from "react-hook-form";
import { zodResolver } from "@hookform/resolvers/zod";
import { z } from "zod";
import { useMutation } from "@tanstack/react-query";
import { KeyRound } from "lucide-react";
import { api } from "@/lib/api";
import { useSessionStore } from "@/store/session";
import { AuthCard, inputCls } from "./AuthCard";

const Schema = z.object({ passphrase: z.string().min(1) });
type Values = z.infer<typeof Schema>;

export function UnlockForm() {
  const setStatus = useSessionStore((s) => s.setStatus);
  const { register, handleSubmit } = useForm<Values>({ resolver: zodResolver(Schema) });
  const m = useMutation({ mutationFn: api.unlock, onSuccess: setStatus });
  return (
    <AuthCard title="Unlock database" subtitle="Enter the master passphrase." icon={<KeyRound className="h-5 w-5" />}>
      <form className="flex flex-col gap-3" onSubmit={handleSubmit((v) => m.mutate({ passphrase: v.passphrase }))}>
        <input type="password" autoFocus autoComplete="current-password" {...register("passphrase")} className={inputCls} />
        {m.error && (
          <div className="rounded-md bg-[var(--color-danger)]/10 p-2 text-sm text-[var(--color-danger)]">
            {String(m.error)}
          </div>
        )}
        <button
          type="submit" disabled={m.isPending}
          className="mt-2 rounded-md bg-[var(--color-primary)] px-3 py-2 text-sm font-medium text-[var(--color-primary-fg)] hover:opacity-90 disabled:opacity-50"
        >
          {m.isPending ? "Unlocking…" : "Unlock"}
        </button>
      </form>
    </AuthCard>
  );
}
