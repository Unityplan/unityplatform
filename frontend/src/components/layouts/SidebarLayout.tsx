import { clsx } from "clsx";
import { Link, useRouterState } from "@tanstack/react-router";
import type React from "react";
import { useContext, useState } from "react";
import * as Dialog from "@radix-ui/react-dialog";
import { IconButton } from "../ui/icon-button";
import { SidebarIcon } from "../icons/sidebar-icon";
import { Navbar } from "./Navbar";
import { SidebarContext } from "../../contexts/SidebarContext";

export interface NavigationSection {
  id: string;
  title: string;
  items: NavigationItem[];
}

export interface NavigationItem {
  id: string;
  title: string;
  path: string;
}

function SidebarNavigation({
  sections,
  onNavigate,
  className,
}: {
  sections: NavigationSection[];
  onNavigate?: () => void;
  className?: string;
}) {
  const router = useRouterState();
  const pathname = router.location.pathname;

  return (
    <div className={clsx(className, "space-y-8")}>
      {sections.map((section) => (
        <div key={section.id}>
          <h2 className="text-base/7 font-semibold text-pretty text-gray-950 sm:text-sm/6 dark:text-white">
            {section.title}
          </h2>
          <ul className="mt-4 flex flex-col gap-4 border-l border-gray-950/10 text-base/7 text-gray-700 sm:mt-3 sm:gap-3 sm:text-sm/6 dark:border-white/10 dark:text-gray-400">
            {section.items.map((item) => (
              <li
                key={item.id}
                className={clsx(
                  "-ml-px flex border-l border-transparent pl-4",
                  "hover:text-gray-950 hover:not-has-aria-[current=page]:border-gray-400 dark:hover:text-white",
                  "has-aria-[current=page]:border-gray-950 dark:has-aria-[current=page]:border-white",
                )}
              >
                <Link
                  to={item.path}
                  aria-current={item.path === pathname ? "page" : undefined}
                  onClick={onNavigate}
                  className="aria-[current=page]:font-medium aria-[current=page]:text-gray-950 dark:aria-[current=page]:text-white"
                >
                  {item.title}
                </Link>
              </li>
            ))}
          </ul>
        </div>
      ))}
    </div>
  );
}

function MobileNavigation({
  open,
  onClose,
  sections,
}: {
  open: boolean;
  onClose: () => void;
  sections: NavigationSection[];
}) {
  return (
    <Dialog.Root open={open} onOpenChange={onClose}>
      <Dialog.Portal>
        <Dialog.Overlay className="fixed inset-0 bg-gray-950/25 xl:hidden" />
        <Dialog.Content
          className={clsx(
            "fixed inset-y-0 left-0 isolate w-sm max-w-[calc(100%-var(--spacing-11))] overflow-y-auto bg-white ring ring-gray-950/10 sm:w-xs dark:bg-gray-950 dark:ring-white/10 xl:hidden",
            "focus:outline-none"
          )}
        >
          <div className="sticky top-0 z-10 px-4 py-4 sm:px-6">
            <div className="flex h-6 shrink-0">
              <Dialog.Close asChild>
                <IconButton>
                  <SidebarIcon className="shrink-0 stroke-gray-950 dark:stroke-white" />
                </IconButton>
              </Dialog.Close>
            </div>
          </div>
          <SidebarNavigation
            sections={sections}
            onNavigate={onClose}
            className="px-4 pb-4 sm:px-6"
          />
        </Dialog.Content>
      </Dialog.Portal>
    </Dialog.Root>
  );
}

export function SidebarLayout({
  sections,
  children,
}: {
  sections: NavigationSection[];
  children: React.ReactNode;
}) {
  const [isSidebarOpen, setIsSidebarOpen] = useState(true);
  const [isMobileDialogOpen, setIsMobileDialogOpen] = useState(false);

  return (
    <SidebarContext.Provider
      value={{
        isSidebarOpen,
        setIsSidebarOpen,
        isMobileDialogOpen,
        setIsMobileDialogOpen,
      }}
    >
      <div
        data-sidebar-collapsed={isSidebarOpen ? undefined : ""}
        className="group"
      >
        <aside className="fixed inset-y-0 left-0 w-2xs overflow-y-auto border-r border-gray-950/10 group-data-sidebar-collapsed:hidden max-xl:hidden dark:border-white/10">
          <nav aria-label="Navigation" className="px-6 py-4">
            <div className="sticky top-4 flex h-6">
              <IconButton onClick={() => setIsSidebarOpen(!isSidebarOpen)}>
                <SidebarIcon className="shrink-0 stroke-gray-950 dark:stroke-white" />
              </IconButton>
              <MobileNavigation
                open={isMobileDialogOpen}
                onClose={() => setIsMobileDialogOpen(false)}
                sections={sections}
              />
            </div>
            <div className="mt-3">
              <SidebarNavigation sections={sections} className="max-xl:hidden" />
            </div>
          </nav>
        </aside>
        <div className="xl:not-group-data-sidebar-collapsed:ml-(--container-2xs)">
          {children}
        </div>
      </div>
    </SidebarContext.Provider>
  );
}

export function SidebarLayoutContent({
  breadcrumbs,
  children,
}: {
  breadcrumbs: React.ReactNode;
  children: React.ReactNode;
}) {
  const {
    isSidebarOpen,
    setIsSidebarOpen,
    isMobileDialogOpen,
    setIsMobileDialogOpen,
  } = useContext(SidebarContext);

  return (
    <>
      <Navbar>
        <div className="flex min-w-0 shrink items-center gap-x-4">
          <IconButton
            onClick={() => setIsMobileDialogOpen(!isMobileDialogOpen)}
            className="xl:hidden"
          >
            <SidebarIcon className="shrink-0 stroke-gray-950 dark:stroke-white" />
          </IconButton>
          {!isSidebarOpen && (
            <IconButton
              onClick={() => setIsSidebarOpen(!isSidebarOpen)}
              className="max-xl:hidden"
            >
              <SidebarIcon className="shrink-0 stroke-gray-950 dark:stroke-white" />
            </IconButton>
          )}
          <div className="min-w-0">{breadcrumbs}</div>
        </div>
      </Navbar>
      <main className="px-4 sm:px-6">{children}</main>
    </>
  );
}
