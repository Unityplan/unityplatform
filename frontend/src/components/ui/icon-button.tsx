import { clsx } from "clsx";
import type React from "react";

export function IconButton({
  className,
  ...props
}: React.ComponentProps<"button">) {
  return (
    <button
      type="button"
      className={clsx(
        className,
        "relative *:relative",
        "before:absolute before:top-1/2 before:left-1/2 before:size-8 before:-translate-x-1/2 before:-translate-y-1/2 before:rounded-md",
        "before:bg-white/75 before:backdrop-blur-sm dark:before:bg-gray-950/75",
        "hover:before:bg-gray-950/5 dark:hover:before:bg-white/5",
        "focus:outline-none focus-visible:before:outline focus-visible:before:outline-2 focus-visible:before:outline-blue-700",
      )}
      {...props}
    />
  );
}
