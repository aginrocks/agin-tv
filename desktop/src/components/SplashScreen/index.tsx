import { cn } from "@lib/utils";
import { SplashSection } from "./SplashSection";
import { Spinner } from "@components/ui/spinner";
import clsx from "clsx";

export type SplashScreenProps = {
  children?: React.ReactNode;
  visible?: boolean;
};

export function SplashScreen({ visible }: SplashScreenProps) {
  return (
    <SplashSection
      className={cn(
        "fixed top-0 left-0 right-0 bottom-0 z-[99999] transition duration-500 ease-in-out",
        clsx({
          "opacity-0 visibility-hidden pointer-events-none": !visible,
        })
      )}
    >
      <Spinner className="mb-3" />
      <div className="font-semibold text-lg mb-0.5">Loading</div>
    </SplashSection>
  );
}

export * from "./SplashSection";
