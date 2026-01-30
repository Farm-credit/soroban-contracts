"use client";

import { useState } from "react";
import Image from "next/image";
import {
  ArrowRight,
  BadgeCheck,
  LineChart,
  Lock,
  Star,
  Zap,
} from "lucide-react";
import ImageWithFallback from "@/components/ImageWithFallback";

export default function Home() {
  const [email, setEmail] = useState("");

  const handleSubmit = (event: React.FormEvent<HTMLFormElement>) => {
    event.preventDefault();
    console.log("Email submitted:", email);
  };

  return (
    <div className="flex min-h-screen flex-col bg-background text-foreground">
      <header className="mx-auto flex w-full max-w-7xl items-center justify-between px-6 py-5">
        <div className="flex items-center gap-2">
          <div className="flex h-6 w-6 items-center justify-center rounded-md bg-[var(--fc-brand)]">
            <svg
              width="14"
              height="14"
              viewBox="0 0 14 14"
              fill="none"
              xmlns="http://www.w3.org/2000/svg"
              aria-hidden="true"
            >
              <path
                d="M7 0L9 5H14L10 8.5L11.5 14L7 10.5L2.5 14L4 8.5L0 5H5L7 0Z"
                fill="white"
              />
            </svg>
          </div>
          <span className="font-bold text-[20px] leading-[100%] tracking-[-0.5px] align-middle">
            Farm Credit
          </span>
        </div>
        <nav className="hidden items-center gap-8 text-sm text-[var(--fc-ink-muted)] md:flex">
          <a href="#" className="transition hover:text-foreground">
            Twitter
          </a>
          <a href="#" className="transition hover:text-foreground">
            Discord
          </a>
          <a
            href="#"
            className="font-medium text-foreground transition hover:text-gray-700"
          >
            Login
          </a>
        </nav>
      </header>

      <main className="flex flex-1 flex-col">
        <section className="flex items-center justify-center px-6 py-16">
          <div className="w-full max-w-2xl text-center">
            <div className="mb-8 inline-block">
              <span className="rounded-full bg-[var(--fc-brand-soft)] px-4 py-2 text-sm font-medium text-[#0F1723]">
                Early Access Open
              </span>
            </div>

            <h1 className="text-center text-[44px] font-bold leading-[52px] tracking-[-1.2px] align-middle bg-linear-to-b from-[#0F1723] to-[#6B7280] bg-clip-text text-transparent sm:text-[56px] sm:leading-[64px] md:text-[64px] md:leading-[70.4px]">
              The future of work
              <br />
              is finally here.
            </h1>

            <p className="mx-auto mt-6 max-w-xl text-center text-[16px] font-normal leading-[26px] tracking-normal text-[#6B7280] sm:text-[14px] sm:leading-[28.8px]">
              Farm Credit connects your entire workflow into one intelligent
              <br />
              hub. Join the waitlist to secure your username and get early
              access.
            </p>

            <form
              onSubmit={handleSubmit}
              className="mx-auto mb-8 mt-8 flex max-w-md flex-col gap-3 sm:flex-row"
            >
              <input
                type="email"
                placeholder="name@company.com"
                value={email}
                onChange={(event) => setEmail(event.target.value)}
                className="h-12 min-h-12 flex-1 rounded-lg border border-[var(--fc-border)] px-4 text-sm focus:border-transparent focus:outline-none focus:ring-2 focus:ring-[var(--fc-brand)]"
                required
                aria-label="Email address"
              />
              <button
                type="submit"
                className="inline-flex h-12 items-center justify-center gap-2 rounded-lg bg-[var(--fc-brand)] px-6 text-sm font-medium text-white transition hover:bg-[var(--fc-brand-dark)]"
              >
                Join Waitlist
                <ArrowRight className="h-4 w-4" aria-hidden="true" />
              </button>
            </form>

            <div className="flex flex-wrap items-center justify-center gap-4">
              <div className="flex items-center -space-x-3">
                <ImageWithFallback
                  src="https://images.unsplash.com/photo-1494790108377-be9c29b29330?w=100&h=100&fit=crop"
                  alt="User avatar"
                  width={40}
                  height={40}
                  className="h-11 w-11 rounded-full border-4 border-white shadow-sm"
                />
                <ImageWithFallback
                  src="https://images.unsplash.com/photo-1507003211169-0a1dd7228f2d?w=100&h=100&fit=crop"
                  alt="User avatar"
                  width={40}
                  height={40}
                  className="h-11 w-11 rounded-full border-4 border-white shadow-sm"
                />
                <ImageWithFallback
                  src="https://images.unsplash.com/photo-1438761681033-6461ffad8d80?w=100&h=100&fit=crop"
                  alt="User avatar"
                  width={40}
                  height={40}
                  className="h-11 w-11 rounded-full border-4 border-white shadow-sm"
                />
                <ImageWithFallback
                  src="https://images.unsplash.com/photo-1500648767791-00dcc994a43e?w=100&h=100&fit=crop"
                  alt="User avatar"
                  width={40}
                  height={40}
                  className="h-11 w-11 rounded-full border-4 border-white shadow-sm"
                />
                <div className="flex h-11 w-11 items-center justify-center rounded-full border-4 border-white bg-[#f3f4f6] text-sm font-semibold text-[#6b7280] shadow-sm">
                  +2k
                </div>
              </div>
              <div className="flex flex-col flex-wrap items-center justify-center gap-1 sm:items-start">
                <div className="flex gap-1">
                  {Array.from({ length: 5 }).map((_, index) => (
                    <Star
                      key={`star-${index}`}
                      className="h-4 w-4 fill-[var(--fc-star)] text-[var(--fc-star)]"
                      aria-hidden="true"
                    />
                  ))}
                </div>
                <p className="text-center text-[14px] font-normal leading-[16.8px] tracking-normal text-[#6B7280]">
                  Join 2,400+ designers & builders
                </p>
              </div>
            </div>
          </div>
        </section>

        <section className="border-y border-[#00000014] bg-white">
          <div className="mx-auto flex w-full max-w-6xl flex-col gap-6 px-6 py-10 text-center">
            <p className="text-center align-middle text-[13px] font-normal uppercase leading-[20.8px] tracking-[0.65px] text-[#94A3B8]">
              TRUSTED BY FORWARD-THINKING FARMS
            </p>
            <div className="flex flex-col items-center justify-center gap-4 text-lg font-semibold text-[#7a8494] sm:flex-row sm:flex-wrap sm:gap-12">
              {[
                { name: "AGRIFUTURE", src: "/Margin.png" },
                { name: "GREENFIELD", src: "/Margin (1).png" },
                { name: "NATUREYIELD", src: "/Margin (2).png" },
                { name: "SOLARHARVEST", src: "/Margin (3).png" },
              ].map((brand) => (
                <span key={brand.name} className="flex items-center">
                  <Image
                    src={brand.src}
                    alt={`${brand.name} icon`}
                    width={28}
                    height={20}
                    className="h-[20px] w-[28px]"
                    style={{
                      filter:
                        "brightness(0) saturate(100%) invert(8%) sepia(14%) saturate(1200%) hue-rotate(187deg) brightness(94%) contrast(94%)",
                    }}
                  />
                  {brand.name}
                </span>
              ))}
            </div>
          </div>
        </section>

        <section className="bg-[#f8fafc]">
          <div className="mx-auto w-full max-w-7xl px-6 py-13 md:max-w-6xl lg:px-30 lg:py-25">
            <div className="mx-auto max-w-2xl text-center">
              <p className="text-[13px] font-semibold uppercase tracking-[0.65px] text-[var(--fc-brand)]">
                Why FarmCredit
              </p>
              <h2 className="mt-4 text-center align-middle text-[36px] font-semibold leading-[36px] tracking-[-0.72px] text-[#0F1723]">
                Built for the field, <br /> powered by data.
              </h2>
              <p className="mt-6 align-middle text-center text-base text-[#94A3B8]">
                We understand that farming isn&apos;t 9-to-5. Our platform is
                designed to move as fast as your season demands.
              </p>
            </div>

            <div className="mt-12 grid gap-6 md:grid-cols-3">
              {[
                {
                  title: "Instant Approvals",
                  body: "Connect your farm management software and get pre-approved for capital in minutes, not weeks.",
                  icon: <Zap className="h-5 w-5 text-[var(--fc-brand)]" />,
                },
                {
                  title: "Risk-Adjusted Rates",
                  body: "We use satellite imagery and yield history to offer fair rates that reflect your actual performance.",
                  icon: (
                    <LineChart className="h-5 w-5 text-[var(--fc-brand)]" />
                  ),
                },
                {
                  title: "Flexible Repayment",
                  body: "Align repayment schedules with your harvest cycles. Pay when you get paid.",
                  icon: (
                    <BadgeCheck className="h-5 w-5 text-[var(--fc-brand)]" />
                  ),
                },
              ].map((item) => (
                <div
                  key={item.title}
                  className="h-[247.78px] w-full max-w-[368px] rounded-lg border border-[#00000014] bg-white p-6 shadow-sm"
                >
                  <div className="flex h-11 w-11 items-center justify-center rounded-xl bg-[var(--fc-brand-soft)]">
                    {item.icon}
                  </div>
                  <h3 className="mt-4 text-lg font-semibold text-[#0F1723]">
                    {item.title}
                  </h3>
                  <p className="mt-2 text-base font-normal leading-[25.6px] text-[#94A3B8] md:text-sm">
                    {item.body}
                  </p>
                </div>
              ))}
            </div>
          </div>
        </section>

        <section className="bg-white">
          <div className="mx-auto w-full max-w-7xl px-6 py-13 md:max-w-6xl lg:px-30 lg:py-25">
            <div className="mx-auto max-w-2xl text-center">
              <p className="text-[13px] font-semibold align-middle text-center uppercase tracking-[0.65px] text-[var(--fc-brand)]">
                How it works
              </p>
              <h2 className="mt-4 text-4xl tracking-[-0.72px] text-center align-middle font-semibold text-[#0F1723] sm:text-4xl">
                Three steps to capital.
              </h2>
              <p className="mt-4 text-base text-center align-middle leading-[25.6px] font-normal text-[#94A3B8]">
                Simple, transparent, and completely digital. No paperwork
                required.
              </p>
            </div>

            <div className="mt-14 grid gap-10 md:grid-cols-3">
              {[
                {
                  title: "Create your profile",
                  body: "Sign up and securely connect your existing farm management accounts or bank feeds.",
                },
                {
                  title: "Review offers",
                  body: "Receive tailored credit offers based on your real-time operational data.",
                },
                {
                  title: "Get funded",
                  body: "Select your terms and receive funds directly in your account within 24 hours.",
                },
              ].map((item) => (
                <div key={item.title} className="text-left">
                  <h3 className="text-[20px] font-semibold text-[#0F1723]">
                    {item.title}
                  </h3>
                  <p className="mt-2 text-base leading-6 text-[#94a3b8]">
                    {item.body}
                  </p>
                </div>
              ))}
            </div>
          </div>
        </section>
      </main>

      <footer className="border-t border-(--fc-border) bg-white">
        <div className="mx-auto w-full max-w-6xl px-6 py-13 lg:pt-[80px] lg:pb-[40px] lg:px-30">
          <div className="grid gap-10 md:grid-cols-[1.4fr_1fr_1fr_0.8fr]">
            <div className="space-y-4">
              <div className="flex items-center gap-2">
                <div className="flex h-7 w-7 items-center justify-center rounded-md bg-[var(--fc-brand)]">
                  <svg
                    width="16"
                    height="16"
                    viewBox="0 0 14 14"
                    fill="none"
                    xmlns="http://www.w3.org/2000/svg"
                    aria-hidden="true"
                  >
                    <path
                      d="M7 0L9 5H14L10 8.5L11.5 14L7 10.5L2.5 14L4 8.5L0 5H5L7 0Z"
                      fill="white"
                    />
                  </svg>
                </div>
                <span className="text-lg font-semibold">FarmCredit</span>
              </div>
              <p className="max-w-xs text-sm text-[#94a3b8]">
                Empowering the next generation of agriculture with smarter
                financial tools.
              </p>
            </div>
            <div className="space-y-3 text-sm">
              <p className="font-semibold text-[#0F1723]">Product</p>
              <ul className="space-y-2 text-[#94a3b8]">
                <li>
                  <a href="#" className="transition hover:text-foreground">
                    Features
                  </a>
                </li>
                <li>
                  <a href="#" className="transition hover:text-foreground">
                    Security
                  </a>
                </li>
                <li>
                  <a href="#" className="transition hover:text-foreground">
                    Pricing
                  </a>
                </li>
              </ul>
            </div>
            <div className="space-y-3 text-sm">
              <p className="font-semibold text-[#0F1723]">Company</p>
              <ul className="space-y-2 text-[#94a3b8]">
                <li>
                  <a href="#" className="transition hover:text-foreground">
                    About Us
                  </a>
                </li>
                <li>
                  <a href="#" className="transition hover:text-foreground">
                    Careers
                  </a>
                </li>
                <li>
                  <a href="#" className="transition hover:text-foreground">
                    Press
                  </a>
                </li>
              </ul>
            </div>
            <div className="space-y-3 text-sm">
              <p className="font-semibold text-[#0F1723]">Legal</p>
              <ul className="space-y-2 text-[#94a3b8]">
                <li>
                  <a href="#" className="transition hover:text-foreground">
                    Privacy
                  </a>
                </li>
                <li>
                  <a href="#" className="transition hover:text-foreground">
                    Terms
                  </a>
                </li>
              </ul>
            </div>
          </div>
          <div className="mt-10 align-middle text-center border-t border-[var(--fc-border)] pt-6 text-xs text-[#94a3b8]">
            © 2026 FarmCredit Inc. All rights reserved.
          </div>
        </div>
      </footer>
    </div>
  );
}
