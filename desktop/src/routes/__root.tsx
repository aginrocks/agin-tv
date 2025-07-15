import { createRootRoute, Outlet } from "@tanstack/react-router";
import "@/index.css";
import { ThemeProvider } from "@components/theme-provider";
import { Toaster } from "@components/ui/sonner";
// import { TanStackRouterDevtools } from '@tanstack/react-router-devtools';

export const Route = createRootRoute({
  component: () => {
    return (
      <ThemeProvider defaultTheme="dark" storageKey="theme">
        <Outlet />
        <Toaster />
      </ThemeProvider>
    );
  },
});
