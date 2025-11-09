import * as DropdownMenu from "@radix-ui/react-dropdown-menu";
import { Link } from "@tanstack/react-router";
import type React from "react";

export function Dropdown(props: DropdownMenu.DropdownMenuProps) {
  return <DropdownMenu.Root {...props} />;
}

export function DropdownButton(
  props: React.ComponentProps<typeof DropdownMenu.Trigger>
) {
  return <DropdownMenu.Trigger {...props} />;
}

export function DropdownMenuComponent({
  children,
  ...props
}: DropdownMenu.DropdownMenuContentProps) {
  return (
    <DropdownMenu.Portal>
      <DropdownMenu.Content
        className="min-w-38 rounded-lg bg-white/75 p-0.5 shadow-sm outline outline-gray-950/5 backdrop-blur-sm dark:bg-gray-950/75 dark:outline-white/10 z-50"
        sideOffset={4}
        {...props}
      >
        {children}
      </DropdownMenu.Content>
    </DropdownMenu.Portal>
  );
}

// Export as DropdownMenu to match Navbar import
export { DropdownMenuComponent as DropdownMenu };

export function DropdownItem({
  to,
  children,
}: {
  to: string;
  children: React.ReactNode;
}) {
  return (
    <DropdownMenu.Item asChild>
      <Link
        to={to}
        className="block rounded-md px-3 py-0.5 text-sm/7 text-gray-950 focus:outline-none data-[highlighted]:bg-blue-500 data-[highlighted]:text-white dark:text-white cursor-pointer"
      >
        {children}
      </Link>
    </DropdownMenu.Item>
  );
}
