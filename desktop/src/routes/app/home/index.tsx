import Carousel from '@components/Carousel';
import useApi from '@lib/providers/useApi';
import { createFileRoute } from '@tanstack/react-router';

export const Route = createFileRoute('/app/home/')({
  component: RouteComponent,
});

function RouteComponent() {
  const api = useApi();

  const { data: home_data } = api.useQuery('get', '/api/home');

  return (
    <div>
      <Carousel slides={home_data?.carousel} />
    </div>
  );
}
