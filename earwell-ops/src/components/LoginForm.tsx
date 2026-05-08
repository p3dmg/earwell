import { useForm } from "react-hook-form";
import { zodResolver } from "@hookform/resolvers/zod";
import { z } from "zod";
import { useMutation } from "@tanstack/react-query";
import { LogIn } from "lucide-react";
import { api } from "@/lib/api";
import { useSessionStore } from "@/store/session";
import { AuthCard, inputCls } from "./AuthCard";

const Schema = z.object({ username: z.string().min(1), password: z.string().min(1) });
type Values = z.infer<typeof Schema>;

export function LoginForm() {
  const setSession = useSessionStore((s) => s.setSession);
  const { register, handleSubmit } = useForm<Values>({ resolver: zodResolver(Schema) });
  const m = useMutation({ mutationFn: api.login, onSuccess: setSession });
  return (
    <AuthCard title="Sign in" subtitle="Database is unlocked. Sign in with your account." icon={<LogIn className="h-5 w-5" />}>
      <form className="flex flex-col gap-3" onSubmit={handleSubmit((v) => m.mutate(v))}>
        <input autoFocus placeholder="Username" {...register("username")} className={inputCls} />
        <input type="password" placeholder="Password" {...register("password")} className={inputCls} />
        {m.error && (
          <div className="rounded-md bg-[var(--color-danger)]/10 p-2 text-sm text-[var(--color-danger)]">
            {String(m.error)}
          </div>
        )}
        <button
          type="submit" disabled={m.isPending}
          className="mt-2 rounded-md bg-[var(--color-primary)] px-3 py-2 text-sm font-medium text-[var(--color-primary-fg)] hover:opacity-90 disabled:opacity-50"
        >
          {m.isPending ? "Signing in…" : "Sign in"}
        </button>
      </form>
    </AuthCard>
  );
}
