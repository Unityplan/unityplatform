import { createContext } from "react";

export const SidebarContext = createContext<{
  isSidebarOpen: boolean;
  setIsSidebarOpen: (isSidebarOpen: boolean) => void;
  isMobileDialogOpen: boolean;
  setIsMobileDialogOpen: (isMobileDialogOpen: boolean) => void;
}>({
  isSidebarOpen: true,
  setIsSidebarOpen: () => {},
  isMobileDialogOpen: false,
  setIsMobileDialogOpen: () => {},
});
