import { clsx } from "clsx";
import { Link } from "@tanstack/react-router";
import type React from "react";
import { useState } from "react";
import * as Dialog from "@radix-ui/react-dialog";
import { IconButton } from "../ui/icon-button";
import { Dropdown, DropdownButton, DropdownItem, DropdownMenu } from "../ui/dropdown";
import { MenuIcon } from "../icons/menu-icon";
import { CloseIcon } from "../icons/close-icon";
import { ChevronDownIcon } from "../icons/chevron-down-icon";

export function Navbar({ children, ...props }: React.ComponentProps<"div">) {
  return (
    <div
      className={clsx(
        "sticky top-0 z-10 bg-white/90 backdrop-blur-sm dark:bg-gray-950/90",
        "flex items-center justify-between gap-x-8 px-4 py-4 sm:px-6",
      )}
      {...props}
    >
      {children}
      <SiteNavigation />
    </div>
  );
}

function MobileNavigation({
  open,
  onClose,
}: {
  open: boolean;
  onClose: () => void;
}) {
  return (
    <Dialog.Root open={open} onOpenChange={onClose}>
      <Dialog.Portal>
        <Dialog.Overlay className="fixed inset-0 bg-gray-950/25 lg:hidden" />
        <Dialog.Content
          className={clsx(
            "fixed inset-0 flex justify-end pl-11 lg:hidden",
            "focus:outline-none"
          )}
        >
          <div className="w-full max-w-2xs bg-white px-4 py-5 ring ring-gray-950/10 sm:px-6 dark:bg-gray-950 dark:ring-white/10">
            <div className="flex justify-end">
              <Dialog.Close asChild>
                <IconButton onClick={onClose}>
                  <CloseIcon className="stroke-gray-950 dark:stroke-white" />
                </IconButton>
              </Dialog.Close>
            </div>
            <div className="mt-4">
              <div className="flex flex-col gap-y-2">
                {[
                  ["Dashboard", "/dashboard"],
                  ["Profile", "/profile"],
                ].map(([title, href]) => (
                  <Dialog.Close key={href} asChild>
                    <Link
                      to={href}
                      className="block rounded-md px-4 py-1.5 text-lg/7 font-medium tracking-tight text-gray-950 hover:bg-gray-950/5 dark:text-white dark:hover:bg-white/5"
                    >
                      {title}
                    </Link>
                  </Dialog.Close>
                ))}
              </div>
              <div className="mt-6 flex flex-col gap-y-2">
                <h3 className="px-4 py-1 text-sm/7 text-gray-500">Account</h3>
                {[
                  ["Settings", "/settings"],
                  ["Support", "/support"],
                  ["Sign out", "/login"],
                ].map(([title, href], index) => (
                  <Dialog.Close key={index} asChild>
                    <Link
                      to={href}
                      className="rounded-md px-4 py-1 text-sm/7 font-semibold text-gray-950 hover:bg-gray-950/5 dark:text-white dark:hover:bg-white/5"
                    >
                      {title}
                    </Link>
                  </Dialog.Close>
                ))}
              </div>
            </div>
          </div>
        </Dialog.Content>
      </Dialog.Portal>
    </Dialog.Root>
  );
}

function SiteNavigation() {
  const [mobileMenuOpen, setMobileMenuOpen] = useState(false);

  return (
    <nav className="flex items-center">
      <IconButton className="lg:hidden" onClick={() => setMobileMenuOpen(true)}>
        <MenuIcon className="fill-gray-950 dark:fill-white" />
      </IconButton>
      <MobileNavigation
        open={mobileMenuOpen}
        onClose={() => setMobileMenuOpen(false)}
      />
      <div className="flex gap-x-6 text-sm/6 text-gray-950 max-lg:hidden dark:text-white">
        <Link to="/dashboard">Dashboard</Link>
        <Link to="/profile">Profile</Link>
        <Dropdown>
          <DropdownButton className="inline-flex items-center gap-x-2 focus:not-data-focus:outline-none">
            Account
            <ChevronDownIcon className="stroke-gray-950 dark:stroke-white" />
          </DropdownButton>
          <DropdownMenu>
            <DropdownItem to="/settings">Settings</DropdownItem>
            <DropdownItem to="/support">Support</DropdownItem>
            <DropdownItem to="/login">Sign out</DropdownItem>
          </DropdownMenu>
        </Dropdown>
      </div>
    </nav>
  );
}
