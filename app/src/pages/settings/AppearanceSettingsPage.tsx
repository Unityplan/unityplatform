import { useTheme } from "next-themes"
import { useRouter, Link } from "@tanstack/react-router"
import { Sun, Moon, Monitor, Palette, Home, PanelLeft, Settings as SettingsIcon, Maximize2 } from "lucide-react"
import { AppLayout } from "@/components/layouts/AppLayout"
import { Card } from "@/components/ui/card"
import { Label } from "@/components/ui/label"
import { RadioGroup, RadioGroupItem } from "@/components/ui/radio-group"
import { Switch } from "@/components/ui/switch"
import { Button } from "@/components/ui/button"
import {
    Breadcrumb,
    BreadcrumbItem,
    BreadcrumbLink,
    BreadcrumbList,
    BreadcrumbPage,
    BreadcrumbSeparator,
} from "@/components/ui/breadcrumb"
import { useState, useEffect } from "react"

export function AppearanceSettingsPage() {
    const router = useRouter()
    const { theme, setTheme } = useTheme()
    const [reducedMotion, setReducedMotion] = useState(false)
    const [wideContentView, setWideContentView] = useState(false)

    // Get sidebar state from cookie
    const [sidebarOpen, setSidebarOpen] = useState(() => {
        if (typeof document !== 'undefined') {
            const cookie = document.cookie
                .split('; ')
                .find(row => row.startsWith('sidebar_state='))
            return cookie ? cookie.split('=')[1] === 'true' : false
        }
        return false
    })

    // Load reduced motion and wide content view preferences
    useEffect(() => {
        const savedReducedMotion = localStorage.getItem("reducedMotion") === "true"
        const savedWideContentView = localStorage.getItem("wideContentView") === "true"
        setReducedMotion(savedReducedMotion)
        setWideContentView(savedWideContentView)

        // Apply reduced motion class to document
        if (savedReducedMotion) {
            document.documentElement.classList.add('reduce-motion')
        }
    }, [])

    const handleCompactModeChange = (checked: boolean) => {
        // Update sidebar state (inverse of compact mode)
        // Compact mode ON = Sidebar collapsed (closed)
        // Compact mode OFF = Sidebar expanded (open)
        const newSidebarState = !checked
        setSidebarOpen(newSidebarState)

        // Update cookie to persist sidebar state
        document.cookie = `sidebar_state=${newSidebarState}; path=/; max-age=${60 * 60 * 24 * 7}`

        // Trigger the sidebar button click instead of reloading
        const sidebarTrigger = document.querySelector('[data-sidebar="trigger"]') as HTMLButtonElement
        if (sidebarTrigger) {
            sidebarTrigger.click()
        }
    }

    const handleReducedMotionChange = (checked: boolean) => {
        setReducedMotion(checked)
        localStorage.setItem("reducedMotion", String(checked))

        // Apply or remove the reduce-motion class from document
        if (checked) {
            document.documentElement.classList.add('reduce-motion')
        } else {
            document.documentElement.classList.remove('reduce-motion')
        }
    }

    const handleWideContentViewChange = (checked: boolean) => {
        setWideContentView(checked)
        localStorage.setItem("wideContentView", String(checked))

        // Dispatch a custom event to notify AppLayout of the change
        window.dispatchEvent(new CustomEvent('wideContentViewChange', { detail: { value: checked } }))
    }

    return (
        <AppLayout
            breadcrumbs={
                <Breadcrumb>
                    <BreadcrumbList>
                        <BreadcrumbItem>
                            <BreadcrumbLink asChild>
                                <Link to="/"><Home className="size-4" /></Link>
                            </BreadcrumbLink>
                        </BreadcrumbItem>
                        <BreadcrumbSeparator />
                        <BreadcrumbItem>
                            <BreadcrumbLink asChild>
                                <Link to="/settings" className="flex items-center gap-1">
                                    <SettingsIcon className="size-4" /> Settings
                                </Link>
                            </BreadcrumbLink>
                        </BreadcrumbItem>
                        <BreadcrumbSeparator />
                        <BreadcrumbItem>
                            <BreadcrumbPage className="flex items-center gap-1">
                                <Palette className="size-4" />
                                Appearance
                            </BreadcrumbPage>
                        </BreadcrumbItem>
                    </BreadcrumbList>
                </Breadcrumb>
            }
        >
            <div className="space-y-6 py-6">
                {/* Page Header */}
                <div className="flex items-center justify-between">
                    <div className="space-y-2">
                        <h1 className="text-3xl font-bold tracking-tight">Appearance</h1>
                        <p className="text-muted-foreground">
                            Customize how Unity Platform looks and feels
                        </p>
                    </div>
                    <Button
                        onClick={() => router.history.back()}
                    >
                        Cancel
                    </Button>
                </div>

                {/* Theme Mode Selection */}
                <Card className="p-6">
                    <div className="space-y-4">
                        <div className="flex items-start gap-3">
                            <Monitor className="h-5 w-5 text-primary mt-0.5" />
                            <div className="flex-1">
                                <h3 className="font-semibold text-lg">Theme Mode</h3>
                                <p className="text-sm text-muted-foreground">
                                    Choose between light, dark, or system preference
                                </p>
                            </div>
                        </div>

                        <RadioGroup
                            value={theme}
                            onValueChange={setTheme}
                            className="grid gap-3 pt-2"
                        >
                            {/* Light Theme */}
                            <Label
                                htmlFor="theme-light"
                                className="flex items-center gap-4 rounded-lg border border-border p-4 cursor-pointer hover:bg-accent/50 transition-colors has-checked:border-primary has-checked:bg-accent"
                            >
                                <RadioGroupItem value="light" id="theme-light" />
                                <div className="flex items-center gap-3 flex-1">
                                    <div className="flex items-center justify-center w-10 h-10 rounded-md bg-background border border-border">
                                        <Sun className="h-5 w-5 text-foreground" />
                                    </div>
                                    <div>
                                        <div className="font-medium">Light</div>
                                        <div className="text-sm text-muted-foreground">
                                            Bright and clear interface
                                        </div>
                                    </div>
                                </div>
                                {theme === "light" && (
                                    <span className="text-primary font-medium">✓</span>
                                )}
                            </Label>

                            {/* Dark Theme */}
                            <Label
                                htmlFor="theme-dark"
                                className="flex items-center gap-4 rounded-lg border border-border p-4 cursor-pointer hover:bg-accent/50 transition-colors has-checked:border-primary has-checked:bg-accent"
                            >
                                <RadioGroupItem value="dark" id="theme-dark" />
                                <div className="flex items-center gap-3 flex-1">
                                    <div className="flex items-center justify-center w-10 h-10 rounded-md bg-slate-900 border border-slate-700">
                                        <Moon className="h-5 w-5 text-slate-100" />
                                    </div>
                                    <div>
                                        <div className="font-medium">Dark</div>
                                        <div className="text-sm text-muted-foreground">
                                            Easy on the eyes in low light
                                        </div>
                                    </div>
                                </div>
                                {theme === "dark" && (
                                    <span className="text-primary font-medium">✓</span>
                                )}
                            </Label>

                            {/* System Theme */}
                            <Label
                                htmlFor="theme-system"
                                className="flex items-center gap-4 rounded-lg border border-border p-4 cursor-pointer hover:bg-accent/50 transition-colors has-checked:border-primary has-checked:bg-accent"
                            >
                                <RadioGroupItem value="system" id="theme-system" />
                                <div className="flex items-center gap-3 flex-1">
                                    <div className="flex items-center justify-center w-10 h-10 rounded-md bg-linear-to-br from-background to-slate-900 border border-border">
                                        <Monitor className="h-5 w-5 text-foreground" />
                                    </div>
                                    <div>
                                        <div className="font-medium">System</div>
                                        <div className="text-sm text-muted-foreground">
                                            Matches your device settings
                                        </div>
                                    </div>
                                </div>
                                {theme === "system" && (
                                    <span className="text-primary font-medium">✓</span>
                                )}
                            </Label>
                        </RadioGroup>
                    </div>
                </Card>

                {/* Color Scheme */}
                <Card className="p-6">
                    <div className="space-y-4">
                        <div className="flex items-start gap-3">
                            <Palette className="h-5 w-5 text-primary mt-0.5" />
                            <div className="flex-1">
                                <h3 className="font-semibold text-lg">Color Scheme</h3>
                                <p className="text-sm text-muted-foreground">
                                    Choose or customize your color palette
                                </p>
                            </div>
                        </div>

                        <div className="grid gap-3 pt-2">
                            {/* Default Green Theme */}
                            <div className="flex items-center gap-4 rounded-lg border border-primary bg-accent/50 p-4">
                                <div className="flex items-center gap-3 flex-1">
                                    <div className="flex items-center gap-1">
                                        <div className="w-8 h-8 rounded-md bg-[hsl(154,26%,31%)] border border-border" />
                                        <div className="w-8 h-8 rounded-md bg-[hsl(79,40%,73%)] border border-border" />
                                        <div className="w-8 h-8 rounded-md bg-[hsl(66,57%,68%)] border border-border" />
                                    </div>
                                    <div>
                                        <div className="font-medium">Forest Green (Default)</div>
                                        <div className="text-sm text-muted-foreground">
                                            Nature-inspired earth tones
                                        </div>
                                    </div>
                                </div>
                                <span className="text-primary font-medium">✓</span>
                            </div>

                            {/* Coming Soon Schemes */}
                            <div className="flex items-center gap-4 rounded-lg border border-border p-4 opacity-60 cursor-not-allowed">
                                <div className="flex items-center gap-3 flex-1">
                                    <div className="flex items-center gap-1">
                                        <div className="w-8 h-8 rounded-md bg-blue-600 border border-border" />
                                        <div className="w-8 h-8 rounded-md bg-sky-400 border border-border" />
                                        <div className="w-8 h-8 rounded-md bg-cyan-300 border border-border" />
                                    </div>
                                    <div>
                                        <div className="font-medium">Ocean Blue</div>
                                        <div className="text-sm text-muted-foreground">
                                            Cool and calming blues
                                        </div>
                                    </div>
                                </div>
                                <span className="text-xs px-2 py-0.5 rounded-full bg-muted text-muted-foreground">
                                    Coming Soon
                                </span>
                            </div>

                            <div className="flex items-center gap-4 rounded-lg border border-border p-4 opacity-60 cursor-not-allowed">
                                <div className="flex items-center gap-3 flex-1">
                                    <div className="flex items-center gap-1">
                                        <div className="w-8 h-8 rounded-md bg-purple-600 border border-border" />
                                        <div className="w-8 h-8 rounded-md bg-violet-400 border border-border" />
                                        <div className="w-8 h-8 rounded-md bg-fuchsia-300 border border-border" />
                                    </div>
                                    <div>
                                        <div className="font-medium">Royal Purple</div>
                                        <div className="text-sm text-muted-foreground">
                                            Rich and vibrant purples
                                        </div>
                                    </div>
                                </div>
                                <span className="text-xs px-2 py-0.5 rounded-full bg-muted text-muted-foreground">
                                    Coming Soon
                                </span>
                            </div>

                            <div className="flex items-center gap-4 rounded-lg border border-border p-4 opacity-60 cursor-not-allowed">
                                <div className="flex items-center gap-3 flex-1">
                                    <div className="flex items-center gap-1">
                                        <div className="w-8 h-8 rounded-md bg-slate-700 border border-border" />
                                        <div className="w-8 h-8 rounded-md bg-slate-400 border border-border" />
                                        <div className="w-8 h-8 rounded-md bg-slate-300 border border-border" />
                                    </div>
                                    <div>
                                        <div className="font-medium">Neutral Gray</div>
                                        <div className="text-sm text-muted-foreground">
                                            Professional monochrome
                                        </div>
                                    </div>
                                </div>
                                <span className="text-xs px-2 py-0.5 rounded-full bg-muted text-muted-foreground">
                                    Coming Soon
                                </span>
                            </div>

                            {/* Custom Theme Builder */}
                            <div className="flex items-center gap-4 rounded-lg border-2 border-dashed border-border p-4 opacity-60 cursor-not-allowed">
                                <div className="flex items-center gap-3 flex-1">
                                    <div className="flex items-center justify-center w-10 h-10 rounded-md bg-muted">
                                        <Palette className="h-5 w-5 text-muted-foreground" />
                                    </div>
                                    <div>
                                        <div className="font-medium">Create Custom Theme</div>
                                        <div className="text-sm text-muted-foreground">
                                            Design your own color palette with live preview
                                        </div>
                                    </div>
                                </div>
                                <span className="text-xs px-2 py-0.5 rounded-full bg-muted text-muted-foreground">
                                    Coming Soon
                                </span>
                            </div>
                        </div>
                    </div>
                </Card>

                {/* Display Options */}
                <Card className="p-6">
                    <div className="space-y-6">
                        <div>
                            <h3 className="font-semibold text-lg">Display Options</h3>
                            <p className="text-sm text-muted-foreground">
                                Adjust how content is displayed
                            </p>
                        </div>

                        {/* Compact Mode */}
                        <div className="flex items-center justify-between py-2">
                            <div className="space-y-0.5 flex items-start gap-2">
                                <PanelLeft className="h-5 w-5 text-primary mt-0.5" />
                                <div>
                                    <Label htmlFor="compact-mode" className="text-base font-medium">
                                        Compact Mode
                                    </Label>
                                    <p className="text-sm text-muted-foreground">
                                        Collapse sidebar for a denser layout
                                    </p>
                                </div>
                            </div>
                            <Switch
                                id="compact-mode"
                                checked={!sidebarOpen}
                                onCheckedChange={handleCompactModeChange}
                            />
                        </div>

                        <div className="border-t" />

                        {/* Wide Content View */}
                        <div className="flex items-center justify-between py-2">
                            <div className="space-y-0.5 flex items-start gap-2">
                                <Maximize2 className="h-5 w-5 text-primary mt-0.5" />
                                <div>
                                    <Label htmlFor="wide-content-view" className="text-base font-medium">
                                        Wide Content View
                                    </Label>
                                    <p className="text-sm text-muted-foreground">
                                        Expand content to use full screen width
                                    </p>
                                </div>
                            </div>
                            <Switch
                                id="wide-content-view"
                                checked={wideContentView}
                                onCheckedChange={handleWideContentViewChange}
                            />
                        </div>

                        <div className="border-t" />

                        {/* Reduced Motion */}
                        <div className="flex items-center justify-between py-2">
                            <div className="space-y-0.5 flex items-start gap-2">
                                <div className="relative mt-0.5">
                                    <Sun className="h-5 w-5 text-primary animate-spin" style={{ animationDuration: '3s' }} />
                                </div>
                                <div>
                                    <Label htmlFor="reduced-motion" className="text-base font-medium">
                                        Reduced Motion
                                    </Label>
                                    <p className="text-sm text-muted-foreground">
                                        Minimize animations and transitions for better accessibility
                                    </p>
                                </div>
                            </div>
                            <Switch
                                id="reduced-motion"
                                checked={reducedMotion}
                                onCheckedChange={handleReducedMotionChange}
                            />
                        </div>
                    </div>
                </Card>

                {/* Coming Soon Section */}
                <Card className="p-6 border-dashed">
                    <div className="space-y-4">
                        <div>
                            <h3 className="font-semibold text-lg text-muted-foreground">
                                Coming Soon
                            </h3>
                            <p className="text-sm text-muted-foreground">
                                Additional appearance options we're working on
                            </p>
                        </div>

                        <ul className="space-y-2 text-sm text-muted-foreground">
                            <li className="flex items-center gap-2">
                                <div className="h-1.5 w-1.5 rounded-full bg-muted-foreground/50" />
                                Font size adjustment (Small, Medium, Large)
                            </li>
                            <li className="flex items-center gap-2">
                                <div className="h-1.5 w-1.5 rounded-full bg-muted-foreground/50" />
                                Color accent customization
                            </li>
                            <li className="flex items-center gap-2">
                                <div className="h-1.5 w-1.5 rounded-full bg-muted-foreground/50" />
                                High contrast mode
                            </li>
                            <li className="flex items-center gap-2">
                                <div className="h-1.5 w-1.5 rounded-full bg-muted-foreground/50" />
                                Custom background images
                            </li>
                        </ul>
                    </div>
                </Card>
            </div>
        </AppLayout>
    )
}
