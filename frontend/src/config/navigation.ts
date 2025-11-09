import type { NavigationSection } from "@/components/layouts/SidebarLayout";

export const dashboardNavigation: NavigationSection[] = [
  {
    id: "main",
    title: "Main",
    items: [
      {
        id: "dashboard",
        title: "Dashboard",
        path: "/dashboard",
      },
      {
        id: "profile",
        title: "Profile",
        path: "/profile",
      },
    ],
  },
  {
    id: "settings",
    title: "Settings",
    items: [
      {
        id: "edit-profile",
        title: "Edit Profile",
        path: "/profile/edit",
      },
    ],
  },
];
