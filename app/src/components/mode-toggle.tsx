import { Moon, Sun, Monitor } from "lucide-react"
import { useTheme } from "next-themes"
import * as DropdownMenu from "@radix-ui/react-dropdown-menu"

export function ModeToggle() {
    const { theme, setTheme } = useTheme()

    return (
        <DropdownMenu.Root>
            <DropdownMenu.Trigger className="inline-flex items-center justify-center rounded-md p-2 text-sm font-medium relative before:absolute before:top-1/2 before:left-1/2 before:size-8 before:-translate-x-1/2 before:-translate-y-1/2 before:rounded-md before:bg-background before:backdrop-blur-sm hover:before:bg-foreground/5 focus:outline-none focus-visible:before:outline-2 focus-visible:before:outline-ring">
                <span className="relative flex items-center justify-center text-foreground">
                    <Sun className="h-5 w-5 rotate-0 scale-100 transition-all dark:-rotate-90 dark:scale-0" />
                    <Moon className="absolute h-5 w-5 rotate-90 scale-0 transition-all dark:rotate-0 dark:scale-100" />
                </span>
                <span className="sr-only">Toggle theme</span>
            </DropdownMenu.Trigger>
            <DropdownMenu.Portal>
                <DropdownMenu.Content
                    className="min-w-38 rounded-lg bg-popover p-0.5 shadow-sm border border-border backdrop-blur-sm z-50"
                    sideOffset={4}
                >
                    <DropdownMenu.Item
                        onClick={() => setTheme("light")}
                        className="flex items-center gap-2 rounded-md px-3 py-2 text-sm text-foreground focus:outline-none data-[highlighted]:bg-accent data-[highlighted]:text-accent-foreground cursor-pointer"
                    >
                        <Sun className="h-4 w-4" />
                        <span>Light</span>
                        {theme === "light" && <span className="ml-auto text-xs">✓</span>}
                    </DropdownMenu.Item>
                    <DropdownMenu.Item
                        onClick={() => setTheme("dark")}
                        className="flex items-center gap-2 rounded-md px-3 py-2 text-sm text-foreground focus:outline-none data-[highlighted]:bg-accent data-[highlighted]:text-accent-foreground cursor-pointer"
                    >
                        <Moon className="h-4 w-4" />
                        <span>Dark</span>
                        {theme === "dark" && <span className="ml-auto text-xs">✓</span>}
                    </DropdownMenu.Item>
                    <DropdownMenu.Item
                        onClick={() => setTheme("system")}
                        className="flex items-center gap-2 rounded-md px-3 py-2 text-sm text-foreground focus:outline-none data-[highlighted]:bg-accent data-[highlighted]:text-accent-foreground cursor-pointer"
                    >
                        <Monitor className="h-4 w-4" />
                        <span>System</span>
                        {theme === "system" && <span className="ml-auto text-xs">✓</span>}
                    </DropdownMenu.Item>
                </DropdownMenu.Content>
            </DropdownMenu.Portal>
        </DropdownMenu.Root>
    )
}
