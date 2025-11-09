import type React from "react";

export function Logo(props: React.ComponentProps<"svg">) {
  return (
    <svg
      viewBox="0 0 200 50"
      fill="currentColor"
      className="h-8 w-auto"
      {...props}
    >
      {/* Placeholder logo - replace with actual UnityPlan logo */}
      <text
        x="10"
        y="35"
        fontSize="28"
        fontWeight="bold"
        fontFamily="system-ui, -apple-system, sans-serif"
      >
        UnityPlan
      </text>
    </svg>
  );
}
