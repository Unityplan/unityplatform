import { clsx } from "clsx";
import { Link, type LinkProps } from "@tanstack/react-router";
import type React from "react";

export function Breadcrumbs(props: React.ComponentProps<"nav">) {
  return (
    <nav
      aria-label="Breadcrumb"
      className="flex items-center gap-x-2 text-sm/6"
      {...props}
    />
  );
}

export function BreadcrumbHome() {
  return (
    <Link to="/" className="min-w-0 shrink-0 text-gray-950 dark:text-white">
      UnityPlan
    </Link>
  );
}

export function Breadcrumb({
  to,
  children,
  className,
}: {
  to?: LinkProps["to"];
  children: React.ReactNode;
  className?: string;
}) {
  if (to) {
    return (
      <Link
        to={to}
        className={clsx(
          className,
          "min-w-0 truncate text-gray-950 dark:text-white",
        )}
      >
        {children}
      </Link>
    );
  }

  return (
    <span
      className={clsx(
        className,
        "min-w-0 truncate text-gray-950 last:text-gray-600 dark:last:text-gray-400",
      )}
    >
      {children}
    </span>
  );
}

export function BreadcrumbSeparator() {
  return (
    <svg
      viewBox="0 0 16 16"
      fill="none"
      aria-hidden="true"
      className="size-4 shrink-0 stroke-gray-950/30 dark:stroke-white/30"
    >
      <path d="M6 4L10 8L6 12" strokeLinecap="square" />
    </svg>
  );
}
