export type Cents = number & { readonly __brand: "Cents" };
export const cents       = (n: number): Cents => Math.round(n) as Cents;
export const fromDollars = (d: number): Cents => Math.round(d * 100) as Cents;
export const toDollars   = (c: Cents): number => c / 100;
const fmt = new Intl.NumberFormat("en-US", { style: "currency", currency: "USD" });
export const formatCents = (c: Cents | number) => fmt.format((c as number) / 100);
