import { Button } from "@components/ui/button";
import { APP_NAME, APP_TAGLINE } from "@lib/constants/names";
import { IconArrowRight } from "@tabler/icons-react";

interface WelcomeComponentProps {
  onClick: () => void;
}

export default function WelcomeComponent({ onClick }: WelcomeComponentProps) {
  return (
    <>
      <div className="font-semibold text-2xl mb-2">{APP_NAME}</div>
      <div className="text-sm text-muted-foreground w-2xl text-center">
        {APP_TAGLINE}
      </div>

      <Button className="mt-4" onClick={onClick}>
        Get Started
        <IconArrowRight />
      </Button>
    </>
  );
}
