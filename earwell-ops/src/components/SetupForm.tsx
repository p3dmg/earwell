import { useForm } from "react-hook-form";
import { zodResolver } from "@hookform/resolvers/zod";
import { z } from "zod";
import { useMutation } from "@tanstack/react-query";
import { ShieldPlus } from "lucide-react";
import { api } from "@/lib/api";
import { useSessionStore } from "@/store/session";
import { AuthCard, Field, inputCls } from "./AuthCard";

const Schema = z.object({
  passphrase: z.string().min(12, "At least 12 characters"),
  passphrase_confirm: z.string(),
  admin_username: z.string().min(3),
  admin_full_name: z.string().min(2),
  admin_password: z.string().min(10, "At least 10 characters"),
}).refine((d) => d.passphrase === d.passphrase_confirm, {
  path: ["passphrase_confirm"], message: "Passphrases must match",
});
type Values = z.infer<typeof Schema>;

export function SetupForm() {
  const setStatus = useSessionStore((s) => s.setStatus);
  const { register, handleSubmit, formState: { errors } } = useForm<Values>({ resolver: zodResolver(Schema) });
  const m = useMutation({ mutationFn: api.setup, onSuccess: setStatus });

  return (
    <AuthCard
      title="First-time setup"
      subtitle="Create the master passphrase that encrypts the database, then your admin account."
      icon={<ShieldPlus className="h-5 w-5" />}
    >
      <form
        className="flex flex-col gap-3"
        onSubmit={handleSubmit((v) => m.mutate({
          passphrase: v.passphrase,
          admin_username: v.admin_username,
          admin_full_name: v.admin_full_name,
          admin_password: v.admin_password,
        }))}
      >
        <Field label="Master passphrase" error={errors.passphrase?.message}>
          <input type="password" autoComplete="new-password" {...register("passphrase")} className={inputCls} />
        </Field>
        <Field label="Confirm passphrase" error={errors.passphrase_confirm?.message}>
          <input type="password" autoComplete="new-password" {...register("passphrase_confirm")} className={inputCls} />
        </Field>
        <hr className="my-1 border-[var(--color-border)]" />
        <Field label="Admin username" error={errors.admin_username?.message}>
          <input {...register("admin_username")} className={inputCls} />
        </Field>
        <Field label="Admin full name" error={errors.admin_full_name?.message}>
          <input {...register("admin_full_name")} className={inputCls} />
        </Field>
        <Field label="Admin password" error={errors.admin_password?.message}>
          <input type="password" autoComplete="new-password" {...register("admin_password")} className={inputCls} />
        </Field>
        {m.error && (
          <div className="rounded-md bg-[var(--color-danger)]/10 p-2 text-sm text-[var(--color-danger)]">
            {String(m.error)}
          </div>
        )}
        <button
          type="submit" disabled={m.isPending}
          className="mt-2 rounded-md bg-[var(--color-primary)] px-3 py-2 text-sm font-medium text-[var(--color-primary-fg)] hover:opacity-90 disabled:opacity-50"
        >
          {m.isPending ? "Creating…" : "Create database"}
        </button>
        <p className="mt-1 text-xs text-[var(--color-muted-fg)]">
          The master passphrase cannot be recovered. Store it somewhere safe.
        </p>
      </form>
    </AuthCard>
  );
}
