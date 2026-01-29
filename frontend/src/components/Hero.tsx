import SiteHeader from "@/components/SiteHeader";
import WaitlistForm from "@/components/WaitlistForm";

export default function Hero() {
  return (
    <div className="relative overflow-hidden">
      <SiteHeader />
      <main className="mx-auto grid w-full max-w-6xl items-center gap-12 px-6 pb-20 pt-12 lg:grid-cols-[1.1fr_0.9fr] lg:pt-16">
        <section>
          <div className="inline-flex items-center gap-2 rounded-full border border-[var(--fc-border)] bg-white/70 px-4 py-1 text-xs font-semibold uppercase tracking-[0.2em] text-[var(--fc-ink-muted)] shadow-sm">
            Early Access Open
          </div>
          <h1 className="font-display mt-6 text-4xl font-semibold leading-tight text-[var(--fc-ink)] sm:text-5xl lg:text-6xl">
            Confidence in every farm finance decision.
          </h1>
          <p className="mt-4 max-w-xl text-base leading-7 text-[var(--fc-ink-muted)] sm:text-lg">
            FarmCredit brings cash-flow planning, loan readiness, and lender collaboration into one
            trusted workspace built for agriculture.
          </p>
          <WaitlistForm />
          <p className="mt-3 text-xs text-[var(--fc-ink-muted)]">
            We will email once when early access opens in your region.
          </p>
          <div className="mt-6 flex items-center gap-4 text-xs text-[var(--fc-ink-muted)]">
            <div className="flex items-center gap-2">
              <span className="h-2 w-2 rounded-full bg-[var(--fc-brand)]" aria-hidden="true" />
              SOC 2-ready security posture
            </div>
            <div className="h-4 w-px bg-[var(--fc-border)]" aria-hidden="true" />
            <div>Designed with ag lenders and operators</div>
          </div>
        </section>
        <section className="relative">
          <div className="absolute -right-16 -top-16 h-64 w-64 rounded-full bg-[var(--fc-brand-soft)] blur-3xl" aria-hidden="true" />
          <div className="absolute -bottom-10 left-6 h-40 w-40 rounded-full bg-[#f3efe5] blur-3xl" aria-hidden="true" />
          <div className="relative rounded-3xl border border-[var(--fc-border)] bg-white/90 p-8 shadow-xl shadow-emerald-100/40">
            <div className="flex items-center justify-between">
              <div>
                <p className="text-xs uppercase tracking-[0.18em] text-[var(--fc-ink-muted)]">Portfolio View</p>
                <p className="mt-2 text-xl font-semibold text-[var(--fc-ink)]">2026 Planning Cycle</p>
              </div>
              <div className="rounded-full bg-[var(--fc-brand-soft)] px-3 py-1 text-xs font-semibold text-[var(--fc-brand-dark)]">
                Stable
              </div>
            </div>
            <div className="mt-6 grid gap-4">
              <div className="rounded-2xl border border-[var(--fc-border)] bg-[var(--fc-surface-muted)] px-4 py-3">
                <div className="flex items-center justify-between text-sm text-[var(--fc-ink)]">
                  <span>Operating line</span>
                  <span className="font-semibold">$480,000</span>
                </div>
                <div className="mt-2 h-2 w-full rounded-full bg-white">
                  <div className="h-2 w-3/4 rounded-full bg-[var(--fc-brand)]" />
                </div>
              </div>
              <div className="rounded-2xl border border-[var(--fc-border)] bg-white px-4 py-3">
                <div className="flex items-center justify-between text-sm text-[var(--fc-ink)]">
                  <span>Equipment renewal</span>
                  <span className="font-semibold">Q3 2026</span>
                </div>
                <div className="mt-2 h-2 w-full rounded-full bg-[var(--fc-surface-muted)]">
                  <div className="h-2 w-1/2 rounded-full bg-[#c7d5c8]" />
                </div>
              </div>
            </div>
            <div className="mt-6 rounded-2xl border border-[var(--fc-border)] bg-white px-4 py-4">
              <p className="text-xs uppercase tracking-[0.18em] text-[var(--fc-ink-muted)]">
                Collaboration
              </p>
              <div className="mt-3 flex items-center gap-3">
                <div className="flex -space-x-2">
                  <div className="h-8 w-8 rounded-full border-2 border-white bg-[#d7e1da]" />
                  <div className="h-8 w-8 rounded-full border-2 border-white bg-[#bcd1c7]" />
                  <div className="h-8 w-8 rounded-full border-2 border-white bg-[#8fb8aa]" />
                </div>
                <div>
                  <p className="text-sm font-semibold text-[var(--fc-ink)]">Advisor sync</p>
                  <p className="text-xs text-[var(--fc-ink-muted)]">All documents in one thread</p>
                </div>
              </div>
            </div>
          </div>
        </section>
      </main>
    </div>
  );
}
