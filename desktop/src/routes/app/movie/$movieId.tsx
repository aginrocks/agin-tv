import { createFileRoute } from '@tanstack/react-router';
import useApi from '@lib/providers/useApi';

export const Route = createFileRoute('/app/movie/$movieId')({
  component: MovieDesctiptionScreen,
});

function MovieDesctiptionScreen() {
  const { movieId } = Route.useParams();

  const api = useApi();

  const { data: movie_data } = api.useQuery('get', '/api/movies/{movie_id}', {
    params: {
      //@ts-expect-error
      path: { movie_id: movieId },
    },
  });

  return (
    <div className="aspect-[16/7.5]  w-full overflow-hidden relative">
      <div className="w-full h-full relative">
        {movie_data?.background_url && (
          <img className="w-full h-full object-cover" src={movie_data?.background_url} />
        )}
        <div className="absolute top-0 left-0 right-0 bottom-0 bg-gradient-to-t from-background to-transparent" />
        <div className="absolute left-0 right-0 bottom-0 flex justify-between items-end p-5 px-7">
          <div className="max-w-full flex md:max-w-1/2 flex-col items-start justify-end z-2 gap-3">
            <div className="text-5xl font-bold">{movie_data?.name}</div>
            <div className="text-lg text-muted-foreground line-clamp-5">
              {movie_data?.description}
            </div>
          </div>
        </div>
      </div>
    </div>
  );
}
