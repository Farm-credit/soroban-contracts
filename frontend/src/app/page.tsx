"use client";

import { useState } from "react";
import { ArrowRight, Star } from "lucide-react";
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
          <div className="flex h-6 w-6 items-center justify-center rounded-md bg-(--fc-brand)">
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
        <nav className="hidden items-center gap-8 text-sm text-(--fc-ink-muted) md:flex">
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

      <main className="flex flex-1 items-center justify-center px-6 py-16">
        <div className="w-full max-w-2xl text-center">
          <div className="mb-8 inline-block">
            <span className="rounded-full bg-(--fc-brand-soft) px-4 py-2 text-sm font-medium text-[#0F1723]">
              Early Access Open
            </span>
          </div>

          <h1 className="text-center text-[64px] font-bold leading-[70.4px] tracking-[-2px] align-middle bg-linear-to-b from-[#0F1723] to-[#6B7280] bg-clip-text text-transparent md:text-[64px]">
            The future of work
            <br />
            is finally here.
          </h1>

          <p className="mx-auto mt-6 max-w-xl text-center ftext-[18px] font-normal leading-[28.8px] tracking-normal align-middle text-[#6B7280]">
            Farm Credit connects your entire workflow into one intelligent
            <br />
            hub. Join the waitlist to secure your username and get early access.
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
              className="h-12 min-h-12 flex-1 rounded-lg border border-(--fc-border) px-4 text-sm focus:border-transparent focus:outline-none focus:ring-2 focus:ring-(--fc-brand)"
              required
              aria-label="Email address"
            />
            <button
              type="submit"
              className="inline-flex h-12 items-center justify-center gap-2 rounded-lg bg-(--fc-brand) px-6 text-sm font-medium text-white transition hover:bg-(--fc-brand-dark)"
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
                    className="h-4 w-4 fill-(--fc-star) text-(--fc-star)"
                    aria-hidden="true"
                  />
                ))}
              </div>
              <p className="text-center text-[14px] font-normal leading-[16.8px] tracking-normal align-middle text-[#6B7280]">
                Join 2,400+ designers & builders
              </p>
            </div>
          </div>
        </div>
      </main>

      <footer className="mx-auto flex w-full max-w-7xl flex-col items-center justify-between gap-4 px-6 py-6 text-sm text-(--fc-ink-muted) md:flex-row">
        <p>© 2026 Farm Credit Inc. All rights reserved.</p>
        <nav className="flex items-center gap-6">
          <a href="#" className="transition hover:text-foreground">
            Privacy
          </a>
          <a href="#" className="transition hover:text-foreground">
            Terms
          </a>
          <a href="#" className="transition hover:text-foreground">
            Contact
          </a>
        </nav>
      </footer>
    </div>
  );
}
