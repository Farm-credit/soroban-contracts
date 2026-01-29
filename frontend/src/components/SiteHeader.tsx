export default function SiteHeader() {
  return (
    <header className="mx-auto flex w-full max-w-6xl items-center justify-between px-6 pt-8">
      <div className="flex items-center gap-3">
        <div className="grid h-10 w-10 place-items-center rounded-xl bg-[var(--fc-brand-soft)] shadow-sm">
          <span className="h-4 w-4 rounded-md bg-[var(--fc-brand)]" aria-hidden="true" />
        </div>
        <div className="text-sm font-semibold tracking-[0.18em] text-[var(--fc-ink)]">
          FARM CREDIT
        </div>
      </div>
      <nav className="hidden items-center gap-8 text-sm text-[var(--fc-ink-muted)] md:flex">
        <a className="transition hover:text-[var(--fc-ink)]" href="#">
          Twitter
        </a>
        <a className="transition hover:text-[var(--fc-ink)]" href="#">
          Discord
        </a>
        <button className="rounded-full border border-[var(--fc-border)] px-4 py-2 text-sm font-semibold text-[var(--fc-ink)] transition hover:border-[var(--fc-brand)] hover:text-[var(--fc-brand)]">
          Login
        </button>
      </nav>
    </header>
  );
}
