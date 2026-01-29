export default function WaitlistForm() {
  return (
    <form className="mt-8 flex w-full flex-col gap-4 sm:flex-row sm:items-center" aria-label="Join the FarmCredit waitlist">
      <label className="sr-only" htmlFor="email">
        Email address
      </label>
      <input
        id="email"
        name="email"
        type="email"
        autoComplete="email"
        placeholder="you@farmco.com"
        className="h-12 w-full rounded-full border border-[var(--fc-border)] bg-white px-5 text-sm text-[var(--fc-ink)] shadow-sm transition focus:border-[var(--fc-brand)] focus:outline-none focus:ring-2 focus:ring-[var(--fc-brand-soft)]"
        required
      />
      <button
        type="submit"
        className="inline-flex h-12 items-center justify-center gap-2 rounded-full bg-[var(--fc-brand)] px-6 text-sm font-semibold text-white shadow-md shadow-emerald-200/60 transition hover:bg-[var(--fc-brand-dark)]"
      >
        Join Waitlist
        <span aria-hidden="true">→</span>
      </button>
    </form>
  );
}
