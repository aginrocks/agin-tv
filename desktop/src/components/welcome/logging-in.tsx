import { Button } from "@components/ui/button";
import { IconCancel, IconRepeat } from "@tabler/icons-react";

interface LoggingInComponentProps {
  onRepeat: () => void;
  onCancel: () => void;
}

export default function LoggingInComponent({
  onRepeat,
  onCancel,
}: LoggingInComponentProps) {
  return (
    <>
      <div className="font-semibold text-2xl mb-2">Logging in...</div>
      <div className="text-sm text-muted-foreground w-2xl text-center">
        Please continue the login process in your browser.
      </div>
      <div className="flex items-center justify-center gap-4">
        <Button className="mt-4" onClick={onRepeat}>
          Reopen Browser
          <IconRepeat />
        </Button>
        <Button className="mt-4" onClick={onCancel}>
          Cancel
          <IconCancel />
        </Button>
      </div>
    </>
  );
}
