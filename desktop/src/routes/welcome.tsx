import { SplashSection } from "@components/SplashScreen";
import { createFileRoute, useNavigate } from "@tanstack/react-router";
import { invoke } from "@tauri-apps/api/core";
import { AnimatePresence, motion } from "motion/react";
import { useState } from "react";
import WelcomeComponent from "@components/welcome";
import LoggingInComponent from "@components/welcome/logging-in";

export const Route = createFileRoute("/welcome")({
  component: RouteComponent,
});

function RouteComponent() {
  const [stage, setStage] = useState<"welcome" | "logging">("welcome");

  const navigate = useNavigate();

  async function handleLogin() {
    try {
      const auth = await invoke<string>("authenticate");
      console.log(auth);
      navigate({ to: "/app/home" });
    } catch (error) {
      console.error("Authentication failed:", error);
    }
  }

  return (
    <SplashSection>
      <AnimatePresence mode="wait" initial={false}>
        <motion.div
          className="flex flex-col justify-center items-center gap-4"
          key={stage}
          initial={{ opacity: 0, x: 10 }}
          animate={{ opacity: 1, x: 0 }}
          exit={{ opacity: 0, x: -10 }}
        >
          {stage === "welcome" ? (
            <WelcomeComponent
              onClick={async () => {
                setStage("logging");
                await handleLogin();
              }}
            />
          ) : (
            <LoggingInComponent
              onRepeat={handleLogin}
              onCancel={async () => {
                setStage("welcome");
                await invoke("cancel_authentication");
              }}
            />
          )}
        </motion.div>
      </AnimatePresence>
    </SplashSection>
  );
}
