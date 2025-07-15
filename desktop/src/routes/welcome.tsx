import { SplashSection } from "@components/SplashScreen";
import { Button } from "@components/ui/button";
import { createFileRoute } from "@tanstack/react-router";
import { IconArrowRight } from "@tabler/icons-react";
import { APP_NAME, APP_TAGLINE } from "@lib/constants/names";

export const Route = createFileRoute("/welcome")({
  component: RouteComponent,
});

function RouteComponent() {
  //   const modals = useModals();
  return (
    <SplashSection>
      <div className="font-semibold text-2xl mb-2">{APP_NAME}</div>
      <div className="text-sm text-muted-foreground w-2xl text-center">
        {APP_TAGLINE}
      </div>

      <Button className="mt-4">
        Get Started
        <IconArrowRight />
      </Button>
    </SplashSection>
  );
}
