import { apiUrl } from '@/config';
import { SplashScreen } from '@components/SplashScreen';
import { tokenAtom } from '@lib/atoms/token';
import { createFileRoute, useNavigate } from '@tanstack/react-router';
import { load } from '@tauri-apps/plugin-store';
import { useSetAtom } from 'jotai';
import { useEffect } from 'react';

export const Route = createFileRoute('/')({
  component: RouteComponent,
});

function RouteComponent() {
  const setToken = useSetAtom(tokenAtom);
  const navigate = useNavigate();
  useEffect(() => {
    (async () => {
      try {
        const store = await load('store.json', { autoSave: false });
        let token = await store.get<string>('token');
        if (token === undefined) {
          navigate({ to: '/welcome' });
          return;
        }

        console.log('Token loaded:', token);

        setToken(token);

        const response = await fetch(`${apiUrl}/auth/user`, {
          headers: {
            Authorization: `Bearer ${token}`,
          },
        });

        console.log(response);

        if (response.status != 200) {
          navigate({ to: '/welcome' });
          return;
        }

        navigate({ to: '/app/home' });
      } catch (error) {
        console.error('Error during authentication check:', error);
        navigate({ to: '/error' });
      }
    })();
  }, []);

  return <SplashScreen visible />;
}
