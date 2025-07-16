import { SplashSection } from "@components/SplashScreen";
import { Button } from "@components/ui/button";
import { createFileRoute } from "@tanstack/react-router";
import { IconArrowRight } from "@tabler/icons-react";
import { APP_NAME, APP_TAGLINE } from "@lib/constants/names";
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
                await invoke("authenticate");
              }}
            />
          ) : (
            <LoggingInComponent
              onRepeat={async () => {
                await invoke("authenticate");
              }}
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
