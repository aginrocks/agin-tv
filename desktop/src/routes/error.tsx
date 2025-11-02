import { SplashSection } from '@components/SplashScreen';
import { Button } from '@components/ui/button';
import { IconLogin } from '@tabler/icons-react';
import { createFileRoute, useNavigate } from '@tanstack/react-router';

export const Route = createFileRoute('/error')({
  component: ErrorPage,
});

function ErrorPage() {
  const navigate = useNavigate();

  return (
    <SplashSection>
      <div className="font-semibold text-2xl mb-2">Error occured</div>
      <div className="text-sm text-muted-foreground w-2xl text-center">
        An error occured during the authentication process. Please try again.
      </div>
      <div className="flex items-center justify-center gap-4">
        <Button
          className="mt-4"
          onClick={() => {
            navigate({ to: '/welcome' });
          }}
        >
          Log in again
          <IconLogin />
        </Button>
      </div>
    </SplashSection>
  );
}
