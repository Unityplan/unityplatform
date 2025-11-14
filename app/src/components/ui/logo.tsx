import type React from "react";

export function Logo(props: React.ComponentProps<"svg">) {
    return (
        <svg
            viewBox="0 0 200 50"
            fill="currentColor"
            className="h-8 w-auto"
            {...props}
        >
            {/* Placeholder logo - replace with actual Unity Platform logo */}
            <text
                x="100"
                y="35"
                fontSize="28"
                fontWeight="bold"
                fontFamily="system-ui, -apple-system, sans-serif"
                textAnchor="middle"
            >
                Unity Platform
            </text>
        </svg>
    );
}
